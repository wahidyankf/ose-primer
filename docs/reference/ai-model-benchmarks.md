---
title: "AI Model Benchmarks Reference"
description: Cited benchmark scores for all Claude and GLM models used in this project, with confidence levels and source URLs
category: reference
tags:
  - ai-models
  - benchmarks
  - model-selection
  - reference
created: 2026-04-19
updated: 2026-04-19
---

# AI Model Benchmarks Reference

Canonical benchmark reference for all AI models used in this project. All scores cited
with source URL, publication date, and confidence level. Last updated: 2026-04-19.

## Purpose and Scope

This document provides cited benchmark data for the five models used across Claude Code
(`.claude/agents/`) and OpenCode (`.opencode/agent/`) runtimes. Tier assignments in
[model-selection.md](../../governance/development/agents/model-selection.md) reference
this document. Every benchmark claim in policy docs links back here; this document links
to primary sources.

**Confidence levels used throughout**:

- `[Verified]` — corroborated across multiple independent sources
- `[Self-reported]` — from vendor only; no independent replication confirmed as of April 2026
- `[Needs Verification]` — circulating in aggregators; primary source not confirmed

## Benchmark Definitions

| Benchmark          | What it measures                                                                       | Why it matters for coding agents                         |
| ------------------ | -------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| SWE-bench Verified | Real-world GitHub issues resolved end-to-end (verified subset, ~500 issues)            | Most task-relevant for agentic software work             |
| SWE-bench Pro      | Harder real-world issues; less solution leakage risk than Verified                     | Discriminates frontier models better than Verified       |
| GPQA Diamond       | Expert-level science questions (graduate difficulty)                                   | Proxy for deep reasoning; correlates with complex tasks  |
| AIME               | Math olympiad problems                                                                 | Mathematical reasoning depth                             |
| OSWorld            | GUI/computer-use task completion                                                       | Relevant for computer-use agents                         |
| HumanEval          | Python function synthesis from docstrings                                              | Largely saturated at frontier level; less discriminative |
| ZClawBench         | Proprietary Z.ai benchmark — no independent validation; included for completeness only | Treat as self-reported context, not evidence             |

## Claude Models (Anthropic)

### Claude Opus 4.7

**Primary sources**:
[Anthropic Models Overview](https://platform.claude.com/docs/en/about-claude/models/overview)
(official API docs, accessed 2026-04-19) ·
[Introducing Claude Opus 4.7](https://www.anthropic.com/news/claude-opus-4-7) (2026-04-16)

| Benchmark          | Score                  | Confidence   | Source                                                          |
| ------------------ | ---------------------- | ------------ | --------------------------------------------------------------- |
| SWE-bench Verified | 87.6%                  | `[Verified]` | Third-party corroboration: VentureBeat, BenchLM.ai (2026-04-16) |
| SWE-bench Pro      | 64.3%                  | `[Verified]` | Official release post via VentureBeat (2026-04-16)              |
| GPQA Diamond       | 94.2%                  | `[Verified]` | Multiple aggregators citing official release (2026-04-16)       |
| CursorBench        | 70%                    | `[Verified]` | Official Anthropic release post (2026-04-16)                    |
| Context window     | 1M tokens              | —            | Official API docs (2026-04-19)                                  |
| Pricing            | $5/$25 per MTok in/out | —            | Official API docs (2026-04-19)                                  |

**Usage in this repo**: Budget-adaptive inherit — Max/Team Premium sessions.

**Note**: System card PDF inaccessible at research time; numbers corroborated across
credible third-party outlets. Confirm against
[Claude Opus 4.7 System Card](https://www.anthropic.com/claude-opus-4-7-system-card)
when accessible.

### Claude Sonnet 4.6

**Primary sources**:
[Anthropic Models Overview](https://platform.claude.com/docs/en/about-claude/models/overview) ·
[Introducing Claude Sonnet 4.6](https://www.anthropic.com/news/claude-sonnet-4-6) (2026-02-17) ·
[Claude Sonnet 4.6 System Card](https://www.anthropic.com/claude-sonnet-4-6-system-card)

| Benchmark              | Score                  | Confidence   | Source                                                             |
| ---------------------- | ---------------------- | ------------ | ------------------------------------------------------------------ |
| SWE-bench Verified     | 79.6%                  | `[Verified]` | Official release post (2026-02-17); 80.2% with prompt modification |
| OSWorld (computer use) | 72.5%                  | `[Verified]` | NxCode, Morph benchmarks citing Anthropic (2026-03-05)             |
| GPQA Diamond           | 89.9%                  | `[Verified]` | System card (10-trial avg, adaptive thinking, max effort)          |
| AIME 2025              | 95.6%                  | `[Verified]` | System card (10-trial avg, adaptive thinking, max effort)          |
| Context window         | 1M tokens              | —            | Official API docs (2026-04-19); beta                               |
| Pricing                | $3/$15 per MTok in/out | —            | Official API docs (2026-04-19)                                     |

**Usage in this repo**: Budget-adaptive inherit for Pro/Standard sessions; explicit
`model: sonnet` for structured/validation tasks.

### Claude Haiku 4.5

**Primary sources**:
[Anthropic Models Overview](https://platform.claude.com/docs/en/about-claude/models/overview) ·
[Introducing Claude Haiku 4.5](https://www.anthropic.com/news/claude-haiku-4-5) (2025-10-15)

| Benchmark          | Score                 | Confidence             | Source                                                              |
| ------------------ | --------------------- | ---------------------- | ------------------------------------------------------------------- |
| SWE-bench Verified | 73.3%                 | `[Verified]`           | Official release post (2025-10-15); 50-trial avg, 128k think budget |
| GPQA Diamond       | 67.2%                 | `[Needs Verification]` | Artificial Analysis aggregator; not traced to system card           |
| AIME 2025          | 83.7%                 | `[Needs Verification]` | Aggregator-cited; primary source not confirmed                      |
| Context window     | 200k tokens           | —                      | Official API docs (2026-04-19)                                      |
| Pricing            | $1/$5 per MTok in/out | —                      | Official API docs (2026-04-19)                                      |

**Usage in this repo**: Explicit `model: haiku` for purely mechanical tasks (URL
validation, deterministic file operations).

**Note**: GPQA/AIME figures circulate in aggregators but primary Haiku 4.5 system card
was not directly accessible. Treat `[Needs Verification]` scores as approximate until
confirmed. Haiku 3 (`claude-3-haiku-20240307`) was retired 2026-04-19.

## GLM Models (Z.ai Coding Plan / OpenCode)

### GLM-5.1

**Primary sources**:
[Z.ai GLM-5.1 release](https://officechai.com/ai/z-ai-glm-5-1-benchmarks-swe-bench-pro/) (OfficeChai, 2026-04-07) ·
[Awesome Agents review](https://awesomeagents.ai/reviews/review-glm-5-1/) (2026-04-17) ·
[WaveSpeedAI comparison](https://wavespeed.ai/blog/posts/glm-5-1-vs-claude-gpt-gemini-deepseek-llm-comparison/) (2026-03-30)

| Benchmark          | Score                       | Confidence        | Source                                                                  |
| ------------------ | --------------------------- | ----------------- | ----------------------------------------------------------------------- |
| SWE-bench Pro      | 58.4                        | `[Self-reported]` | Z.ai self-reported; no independent third-party replication (2026-04-17) |
| SWE-bench Verified | 77.8%                       | `[Self-reported]` | WaveSpeedAI citing Z.ai (2026-03-30)                                    |
| GPQA Diamond       | 86.2                        | `[Self-reported]` | OfficeChai citing Z.ai (2026-04-07)                                     |
| Arena.ai Code Elo  | 1530 (rank 3)               | `[Verified]`      | Arena.ai leaderboard (partial corroboration; 2026-04-17)                |
| Context window     | 200k tokens                 | —                 | Multiple sources (2026-04-07)                                           |
| Pricing            | $1.00/$3.20 per MTok in/out | —                 | OfficeChai (2026-04-07)                                                 |

**Usage in this repo**: OpenCode equivalent for both opus-tier (omit) and sonnet-tier
agents. The 3-to-2 tier collapse means both map to this model.

**Critical flag**: As of 2026-04-17, "a fully independent evaluation on SWE-Bench Pro
from a third-party lab hasn't been published" (Awesome Agents review). SWE-bench Pro 58.4
is a self-reported headline claim. Arena.ai Code Elo (rank 3) provides partial
corroboration only.

### GLM-5-turbo

**Primary sources**:
[Z.ai GLM-5-turbo Developer Docs](https://docs.z.ai/guides/llm/glm-5-turbo) (official) ·
[OpenRouter GLM-5-turbo](https://openrouter.ai/z-ai/glm-5-turbo) (pricing)

| Benchmark                           | Score                       | Confidence        | Source                                                    |
| ----------------------------------- | --------------------------- | ----------------- | --------------------------------------------------------- |
| ZClawBench                          | 56.4                        | `[Self-reported]` | Proprietary Z.ai benchmark; no independent validation     |
| SWE-bench / GPQA / MMLU / HumanEval | N/A                         | —                 | **No standard benchmark scores published** for this model |
| Context window                      | 202k tokens                 | —                 | OpenRouter (2026-03-16)                                   |
| Pricing                             | $1.20/$4.00 per MTok in/out | —                 | OpenRouter (2026-03-16)                                   |

**Usage in this repo**: OpenCode equivalent for haiku-tier agents only.

> **Critical flag**: GLM-5-turbo has **no published scores on any standard academic
> benchmark** (no SWE-bench, GPQA, MMLU, or HumanEval data as of April 2026). Its use
> as the OpenCode fast tier is a platform constraint — it is the only alternative to
> GLM-5.1 in Z.ai Coding Plan — not a benchmark-validated choice. ZClawBench is
> proprietary and unverifiable.

## Model Selection Mapping

Cross-reference between Claude Code tiers and OpenCode models. For full tier rationale,
see [model-selection.md](../../governance/development/agents/model-selection.md).

| Claude Code alias | Claude model (April 2026)   | OpenCode model                | SWE-bench Verified (best confidence)    |
| ----------------- | --------------------------- | ----------------------------- | --------------------------------------- |
| `""` (omit)       | Inherits session model      | `zai-coding-plan/glm-5.1`     | 87.6% (Opus 4.7) / 79.6% (Sonnet 4.6)   |
| `sonnet`          | `claude-sonnet-4-6`         | `zai-coding-plan/glm-5.1`     | 79.6% `[Verified]`                      |
| `haiku`           | `claude-haiku-4-5-20251001` | `zai-coding-plan/glm-5-turbo` | 73.3% `[Verified]` (Claude) / N/A (GLM) |
| `opus`            | `claude-opus-4-7`           | `zai-coding-plan/glm-5.1`     | 87.6% `[Verified]`                      |

**Note**: All benchmark citations in policy docs link to the relevant section anchors
above (e.g., `#claude-opus-47`, `#glm-51`).

## Limitations and Caveats

1. **System card inaccessibility**: Claude Opus 4.7 system card PDF was inaccessible at
   research time (2026-04-19). Numbers corroborated through third-party outlets. Verify
   against official system card when accessible.

2. **GLM-5.1 self-reported scores**: No independent third-party replication of SWE-bench
   Pro 58.4 published as of 2026-04-17. Arena.ai Code Elo provides partial corroboration.
   Treat all GLM-5.1 benchmark numbers as `[Self-reported]` pending independent validation.

3. **GLM-5-turbo: no standard benchmarks**: No SWE-bench, GPQA, MMLU, or HumanEval
   scores published for this model. ZClawBench is proprietary and unverifiable. Capability
   claims are unsubstantiated by standard academic evaluation.

4. **Haiku 4.5 aggregator figures**: GPQA Diamond (67.2%) and AIME 2025 (83.7%) circulate
   in aggregators but primary system card was not directly accessible. Marked
   `[Needs Verification]`.

5. **Accuracy as of**: 2026-04-19. Benchmark landscapes shift; re-verify scores before
   using for major tier assignment decisions.

## Sources

1. Anthropic Models Overview — <https://platform.claude.com/docs/en/about-claude/models/overview> (accessed 2026-04-19)
2. Introducing Claude Opus 4.7 — <https://www.anthropic.com/news/claude-opus-4-7> (2026-04-16)
3. Claude Opus 4.7 System Card — <https://www.anthropic.com/claude-opus-4-7-system-card>
4. Introducing Claude Sonnet 4.6 — <https://www.anthropic.com/news/claude-sonnet-4-6> (2026-02-17)
5. Claude Sonnet 4.6 System Card — <https://www.anthropic.com/claude-sonnet-4-6-system-card>
6. Introducing Claude Haiku 4.5 — <https://www.anthropic.com/news/claude-haiku-4-5> (2025-10-15)
7. Z.ai GLM-5.1 benchmarks (OfficeChai) — <https://officechai.com/ai/z-ai-glm-5-1-benchmarks-swe-bench-pro/> (2026-04-07)
8. Awesome Agents GLM-5.1 review — <https://awesomeagents.ai/reviews/review-glm-5-1/> (2026-04-17)
9. WaveSpeedAI GLM-5.1 comparison — <https://wavespeed.ai/blog/posts/glm-5-1-vs-claude-gpt-gemini-deepseek-llm-comparison/> (2026-03-30)
10. Z.ai GLM-5-turbo Developer Docs — <https://docs.z.ai/guides/llm/glm-5-turbo>
11. OpenRouter GLM-5-turbo — <https://openrouter.ai/z-ai/glm-5-turbo> (2026-03-16)
12. Arena.ai Code leaderboard — <https://arena.ai> (2026-04-17)

---

**Last Updated**: 2026-04-19
