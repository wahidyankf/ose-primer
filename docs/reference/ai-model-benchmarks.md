---
title: "AI Model Benchmarks Reference"
description: Cited benchmark scores for all Claude and OpenCode Go models used in this project, with confidence levels and source URLs
category: reference
tags:
  - ai-models
  - benchmarks
  - model-selection
  - reference
---

# AI Model Benchmarks Reference

Canonical benchmark reference for all AI models used in this project. All scores cited
with source URL, publication date, and confidence level. Last updated: 2026-07-05.

## Purpose and Scope

This document provides cited benchmark data for the models used across Claude Code
(`.claude/agents/`) and OpenCode (`.opencode/agents/`) runtimes. Tier assignments in
[model-selection.md](../../repo-governance/development/agents/model-selection.md) reference
this document. Every benchmark claim in policy docs links back here; this document links
to primary sources.

**Confidence levels used throughout**:

- `[Verified]` — corroborated across multiple independent sources
- `[Self-reported]` — from vendor only; no independent replication confirmed
- `[Needs Verification]` — circulating in aggregators; primary source not confirmed

## Benchmark Definitions

| Benchmark          | What it measures                                                            | Why it matters for coding agents                         |
| ------------------ | --------------------------------------------------------------------------- | -------------------------------------------------------- |
| SWE-bench Verified | Real-world GitHub issues resolved end-to-end (verified subset, ~500 issues) | Most task-relevant for agentic software work             |
| SWE-bench Pro      | Harder real-world issues; less solution leakage risk than Verified          | Discriminates frontier models better than Verified       |
| Terminal-Bench 2.1 | Autonomous agent navigation of real shell and system environments           | Direct signal for CLI-native agent capability            |
| GPQA Diamond       | Expert-level science questions (graduate difficulty)                        | Proxy for deep reasoning; correlates with complex tasks  |
| AIME               | Math olympiad problems                                                      | Mathematical reasoning depth                             |
| OSWorld-Verified   | GUI/computer-use task completion                                            | Relevant for computer-use agents                         |
| HumanEval          | Python function synthesis from docstrings                                   | Largely saturated at frontier level; less discriminative |

## Claude Models (Anthropic)

### Claude Opus 4.8

**Primary sources**:
[Anthropic Models Overview](https://platform.claude.com/docs/en/about-claude/models/overview) ·
[VentureBeat: Claude Opus 4.8 is here](https://venturebeat.com/technology/anthropics-claude-opus-4-8-is-here-with-3x-cheaper-fast-mode-and-near-mythos-level-alignment) (2026-05-28)

| Benchmark          | Score                  | Confidence   | Source                                   |
| ------------------ | ---------------------- | ------------ | ---------------------------------------- |
| SWE-bench Verified | 88.6%                  | `[Verified]` | VentureBeat launch coverage (2026-05-28) |
| SWE-bench Pro      | 69.2%                  | `[Verified]` | VentureBeat launch coverage              |
| OSWorld-Verified   | 83.4%                  | `[Verified]` | VentureBeat launch coverage              |
| Pricing            | $5/$25 per MTok in/out | —            | Official API docs                        |

**Usage in this repo**: Budget-adaptive inherit — Max/Team Premium sessions; the "thinking"-tier
comparison bar for the OpenCode/Pi secondary-binding mapping below.

**Scope note (2026-07-05 refresh)**: this repo's benchmarks doc previously tracked Claude Opus 4.7
(now superseded); only SWE-bench Verified/Pro and OSWorld-Verified were re-verified for Opus 4.8 in
this refresh. GPQA Diamond, CursorBench, and other secondary benchmarks were not re-researched for
the current model.

### Claude Sonnet 5

**Primary sources**:
[Introducing Claude Sonnet 5](https://www.anthropic.com/news/claude-sonnet-5) (2026-06-30) ·
[MarkTechPost: Sonnet 5 vs Sonnet 4.6 vs Opus 4.8](https://www.marktechpost.com/2026/06/30/anthropic-claude-sonnet-5-vs-sonnet-4-6-vs-opus-4-8-agentic-coding-benchmarks-api-pricing-and-cost-performance-tradeoffs-compared/)

| Benchmark          | Score                                                      | Confidence   | Source                                             |
| ------------------ | ---------------------------------------------------------- | ------------ | -------------------------------------------------- |
| SWE-bench Verified | 85.2%                                                      | `[Verified]` | Official launch post; corroborated by MarkTechPost |
| SWE-bench Pro      | 63.2%                                                      | `[Verified]` | Official launch post; corroborated by MarkTechPost |
| Terminal-Bench 2.1 | 80.4%                                                      | `[Verified]` | Official launch post; corroborated by MarkTechPost |
| OSWorld-Verified   | 81.2%                                                      | `[Verified]` | Official launch post; corroborated by MarkTechPost |
| Pricing            | $2→$3 (intro through 2026-08-31) / $10→$15 per MTok in/out | —            | Official API docs                                  |

**Usage in this repo**: Budget-adaptive inherit for Pro/Standard sessions; explicit
`model: sonnet` for structured/validation tasks; the "execution"-tier comparison bar for the
OpenCode/Pi secondary-binding mapping below.

### Claude Haiku 4.5

**Primary sources**:
[Anthropic Models Overview](https://platform.claude.com/docs/en/about-claude/models/overview) ·
[Introducing Claude Haiku 4.5](https://www.anthropic.com/news/claude-haiku-4-5) (2025-10-15)

| Benchmark          | Score                 | Confidence             | Source                                                              |
| ------------------ | --------------------- | ---------------------- | ------------------------------------------------------------------- |
| SWE-bench Verified | 73.3%                 | `[Verified]`           | Official release post (2025-10-15); 50-trial avg, 128k think budget |
| GPQA Diamond       | 67.2%                 | `[Needs Verification]` | Artificial Analysis aggregator; not traced to system card           |
| AIME 2025          | 83.7%                 | `[Needs Verification]` | Aggregator-cited; primary source not confirmed                      |
| Context window     | 200k tokens           | —                      | Official API docs                                                   |
| Pricing            | $1/$5 per MTok in/out | —                      | Official API docs                                                   |

**Usage in this repo**: Explicit `model: haiku` for purely mechanical tasks (URL
validation, deterministic file operations); the "fast"-tier alias mapped to OpenCode below.

**Note**: GPQA/AIME figures circulate in aggregators but primary Haiku 4.5 system card
was not directly accessible. Treat `[Needs Verification]` scores as approximate until
confirmed. Haiku 3 (`claude-3-haiku-20240307`) was retired 2026-04-19.

### Claude Opus 4.7

| Model           | SWE-bench Verified | SWE-bench Pro      | GPQA Diamond       | Pricing (in/out MTok) | Release    |
| --------------- | ------------------ | ------------------ | ------------------ | --------------------- | ---------- |
| Claude Opus 4.7 | 87.6% `[Verified]` | 64.3% `[Verified]` | 94.2% `[Verified]` | $5/$25                | 2026-04-16 |

Superseded by Claude Opus 4.8 (2026-05-28, above). Retained here for historical reference, not as a
current tier bar.

### Claude Sonnet 4.6

| Model             | SWE-bench Verified | SWE-bench Pro | GPQA Diamond       | Pricing (in/out MTok) | Release    |
| ----------------- | ------------------ | ------------- | ------------------ | --------------------- | ---------- |
| Claude Sonnet 4.6 | 79.6% `[Verified]` | —             | 89.9% `[Verified]` | $3/$15                | 2026-02-17 |

Superseded by Claude Sonnet 5 (2026-06-30, above). Retained here for historical reference, not as a
current tier bar.

## OpenCode Go Models (opencode-go/ provider)

**"Claude Opus 5" does not exist.** An earlier draft of the `upgrade-opencode-go-models` plan
referenced it as the thinking-tier comparison bar; research confirmed no such model has been
released as of 2026-07-05 — Claude Opus 4.8 (above) is the actual current Opus generation and the
correct thinking-tier bar. Anthropic did ship a tier _above_ Opus on 2026-06-09 — **Claude Fable 5**
(GA, SWE-bench Pro ~80.3% `[Verified]`) and **Claude Mythos 5** (gated, not GA) — but neither is
what the `opus` alias resolves to; noted here for completeness only, not tracked as a tier in this
document. No `opencode-go` roster model clears Opus 4.8's 69.2% SWE-bench Pro bar, so the thinking
tier collapses onto the execution tier's target (`glm-5.2`) — an explicit, accepted tradeoff, not an
oversight. See [AI Agent Model Selection Convention](../../repo-governance/development/agents/model-selection.md)
for the full tier-collapse rationale.

**Correction (2026-07-05)**: this section previously described a `zai-coding-plan/*` provider
(GLM-5.1/GLM-5-turbo) that does not match this repo's actual `.opencode/opencode.json` (which uses
the `opencode-go/*` provider). This refresh corrects the mismatch — the models below are what this
repo's OpenCode/Pi configuration actually resolves to.

### opencode-go/glm-5.2

**Primary sources**:
[Z.ai GLM-5.2 Docs](https://docs.z.ai/guides/llm/glm-5.2) ·
[HuggingFace zai-org/GLM-5.2](https://huggingface.co/zai-org/GLM-5.2)

| Benchmark          | Score                            | vs. Sonnet 5 (63.2%) | vs. Opus 4.8 (69.2%) | Confidence        | Source                                                    |
| ------------------ | -------------------------------- | -------------------- | -------------------- | ----------------- | --------------------------------------------------------- |
| SWE-bench Pro      | 62.1%                            | −1.1pp (noise-level) | −7.1pp               | `[Self-reported]` | Z.ai docs; HF model card                                  |
| Terminal-Bench 2.1 | 81.0%                            | +0.6pp               | —                    | `[Self-reported]` | Z.ai docs; HF model card                                  |
| Pricing            | $1.40/$4.40 per 1M tokens in/out | —                    | —                    | `[Verified]`      | [Z.ai Pricing](https://docs.z.ai/guides/overview/pricing) |
| Rate limit         | 880 req/5h                       | —                    | —                    | —                 | Tightest in the 13-model roster                           |

**Usage in this repo**: OpenCode/Pi equivalent for both thinking-tier (`opus`) and execution-tier
(`sonnet`/omit) agents — the strongest model in the opencode-go roster on every published benchmark
checked, but does not clear Claude Opus 4.8's tier separately.

### opencode-go/minimax-m3

**Primary sources**: [MiniMax M3 blog](https://www.minimax.io/blog/minimax-m3)

| Benchmark          | Score                                                               | vs. Sonnet 5 (63.2%) | Confidence        | Source                                                                         |
| ------------------ | ------------------------------------------------------------------- | -------------------- | ----------------- | ------------------------------------------------------------------------------ |
| SWE-bench Pro      | 59.0%                                                               | −4.2pp               | `[Self-reported]` | Official blog                                                                  |
| Terminal-Bench 2.1 | 66.0%                                                               | −14.4pp              | `[Self-reported]` | Official blog                                                                  |
| Pricing            | $0.30 (≤512K)/$0.60 (>512K) input, $1.20/$2.40 output per 1M tokens | —                    | `[Verified]`      | [MiniMax Pay-as-you-go](https://platform.minimax.io/docs/guides/pricing-paygo) |

**Usage in this repo**: OpenCode/Pi equivalent for fast-tier (`haiku`) agents — the closest roster
model to Claude Sonnet 5's tier without exceeding it, chosen so the fast tier stays genuinely
lighter/cheaper than doubling up on `glm-5.2`.

**Note on the live roster**: the `opencode-go` marketplace carries 13 models as of 2026-07-05
(confirmed via the `opencode models` CLI). Only the two models above are this repo's actual mapping
targets; see [docs.opencode.ai/go](https://opencode.ai/docs/go/) for the full roster if evaluating
alternatives.

## Model Selection Mapping

Cross-reference between Claude Code tiers and OpenCode models. For full tier rationale,
see [model-selection.md](../../repo-governance/development/agents/model-selection.md).

| Claude Code alias         | Claude model (2026)         | OpenCode model           | SWE-bench Verified (best confidence)        |
| ------------------------- | --------------------------- | ------------------------ | ------------------------------------------- |
| `opus` (thinking)         | `claude-opus-4-8`           | `opencode-go/glm-5.2`    | 88.6% (Opus 4.8) `[Verified]`               |
| `sonnet`/omit (execution) | `claude-sonnet-5`           | `opencode-go/glm-5.2`    | 85.2% (Sonnet 5) `[Verified]`               |
| `haiku` (fast)            | `claude-haiku-4-5-20251001` | `opencode-go/minimax-m3` | 73.3% (Claude) `[Verified]` / N/A (MiniMax) |

**Note**: `opus` and `sonnet`/omit resolve to the identical OpenCode model (`glm-5.2`) — an
intentional tier collapse (see the "OpenCode Go Models" section above), not an error. All benchmark
citations in policy docs link to the relevant section anchors above.

## Standard API Pricing (Non-Subscription, Per-Token)

Retrieved 2026-07-05 for the `upgrade-opencode-go-models` plan. These are each model's own
provider's direct pay-as-you-go rate — not the flat-rate `opencode-go` subscription price paid by
this repo's actual subscribers.

| Model             | Input $/1M                                  | Output $/1M                      | Source                                                                                                | Confidence       |
| ----------------- | ------------------------------------------- | -------------------------------- | ----------------------------------------------------------------------------------------------------- | ---------------- |
| `glm-5.2`         | $1.40                                       | $4.40                            | [Z.ai Pricing](https://docs.z.ai/guides/overview/pricing)                                             | `[Verified]`     |
| `glm-5.1`         | $1.40                                       | $4.40                            | [Z.ai Pricing](https://docs.z.ai/guides/overview/pricing)                                             | `[Verified]` (a) |
| `minimax-m3`      | $0.30 (≤512K) / $0.60 (>512K)               | $1.20 (≤512K) / $2.40 (>512K)    | [MiniMax Pay-as-you-go](https://platform.minimax.io/docs/guides/pricing-paygo)                        | `[Verified]`     |
| `minimax-m2.7`    | $0.30                                       | $1.20                            | [MiniMax Pay-as-you-go](https://platform.minimax.io/docs/guides/pricing-paygo)                        | `[Verified]` (b) |
| `kimi-k2.7-code`  | $0.95 (cache miss) / $0.19 (cache hit)      | $4.00                            | [Kimi Platform Pricing](https://platform.kimi.ai/docs/pricing/chat-k27-code)                          | `[Verified]`     |
| `kimi-k2.6`       | $0.95 (cache miss) / $0.16 (cache hit)      | $4.00                            | [Kimi Platform Pricing](https://platform.kimi.ai/docs/pricing/chat-k26)                               | `[Verified]`     |
| `deepseek-v4-pro` | $0.435 (cache miss) / $0.003625 (cache hit) | $0.87                            | [DeepSeek API Pricing](https://api-docs.deepseek.com/quick_start/pricing)                             | `[Verified]` (c) |
| `mimo-v2.5-pro`   | ¥3 / ¥0.025 (cache) ≈ $0.44 / $0.004        | ¥6 ≈ $0.88                       | [Xiaomi MiMo Pay-as-you-go](https://mimo.mi.com/docs/price/pay-as-you-go)                             | `[Verified]` (d) |
| `mimo-v2.5`       | ¥1 / ¥0.02 (cache) ≈ $0.15 / $0.003         | ¥2 ≈ $0.29                       | [Xiaomi MiMo Pay-as-you-go](https://mimo.mi.com/docs/price/pay-as-you-go)                             | `[Verified]` (d) |
| `qwen3.7-max`     | $2.50 (Intl)                                | $7.50 (Intl)                     | [Alibaba Cloud Model Studio Pricing](https://www.alibabacloud.com/help/en/model-studio/model-pricing) | `[Verified]` (e) |
| `qwen3.7-plus`    | $0.40 (0-256K) / $1.20 (256K-1M)            | $1.60 (0-256K) / $4.80 (256K-1M) | [Alibaba Cloud Model Studio Pricing](https://www.alibabacloud.com/help/en/model-studio/model-pricing) | `[Verified]` (e) |

Notes: (a) GLM-5.1/5.2 show identical official rates on Z.ai's own pricing page — some aggregators
list a lower third-party-hosted GLM-5.1 rate; that is reseller pricing, not Z.ai's. (b)
MiniMax-M2.7/M3 show identical standard-tier rates on MiniMax's own pricing page — unusual for two
model generations, worth a spot-check on next refresh. (c) DeepSeek's live official page shows no
expiry note on this rate, but a secondary source claims it is a promotion that may revert to
$1.74/$3.48 — re-verify before relying on it long-term. (d) Xiaomi MiMo publishes only CNY pricing;
USD figures converted at the CNY/USD spot rate as of 2026-07-04, not an official USD list price. (e)
Alibaba Cloud Model Studio prices by region; Singapore/International rates shown as the
globally-reachable rate — China-mainland pricing is substantially lower.

## Frontier/Big-Brand Model Reference (Informational Only — Not Available via `opencode-go`)

Current Anthropic/OpenAI/Google flagship pricing and benchmarks, retrieved 2026-07-05, purely for
cost/capability contrast — **none of these are, or will be, routed to by this repo's `convert_model()`
or Pi's model pin** (see Decision 0, `upgrade-opencode-go-models` plan `tech-docs.md`: BYOM harnesses
in this repo must not route to Anthropic, OpenAI, Google, or other frontier/big-brand providers).

| Provider  | Model                    | SWE-bench Pro                             | SWE-bench Verified    | Input $/1M                    | Output $/1M                     | Release date | Confidence                                             |
| --------- | ------------------------ | ----------------------------------------- | --------------------- | ----------------------------- | ------------------------------- | ------------ | ------------------------------------------------------ |
| Anthropic | Claude Opus 4.8          | 69.2%                                     | 88.6%                 | $5.00                         | $25.00                          | 2026-05-28   | `[Verified]`                                           |
| Anthropic | Claude Sonnet 5          | 63.2%                                     | 85.2%                 | $2.00→$3.00 (a)               | $10.00→$15.00                   | 2026-06-30   | `[Verified]`                                           |
| Anthropic | Claude Fable 5           | 80.3%                                     | ~95.0% (b)            | not confirmed                 | not confirmed                   | 2026-06-09   | Benchmark `[Verified]`; pricing `[Needs Verification]` |
| OpenAI    | GPT-5.5 (flagship)       | 58.6% (c)                                 | not reported (d)      | $5.00                         | $30.00                          | 2026-04-24   | Pricing `[Verified]`; benchmark `[Unverified]`         |
| OpenAI    | GPT-5.4 (prior flagship) | 59.10% ±3.56% (e)                         | not reported (d)      | $2.50                         | $15.00                          | 2026-03-05   | `[Verified]`                                           |
| OpenAI    | GPT-5.4-mini             | not found                                 | not reported          | $0.75                         | $4.50                           | 2026-03-17   | Pricing `[Verified]`; benchmark `[Needs Verification]` |
| OpenAI    | GPT-5.4-nano             | not found                                 | not reported          | $0.20                         | $1.25                           | 2026-03-17   | Pricing `[Verified]`; benchmark `[Needs Verification]` |
| Google    | Gemini 3.1 Pro (Preview) | 54.2% (self-reported) / 46.10% ±3.60% (f) | 80.6%                 | $2.00 (≤200k) / $4.00 (>200k) | $12.00 (≤200k) / $18.00 (>200k) | 2026-02-19   | `[Verified]` (dual-sourced conflict noted)             |
| Google    | Gemini 3.5 Flash         | 55.1%                                     | not on model card (g) | $1.50                         | $9.00                           | 2026-05-19   | `[Verified]`                                           |

Notes: (a) introductory rate through 2026-08-31, then standard rate. (b) third-party transcription
of an image-embedded table on Anthropic's own announcement page — treat as directionally correct,
not exact. (c) quoted consistently across independent outlets citing OpenAI's own announcement; the
primary page returned HTTP 403 on every direct fetch attempt. (d) OpenAI has publicly stopped
reporting SWE-bench Verified for current-generation models (training-data contamination/reward-
hacking concerns); recommends SWE-bench Pro instead. Last officially-reported Verified figure was
GPT-5.2 Thinking at 80% (2026-12-11), two generations behind current. (e) Scale AI's independent
SWE-bench Pro leaderboard, xHigh reasoning setting — not vendor-self-reported. (f) Google's own
model card self-reports 54.2%; the independent Scale AI leaderboard scores the same model at
46.10% ± 3.60% — a real self-reported-vs-independent gap, both cited rather than silently picking
one. (g) a 78% figure circulates across secondary sources but could not be confirmed on Google's own
model card.

**Not shown**: Gemini 3.5 Pro (still limited enterprise preview, not GA/priced as of 2026-07-05).
Claude Mythos 5 (gated to Project Glasswing, not generally accessible).

## Limitations and Caveats

1. **Claude Opus 4.8/Sonnet 5 secondary benchmarks not re-researched this pass**: only SWE-bench Verified/Pro, Terminal-Bench 2.1 (Sonnet 5 only), and OSWorld-Verified were independently re-verified for the 2026-07-05 refresh. GPQA Diamond and other secondary benchmarks tracked for the superseded Opus 4.7/Sonnet 4.6 were not re-derived for the current models.

2. **"Claude Opus 5" does not exist**: an earlier plan draft referenced it as the thinking-tier bar; confirmed no such model has been released. Claude Opus 4.8 is the correct current bar.

3. **`opencode-go/glm-5.2` and `opencode-go/minimax-m3` scores are self-reported**: no independent third-party replication has been identified. Treat with appropriate skepticism compared to Claude scores corroborated by multiple outlets.

4. **Prior "GLM Models (Z.ai Coding Plan)" section was inaccurate**: it described a `zai-coding-plan/*` provider and GLM-5.1/GLM-5-turbo models that did not match this repo's actual `.opencode/opencode.json` configuration. Corrected in this refresh (2026-07-05) — see the "OpenCode Go Models" section above.

5. **Haiku 4.5 aggregator figures**: GPQA Diamond (67.2%) and AIME 2025 (83.7%) circulate in aggregators but the primary system card was not directly accessible. Marked `[Needs Verification]`.

6. **Accuracy as of**: 2026-07-05. Benchmark landscapes and the `opencode-go` roster shift without fixed cadence; re-verify scores before using for major tier assignment decisions.

## Sources

1. Anthropic Models Overview — <https://platform.claude.com/docs/en/about-claude/models/overview> (accessed 2026-07-05)
2. Introducing Claude Sonnet 5 — <https://www.anthropic.com/news/claude-sonnet-5> (2026-06-30)
3. VentureBeat: Claude Opus 4.8 is here — <https://venturebeat.com/technology/anthropics-claude-opus-4-8-is-here-with-3x-cheaper-fast-mode-and-near-mythos-level-alignment> (2026-05-28)
4. MarkTechPost: Claude Sonnet 5 vs Sonnet 4.6 vs Opus 4.8 — <https://www.marktechpost.com/2026/06/30/anthropic-claude-sonnet-5-vs-sonnet-4-6-vs-opus-4-8-agentic-coding-benchmarks-api-pricing-and-cost-performance-tradeoffs-compared/>
5. Introducing Claude Opus 4.7 — <https://www.anthropic.com/news/claude-opus-4-7> (2026-04-16) (historical)
6. Introducing Claude Sonnet 4.6 — <https://www.anthropic.com/news/claude-sonnet-4-6> (2026-02-17) (historical)
7. Introducing Claude Haiku 4.5 — <https://www.anthropic.com/news/claude-haiku-4-5> (2025-10-15)
8. Z.ai GLM-5.2 Docs — <https://docs.z.ai/guides/llm/glm-5.2>
9. HuggingFace zai-org/GLM-5.2 — <https://huggingface.co/zai-org/GLM-5.2>
10. Z.ai Pricing — <https://docs.z.ai/guides/overview/pricing>
11. MiniMax M3 blog — <https://www.minimax.io/blog/minimax-m3>
12. MiniMax Pay-as-you-go Pricing — <https://platform.minimax.io/docs/guides/pricing-paygo>
13. OpenCode Go Docs — <https://opencode.ai/docs/go/>

---
