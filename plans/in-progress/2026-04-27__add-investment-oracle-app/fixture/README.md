# Investment-Oracle Fixture PDFs

Sample financial reports used by the `investment-oracle` demo for manual smoke
tests, integration tests against a real PDF parser, and as the default sources a
template consumer can drag into the desktop app on first run.

## Files

| File                            | Issuer                  | Period covered         | Source                             | Size    | SHA-256 (first 12) |
| ------------------------------- | ----------------------- | ---------------------- | ---------------------------------- | ------- | ------------------ |
| `aapl-fy2024-10k.pdf`           | Apple Inc.              | FY2024 (≤ 28 Sep 2024) | `s2.q4cdn.com` (Apple IR CDN)      | 941 KB  | `319e1217afe7…`    |
| `msft-fy2024-annual-report.pdf` | Microsoft Corp.         | FY2024 (≤ 30 Jun 2024) | `annualreports.com` mirror         | 1.39 MB | `96de32a72064…`    |
| `tsla-fy2024-annual-report.pdf` | Tesla, Inc.             | FY2024 (≤ 31 Dec 2024) | `annualreports.com` mirror         | 1.72 MB | `2957d89daa20…`    |
| `brka-fy2024-annual-report.pdf` | Berkshire Hathaway Inc. | FY2024 (≤ 31 Dec 2024) | `berkshirehathaway.com` (official) | 1.85 MB | `6aa0a5016806…`    |

Total: ~5.9 MB across 4 files. Each PDF starts with `%PDF` magic bytes; SHA-256
hashes captured at download time (2026-04-27) and listed above.

## Why these four

They are well-known mid-cap-and-up filers spanning four industries (consumer
tech, software/cloud, automotive, holding/insurance), so a single demo run
exercises markedly different financial profiles. All four are filed annually
with the SEC (or, in BRK's case, the SEC plus a separate "owners' letter"
shareholder report), giving the demo report-generation pipeline a heterogeneous
prompt input.

NVIDIA was deliberately excluded — its FY2025 annual report PDF is ~35 MB,
which would bloat the repo for marginal demo value over an additional 10-K of
similar shape.

## License and reuse

The underlying filings are public information by SEC policy:

> Information presented on sec.gov is considered public information and may be
> copied or further distributed by users of the web site without the SEC's
> permission.
>
> — [SEC Webmaster FAQ](https://www.sec.gov/about/webmaster-frequently-asked-questions),
> accessed 2026-04-27

The Apple and Berkshire PDFs come from the issuer's own investor-relations
hosting; the Microsoft and Tesla PDFs come from `annualreports.com`, a public
aggregator that re-hosts the same filings. The MIT license that covers this
template repo applies to the fixture infrastructure (this README, the way the
files are wired into tests) — not to the filings themselves, which remain the
work of their respective issuers.

For a downstream cloner who plans to ship these fixtures inside a public
release artifact: re-verify the issuer's hosting terms first. For demo and
local development, the files are safe to use as-is.

## How the demo consumes these

- Manual smoke test (delivery checklist Phase "Manual smoke") drags one or more
  of these PDFs into the desktop app's sources panel and triggers report
  generation.
- BE integration tests load `aapl-fy2024-10k.pdf` to exercise the real `pypdf`
  ingest path and the real pgvector embedding pipeline (no mocked LLM call —
  the embedding step is mocked, the parser step is real). See PRD FR-15d and
  `tech-docs.md` Test Strategy section.
- E2E suites do **not** use these fixtures directly; they ship a tiny synthetic
  PDF (≤ 10 KB, hand-authored in `pdfkit` or similar) so the E2E run stays
  under a few hundred milliseconds for ingest. The four real reports are too
  large to push through E2E on every CI run.

## Update cadence

Refresh fixtures when:

1. A filer publishes a more recent annual report and we want the demo to feel
   current (no obligation — older reports still demonstrate the pipeline).
2. A canonical source URL stops resolving — replace with the next-most-stable
   URL and update the table above.

When refreshing, re-record the SHA-256 in the table and bump the access date
note in this README.

## Source URLs (recorded 2026-04-27)

- Apple FY2024 10-K — <https://s2.q4cdn.com/470004039/files/doc_earnings/2024/q4/filing/10-Q4-2024-As-Filed.pdf>
- Microsoft FY2024 annual report — <https://www.annualreports.com/HostedData/AnnualReports/PDF/NASDAQ_MSFT_2024.pdf>
- Tesla FY2024 annual report — <https://www.annualreports.com/HostedData/AnnualReports/PDF/NASDAQ_TSLA_2024.pdf>
- Berkshire Hathaway FY2024 annual report — <https://www.berkshirehathaway.com/2024ar/2024ar.pdf>

The canonical SEC filings (HTML) for each issuer are linked in the plan
`tech-docs.md` citation table.
