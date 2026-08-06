---
title: "Run a Next.js Demo from a Fresh Checkout"
description: "Start the ose-primer Next.js reference app and understand what the template gives you."
category: tutorial
tags:
  - onboarding
  - nextjs
  - nx
---

# Run a Next.js Demo from a Fresh Checkout

In this tutorial, you will run the `crud-fe-ts-nextjs` reference application from a new clone of `ose-primer`. At the end, you will have a browser tab open at `http://localhost:3301` and a concrete sense of how this template starts.

`ose-primer` is a reusable Nx starter, not a product deployment. The frontend you run is a reference implementation you can learn from or take into a fork.

## Before you begin

Use macOS or Ubuntu Linux. On Windows, an Ubuntu environment in WSL2 may work, but it is not a separately supported native-Windows setup.

Install [Volta](https://docs.volta.sh/guide/getting-started) and open a new terminal. Volta reads the repository's pinned Node.js `24.16.0` and npm `11.10.1` versions.

## Clone the starter

```bash
git clone https://github.com/wahidyankf/ose-primer.git
cd ose-primer
npm install
```

`npm install` prepares the JavaScript workspace and Git hooks. You do not need Docker or every native language toolchain to reach this first frontend screen.

## Start the application

Run the verified Nx development target:

```bash
npm exec nx -- dev crud-fe-ts-nextjs
```

When Next.js reports that it is ready, open <http://localhost:3301>. You should see the CRUD frontend reference application.

Keep the terminal running while you explore. Press `Ctrl+C` there to stop the server.

## What you just used

- `apps/crud-fe-ts-nextjs/` contains the Next.js reference app.
- `npm exec nx --` runs the version of Nx declared by this workspace rather than relying on a global install.
- The app's `dev` target listens on port `3301`; its definition is in `apps/crud-fe-ts-nextjs/project.json`.

You have completed the smallest working path. A backend is not required to start the frontend; one is relevant when you want to run the full end-to-end flow.

## Where to go next

- Read [`apps/crud-fe-ts-nextjs/README.md`](../../apps/crud-fe-ts-nextjs/README.md) for its tests, build, and E2E relationship.
- Run `npm run doctor -- --fix` before working across the template's other native-language examples.
- Explore [Reference](../reference/README.md) for the workspace map and [Explanation](../explanation/README.md) for the design rationale.
- When you are ready to adopt the starter, return to the [root README](../../README.md#make-the-starter-yours).

Keep real credentials out of the repository. The starter is intended to be forked and adapted; it is not accepting external pull requests or support requests.
