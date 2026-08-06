# Working with ose-primer

👋 `ose-primer` is published so you can learn from it, run it, and fork it into something that serves your own product. It is not currently open to external contributions.

## What that means

Please do not open pull requests, issues, feature requests, or support requests against this repository. There is no public review, triage, or discussion channel for them. If an idea is useful for your work, fork the repository and make the choice in your own copy.

Authorized maintainers follow the repository's internal planning and delivery workflow. This boundary keeps the template dependable without asking outside contributors to wait for a review that will not happen.

## Use the template well

Start with the [README](./README.md#start-here): run the Next.js demo first, then decide which examples and repository foundations belong in your project.

For a fresh checkout on macOS or Ubuntu Linux, install [Volta](https://docs.volta.sh/guide/getting-started), clone the repository, and run:

```bash
npm install
npm exec nx -- dev crud-fe-ts-nextjs
```

Open <http://localhost:3301> to confirm the starter works. WSL2 with Ubuntu may also work on Windows; native Windows is not a separately supported setup.

When your work reaches another language or framework, converge the wider toolchain deliberately:

```bash
npm run doctor -- --fix
```

Do not commit real credentials. Read only the tracked examples and keep any local secrets outside version control. See [SECURITY.md](./SECURITY.md) for security reporting guidance.

## For authorized maintainers

These links describe the local rules that shape authorized maintenance work:

- [Development practices](./repo-governance/development/README.md)
- [Commit messages](./repo-governance/development/workflow/commit-messages.md)
- [Testing standard](./repo-governance/development/quality/three-level-testing-standard.md)
- [Documentation conventions](./repo-governance/conventions/README.md)

Useful commands:

```bash
npm exec nx -- run crud-fe-ts-nextjs:test:quick
npm exec nx -- affected -t build,test:quick,lint
npm run lint:md
npm run validate:sync
```

The workspace's `npm exec nx --` form makes npm use the repository's pinned Nx version.

## Security

Do not report a vulnerability in a public issue or pull request. Use the process in [SECURITY.md](./SECURITY.md), and avoid including secrets in messages, logs, or committed files.
