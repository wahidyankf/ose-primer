# Component Diagram: rhino-cli `docs` Command Handler

Level 3 of the C4 model for the rhino-cli demo application. Shows the internal structure of the
`docs` command namespace — the command handlers, validation engines, and output layer — in the
CLI implementation (`apps/rhino-cli/`).

```mermaid
%% Color Palette: Blue #0173B2 | Orange #DE8F05 | Teal #029E73 | Purple #CC78BC | Brown #CA9161 | Gray #808080
graph LR
    DEV("Developer"):::actor

    subgraph DOCS["docs namespace"]

        subgraph CMDS["Command Handlers"]
            VL["validate-links handler<br/>────────────────<br/>Parse flags<br/>Resolve file list<br/>Delegate to engine"]:::handler
            VM["validate-mermaid handler<br/>────────────────<br/>Parse flags<br/>Resolve file list<br/>Delegate to engine"]:::handler
            VH["validate-heading handler<br/>────────────────<br/>Parse flags<br/>Apply allowlist<br/>Delegate to engine"]:::handler
        end

        subgraph ENGINES["Validation Engines"]
            LE["Link Validation Engine<br/>────────────────<br/>Scan markdown<br/>Resolve links<br/>Detect broken refs"]:::service
            ME["Mermaid Validation Engine<br/>────────────────<br/>Extract blocks<br/>Parse flowcharts<br/>Check thresholds"]:::service
            HE["Heading Validation Engine<br/>────────────────<br/>Scan headings<br/>Enforce allowlist<br/>Detect violations"]:::service
        end

        subgraph OUTPUT["Output Layer"]
            FMT["Formatter<br/>────────────────<br/>text (default)<br/>json<br/>markdown"]:::infra
        end

    end

    FS[("Repository<br/>markdown files")]:::datastore
    GIT[("git index<br/>staged / upstream diff")]:::datastore

    DEV -->|"rhino-cli docs validate-links [flags]"| VL
    DEV -->|"rhino-cli docs validate-mermaid [flags]"| VM
    DEV -->|"rhino-cli docs validate-heading-hierarchy [flags]"| VH

    VL --> LE
    VM --> ME
    VH --> HE
    LE --> FMT
    ME --> FMT
    HE --> FMT
    LE --> FS
    LE --> GIT
    ME --> FS
    ME --> GIT
    HE --> FS
    HE --> GIT

    classDef actor fill:#DE8F05,stroke:#000000,color:#000000,stroke-width:2px
    classDef handler fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef service fill:#029E73,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef infra fill:#808080,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef datastore fill:#029E73,stroke:#000000,color:#FFFFFF,stroke-width:2px
```

## `docs validate-links`

Scans markdown files for broken internal links. External URLs, absolute paths, and placeholder
links are skipped automatically. By default the command scans `docs/`, `repo-governance/`,
`.claude/`, `plans/`, and root `*.md` files. Auto-generated skill files under `.opencode/skill/`
are always excluded.

### Flags

| Flag            | Type                | Default | Description                                                                                                                                                                     |
| --------------- | ------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--staged-only` | bool                | `false` | Only validate files currently staged in git. Useful in pre-commit hooks.                                                                                                        |
| `--exclude`     | string (repeatable) | —       | **Incoming.** Exclude a path prefix from validation. Can be supplied multiple times. Values are appended to the internal skip list after the built-in `.opencode/skill/` entry. |

### Global flags (inherited)

| Flag         | Short | Type   | Default | Description                                |
| ------------ | ----- | ------ | ------- | ------------------------------------------ |
| `--output`   | `-o`  | string | `text`  | Output format: `text`, `json`, `markdown`. |
| `--verbose`  | `-v`  | bool   | `false` | Verbose output with timestamps.            |
| `--quiet`    | `-q`  | bool   | `false` | Quiet mode — errors only.                  |
| `--no-color` | —     | bool   | `false` | Disable colored output.                    |
| `--say`      | —     | string | `""`    | Echo a message to stdout (utility flag).   |
| `--help`     | `-h`  | bool   | `false` | Print help.                                |

### Examples

```bash
# Validate all markdown files
rhino-cli docs validate-links

# Validate only staged files (pre-commit hook)
rhino-cli docs validate-links --staged-only

# Output as JSON
rhino-cli docs validate-links -o json

# Output as markdown report
rhino-cli docs validate-links -o markdown

# Exclude a directory tree from validation (incoming)
rhino-cli docs validate-links --exclude plans/done

# Combine exclusions
rhino-cli docs validate-links --exclude plans/done --exclude archived
```

### Implementation references

| Implementation | Flag struct         | Handler              | Source                                |
| -------------- | ------------------- | -------------------- | ------------------------------------- |
| Rust (clap)    | `ValidateLinksArgs` | `run_validate_links` | `apps/rhino-cli/src/commands/docs.rs` |

---

## `docs validate-mermaid`

Scans markdown files and validates Mermaid flowchart diagrams for structural issues. Three rules
are enforced on `flowchart` and `graph` blocks:

1. Node label length must not exceed `--max-label-len`.
2. Maximum parallel nodes at one rank must not exceed `--max-width`. When both span exceeds
   `--max-width` AND depth exceeds `--max-depth`, a warning is emitted instead of an error.
3. Each Mermaid code block must contain exactly one diagram.

Non-flowchart Mermaid types (`sequenceDiagram`, `classDiagram`, `gantt`, etc.) are silently
ignored. The command is read-only — it never modifies any file.

### Flags

| Flag                   | Type                | Default         | Description                                                                                                                                                                                                   |
| ---------------------- | ------------------- | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--staged-only`        | bool                | `false`         | Only validate files currently staged in git (pre-commit use).                                                                                                                                                 |
| `--changed-only`       | bool                | `false`         | Only validate files changed since upstream (`@{u}..HEAD`). Falls back to default directory scan when no upstream exists or the diff is empty. Pre-push use.                                                   |
| `--max-label-len`      | int                 | `30`            | Maximum characters in a node label. Default approximates Mermaid `wrappingWidth:200px` at 16 px font.                                                                                                         |
| `--max-width`          | int                 | `4`             | Maximum nodes at the same rank.                                                                                                                                                                               |
| `--max-depth`          | int                 | `0` (unlimited) | Depth threshold for the both-exceeded warning. `0` is treated as unlimited (`MaxInt`). When span > `--max-width` AND depth > `--max-depth`, a warning is emitted rather than an error.                        |
| `--max-subgraph-nodes` | int                 | `6`             | Maximum direct child nodes per subgraph. Exceeding this limit emits a `subgraph_density` warning.                                                                                                             |
| `--exclude`            | string (repeatable) | —               | **Incoming.** Exclude a path prefix from validation. Can be supplied multiple times. Values are appended to the internal skip list after the built-in noise-directory entries (e.g. `node_modules/`, `.git`). |

In addition, the command accepts zero or more positional `PATH` arguments (files or directories).
When paths are given, only those paths are scanned; `--staged-only` and `--changed-only` take
precedence over positional paths.

### Global flags (inherited)

| Flag         | Short | Type   | Default | Description                                |
| ------------ | ----- | ------ | ------- | ------------------------------------------ |
| `--output`   | `-o`  | string | `text`  | Output format: `text`, `json`, `markdown`. |
| `--verbose`  | `-v`  | bool   | `false` | Verbose output with timestamps.            |
| `--quiet`    | `-q`  | bool   | `false` | Quiet mode — errors only.                  |
| `--no-color` | —     | bool   | `false` | Disable colored output.                    |
| `--say`      | —     | string | `""`    | Echo a message to stdout (utility flag).   |
| `--help`     | `-h`  | bool   | `false` | Print help.                                |

### Examples

```bash
# Validate all markdown files in default directories
rhino-cli docs validate-mermaid

# Validate specific files or directories
rhino-cli docs validate-mermaid docs/ repo-governance/

# Only validate staged files (pre-commit)
rhino-cli docs validate-mermaid --staged-only

# Only validate files changed since upstream (batch use)
rhino-cli docs validate-mermaid --changed-only

# Output as JSON
rhino-cli docs validate-mermaid -o json

# Set custom thresholds
rhino-cli docs validate-mermaid --max-label-len 20 --max-width 4

# Exclude a directory tree from validation (incoming)
rhino-cli docs validate-mermaid --exclude plans/done

# Combine exclusions (incoming)
rhino-cli docs validate-mermaid --exclude plans/done --exclude archived
```

### Implementation references

| Implementation | Flag struct           | Handler                | Source                                |
| -------------- | --------------------- | ---------------------- | ------------------------------------- |
| Rust (clap)    | `ValidateMermaidArgs` | `run_validate_mermaid` | `apps/rhino-cli/src/commands/docs.rs` |

---

## `docs validate-heading-hierarchy` (incoming)

Scans markdown files for heading hierarchy violations. Three finding kinds are enforced:

- `missing-h1` — the file contains no H1 heading
- `duplicate-h1` — the file contains more than one H1 heading
- `skipped-level` — a heading jumps more than one level (e.g. `#` directly to `###`)

The command applies a **prose allowlist** (default-deny): only files under the listed trees are
scanned. Everything else is silently skipped.

**Allowlist scope:**

| Path pattern              | Included          |
| ------------------------- | ----------------- |
| `docs/`                   | Yes               |
| `repo-governance/`        | Yes               |
| `specs/`                  | Yes               |
| `plans/` (except `done/`) | Yes               |
| Root `*.md` files         | Yes               |
| `apps/*/README.md`        | Yes               |
| `libs/*/README.md`        | Yes               |
| `apps/*/docs/**`          | Yes               |
| `libs/*/docs/**`          | Yes               |
| `.claude/agents/`         | No — default-deny |
| `.claude/skills/`         | No — default-deny |
| `apps/*/src/`             | No — default-deny |
| `plans/done/`             | No — excluded     |
| Everything else           | No — default-deny |

### Flags

| Flag            | Type                | Default | Description                                                                                                                                                                    |
| --------------- | ------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `[PATH...]`     | positional          | —       | Zero or more paths to scan. When supplied, only those paths are scanned; the prose allowlist is still applied to filter non-allowlisted files within the given paths.          |
| `--staged-only` | bool                | `false` | Only validate files currently staged in git. Useful in pre-commit hooks.                                                                                                       |
| `--exclude`     | string (repeatable) | —       | **Incoming.** Exclude a path prefix from validation. Can be supplied multiple times. Values are appended to the internal skip list after the built-in default-deny exclusions. |

### Global flags (inherited)

| Flag         | Short | Type   | Default | Description                                |
| ------------ | ----- | ------ | ------- | ------------------------------------------ |
| `--output`   | `-o`  | string | `text`  | Output format: `text`, `json`, `markdown`. |
| `--verbose`  | `-v`  | bool   | `false` | Verbose output with timestamps.            |
| `--quiet`    | `-q`  | bool   | `false` | Quiet mode — errors only.                  |
| `--no-color` | —     | bool   | `false` | Disable colored output.                    |
| `--say`      | —     | string | `""`    | Echo a message to stdout (utility flag).   |
| `--help`     | `-h`  | bool   | `false` | Print help.                                |

### Examples

```bash
# Validate all allowlisted markdown files
rhino-cli docs validate-heading-hierarchy

# Validate only staged files (pre-commit hook)
rhino-cli docs validate-heading-hierarchy --staged-only

# Output as JSON
rhino-cli docs validate-heading-hierarchy -o json

# Exclude a directory tree from validation
rhino-cli docs validate-heading-hierarchy --exclude docs

# Combine exclusions
rhino-cli docs validate-heading-hierarchy --exclude docs --exclude plans/in-progress
```

### Implementation references

| Implementation | Flag struct                    | Handler                          | Source                                |
| -------------- | ------------------------------ | -------------------------------- | ------------------------------------- |
| Rust (clap)    | `ValidateHeadingHierarchyArgs` | `run_validate_heading_hierarchy` | `apps/rhino-cli/src/commands/docs.rs` |

---

## Default scan scope

`docs validate-links` and `docs validate-heading-hierarchy` share the same four-directory default
scan. `docs validate-mermaid` default scan is **incoming** repo-wide: when no targeting flags or
positional paths are supplied, the mermaid validator walks the entire repository minus the
standardized noise-skip set.

| Directory              | `validate-links` / `validate-heading-hierarchy` | `validate-mermaid` (incoming) |
| ---------------------- | ----------------------------------------------- | ----------------------------- |
| `docs/`                | Yes                                             | Yes                           |
| `repo-governance/`     | Yes                                             | Yes                           |
| `.claude/`             | Yes                                             | Yes                           |
| `plans/`               | Yes                                             | Yes                           |
| Root `*.md` files      | Yes                                             | Yes                           |
| `specs/`, `apps/`, etc | No — outside four-dir scope                     | Yes — **Incoming** repo-wide  |
| `.opencode/skill/`     | No — always excluded (auto-generated)           | No — always excluded          |
| `node_modules/`        | No — skipped during walk                        | No — skipped during walk      |
| `.next/`               | No — skipped during walk                        | No — skipped during walk      |
| `.git/`                | No — skipped during walk                        | No — skipped during walk      |

---

## Gherkin Coverage

Behavior scenarios for all commands live in
[`specs/apps/rhino/behavior/rhino-cli/gherkin/docs/`](../../behavior/rhino-cli/gherkin/md/README.md):

| Feature file                              | Command                           | Scenarios |
| ----------------------------------------- | --------------------------------- | --------- |
| `docs-validate-links.feature`             | `docs validate-links`             | 9         |
| `docs-validate-mermaid.feature`           | `docs validate-mermaid`           | 27        |
| `docs-validate-heading-hierarchy.feature` | `docs validate-heading-hierarchy` | 9         |

---

## Related

- **Parent**: [cli component](./README.md)
- **Behavior specs**: [behavior/rhino-cli/gherkin/docs/](../../behavior/rhino-cli/gherkin/md/README.md)
- **Implementation**: `apps/rhino-cli/src/commands/docs.rs`
