# Container Diagram: rhino-cli

Level 2 of the C4 model. Shows the internal containers of the rhino-cli system.

```mermaid
%% Color Palette: Blue #0173B2 | Orange #DE8F05 | Teal #029E73 | Purple #CC78BC | Brown #CA9161 | Gray #808080
graph LR
    DEV("Developer / CI"):::actor

    subgraph RHINOCLI["rhino-cli binary"]
        CMD["Command Layer<br/>──────────────────<br/>clap CLI parser<br/>subcommand dispatch"]:::container
        APP["Application Layer<br/>──────────────────<br/>Orchestration<br/>Config loading"]:::container
        DOM["Domain Layer<br/>──────────────────<br/>Validation engines<br/>Business rules"]:::container
        INF["Infrastructure Layer<br/>──────────────────<br/>File I/O<br/>Process execution"]:::container
    end

    REPO("Repository Files<br/>──────────────────<br/>.md / .feature<br/>project.json<br/>repo-config.yml"):::external

    DEV -->|"invoke"| CMD
    CMD --> APP
    APP --> DOM
    APP --> INF
    INF -->|"reads"| REPO

    classDef actor fill:#DE8F05,stroke:#000000,color:#000000,stroke-width:2px
    classDef container fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef external fill:#808080,stroke:#000000,color:#FFFFFF,stroke-width:2px
```

## Related

- **Product overview**: [product.md](../product/product.md)
- **System context**: [context.md](../system-context/context.md)
- **CLI component diagram**: [component-cli.md](../components/cli/component-cli.md)
- **Parent**: [rhino specs](../README.md)
