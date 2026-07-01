# elixir-openapi-codegen — Product Overview

`elixir-openapi-codegen` exposes a single entry point:

```elixir
OpenApiCodegen.generate(
  "path/to/openapi-bundled.yaml",
  "path/to/output_dir",
  namespace: "MyApp.Schemas"
)
```

`generate/3` reads the OpenAPI spec at `spec_path` via `OpenApiCodegen.Parser`, then for every
schema under `components.schemas` writes one generated `.ex` file
(`output_dir/<namespace_dir>/<schema_name>.ex`) via `OpenApiCodegen.Generator`. Each generated
module defines a `defstruct` with `@enforce_keys` for required fields, an inline `@type t()`
typespec derived from the schema's property types, and a `# DO NOT EDIT` header. It returns
`{:ok, [file_path]}` on success or `{:error, reason}` if any schema fails to parse or generate.

See [README.md](./README.md) for C4 L1 product framing.
