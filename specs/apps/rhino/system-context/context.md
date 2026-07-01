# Context Diagram: rhino-cli

Level 1 of the C4 model. Shows rhino-cli as a single CLI system with its external actors and
adjacent systems.

```mermaid
%% Color Palette: Blue #0173B2 | Orange #DE8F05 | Teal #029E73 | Purple #CC78BC | Brown #CA9161 | Gray #808080
graph LR
    DEV("Developer<br/>──────────────────<br/>Runs validators<br/>via hooks or CLI"):::actor

    CI("CI Pipeline<br/>──────────────────<br/>main-ci.yml<br/>pr-quality-gate.yml"):::actor_ci

    SYSTEM["rhino-cli<br/>──────────────────<br/>Docs validation<br/>Specs validation<br/>Harness checks<br/>Naming checks"]:::system

    REPO("Repository<br/>──────────────────<br/>Markdown files<br/>Gherkin features<br/>Nx project.json<br/>repo-config.yml"):::external

    DEV -->|"cargo run --release"| SYSTEM
    CI -->|"nx run project:target"| SYSTEM
    SYSTEM -->|"reads"| REPO

    classDef actor fill:#DE8F05,stroke:#000000,color:#000000,stroke-width:2px
    classDef actor_ci fill:#CA9161,stroke:#000000,color:#000000,stroke-width:2px
    classDef system fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:3px
    classDef external fill:#808080,stroke:#000000,color:#FFFFFF,stroke-width:2px
```

## Related

- **Product overview**: [product.md](../product/product.md)
- **Container diagram**: [container.md](../containers/container.md)
- **CLI component diagram**: [component-cli.md](../components/cli/component-cli.md)
- **Parent**: [rhino specs](../README.md)
