# Tech Docs: Add `pdf-chat-*` Demo App Family

## Reading this document

This document is the architecture, schemas, and code shapes for the `pdf-chat-*`
demo family. It uses standard AI-application vocabulary (RAG, embeddings, vector
DB, SSE streaming, tokens, guardrails) without re-defining each term inline.

**Recommended path** for engineers without prior AI-app exposure:

1. Read the repo-wide primer first:
   [AI Application Development](../../../docs/explanation/software-engineering/ai-application-development/README.md).
   It is the canonical, generic explainer; the rest of this document assumes its
   vocabulary.
2. Then read this document top-to-bottom.

**Skipping the primer?** The mini-glossary below is enough to parse this file,
but the primer is still required reading before executing the plan.

### Mini-glossary (the absolute minimum)

| Term            | Plain definition                                                                                                                           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| LLM             | Large language model. Stateless text generator we call over HTTP. Same input → not necessarily same output.                                |
| Token           | Sub-word unit. ~4 chars / ~0.75 English words. LLMs are billed and budgeted per token (input + output).                                    |
| Context window  | Max tokens per LLM call (input + output combined). Claude Haiku 4.5 = 200k.                                                                |
| Embedding       | Fixed-size numeric vector (1536 floats here) encoding meaning of a piece of text. Used for similarity search.                              |
| Cosine distance | A number in [0, 2] measuring how different two vectors are. Smaller = more similar. pgvector operator: `<=>`.                              |
| Vector DB       | Storage that supports fast top-k similarity search. We use Postgres + the pgvector extension.                                              |
| Chunk           | A bounded slice of a document (~800 tokens here), embedded as one vector and stored as one row.                                            |
| RAG             | Retrieval-Augmented Generation. Find relevant chunks, paste them into the prompt, ask the LLM to answer.                                   |
| top-k           | The number of best-matching chunks to retrieve (we use k=4).                                                                               |
| SSE             | Server-Sent Events. One-way HTTP streaming with `text/event-stream`; the LLM emits one frame per token.                                    |
| Session         | Server-persisted chat thread (rows in `sessions` and `messages`). Lets the conversation survive reload.                                    |
| OpenRouter      | Multi-provider LLM proxy. One API ([https://openrouter.ai/api/v1](https://openrouter.ai/api/v1)) routes to Anthropic, Google, OpenAI, etc. |
| Guardrail       | Pre-/post-LLM defensive layer. Three here: rate limit, content filter, token cost cap.                                                     |
| Mocked LLM      | Test mode (`MOCK_OPENROUTER=true`) that replaces the network call with a fixture so CI is offline + free.                                  |

When this document writes "the LLM", it means whichever upstream model OpenRouter
routes to (default `anthropic/claude-haiku-4.5`). When it writes "the model id",
it means the OpenRouter-format string `provider/model-version`.

## Architecture summary

The diagram below shows two flows: PDF **ingestion** (top half) and chat
**conversation** (bottom half).

- **Ingest**: the user uploads a PDF; the backend extracts text, slices it into
  chunks, asks OpenRouter to convert each chunk to an embedding (a numeric
  fingerprint of its meaning), and stores text + embeddings in Postgres.
- **Conversation**: the user creates a chat session and attaches one or more
  PDFs to it; for each user message, the backend embeds the question, runs a
  similarity query against the stored chunk embeddings to find the most
  relevant excerpts, glues them together with prior chat history into a
  prompt, and asks OpenRouter to stream an answer back token-by-token over SSE.
  The frontend renders tokens as they arrive.

This is the standard "chat with my documents" pattern (RAG). If any sentence
above feels novel, read the
[AI primer](../../../docs/explanation/software-engineering/ai-application-development/README.md)
before continuing.

```mermaid
%% Color Palette: Blue #0173B2 | Orange #DE8F05 | Teal #029E73 | Purple #CC78BC | Gray #808080
sequenceDiagram
    autonumber
    participant U as User
    participant FE as pdf-chat-fe (Next.js 16)
    participant RH as Next.js Route Handler /api/chat
    participant BE as pdf-chat-be (FastAPI)
    participant PG as Postgres + pgvector
    participant OR as OpenRouter

    U->>FE: drag-drop PDF
    FE->>BE: POST /api/v1/pdfs (multipart)
    BE->>BE: pypdf extract → chunk
    BE->>OR: POST /api/v1/embeddings
    OR-->>BE: embeddings (1536-dim)
    BE->>PG: INSERT chunks + vectors
    BE-->>FE: 201 {pdfId, pages, chunks}
    FE->>U: navigate / (home — PDF now in library)

    U->>FE: create session(pdfIds, model)
    FE->>BE: POST /api/v1/sessions
    BE->>PG: INSERT session + session_pdfs
    BE-->>FE: Session

    U->>FE: type prompt
    FE->>RH: POST /api/chat (sessionId, content)
    RH->>BE: POST /api/v1/sessions/{id}/messages
    BE->>BE: rate limit + content filter (in) + cost cap check
    BE->>PG: INSERT user message
    BE->>OR: POST /api/v1/embeddings (query)
    OR-->>BE: query embedding
    BE->>PG: SELECT top-k by cosine across session_pdfs
    PG-->>BE: chunks (multi-PDF)
    BE->>PG: SELECT prior messages (history)
    PG-->>BE: prior messages
    BE->>OR: POST /api/v1/chat/completions (stream=true)
    OR-->>BE: SSE chunks
    BE->>BE: content filter (out) on accumulated tokens
    BE-->>RH: SSE stream
    RH-->>FE: SSE stream (passthrough)
    FE-->>U: incremental tokens
    BE->>PG: INSERT assistant message + UPSERT token_usage
```

## Stack decisions and rationale

| Concern                 | Choice                                                   | Why                                                                                                                                                                                                                                     |
| ----------------------- | -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Backend language        | Python 3.13                                              | Matches existing `crud-be-python-fastapi` conventions; richest LLM/RAG ecosystem                                                                                                                                                        |
| Backend framework       | FastAPI + uvicorn                                        | Established in repo; native async; `StreamingResponse`/`sse-starlette` ergonomic                                                                                                                                                        |
| PDF text extraction     | `pypdf` (BSD-3-Clause) primary                           | BSD-3-Clause is permissive and compatible with the MIT-licensed template; PyMuPDF is AGPL and is **explicitly forbidden**                                                                                                               |
| Optional table fallback | `pdfplumber` (MIT)                                       | Used only when a chunk is detected to be table-shaped; MIT license                                                                                                                                                                      |
| Chunking                | Naive token-window with overlap (800/100)                | Sufficient for a demo; explicitly out-of-scope for production-grade chunking                                                                                                                                                            |
| Embedding               | OpenRouter `/api/v1/embeddings`                          | Single egress; default `openai/text-embedding-3-small` (1536 dims)                                                                                                                                                                      |
| Vector DB               | pgvector on the existing Postgres image                  | Zero new infra; reuses CRUD's docker-compose pattern with a different image: `pgvector/pgvector:pg16`                                                                                                                                   |
| Chat completion         | OpenRouter `/api/v1/chat/completions` with `stream=true` | OpenAI-compatible, multi-provider routing in one place                                                                                                                                                                                  |
| Default chat model      | `anthropic/claude-haiku-4.5`                             | Verified routable via OpenRouter on 2026-04-26                                                                                                                                                                                          |
| Alternate chat model    | `google/gemini-2.5-flash-lite`                           | Closest Haiku-tier Gemini ($0.10/M in, $0.40/M out)                                                                                                                                                                                     |
| Streaming transport     | `sse-starlette.EventSourceResponse`                      | Idiomatic 2026; correct `event:`/`id:`/`retry:` field handling                                                                                                                                                                          |
| Frontend framework      | Next.js 16 App Router + TypeScript                       | Matches `crud-fe-ts-nextjs`; same toolchain, same lint, same vitest setup                                                                                                                                                               |
| Frontend UI base        | `@open-sharia-enterprise/ts-ui` (libs/ts-ui)             | **Mandatory** shared component library. Use `Button`, `Input`, `Card`, `Label`, `Dialog`, `Alert`, `cn`. No bespoke components in `pdf-chat-fe` for primitives that exist in `ts-ui`. Missing primitives must be added to `ts-ui` first |
| Frontend design tokens  | `@open-sharia-enterprise/ts-ui-tokens`                   | Imported once in `globals.css` (matches `crud-fe-ts-nextjs` pattern); semantic tokens only                                                                                                                                              |
| Frontend chat hook      | `@ai-sdk/react` `useChat` (ai SDK ^5)                    | Idiomatic 2026; proxy through Next.js Route Handler avoids custom `ChatTransport`                                                                                                                                                       |
| FE vector / state       | Plain React state, server-driven                         | No client-side persistence (matches non-goal)                                                                                                                                                                                           |
| E2E runner              | Playwright + playwright-bdd                              | Mirrors `crud-be-e2e` and `crud-fe-e2e`                                                                                                                                                                                                 |

## Repository layout (post-plan)

```
apps/
  pdf-chat-be/                      # Python/FastAPI backend
    src/pdf_chat_be/
      __init__.py
      main.py                       # FastAPI app
      config.py                     # pydantic-settings
      routers/
        pdfs.py                     # upload, list, delete
        sessions.py                 # CRUD sessions + POST messages (streams SSE)
        health.py
      services/
        pdf_extraction.py           # pypdf wrapper
        chunking.py                 # naive token chunker
        embedding.py                # OpenRouter embedding client
        vector_store.py             # pgvector queries (multi-PDF, session-scoped)
        rag.py                      # retrieve + prompt assemble (history-aware)
        openrouter_chat.py          # OpenRouter chat client w/ streaming
        sessions_service.py         # session lifecycle + message persistence
        rate_limiter.py             # slowapi-backed token bucket
        content_filter.py           # ContentFilter Protocol + RegexBlocklistFilter + Noop
        cost_cap.py                 # session + day token-budget enforcement
        token_counter.py            # tiktoken (in) + char-approx (out)
      infrastructure/
        repositories.py             # SQLAlchemy + asyncpg
        openrouter_client.py        # httpx wrapper
      auth/                         # (none — placeholder for future)
    tests/
      unit/
        steps/                      # pytest-bdd step defs (mocked)
        support/
      integration/
        steps/                      # pytest-bdd step defs (real DB, mocked OR)
        support/
      fixtures/
        sample.pdf
        openrouter.json             # cassette for MOCK_OPENROUTER
    alembic/                        # vector schema migration
    docker-compose.integration.yml  # pgvector image + test runner
    Dockerfile.integration
    pyproject.toml
    project.json
    .python-version
    README.md

  pdf-chat-fe/                      # Next.js 16 frontend (consumes libs/ts-ui)
    src/
      app/
        page.tsx                    # library — uses ts-ui Card, Button
        chat/[sessionId]/page.tsx   # chat — uses ts-ui Card, Input, Button
        api/chat/route.ts           # Route Handler proxy
        layout.tsx
        globals.css                 # imports @open-sharia-enterprise/ts-ui-tokens
      components/                   # PDF-chat-specific composites only
        UploadZone.tsx              # composes ts-ui Card + Button + Alert
        ChatTranscript.tsx          # composes ts-ui Card
        ChatComposer.tsx            # composes ts-ui Input + Button
        ModelSelector.tsx           # composes ts-ui Button + Label
      lib/
        api.ts                      # typed client over generated-contracts
        env.ts
      generated-contracts/          # codegen output (gitignored)
    test/
      unit/
        components/*.test.tsx
        steps/*                     # vitest-cucumber if used
    next.config.ts
    package.json
    project.json
    tsconfig.json
    vitest.config.ts
    README.md

  pdf-chat-be-e2e/                  # Playwright HTTP suite
    tests/
      steps/
      fixtures/
    .features-gen/
    package.json
    playwright.config.ts
    project.json
    README.md

  pdf-chat-fe-e2e/                  # Playwright UI suite
    tests/
      steps/
      fixtures/
    package.json
    playwright.config.ts
    project.json
    README.md

specs/apps/pdf-chat/
  README.md
  c4/
    context.md
    container.md
    component-be.md
    component-fe.md
    README.md
  be/
    README.md
    gherkin/
      README.md
      health/
        health-check.feature
      pdfs/
        upload.feature
        list.feature
        delete.feature
      chat/
        streaming.feature
        rag-retrieval.feature
        model-selection.feature
      test-support/
        test-api.feature
  fe/
    README.md
    gherkin/
      README.md
      library/
        library-list.feature
        delete-pdf.feature
      upload/
        drag-drop.feature
        validation.feature
      chat/
        chat-flow.feature
        model-toggle.feature
        streaming-display.feature
  contracts/
    openapi.yaml
    project.json
    README.md
    .spectral.yaml
    redocly.yaml
    paths/
      health.yaml
      pdfs.yaml
      chat.yaml
    schemas/
      pdf.yaml
      chat.yaml
      error.yaml
      health.yaml
    examples/
    generated/                      # gitignored

infra/dev/pdf-chat-be/
  docker-compose.dev.yml            # pgvector + dev backend, optional

.github/workflows/
  test-pdf-chat-be.yml
  test-pdf-chat-fe.yml
  test-pdf-chat-be-e2e.yml
  test-pdf-chat-fe-e2e.yml
```

## OpenAPI contract design

`specs/apps/pdf-chat/contracts/openapi.yaml` is OpenAPI 3.1 with the following
endpoints:

| Method | Path                                    | Tag      | Notes                                                   |
| ------ | --------------------------------------- | -------- | ------------------------------------------------------- |
| GET    | `/health`                               | Health   | Returns `HealthResponse`                                |
| POST   | `/api/v1/pdfs`                          | Pdfs     | `multipart/form-data` with `file` (binary)              |
| GET    | `/api/v1/pdfs`                          | Pdfs     | Returns `PdfListResponse`                               |
| DELETE | `/api/v1/pdfs/{pdfId}`                  | Pdfs     | Returns `204`                                           |
| POST   | `/api/v1/sessions`                      | Sessions | Body `CreateSessionRequest`; returns `Session`          |
| GET    | `/api/v1/sessions`                      | Sessions | Returns `SessionListResponse`                           |
| GET    | `/api/v1/sessions/{sessionId}`          | Sessions | Returns `SessionDetail` (session + messages)            |
| PATCH  | `/api/v1/sessions/{sessionId}`          | Sessions | Body `UpdateSessionRequest`                             |
| DELETE | `/api/v1/sessions/{sessionId}`          | Sessions | Returns `204` (cascades to messages, leaves PDFs alone) |
| POST   | `/api/v1/sessions/{sessionId}/messages` | Chat     | Body `PostMessageRequest`; response `text/event-stream` |

Error responses for chat: `429` with `error.code ∈ { "rate_limit_exceeded",
"token_budget_exceeded" }`, `422` with `error.code = "content_filter_blocked"`.
Out-of-stream errors during streaming surface as
`data: {"error": "<code>"}` followed by `data: [DONE]`.

### SSE shape in OpenAPI 3.1

OpenAPI 3.1 cannot natively describe streaming (3.2 will). The plan documents the
streaming endpoint with:

```yaml
responses:
  "200":
    description: |
      Server-sent events stream of incremental chat tokens.
      Each event has the shape `data: {"delta": "<text>"}` or `data: [DONE]`.
      OpenAPI 3.1 cannot describe the per-frame envelope, so the schema below
      describes a single frame; the response is a sequence of these frames.
    content:
      text/event-stream:
        schema:
          $ref: "./schemas/chat.yaml#/ChatStreamFrame"
```

A `ChatStreamFrame` schema models `{ "delta": string }` and a comment notes the
`[DONE]` sentinel. Codegen produces the frame type; the streaming reader is
hand-written in both the Python client (used by tests) and the TypeScript Route
Handler.

### Codegen targets

| Project       | Tool                              | Output path                                        |
| ------------- | --------------------------------- | -------------------------------------------------- |
| `pdf-chat-be` | `datamodel-codegen` (Pydantic v2) | `apps/pdf-chat-be/generated_contracts/__init__.py` |
| `pdf-chat-fe` | `@hey-api/openapi-ts`             | `apps/pdf-chat-fe/src/generated-contracts/`        |

Both `codegen` targets `dependsOn: ["pdf-chat-contracts:bundle"]` and feed their output
directories as Nx cache inputs for `typecheck` and `test:quick`.

## Backend internals

### Settings (`config.py`)

```python
from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    openrouter_api_key: str
    openrouter_base_url: str = "https://openrouter.ai/api/v1"
    openrouter_default_model: str = "anthropic/claude-haiku-4.5"
    openrouter_embedding_model: str = "openai/text-embedding-3-small"
    database_url: str
    max_upload_mb: int = 25
    rag_top_k: int = 4
    mock_openrouter: bool = False
    enable_test_api: bool = False
    rate_limit_chat_per_minute: int = 20
    rate_limit_upload_per_minute: int = 60
    rate_limit_read_per_minute: int = 120
    enable_content_filter: bool = True
    content_filter_blocklist_path: str = "tests/fixtures/blocklist.txt"
    max_tokens_per_session: int = 200_000
    max_tokens_per_day: int = 2_000_000

    class Config:
        env_file = ".env"
```

### Vector schema (Alembic migration)

Six tables, all created in one Alembic revision. Read this list before reading
the SQL — it explains why each table exists:

- `pdfs` — one row per uploaded document. Stores filename, page count, upload
  timestamp.
- `pdf_chunks` — one row per ~800-token slice of a PDF. Each row has the chunk
  text and a `vector(1536)` column holding the embedding (1536 floating-point
  numbers — the meaning fingerprint produced by the embedding model). The
  `ivfflat` index makes similarity search fast.
- `sessions` — one row per chat thread. Stores title, the chosen LLM model id,
  timestamps. Sessions are anonymous (no auth in this demo) and addressable
  by their UUID.
- `session_pdfs` — many-to-many link table. A session may attach one or many
  PDFs; this is what makes multi-document conversations possible.
- `messages` — persisted chat history (`user` / `assistant` / `system` rows).
  Each row tracks token counts and a `status` flag for content-filter blocks.
  This is what makes a conversation survive reload.
- `token_usage` — running tally of input + output tokens consumed per session
  per calendar day. Read by the cost-cap guardrail before each chat call;
  upserted after each call.

```sql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE pdfs (
    id UUID PRIMARY KEY,
    filename TEXT NOT NULL,
    pages INT NOT NULL,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE pdf_chunks (
    id UUID PRIMARY KEY,
    pdf_id UUID NOT NULL REFERENCES pdfs(id) ON DELETE CASCADE,
    page INT NOT NULL,
    chunk_index INT NOT NULL,
    text TEXT NOT NULL,
    embedding vector(1536) NOT NULL
);

CREATE INDEX pdf_chunks_pdf_id_idx ON pdf_chunks(pdf_id);
CREATE INDEX pdf_chunks_embedding_idx ON pdf_chunks
    USING ivfflat (embedding vector_cosine_ops)
    WITH (lists = 100);

-- Sessions (persistent threads)
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    model TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Many-to-many session ↔ pdfs (multi-document conversation)
CREATE TABLE session_pdfs (
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    pdf_id     UUID NOT NULL REFERENCES pdfs(id)     ON DELETE CASCADE,
    PRIMARY KEY (session_id, pdf_id)
);
CREATE INDEX session_pdfs_pdf_id_idx ON session_pdfs(pdf_id);

-- Persisted chat history
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('user','assistant','system')),
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'ok' CHECK (status IN ('ok','blocked','error')),
    input_tokens  INT NOT NULL DEFAULT 0,
    output_tokens INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX messages_session_id_created_at_idx
    ON messages(session_id, created_at);

-- Per-session per-day token usage (cost cap accounting)
CREATE TABLE token_usage (
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    usage_date DATE NOT NULL,
    input_tokens  INT NOT NULL DEFAULT 0,
    output_tokens INT NOT NULL DEFAULT 0,
    PRIMARY KEY (session_id, usage_date)
);
```

### Multi-document retrieval SQL

Plain English: find the **k chunks whose embeddings are closest in meaning to
the user's question**, restricted to PDFs attached to this session. The query
embedding is the vector OpenRouter returned for the user's question text.

```sql
SELECT id, pdf_id, page, chunk_index, text,
       1 - (embedding <=> :query_embedding) AS similarity
FROM pdf_chunks
WHERE pdf_id IN (SELECT pdf_id FROM session_pdfs WHERE session_id = :session_id)
ORDER BY embedding <=> :query_embedding
LIMIT :k;
```

- `<=>` is pgvector's cosine-distance operator. Smaller distance = more similar.
- `1 - distance` flips the number into a similarity score in roughly [0, 1].
- `LIMIT :k` (k=4 by default) returns the top matches.
- The `WHERE pdf_id IN (...)` clause is what makes retrieval session-scoped:
  if the session attached three PDFs, all three are searched in one query and
  the result set may interleave chunks from any of them.

Retrieval is **always** session-scoped — per-PDF retrieval is not an endpoint.

### Guardrails layer

LLM endpoints are public-facing **and** charge real money per request. Without
defensive layers, a single misbehaving client can run up the bill or surface
problematic content. Three guardrails wrap every chat call here. Tokens (the
sub-word units LLMs are billed in) are the unit of cost; that's why two of the
three guardrails count tokens.

| Layer          | What it protects against                                | Implementation                                                                              |
| -------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Rate limit     | Spam, accidental loops, single-IP DoS                   | `slowapi` token bucket per IP per minute                                                    |
| Content filter | Abusive input; problematic LLM output reaching the user | Regex blocklist (swap-able for a real provider) — runs on input AND on the assembled output |
| Token cost cap | Runaway billing — a session or day exceeding budget     | Read-modify-write on `token_usage`                                                          |

`slowapi`, `sse-starlette`, and `tiktoken` are off-the-shelf libraries —
respectively: an in-process rate limiter for FastAPI, an SSE response helper,
and OpenAI's tokenizer used here for input-side token counting.

Three middlewares wrap every chat call. Order matters:

```python
# routers/sessions.py (chat handler, simplified)
@router.post("/api/v1/sessions/{session_id}/messages")
async def post_message(...):
    rate_limiter.check(request.client.host, "chat")            # 1. rate limit
    content_filter.scan_input(body.content)                     # 2. content filter (in)
    cost_cap.assert_session_under_cap(session_id)               # 3a. session cap
    cost_cap.assert_day_under_cap(date.today())                 # 3b. daily cap

    user_msg = await messages.persist(session_id, "user", body.content)

    async def event_gen():
        buffer = []
        async for token in openrouter_chat.stream(...):
            buffer.append(token)
            yield {"data": json.dumps({"delta": token})}
        full = "".join(buffer)
        try:
            content_filter.scan_output(full)                    # 2b. content filter (out)
        except ContentFilterBlocked:
            await messages.persist(session_id, "assistant", full, status="blocked")
            yield {"data": json.dumps({"error": "content_filter_blocked"})}
            yield {"data": "[DONE]"}
            return
        await messages.persist(session_id, "assistant", full,
                               input_tokens=count(prompt),
                               output_tokens=count(full))
        await cost_cap.record(session_id, date.today(), in_tok, out_tok)  # 3c. update
        yield {"data": "[DONE]"}

    return EventSourceResponse(event_gen())
```

#### Rate limit

`slowapi` with an in-memory storage backend (no Redis dependency for the demo). Limits
are configured per route via FastAPI dependencies. On limit exceeded, slowapi returns
`429` and FastAPI rewrites the body to the canonical `ErrorResponse` envelope via an
exception handler.

#### Content filter

`services/content_filter.py` exposes:

```python
class ContentFilter(Protocol):
    def scan_input(self, text: str) -> None: ...   # raises ContentFilterBlocked
    def scan_output(self, text: str) -> None: ...  # raises ContentFilterBlocked

class RegexBlocklistFilter(ContentFilter):
    def __init__(self, patterns: list[re.Pattern]): ...

class NoopFilter(ContentFilter):
    def scan_input(self, text): pass
    def scan_output(self, text): pass
```

`ENABLE_CONTENT_FILTER=false` swaps `RegexBlocklistFilter` for `NoopFilter`. Every test
ships with the noop unless the test explicitly exercises filtering. Production
deployments are expected to swap `RegexBlocklistFilter` for a real-provider
implementation (Anthropic moderation, OpenAI moderation) — the `Protocol` is the
extension seam.

#### Cost cap

```python
class CostCap:
    async def assert_session_under_cap(self, session_id):
        used = await db.scalar("SELECT COALESCE(SUM(input_tokens+output_tokens),0) "
                               "FROM token_usage WHERE session_id=:s",
                               {"s": session_id})
        if used >= settings.max_tokens_per_session:
            raise TokenBudgetExceeded("session")

    async def assert_day_under_cap(self, day):
        used = await db.scalar("SELECT COALESCE(SUM(input_tokens+output_tokens),0) "
                               "FROM token_usage WHERE usage_date=:d",
                               {"d": day})
        if used >= settings.max_tokens_per_day:
            raise TokenBudgetExceeded("day")

    async def record(self, session_id, day, in_tok, out_tok):
        await db.execute(
          """INSERT INTO token_usage(session_id, usage_date, input_tokens, output_tokens)
             VALUES (:s, :d, :i, :o)
             ON CONFLICT (session_id, usage_date)
             DO UPDATE SET input_tokens  = token_usage.input_tokens  + EXCLUDED.input_tokens,
                           output_tokens = token_usage.output_tokens + EXCLUDED.output_tokens""",
          {"s": session_id, "d": day, "i": in_tok, "o": out_tok})
```

Token counting uses `tiktoken` for OpenAI-family inputs and falls back to a rough
character-based approximation for Claude / Gemini outputs (the demo deliberately
trades accuracy for simplicity).

### SSE handler shape

The minimal handler shape is shown in the **Guardrails layer** section above. It uses
`sse_starlette.EventSourceResponse`, runs all three guardrail checks before streaming,
buffers tokens for an output content-filter pass, persists the assembled assistant
message, and updates `token_usage` in a single UPSERT. Any guardrail violation either
returns `429`/`422` before streaming starts or terminates the in-flight stream with a
`data: {"error": "<code>"}` frame followed by `data: [DONE]`.

### Mock OpenRouter

When `mock_openrouter=True`, both `openrouter_chat.stream()` and `embedding.embed()`
read from `tests/fixtures/openrouter.json` instead of issuing HTTP. The fixture maps:

```json
{
  "embeddings": { "default": [0.01, 0.02, ...] },
  "chat": {
    "default": ["Hello", " from", " the", " mock", " stream", "!"]
  }
}
```

Tests assert the _requests_ the backend would have made (model id, prompt content),
without burning real tokens.

## Frontend internals

### ts-ui consumption (mandatory)

`pdf-chat-fe` consumes `@open-sharia-enterprise/ts-ui` (source: `libs/ts-ui/`) as its
**only** UI primitive layer. The same posture as `crud-fe-ts-nextjs`. Rules:

- Add `"@open-sharia-enterprise/ts-ui": "*"` and `"@open-sharia-enterprise/ts-ui-tokens": "*"`
  to `apps/pdf-chat-fe/package.json` `dependencies`.
- `globals.css` first line: `@import "@open-sharia-enterprise/ts-ui-tokens/src/tokens.css";`
- Import primitives only from the package:
  `import { Button, Card, CardHeader, CardTitle, CardContent, Input, Label, Dialog, DialogContent, Alert, cn } from "@open-sharia-enterprise/ts-ui";`
- Do **not** re-implement `Button`, `Input`, `Card`, `Label`, `Dialog`, `Alert` inside
  `apps/pdf-chat-fe/src/components/`. Files in `components/` are demo-specific
  composites only (UploadZone, ChatTranscript, ChatComposer, ModelSelector).
- If a primitive is missing from `ts-ui` (e.g., `Textarea`, `Avatar`, `ScrollArea`),
  add it to `libs/ts-ui` first via `swe-ui-maker` following the
  [Component Patterns Convention](../../../governance/development/frontend/component-patterns.md),
  land that in its own commit, then consume it here. Do **not** inline a one-off in
  `pdf-chat-fe`.
- Add `pdf-chat-fe` to `libs/ts-ui`'s reverse-dependency list (Nx auto-tracks via
  imports; `nx graph` should show the edge after the first import).

Example composite (composes ts-ui primitives, owns demo-specific behaviour):

```tsx
// apps/pdf-chat-fe/src/components/ChatComposer.tsx
import { Button, Input, cn } from "@open-sharia-enterprise/ts-ui";

type Props = {
  disabled?: boolean;
  onSubmit: (text: string) => void;
};

export function ChatComposer({ disabled, onSubmit }: Props) {
  // … local state + Enter / Shift+Enter handling …
  return (
    <form
      className={cn("flex gap-2 border-t p-3", disabled && "opacity-60")}
      onSubmit={(e) => {
        e.preventDefault();
        // …
      }}
    >
      <Input name="prompt" placeholder="Ask the PDF…" disabled={disabled} />
      <Button type="submit" disabled={disabled}>
        Send
      </Button>
    </form>
  );
}
```

### Route Handler proxy (`/api/chat/route.ts`)

A Next.js Route Handler is a server-side endpoint that lives inside the
frontend project. We use it as a thin proxy: the browser POSTs to
`/api/chat` (same-origin), the Route Handler forwards to the FastAPI backend,
and the upstream SSE body is piped back to the browser unchanged.

Two reasons for the proxy:

- **Hide the backend URL** from the browser. The browser only sees same-origin
  `/api/chat`; the real `PDF_CHAT_BE_URL` stays server-side.
- **Same-origin endpoint for `useChat`**. The Vercel AI SDK's `useChat` hook
  (next section) is happiest pointed at a same-origin URL.

```ts
export const runtime = "nodejs";

export async function POST(req: Request) {
  const body = await req.json();
  const upstream = await fetch(`${process.env.PDF_CHAT_BE_URL}/api/v1/sessions/${body.sessionId}/messages`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ content: body.content }),
  });
  return new Response(upstream.body, {
    status: upstream.status,
    headers: {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      "X-Accel-Buffering": "no",
    },
  });
}
```

The `X-Accel-Buffering: no` header tells reverse proxies (e.g., nginx) not to
buffer the SSE response — without it, tokens stack up in the proxy and arrive
in bursts instead of streaming.

### `useChat` integration

`useChat` is a React hook from the **Vercel AI SDK** (`@ai-sdk/react`). It
manages the chat-message list state, fires the POST to `api`, reads the SSE
stream as it arrives, appends streamed token deltas to the latest assistant
message, and re-renders on each frame. Engineers used to manually wiring SSE
in React can think of it as "`useState` + `fetch` + SSE reader, packaged."

```tsx
import { useChat } from "@ai-sdk/react";

const { messages, append, status } = useChat({
  api: "/api/chat",
  body: { sessionId }, // sessionId binds useChat to the Route Handler and onward to the backend session
  streamProtocol: "data", // raw SSE passthrough
});
```

When AI SDK 5's transport layer requires more nuance, swap to a custom `ChatTransport`
implementation. tech-docs records this as a known seam.

### Env vars (`apps/pdf-chat-fe/.env.example`)

```dotenv
PDF_CHAT_BE_URL=http://localhost:8501
NEXT_PUBLIC_DEFAULT_MODEL=anthropic/claude-haiku-4.5
```

## Test levels and tooling

The two sides of `pdf-chat` use different test-level conventions, mirroring the
`crud-*` family.

### Backend (`pdf-chat-be`) — three levels

| Level              | Runner                         | Tool stack                                                            | OpenRouter                                   | Postgres              | Coverage            |
| ------------------ | ------------------------------ | --------------------------------------------------------------------- | -------------------------------------------- | --------------------- | ------------------- |
| `test:unit`        | `pytest` + `pytest-bdd`        | pytest, pytest-bdd, pytest-asyncio, pytest-httpx, freezegun, coverage | Mocked (httpx fixture)                       | None (mocked repos)   | **Measured (≥90%)** |
| `test:integration` | `pytest-bdd` in docker-compose | Same pytest stack, real Postgres + pgvector                           | Mocked (httpx fixture)                       | Real (pgvector image) | Not measured        |
| `test:e2e`         | Playwright + playwright-bdd    | runs from `pdf-chat-be-e2e` against running BE                        | Mocked (default) or real (workflow_dispatch) | Real                  | Not measured        |

Backend lint + typecheck stack: **`ruff`** (lint), **`pyright`** (typecheck), via
`nx run pdf-chat-be:lint` and `nx run pdf-chat-be:typecheck`. Test framework is
**`pytest` + `pytest-bdd`** at all three levels, identical to
`crud-be-python-fastapi`.

### Frontend (`pdf-chat-fe`) — two levels

| Level       | Runner                      | Tool stack                                                 | Backend dep                                  | Coverage            |
| ----------- | --------------------------- | ---------------------------------------------------------- | -------------------------------------------- | ------------------- |
| `test:unit` | `vitest`                    | vitest, @testing-library/react, jsdom, @vitest/coverage-v8 | none (Route Handler mocked)                  | **Measured (≥70%)** |
| `test:e2e`  | Playwright + playwright-bdd | runs from `pdf-chat-fe-e2e` against running BE + FE        | Real (BE booted with `MOCK_OPENROUTER=true`) | Not measured        |

The frontend has **no `test:integration` level**. Integration concerns (backend
contract, real DB) are covered by `pdf-chat-be:test:integration` and the
end-to-end pair. This matches the `crud-fe-ts-nextjs` pattern.

Frontend lint + typecheck stack: **`oxlint`** (lint, with `--jsx-a11y-plugin`),
**`tsc --noEmit`** (typecheck). Source language is **TypeScript** end-to-end —
no JavaScript files in the app source.

### Coverage measurement

- **BE**: `pdf-chat-be:test:quick` runs `pytest -m unit` with coverage, emits
  `coverage/lcov.info`, then invokes `rhino-cli test-coverage validate ... 90`.
- **FE**: `pdf-chat-fe:test:quick` runs `vitest --coverage`, emits
  `coverage/lcov.info`, then invokes `rhino-cli test-coverage validate ... 70`.

## Test determinism strategy (the LLM problem)

LLMs are non-deterministic by default — same prompt produces different output
across calls, model versions, and provider releases. A naïve test suite
(`assert chat("What's 2+2?") == "4"`) breaks the moment the upstream model is
patched, the temperature parameter changes, or a network timeout retries with a
slightly different prefix. This plan's tests are deliberately structured so
that **none of that affects test outcomes**.

### Sources of non-determinism we control for

| Source                                | Mitigation                                                                                   |
| ------------------------------------- | -------------------------------------------------------------------------------------------- |
| Sampling (temperature, top-p)         | All test calls go through the OpenRouter mock; real-API calls use `temperature=0`            |
| Provider-side model updates           | Tests never assert on returned text content; only on what we **sent** + structural shape     |
| Tokenizer drift (`tiktoken` versions) | `tiktoken` is pinned in `pyproject.toml`; cost-cap tests use synthetic counts, not live ones |
| Network jitter, timeouts, retries     | `MOCK_OPENROUTER=true` removes the network entirely from unit + integration                  |
| SSE chunk arrival timing              | Mock cassette emits a fixed sequence; tests assert the **set** of frames, not timing         |
| Embedding similarity drift            | Embedding fixtures are deterministic vectors (zeros + one marker dim per chunk)              |
| `datetime.now()` for daily token cap  | `freezegun.freeze_time(...)` pins the clock for daily-cap tests                              |
| Random ids (`uuid4()`)                | Tests assert on shape (`UUID v4`), never on exact value; or fixtures pin via patch           |

### The four assertion patterns we use (and the one we don't)

1. **Structural assertions** (preferred everywhere). Assert response is
   well-formed JSON / SSE / matches the OpenAPI contract; status code is correct;
   `Content-Type` is right; required fields exist with right types. Never asserts
   on prose content of an LLM-generated string.
2. **Outbound-request assertions** (the strongest signal we have for LLM-shaped
   code). Assert on what the backend **sent to OpenRouter**: model id, system
   prompt content, retrieved chunks, message history shape. The cassette captures
   the request side; tests inspect it. This catches bugs in chunking, retrieval,
   prompt assembly, model selection, history persistence — without ever caring
   what the LLM said back.
3. **Side-effect assertions**. Assert on database state after a chat call:
   user message persisted, assistant message persisted with the assembled output,
   `token_usage` upserted with non-zero counts, `messages` ordered by
   `created_at`. State assertions are deterministic in a way text assertions
   never can be.
4. **Snapshot assertions** (for assembled prompts only). The full
   `[system, retrieved_chunks, history, user_message]` payload that gets sent to
   OpenRouter is snapshot-tested via `pytest`'s built-in fixtures. Snapshot
   diffs are reviewed deliberately when prompt logic changes; they never fail
   because the LLM said something different.
5. **Anti-pattern (forbidden)**: asserting that the assistant response contains
   a specific phrase, has a specific length, "answers correctly", or matches a
   regex on prose. **No test in this plan does this.** The cassette controls the
   tokens; if we want to assert on the response, we control what the cassette
   yields and assert on the persisted/structural shape, not on text quality.

### Mock cassette structure

`tests/fixtures/openrouter.json` is a deterministic stand-in for OpenRouter:

```json
{
  "embeddings": {
    "<query-fingerprint>": [0.0, 0.0, ..., 1.0, ..., 0.0]
  },
  "chat": {
    "<prompt-fingerprint>": [
      "Hello", " from", " the", " mock", " stream", "!"
    ],
    "default": ["Mock", " response"]
  },
  "scenarios": {
    "content_filter_block_output": ["This", " contains", " <BLOCKLISTED>"],
    "long_response_for_token_cap":  ["..." ]
  }
}
```

- **Fingerprint**: deterministic hash of the request payload (chat: messages
  array; embedding: input string). The mock client looks up the fingerprint
  and falls back to `default` when no scenario is registered.
- **Embeddings** are hand-crafted so similarity ordering is testable. For two
  fixture PDFs we set distinct marker dimensions; the retrieval test asserts
  that for query `Q_A` chunks from PDF A rank above chunks from PDF B.
- **Streaming**: chat fixtures are arrays of token strings. The mock streamer
  yields each in order, with no delay (or a fixed `asyncio.sleep(0)` if a test
  needs to assert mid-stream behaviour).
- **Scenarios** are named keys exercised by specific Gherkin steps (e.g.,
  `Given the model returns a blocklisted token`). One cassette serves every
  test; no per-test fixture file.

### Real-LLM smoke (workflow_dispatch only)

A single workflow (`test-pdf-chat-be-e2e.yml` with `mock_openrouter=false`
input parameter) runs against the **real** OpenRouter for manual smoke. Its
assertions are **structural only**: response status, `Content-Type:
text/event-stream`, ≥1 frame received, response well-formed. It never asserts
on returned text. CI on PR / on push uses `MOCK_OPENROUTER=true`.

This split keeps PR runs fast, free, deterministic; pre-release smokes
exercise the real provider without polluting the deterministic test set.

### Retrieval determinism

pgvector's `<=>` operator is exact for small corpora; the `ivfflat` index is
**approximate**, which means top-k can vary if the index is rebuilt. Tests
defend against this two ways:

- **Unit tests** mock the vector store entirely; retrieval results are a
  hand-curated list returned by the mock.
- **Integration tests** populate a fresh DB per run (`docker-compose down -v`),
  build the ivfflat index in the same migration, and seed with deterministic
  embedding vectors. The test then asserts the **set** of returned chunk ids,
  not the order — because ivfflat is approximate even on the same index. When
  ordering matters, the test uses `<=>` directly with `SET enable_indexscan =
off` to force the deterministic sequential scan.

### Eval suites (out of scope)

Production-grade RAG ships eval suites (Promptfoo, Langfuse, Inspect AI,
custom rubrics) that score real LLM output against a held-out set. Those are
explicitly out of scope here — see the
[AI primer §13](../../../docs/explanation/software-engineering/ai-application-development/README.md#13-evaluation-and-the-lack-of-unit-tests-for-llm-output)
for the broader picture. The deterministic test set above is what runs on
every PR; eval is a separate, slower, costlier loop.

## Mandatory Nx targets per project

```text
pdf-chat-contracts:  lint, bundle, docs
pdf-chat-be:         codegen, typecheck, lint, build, test:unit, test:quick, test:integration, dev, start, spec-coverage
pdf-chat-fe:         codegen, typecheck, lint, build, test:unit, test:quick, dev, start, spec-coverage
pdf-chat-be-e2e:     install, lint, typecheck, test:quick, test:e2e, test:e2e:report, spec-coverage
pdf-chat-fe-e2e:     install, lint, typecheck, test:quick, test:e2e, test:e2e:report, spec-coverage
```

`pdf-chat-be:test:quick` invokes `rhino-cli test-coverage validate apps/pdf-chat-be/coverage/lcov.info 90`
identical to `crud-be-python-fastapi`.

`pdf-chat-fe:test:quick` invokes `rhino-cli test-coverage validate apps/pdf-chat-fe/coverage/lcov.info 70`.

## Mermaid: Nx project graph (post-plan)

```mermaid
%% Color Palette: Blue #0173B2 | Teal #029E73 | Orange #DE8F05 | Purple #CC78BC
graph LR
    PCC[pdf-chat-contracts]:::contract
    PCB[pdf-chat-be]:::be
    PCF[pdf-chat-fe]:::fe
    PCBE2E[pdf-chat-be-e2e]:::e2e
    PCFE2E[pdf-chat-fe-e2e]:::e2e
    RHN[rhino-cli]:::tool
    TSU[ts-ui]:::lib
    TSUT[ts-ui-tokens]:::lib

    PCC --> PCB
    PCC --> PCF
    PCB --> PCBE2E
    PCF --> PCFE2E
    PCB --> PCFE2E
    RHN --> PCB
    RHN --> PCF
    TSU --> PCF
    TSUT --> PCF
    TSUT --> TSU

    classDef contract fill:#CC78BC,stroke:#000,color:#000
    classDef be fill:#029E73,stroke:#000,color:#FFF
    classDef fe fill:#0173B2,stroke:#000,color:#FFF
    classDef e2e fill:#DE8F05,stroke:#000,color:#000
    classDef tool fill:#808080,stroke:#000,color:#FFF
    classDef lib fill:#CA9161,stroke:#000,color:#000
```

## Web research citations (2026-04-26)

All rows accessed 2026-04-26.

| Claim                                                    | Inline excerpt                                                                                                                                                                         | Source                                                                                         |
| -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| OpenRouter base URL + auth + model id format             | Base URL `https://openrouter.ai/api/v1/chat/completions`; auth header `Authorization: Bearer <OPENROUTER_API_KEY>`; model ids use org-prefix format e.g. `anthropic/claude-sonnet-4.6` | <https://openrouter.ai/docs/api/reference/overview>                                            |
| `anthropic/claude-haiku-4.5` (dot, not hyphen)           | Model id confirmed as `anthropic/claude-haiku-4.5` (dot separator, not hyphen) on the model page                                                                                       | <https://openrouter.ai/anthropic/claude-haiku-4.5>                                             |
| `google/gemini-2.5-flash-lite` pricing $0.10/$0.40 per M | Pricing listed as $0.10/M input tokens, $0.40/M output tokens on the model page                                                                                                        | <https://openrouter.ai/google/gemini-2.5-flash-lite>                                           |
| PyMuPDF AGPL licensing trap                              | Artifex licensing page states PyMuPDF is "available under AGPL v3 open source license" requiring source disclosure for derived works                                                   | <https://artifex.com/licensing>                                                                |
| pypdf BSD-3-Clause license (permissive, not MIT)         | PyPI page states `License Expression: BSD-3-Clause`; BSD-3-Clause is permissive and compatible with MIT-licensed template                                                              | <https://pypi.org/project/pypdf/>                                                              |
| pgvector docker image `pgvector/pgvector:pg16`           | Docker Hub page lists `pgvector/pgvector` image with PostgreSQL version-specific tags including `pg16` variants                                                                        | <https://hub.docker.com/r/pgvector/pgvector>                                                   |
| OpenRouter embeddings endpoint                           | Embeddings endpoint documented at `/api/v1/embeddings`; accepts `model` and `input` fields; returns embedding vectors                                                                  | <https://openrouter.ai/docs/api/api-reference/embeddings/create-embeddings>                    |
| `sse-starlette.EventSourceResponse` idiomatic SSE        | PyPI page describes `EventSourceResponse` as a "production-ready Server-Sent Events implementation for Starlette and FastAPI" per W3C SSE spec                                         | <https://pypi.org/project/sse-starlette/>                                                      |
| FastAPI `StreamingResponse` SSE pattern                  | FastAPI docs show `StreamingResponse` and `EventSourceResponse` patterns for server-sent events with `text/event-stream` content type                                                  | <https://fastapi.tiangolo.com/tutorial/server-sent-events/>                                    |
| AI SDK 5 `useChat` + transport                           | `useChat` hook docs show `api`, `body`, and `streamProtocol` options; transport layer is extensible via custom `ChatTransport` implementations                                         | <https://ai-sdk.dev/docs/reference/ai-sdk-ui/use-chat>                                         |
| AI SDK 5 announcement (breaking change from v4)          | Vercel blog announces AI SDK 5 with "breaking changes that remove deprecated APIs" and new transport architecture; recommends `npx @ai-sdk/codemod upgrade` for migration              | <https://vercel.com/blog/ai-sdk-5>                                                             |
| OpenAPI 3.2 streaming improvements (`itemSchema`)        | Article describes `itemSchema` keyword added in OpenAPI 3.2 to describe individual items in a stream, addressing SSE documentation gap                                                 | <https://developerhub.io/blog/event-streaming-in-openapi-3-2-what-changed-and-why-it-matters/> |
| OpenAPI 3.1 multipart file upload                        | Speakeasy guide shows `requestBody` with `multipart/form-data` content type and `format: binary` for file upload fields in OpenAPI 3.1                                                 | <https://www.speakeasy.com/openapi/content/file-uploads>                                       |

## Dependencies

Execution prerequisites:

- `git`, `npm`, `npx nx`, Volta-pinned Node.
- Python 3.13 via `uv` (already in doctor's required toolchain).
- Docker for `test:integration`.
- An `OPENROUTER_API_KEY` for any _real-API_ runs (E2E workflow_dispatch with
  `MOCK_OPENROUTER=false`); not needed for unit / integration / `test:quick`.

## Rollback

All commits land directly on `main`. To roll back: identify the last good commit hash
with `git log --oneline`, then `git revert` the unwanted commits in reverse order. The
plan is structured so each phase yields a green workspace at the end, so partial
rollback to any phase boundary is safe.

## Open questions tracked in delivery

- Whether to ship a doctor-managed `pgvector` Postgres image at the workspace level
  (would benefit future plans) — deferred to a follow-up plan.
- Whether the `pdf-chat-fe-e2e` suite should depend on `pdf-chat-be` running, or use a
  full mock — current decision: launch backend with `MOCK_OPENROUTER=true` for true
  end-to-end realism.
- Whether `pdf-chat-contracts` should ship a streaming type via `x-streaming` extension
  to nudge codegen — deferred until OpenAPI 3.2 tooling matures.
