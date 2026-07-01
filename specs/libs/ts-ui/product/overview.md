# ts-ui — Product Overview

`ts-ui` is the shared React component library for the `ose-primer` monorepo. It provides
accessible, themeable UI primitives built on shadcn/ui patterns, Radix UI primitives, and
Tailwind CSS:

- **Button** — CVA variants (`default`, `destructive`, `outline`, `secondary`, `ghost`, `link`),
  8 sizes, `asChild` composition
- **Alert** — CVA variants, `role="alert"` semantics, title/description subcomponents
- **Dialog** — Radix Dialog primitive, portal, overlay, close button
- **Input** — `focus-visible` ring, `aria-invalid` styling
- **Card** — subcomponents (`CardHeader`, `CardTitle`, `CardContent`, etc.) with `data-slot`
- **Label** — Radix Label primitive

Every component follows the
[Component Patterns Convention](../../../../repo-governance/development/frontend/component-patterns.md):
`React.ComponentProps<"element">` (not `forwardRef`), `radix-ui` unified imports, a `data-slot`
attribute on every root element, the `cn()` class-merge utility, and semantic design tokens only
(no hardcoded colors) sourced from `@open-sharia-enterprise/ts-ui-tokens`.

See [README.md](./README.md) for C4 L1 product framing.
