# Assets — Worked Example of the Design Funnel

These files walk the full **prior-art → diverge → narrow → select → justify** funnel for one screen —
a CRUD **entity list + create/edit form** (an expense tracker, the kind `apps/crud-fe-dart-flutterweb`
and its sibling `crud-fe-*` apps ship) — so the process and both mockup tiers can be seen end-to-end.

## Grounding note (R5) — surveyed `libs/ts-ui` inventory

Before drawing, the shared kit was surveyed. `libs/ts-ui/src/components/` ships: **`Dialog`, `Button`,
`Input`, `Card`, `Label`, `Alert`** (plus the `libs/ts-ui-tokens` palette — indigo primary, slate
neutrals). The mockups reuse those real components and tokens.

- **Reused from `libs/ts-ui`** (≥3): `Dialog` (the modal form shell), `Button` (New / Save / Cancel),
  `Input` (Date / Description / Amount fields), `Card` (Option C's record cards), `Label`.
- **Net-new primitives flagged for build** (do not yet exist in `libs/ts-ui`): a **`Table`** (the
  entity list) and a **`Select`** (the Category and currency dropdowns). These are called out here so
  the build gap is visible up front, exactly as the grounding rule (R5) requires.

| File                                                                                   | Funnel stage         | Tier   | Renders: VSCode / GitHub |
| -------------------------------------------------------------------------------------- | -------------------- | ------ | ------------------------ |
| [example-low-fi-wireframe.md](./example-low-fi-wireframe.md)                           | 1. Diverge           | Low-fi | Yes / Yes                |
| [example-hi-fi-option-a-table-modal.png](./example-hi-fi-option-a-table-modal.png)     | 2. Narrow (finalist) | Hi-fi  | Yes / Yes                |
| [example-hi-fi-option-b-master-detail.png](./example-hi-fi-option-b-master-detail.png) | 2. Narrow (finalist) | Hi-fi  | Yes / Yes                |

## Stage 0 — Prior art (cited)

Before diverging, comparable CRUD admin screens were surveyed (R7, via `web-researcher`). Every
major enterprise design system names the same triad of list + create/edit layouts, which directly
seeded Options A / B / C below:

- **Table list + modal form** — [Ant Design "List" patterns](https://2x.ant.design/docs/pattern/list)
  (accessed 2026-06-16) ("Modal");
  [Carbon Design System Create Flows](https://carbondesignsystem.com/community/patterns/create-flows/)
  (accessed 2026-06-16) ("Modal", for low-commitment single-step creates);
  [Material React Table](https://www.material-react-table.com/docs/examples/editing-crud)
  (accessed 2026-06-16) (Modal is the default editing mode). Trade-off cited: the modal "loses the
  contextual tie to the clicked list item" but keeps the list in view.
- **Master-detail (split pane)** — Ant Design "Dual-Panel Selector"; Carbon "Side-panel / Tearsheet".
  Preferred when users compare list rows against detail fields simultaneously; heavier and competes
  for width.
- **Card grid + full-page form** — Ant Design "Full-Window" / Carbon "Full-page"; preferred when the
  entity is complex or "completely irrelevant to the list", at the cost of losing list context.

## Stage 1 — Diverge (low-fi)

Three genuinely different layouts, named Option A / B / C, in
[example-low-fi-wireframe.md](./example-low-fi-wireframe.md): **A — Table list + modal form**,
**B — Master-detail**, **C — Card grid + full-page form**. Cheap ASCII, so divergence is painless
and diffable.

## Stage 2 — Narrow (hi-fi shortlist)

The two strongest low-fi options are promoted to high fidelity. Option C (Card grid + full-page form)
is dropped here.

### Finalist 1 — Option A (Table list + modal form)

![Option A — Table list with a New expense modal form, high-fidelity mockup](./example-hi-fi-option-a-table-modal.png)

### Finalist 2 — Option B (Master-detail)

![Option B — Master-detail layout with list left and edit form right, high-fidelity mockup](./example-hi-fi-option-b-master-detail.png)

## Stage 3 — Selection

**Selected: Option A — Table list + modal form.**

## Stage 4 — Rationale (decision record)

| Option                          | Outcome           | Why                                                                                                                                                                                                                                                                                                                            |
| ------------------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **A — Table list + modal form** | **Chosen**        | Simplest shape for a template demo; the table scales as records grow; the modal keeps the list in context while editing; reuses the `ts-ui` `Dialog` + `Button` + `Input` (with `Table` + `Select` as the two flagged net-new primitives); portable across every `crud-fe-*` framework (Next.js, TanStack Start, Flutter Web). |
| B — Master-detail               | Runner-up         | Comfortable on wide screens, but the two-pane layout is heavier, the panes compete for width, and it stacks awkwardly on mobile — no advantage over A for the core list + edit task.                                                                                                                                           |
| C — Card grid + full-page form  | Dropped (Stage 2) | Full-page navigation to edit loses the list context and adds routing the other two avoid; weaker for scanning many records — the primary job of this screen.                                                                                                                                                                   |

## How the hi-fi artefacts were produced

- The real plan workflow uses **Excalidraw `.excalidraw.png`** (the PNG carries an editable scene;
  edit it with the `pomdtr.excalidraw-editor` VSCode extension).
- These examples are instead **hand-authored SVGs** rasterised to PNG with `rsvg-convert -z 2`, so
  the source is fully diffable and reproducible from text. A hand-authored SVG uses system fonts, so —
  unlike `.excalidraw.svg` — it renders correctly on GitHub without the custom-font CSP fallback.
  Either route satisfies the hi-fi tier; pick Excalidraw for a drawing canvas, hand-SVG for a
  text-diffable vector source.
- Regenerate a PNG after editing its SVG:

  ```bash
  rsvg-convert -z 2 example-hi-fi-option-a-table-modal.svg -o example-hi-fi-option-a-table-modal.png
  rsvg-convert -z 2 example-hi-fi-option-b-master-detail.svg -o example-hi-fi-option-b-master-detail.png
  ```
