# Shared libraries

`libs/` holds small reusable building blocks for the primer’s reference apps. They are examples of
how to draw a boundary around code that more than one project can use—not a promise that every
downstream repository needs every package. Keep the libraries that help your product; remove or
replace the rest as your own architecture becomes clear. 🧩

## Current libraries

| Library                                         | What it demonstrates                                         | Used by                 |
| ----------------------------------------------- | ------------------------------------------------------------ | ----------------------- |
| [`golang-commons/`](./golang-commons/README.md) | Focused Go utilities and Gherkin-backed integration behavior | Go tooling and examples |
| [`elixir-cabbage/`](./elixir-cabbage/README.md) | Compiling Gherkin behavior for Elixir tests                  | Elixir reference apps   |
| [`elixir-gherkin/`](./elixir-gherkin/README.md) | Parsing Gherkin feature files in Elixir                      | Elixir reference apps   |
| [`ts-ui-tokens/`](./ts-ui-tokens/README.md)     | Structural design tokens shared by web interfaces            | TypeScript frontends    |
| [`ts-ui/`](./ts-ui/README.md)                   | Accessible React components built on those tokens            | TypeScript frontends    |

The repository also contains language-specific OpenAPI code-generation libraries. Their behavior
and architecture records are in [specs/libs/](../specs/README.md#library-specs); open the consuming
app README to understand when a particular generator is useful.

## Rules of thumb

- Apps may import libraries; apps do not import one another.
- Give a library one clear job and a documented public API.
- Keep dependencies narrow and avoid circular library relationships.
- Let each language use its native packaging and test tooling, with Nx providing a consistent
  workspace command surface.

## Work with a library

```bash
# Inspect a library’s available targets
npm exec nx -- show project ts-ui

# Run one library’s quick verification
npm exec nx -- run ts-ui:test:quick

# Inspect workspace dependencies
npm exec nx -- graph
```

For the reusable conventions behind these examples, see the
[monorepo structure reference](../docs/reference/monorepo-structure.md).
