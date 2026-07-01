# clojure-openapi-codegen — Product Overview

`clojure-openapi-codegen` exposes a single entry point:

```clojure
(require '[openapi-codegen.core :as core])
(core/generate "specs/.../openapi-bundled.yaml" "generated/schemas")
```

`generate` reads the OpenAPI spec at `spec-path` via `openapi-codegen.parser/parse-schemas`
(backed by SnakeYAML), then for every schema under `components.schemas` writes one generated
`.clj` file (`output-dir/openapi_codegen/schemas/<kebab_to_underscore_name>.clj`) via
`openapi-codegen.generator/generate-schema-files`. Each generated file declares a namespace and a
`schema` def holding a Malli `[:map ...]` form, with OpenAPI types mapped via
`openapi-type->malli` (`string` → `:string`, `integer` → `:int`, `number` → `number?`, `boolean`
→ `:boolean`, `array` → `[:vector <item-type>]`, `object` → `:map`, unknown → `:any`), and optional
properties wrapped as `[:kw {:optional true} type]`. It returns the sequence of written file
paths.

See [README.md](./README.md) for C4 L1 product framing.
