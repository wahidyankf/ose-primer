# ose-primer

🚀 A reusable Nx starter for teams that want a thoughtful, polyglot workspace before their product gets complicated.

`ose-primer` gives you a working place to begin: example CRUD applications, shared contracts, quality gates, AI-ready repository guidance, and a documentation structure that can grow with your project. It is a template to adapt, not an OSE product to operate.

## Start here

Choose the route that matches why you opened the repository:

- **I want to see something run.** Follow [Run the Next.js demo](#run-the-nextjs-demo). You will start `crud-fe-ts-nextjs` and open it at `http://localhost:3301`.
- **I want a starter for my own project.** Follow [Make the starter yours](#make-the-starter-yours) after the demo is running.
- **I want to understand the system.** Read the [documentation map](./docs/README.md), then use the [architecture reference](./docs/reference/system-architecture/README.md) when you need detail.

## What problem it solves

Starting a product repository usually means solving the same setup problems again: predictable tooling, a safe development workflow, a place for decisions, a way to test across languages, and a project structure people can learn. `ose-primer` packages those foundations with runnable examples so a team can spend its early energy on its product rather than reassembling repository mechanics.

It is deliberately different from [ose-public](https://github.com/wahidyankf/ose-public):

| Repository                                               | Start here when you need…                                                                                      |
| -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| [`ose-public`](https://github.com/wahidyankf/ose-public) | The public Open Sharia Enterprise platform, its product context, and its active applications.                  |
| `ose-primer`                                             | A reusable MIT-licensed Nx foundation and polyglot reference implementations to reshape into a new repository. |

## Run the Next.js demo

This is the smallest useful fresh-checkout journey. It needs Node.js and npm; Docker and the rest of the polyglot toolchain are not required for this first screen.

### 1. Prepare your machine

- **macOS and Ubuntu Linux:** install [Volta](https://docs.volta.sh/guide/getting-started), then open a new terminal. The repository pins Node.js `24.16.0` and npm `11.10.1`.
- **Windows:** this path may work through WSL2. Use an Ubuntu distribution in WSL2 and follow the Linux instructions there; it is not a separately supported native-Windows setup.

### 2. Clone and install

```bash
git clone https://github.com/wahidyankf/ose-primer.git
cd ose-primer
npm install
```

`npm install` prepares the JavaScript workspace and hooks. If you later work with another language, run `npm run doctor -- --fix` to converge the complete native toolchain.

### 3. Start the demo

```bash
npm exec nx -- dev crud-fe-ts-nextjs
```

Open <http://localhost:3301>. You now have a running Next.js reference application. Stop it with `Ctrl+C` when you are done.

If the port is occupied, stop the process using port `3301` or choose a different demo. For project-specific commands, start with [`apps/crud-fe-ts-nextjs/README.md`](./apps/crud-fe-ts-nextjs/README.md).

## Make the starter yours

Once the demo is running, create a copy that you control:

1. Fork or clone this repository into a new project directory.
2. Keep the apps, libraries, specifications, and governance pieces that help your team; remove examples you do not need alongside their related specifications.
3. Rename `ose-primer`, set your own remote, and describe your product in the root README.
4. Keep the quality gates and documentation habits that serve your team; adapt the rest deliberately.

The template is MIT-licensed, so downstream projects may choose their own license and delivery policy.

## What is included

- **Reference applications:** CRUD backends in multiple languages, frontend examples, a full-stack Next.js example, and paired end-to-end test harnesses.
- **Shared contract:** an OpenAPI contract and generated clients that make the demos useful for comparing implementation choices.
- **`rhino-cli`:** local repository checks, toolchain doctor, and documentation validation used by the workspace.
- **Repository foundations:** Nx configuration, Husky quality hooks, a Diátaxis documentation tree, planning conventions, and AI-agent guidance.

See [apps/README.md](./apps/README.md) for the available demos and [docs/reference/README.md](./docs/reference/README.md) for reference material.

## Everyday commands

```bash
# Check or converge the full polyglot toolchain
npm run doctor
npm run doctor -- --fix

# Work with one project
npm exec nx -- build crud-fe-ts-nextjs
npm exec nx -- run crud-fe-ts-nextjs:test:quick

# Work across the workspace
npm exec nx -- affected -t build,test:quick,lint
npm run lint:md
npm run validate:sync
```

Use `npm exec nx --` so npm forwards the command to the workspace version of Nx. The [development reference](./repo-governance/development/README.md) explains the testing and quality-gate vocabulary.

## Documentation map

📚 The documentation is arranged by the kind of help you need:

- [Tutorials](./docs/tutorials/README.md) — learn by completing a guided outcome.
- [How-to guides](./docs/how-to/README.md) — solve a focused problem.
- [Reference](./docs/reference/README.md) — look up commands, structure, and contracts.
- [Explanation](./docs/explanation/README.md) — understand the decisions behind the workspace.

## External contributions

This repository is shared as a reusable reference, but it is not accepting external pull requests, issues, feature requests, or support requests. You are welcome to fork it and shape the copy around your own product. Authorized maintainers use the repository's internal delivery workflow.

For a security concern, follow [SECURITY.md](./SECURITY.md); do not publish sensitive details in a public channel.

## Related repositories

`ose-primer` is one of the Open Sharia Enterprise repositories. It shares selected governance and tooling conventions with its siblings while staying independently clonable:

- [`ose-public`](https://github.com/wahidyankf/ose-public) — the public OSE platform.
- [`ose-primer`](https://github.com/wahidyankf/ose-primer) — this reusable template and reference workspace.
- [`ose-private`](https://github.com/wahidyankf/ose-private) — private operations for authorized maintainers.
- [`beaver-nest`](https://github.com/wahidyankf/beaver-nest) — a separate product in the ecosystem.

See [Related Repositories](./docs/reference/related-repositories.md) for the documented boundaries.

## License

MIT. See [LICENSE](./LICENSE) and [LICENSING-NOTICE.md](./LICENSING-NOTICE.md).
