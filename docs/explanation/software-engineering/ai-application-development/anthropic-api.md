---
title: "Anthropic API Primer"
description: When and how to use Anthropic Claude directly — model lineup, exact ids, SDKs, streaming, PDF input, the no-embeddings caveat, and CI mocking
category: explanation
subcategory: development
tags:
  - ai
  - anthropic
  - claude
  - llm
  - streaming
  - pdf
principles:
  - explicit-over-implicit
  - reproducibility
  - documentation-first
---

# Anthropic API Primer

**Audience**: software engineers wiring Anthropic Claude into a backend, having
read the [AI Application Development primer](./README.md) first. This doc is
vendor-specific reference framed as explanation: read top-to-bottom on first
exposure, dip in by section thereafter. All facts dated 2026-04-27.

## When to reach for Anthropic

Reach for Anthropic when the workload is:

- Multi-step reasoning over private context (long PDFs, internal codebases,
  proprietary specs) where output quality matters more than per-token cost.
- Tool use / agentic loops (the Anthropic SDK has a fluent tool-use API and
  models trained for it).
- Latency-sensitive UX where Haiku's ~80 tok/s and ~500 ms first-token are the
  product.

Skip Anthropic when:

- You need embeddings — Anthropic does **not** ship an embeddings endpoint.
  Pair Claude with Gemini's `gemini-embedding-001` (see the
  [Gemini primer](./google-gemini-api.md)) or with Voyage AI per Anthropic's
  own recommendation.
- You need live web search grounding — see the
  [Perplexity primer](./perplexity-api.md).

## Model lineup (2026-Q2)

Anthropic surfaces models by family (Haiku, Sonnet, Opus) and version (4.5,
4.6, …). Each model has both a **dated id** (immutable, version-pinned) and an
**alias** (resolves to the latest minor). For reproducible CI, use the dated
id. For demos and exploratory work, the alias is fine.

| Model      | Tier         | Alias               | Dated id                         | Context window | Status     |
| ---------- | ------------ | ------------------- | -------------------------------- | -------------- | ---------- |
| Haiku 4.5  | small / fast | `claude-haiku-4-5`  | `claude-haiku-4-5-20251001`      | 200 k tokens   | current    |
| Sonnet 4.5 | mid          | `claude-sonnet-4-5` | `claude-sonnet-4-5-20250929`     | 200 k tokens   | **legacy** |
| Sonnet 4.6 | mid          | `claude-sonnet-4-6` | (dated suffix per release notes) | 1 M tokens     | current    |

Notes:

- Model ids use **hyphens** between version parts (`claude-haiku-4-5`), not
  dots. A previous repo convention used dots; that was wrong and is being
  retired across all docs and demos.
- Sonnet 4.5 still works but Anthropic now classes it as legacy. New work
  picks Sonnet 4.6 unless a deliberate reason exists.
- Verify the current set at any time:
  [models overview](https://platform.claude.com/docs/en/about-claude/models/overview).

## SDKs and authentication

Two officially supported SDKs:

| Language   | Package             | Latest version (2026-04-27) | Repo                                                     |
| ---------- | ------------------- | --------------------------: | -------------------------------------------------------- |
| Python     | `anthropic`         |                      0.97.0 | <https://github.com/anthropics/anthropic-sdk-python>     |
| TypeScript | `@anthropic-ai/sdk` |                      0.90.0 | <https://github.com/anthropics/anthropic-sdk-typescript> |

Both SDKs read `ANTHROPIC_API_KEY` from the environment by default. For server
deployment use a secret manager; for local dev use `.env` (gitignored) plus an
`.env.example` shipping placeholders only.

## Minimal request — Python

```python
from anthropic import Anthropic

client = Anthropic()  # reads ANTHROPIC_API_KEY

resp = client.messages.create(
    model="claude-haiku-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Summarise this 10-K in 3 bullets."}],
)
print(resp.content[0].text)
```

## Minimal request — TypeScript

```ts
import Anthropic from "@anthropic-ai/sdk";

const client = new Anthropic(); // reads ANTHROPIC_API_KEY

const resp = await client.messages.create({
  model: "claude-haiku-4-5",
  max_tokens: 1024,
  messages: [{ role: "user", content: "Summarise this 10-K in 3 bullets." }],
});
console.log(resp.content[0].type === "text" ? resp.content[0].text : "");
```

## Streaming

Both SDKs ship a fluent streaming helper that wraps the underlying SSE wire
format and exposes a simple text-stream iterator.

```python
with client.messages.stream(
    model="claude-haiku-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Stream three short bullets."}],
) as stream:
    for chunk in stream.text_stream:
        print(chunk, end="", flush=True)
    final = stream.get_final_message()  # full response with usage block
```

```ts
const stream = client.messages.stream({
  model: "claude-haiku-4-5",
  max_tokens: 1024,
  messages: [{ role: "user", content: "Stream three short bullets." }],
});
for await (const event of stream) {
  if (event.type === "content_block_delta" && event.delta.type === "text_delta") {
    process.stdout.write(event.delta.text);
  }
}
const final = await stream.finalMessage();
```

For details — including how to inject streaming chunks into a
`sse-starlette.EventSourceResponse` to forward them to a browser — see the
[official streaming guide](https://platform.claude.com/docs/en/build-with-claude/streaming).

## PDF input

Anthropic accepts PDFs as a first-class content block. For a 200 k-context
model (Haiku 4.5, Sonnet 4.5):

- **Maximum 100 pages per request** (the higher 600-page limit applies only to
  1 M-context models).
- **Maximum request size 32 MB** including all encoded content.
- Three delivery shapes: URL reference (Anthropic fetches), inline base64, or
  Files API `file_id` (upload once, reference many times).

```python
import base64, pathlib

pdf_bytes = pathlib.Path("aapl-fy2024-10k.pdf").read_bytes()
encoded = base64.standard_b64encode(pdf_bytes).decode()

resp = client.messages.create(
    model="claude-haiku-4-5",
    max_tokens=2048,
    messages=[{
        "role": "user",
        "content": [
            {"type": "document", "source": {
                "type": "base64",
                "media_type": "application/pdf",
                "data": encoded,
            }},
            {"type": "text", "text": "Identify the three biggest risks."},
        ],
    }],
)
```

For RAG-style workloads where a single PDF exceeds 100 pages or the total
prompt blows past the context window, do not send the PDF whole — embed and
retrieve top-k chunks (see §5–§7 of the [main primer](./README.md)).

## The no-embeddings caveat

Anthropic intentionally does not ship an embedding endpoint:

> Anthropic does not offer its own embedding model. One embeddings provider
> that has a wide variety of options and capabilities encompassing all of the
> above considerations is Voyage AI.
>
> — [Anthropic embeddings docs](https://platform.claude.com/docs/en/build-with-claude/embeddings)

Practical consequences for an AI app that pairs Anthropic with RAG:

- Pick **one** embedding provider and stick to it across the whole pipeline.
  Mixing providers breaks the vector space — a chunk embedded by Voyage cannot
  be compared with a query embedded by Gemini.
- Demos in this repo standardise on `gemini-embedding-001` (768-dim
  configuration) for the embedding step regardless of which provider serves
  chat. See the [Gemini primer](./google-gemini-api.md) §Embeddings.

## Context-window economics

Both 200 k and 1 M context options exist; bigger is not always better:

| Use case                                                  | Recommended model | Why                                                 |
| --------------------------------------------------------- | ----------------- | --------------------------------------------------- |
| Chat over RAG-retrieved chunks (top-k ≤ 8, ≤ 10 k tokens) | Haiku 4.5         | Cheapest; latency feels instant                     |
| Long-document analysis (single 50 k–150 k token doc)      | Haiku 4.5         | Fits in 200 k window with conversation history room |
| Multi-document analysis (≥ 200 k combined input)          | Sonnet 4.6        | 1 M window swallows the entire corpus               |
| Tool-use loops with intermediate scratchpad turns         | Sonnet 4.6        | Better instruction following on multi-step plans    |

Long context is also expensive — every token is billed every turn. RAG (§7 of
the main primer) almost always beats raw long-context for cost.

## Prompt caching (briefly)

Anthropic offers prompt caching for repeated prefixes (system prompt, large
fixed context). Cached writes cost more than uncached input on the first turn
but ~90 % less on subsequent turns. Out of scope for most demos but worth
knowing it exists when a system prompt grows large enough to dominate per-turn
cost. See the [prompt-caching docs](https://platform.claude.com/docs/en/build-with-claude/prompt-caching).

## Reference cost (2026-Q2)

Indicative public list prices per million tokens; verify current values at
[platform.claude.com/pricing](https://platform.claude.com/pricing) before
publishing user-facing budget tools.

| Model      | Input | Output |
| ---------- | ----: | -----: |
| Haiku 4.5  | $1.00 |  $5.00 |
| Sonnet 4.5 | $3.00 | $15.00 |
| Sonnet 4.6 | $3.00 | $15.00 |

A 5-turn RAG-shaped chat session at ~5 k input / ~400 output tokens per turn
costs about $0.04 on Haiku — the cost cap layer in §12 of the main primer
exists to bound the worst case, not the steady state.

## CI mocking pattern

Real Anthropic calls are non-deterministic and cost money — neither belongs in
CI. The convention in this repo: route the SDK through `httpx` (which both
Python SDKs use under the hood) and inject a fixture via `pytest-httpx`.

```python
import json
import pytest
from httpx import Response

@pytest.fixture
def mock_anthropic(httpx_mock):
    httpx_mock.add_response(
        url="https://api.anthropic.com/v1/messages",
        method="POST",
        json={
            "id": "msg_test_001",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "FIXTURE_RESPONSE"}],
            "model": "claude-haiku-4-5",
            "stop_reason": "end_turn",
            "usage": {"input_tokens": 10, "output_tokens": 5},
        },
    )
    return httpx_mock
```

Tests then assert on:

- the **outbound request** the SDK produced (model id, message shape,
  headers) — the fingerprint of what we sent;
- the **side effects** the handler had (DB rows written, usage row upserted);
- the **structural shape** of the response the handler emitted (well-formed
  SSE / JSON, schema correct);
- never on the **prose** of the LLM output.

See the main primer §13 for the full determinism strategy.

## Related

- [AI Application Development](./README.md) — generic primer covering tokens,
  embeddings, RAG, streaming, guardrails, evaluation, cost
- [Google Gemini API Primer](./google-gemini-api.md) — paired vendor doc;
  embeddings live there
- [Perplexity API Primer](./perplexity-api.md) — when web-grounded answers are
  the requirement
- [Anthropic API docs](https://platform.claude.com/docs/) — authoritative
  reference, supersedes anything here on conflict
