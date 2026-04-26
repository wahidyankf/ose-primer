import { defineConfig } from "drizzle-kit";

export default defineConfig({
  schema: "./src/db/schema.ts",
  out: "./src/db/migrations",
  dialect: "postgresql",
  dbCredentials: {
    url:
      process.env.DATABASE_URL ?? "postgresql://crud_fs_ts_nextjs:crud_fs_ts_nextjs@localhost:5432/crud_fs_ts_nextjs",
  },
});
