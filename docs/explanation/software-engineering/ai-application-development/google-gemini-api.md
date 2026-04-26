---
title: "Google Gemini API Primer"
description: When and how to use Google Gemini directly — model lineup, exact ids, the unified google-genai SDK, streaming, embeddings via gemini-embedding-001, the 1 M-token context window, and CI mocking
category: explanation
subcategory: development
tags:
  - ai
  - gemini
  - google
  - llm
  - embeddings
  - streaming
  - long-context
principles:
  - explicit-over-implicit
  - reproducibility
  - documentation-first
---

# Google Gemini API Primer

**Audience**: software engineers wiring Gemini into a backend, having read the
[AI Application Development primer](./README.md) first. All facts dated
2026-04-27. The Gemini API is the cheapest credible path to (a) very long
context, (b) embeddings, and (c) a low-cost chat tier — three reasons the
demos in this repo lean on it heavily.

## When to reach for Gemini

Reach for Gemini when the workload is:

- **Embeddings** — Gemini ships the only embedding model in the lineup of the
  three vendors covered here. Even when chat is served by Anthropic or
  Perplexity, embeddings will usually come from `gemini-embedding-001`.
- **Long context** — Flash and Flash-Lite both expose a 1 M-token input
  window. Useful for passing several whole 10-Ks in one call when RAG would
  add complexity for marginal gain.
- **Cost-sensitive chat** — Flash-Lite is the cheapest credible chat tier in
  the lineup; ideal for high-volume background tasks (batch summarisation,
  classification).

Skip Gemini when:

- You need top-tier reasoning quality on a hard problem — Anthropic Sonnet is
  usually a better default.
- You need live web search grounding — see the [Perplexity primer](./perplexity-api.md).

## Model lineup (2026-Q2)

| Model                 | Tier                | Model id                | Context window | Output limit |
| --------------------- | ------------------- | ----------------------- | -------------- | ------------ |
| Gemini 2.5 Flash-Lite | small / cheap       | `gemini-2.5-flash-lite` | 1 048 576 in   | 65 536 out   |
| Gemini 2.5 Flash      | mid / fast          | `gemini-2.5-flash`      | 1 048 576 in   | 65 536 out   |
| Gemini 2.5 Pro        | premium / reasoning | `gemini-2.5-pro`        | 1 048 576 in   | 65 536 out   |

Notes:

- Model ids are **hyphen-separated** (`gemini-2.5-flash-lite`), not dot-separated.
- Stable ids carry no date suffix; preview suffixes like
  `gemini-2.5-flash-lite-preview-09-2025` are deprecated. New code uses the
  bare stable id.
- Verify the current set at any time:
  [Gemini models reference](https://ai.google.dev/gemini-api/docs/models).

## SDKs and authentication

Google ships a **unified GenAI SDK** (`google-genai`) that targets both the
Gemini API and Vertex AI from one client surface. The older
`google-generativeai` package is **legacy** and no longer recommended.

| Language   | Package         | Latest version (2026-04-27) | Repo                                         |
| ---------- | --------------- | --------------------------: | -------------------------------------------- |
| Python     | `google-genai`  |                      1.73.1 | <https://github.com/googleapis/python-genai> |
| TypeScript | `@google/genai` |                (npm latest) | <https://github.com/googleapis/js-genai>     |

Both SDKs read `GOOGLE_API_KEY` (or `GEMINI_API_KEY` in newer docs) from the
environment. Vertex AI deployments switch to ADC; the unified client picks the
right path automatically once `vertexai=True` is set.

## Minimal request — Python

```python
from google import genai

client = genai.Client()  # reads GOOGLE_API_KEY

resp = client.models.generate_content(
    model="gemini-2.5-flash",
    contents="Summarise this 10-K in three bullets.",
)
print(resp.text)
```

## Minimal request — TypeScript

```ts
import { GoogleGenAI } from "@google/genai";

const ai = new GoogleGenAI({}); // reads GOOGLE_API_KEY

const resp = await ai.models.generateContent({
  model: "gemini-2.5-flash",
  contents: "Summarise this 10-K in three bullets.",
});
console.log(resp.text);
```

## Streaming

```python
stream = client.models.generate_content_stream(
    model="gemini-2.5-flash",
    contents="Stream three short bullets.",
)
for chunk in stream:
    print(chunk.text, end="", flush=True)
```

```ts
const stream = await ai.models.generateContentStream({
  model: "gemini-2.5-flash",
  contents: "Stream three short bullets.",
});
for await (const chunk of stream) {
  process.stdout.write(chunk.text ?? "");
}
```

The wire format is SSE; the SDK abstracts it. To forward chunks unchanged
through a `sse-starlette.EventSourceResponse`, yield `{"data": chunk.text}`
on each iteration. See the [streaming docs](https://ai.google.dev/gemini-api/docs/text-generation).

## PDF input

Gemini accepts PDFs through two paths:

- **Inline parts**: small PDFs (up to ~20 MB) base64-encoded into a content
  part with `mime_type: "application/pdf"`. Same call cycle as a text prompt.
- **Files API**: upload once (up to 2 GB), reference by `file_uri` in
  subsequent calls. Cheaper across many turns over the same document; files
  expire after 48 h.

```python
from google.genai import types
import pathlib

pdf = client.files.upload(path=pathlib.Path("aapl-fy2024-10k.pdf"))

resp = client.models.generate_content(
    model="gemini-2.5-flash",
    contents=[pdf, "Identify the three biggest risks."],
)
```

For a 50-page 10-K (~50 k tokens) the inline path is fine. For a 500-page
master document or a session that asks twenty questions of the same PDF, the
Files API is the right tool.

## Embeddings (the headline feature)

`gemini-embedding-001` is the current GA embedding model. Defaults and knobs:

| Parameter               | Default | Allowed                                                           | Notes                                                    |
| ----------------------- | ------: | ----------------------------------------------------------------- | -------------------------------------------------------- |
| `output_dimensionality` |    3072 | 768, 1536, 3072 (recommended)                                     | Matryoshka representation: same model, truncated vectors |
| `task_type`             |    none | `RETRIEVAL_QUERY`, `RETRIEVAL_DOCUMENT`, `SEMANTIC_SIMILARITY`, … | Tunes the projection for the use case                    |

**Why pick 768 instead of 3072 for a demo?** Vector storage is linear in
dimensions; pgvector ivfflat performance degrades faster on higher-dim
vectors. 768 dims is the sweet spot for most demos: 4× less storage and CPU
than 3072 with negligible recall loss on small corpora.

```python
embed = client.models.embed_content(
    model="gemini-embedding-001",
    contents=["The cat sat on the mat."],
    config=types.EmbedContentConfig(
        output_dimensionality=768,
        task_type="RETRIEVAL_DOCUMENT",
    ),
)
print(len(embed.embeddings[0].values))  # 768
```

For RAG, **always** call with `task_type="RETRIEVAL_DOCUMENT"` when embedding
chunks for storage and `task_type="RETRIEVAL_QUERY"` when embedding the user
question. The model produces different (but compatible) projections optimised
for each side of the search.

For the conceptual side of embeddings (cosine similarity, vector spaces, why
keyword search fails) see §5–§7 of the [main primer](./README.md).

## The 1 M-token context window

Long context is genuinely useful in three patterns:

1. **Whole-document Q&A without RAG.** A 200 k-token PDF fits five times over;
   skip chunking, embedding, retrieval — paste the document directly. Lower
   complexity for batch jobs that don't need conversation memory.
2. **Multi-document fusion.** Pass three 200 k-token reports in one prompt and
   ask the model to compare. Beats stitching three RAG calls together.
3. **Tool-use scratchpads.** Long tool-call traces fit naturally; no risk of
   the model losing context mid-loop.

Long context is **not** a free lunch:

- Every input token is billed every turn — a 500 k-token prompt across 5
  conversational turns costs as much as 2.5 M input tokens.
- Latency grows superlinearly past ~200 k tokens; first-token latency on a
  full 1 M prompt can exceed 10 s.
- The model still pays attention; quality on facts buried mid-document
  ("needle in haystack") is materially below quality on facts at the start or
  end. RAG narrows the model's attention to relevant slices and often wins on
  quality despite a smaller token count.

In short: long context is a tool, not a default. The default is RAG.

## Reference cost (2026-Q2)

Indicative pricing per million tokens; verify at
[ai.google.dev/pricing](https://ai.google.dev/pricing) before publishing.

| Model                  | Input | Output |
| ---------------------- | ----: | -----: |
| Gemini 2.5 Flash-Lite  | $0.10 |  $0.40 |
| Gemini 2.5 Flash       | $0.30 |  $2.50 |
| Gemini 2.5 Pro         | $1.25 | $10.00 |
| `gemini-embedding-001` | $0.15 |      — |

Gemini Flash-Lite is roughly an order of magnitude cheaper than Anthropic
Haiku at comparable quality for short Q&A tasks; it is the go-to demo default
for cost-sensitive runs.

## CI mocking pattern

Same shape as Anthropic — intercept the `httpx` layer that the SDK uses,
return a fixture, and assert on the outbound request and side effects rather
than the response prose.

```python
import pytest

@pytest.fixture
def mock_gemini_chat(httpx_mock):
    httpx_mock.add_response(
        url__regex=r".*generativelanguage\.googleapis\.com.*generateContent.*",
        method="POST",
        json={
            "candidates": [{"content": {"parts": [{"text": "FIXTURE"}]}}],
            "usageMetadata": {"promptTokenCount": 10, "candidatesTokenCount": 1},
        },
    )

@pytest.fixture
def mock_gemini_embed(httpx_mock):
    httpx_mock.add_response(
        url__regex=r".*generativelanguage\.googleapis\.com.*embedContent.*",
        method="POST",
        json={"embedding": {"values": [0.0] * 768}},
    )
```

Real-LLM smoke tests live behind a workflow-dispatch flag, not on every CI
run. See the main primer §13 for the full determinism strategy.

## Related

- [AI Application Development](./README.md) — generic primer covering tokens,
  embeddings, RAG, streaming, guardrails, evaluation, cost
- [Anthropic API Primer](./anthropic-api.md) — paired vendor doc; chat lives
  there for premium-quality reasoning
- [Perplexity API Primer](./perplexity-api.md) — when web-grounded answers are
  the requirement
- [Gemini API docs](https://ai.google.dev/gemini-api/docs) — authoritative
  reference, supersedes anything here on conflict
