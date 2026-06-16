# Example — Funnel Stage 1 (Low-Fidelity Alternatives)

ASCII / Unicode wireframes in fenced code blocks are cheap, so this is where the design **diverges**:
present several genuinely different layouts for the screen, name them, and let reviewers compare
line-by-line. All three options below target the same screen — a CRUD **entity list + create/edit
form** (an expense tracker, the kind `apps/crud-fe-dart-flutterweb` ships) — and all reuse `libs/ts-ui`
components (`dialog`, `input`, `button`, `card`, `label`) plus two net-new primitives (`table` for the
list, `select` for the dropdowns) flagged in [assets/README.md](./README.md).

These feed Stage 2 (hi-fi shortlist) and Stage 3 (selection) — see
[assets/README.md](./README.md) for which option was chosen and why.

## Option A — Table list + modal form

A table of records with a **New** button that opens a modal create/edit form. Densest; keeps the
full list in view behind the dialog.

```
┌────────────────────────────────────────────────────────────┐
│  Expenses                                      [ + New ]   │
├────────────────────────────────────────────────────────────┤
│  Date        Description     Category   Amount       ⋯     │
│  ──────────  ──────────────  ────────   ──────             │
│  2026-06-01  Office lunch     Meals     $42.00     ✎ ✕     │
│  2026-06-03  Train pass       Transit   $88.00     ✎ ✕     │
│  2026-06-05  Cloud hosting    Software  $120.00    ✎ ✕     │
│                                                            │
│      ┌── New expense ──────────────────┐ ← modal          │
│      │ Date        [ 2026-06-08      ]  │                  │
│      │ Description [__________________] │                  │
│      │ Category    [ Meals          ▼] │                  │
│      │ Amount      [ 0.00 ]  [ USD ▼ ] │                  │
│      │           [ Cancel ]  [ Save ]  │                  │
│      └──────────────────────────────────┘                  │
└────────────────────────────────────────────────────────────┘
```

## Option B — Master-detail (list left, form right)

List on the left, edit form pinned in a right pane; selecting a row loads it into the form. Good on
wide screens; the two panes compete for width and stack awkwardly on mobile.

```
┌─────────────────────────┬──────────────────────────────────┐
│ Expenses      [ + New ] │  Edit expense                    │
│ ─────────────────────── │  ─────────────────────────────   │
│ ▸ Office lunch   $42    │  Date        [ 2026-06-03      ]  │
│ ▸ Train pass     $88  ◀ │  Description [ Train pass       ] │
│ ▸ Cloud hosting  $120   │  Category    [ Transit       ▼]  │
│ ▸ Domain renew   $14    │  Amount      [ 88.00 ] [ USD ▼ ] │
│                         │                                  │
│                         │           [ Cancel ]  [ Save ]   │
└─────────────────────────┴──────────────────────────────────┘
```

## Option C — Card grid + full-page form

Records as a grid of cards; editing navigates to a dedicated full-page form. More visual, fewer
records per screen, and the page navigation loses the list context.

```
┌────────────────────────────────────────────────────────────┐
│  Expenses                                      [ + New ]   │
├────────────────────────────────────────────────────────────┤
│  ┌── Office lunch ──┐   ┌── Train pass ────┐               │
│  │  Meals · 06-01   │   │  Transit · 06-03 │               │
│  │  $42.00      ✎ ✕ │   │  $88.00      ✎ ✕ │               │
│  └──────────────────┘   └──────────────────┘               │
│  ┌── Cloud hosting ─┐   ┌── Domain renew ──┐               │
│  │  Software · 06-05│   │  Software · 06-06│               │
│  │  $120.00     ✎ ✕ │   │  $14.00      ✎ ✕ │               │
│  └──────────────────┘   └──────────────────┘               │
│            ( edit ✎ → opens full-page form )               │
└────────────────────────────────────────────────────────────┘
```
