import { z } from "zod";

const schema = z.object({
  CRUD_FS_TS_NEXTJS_JWT_SECRET: z.string().min(1, "CRUD_FS_TS_NEXTJS_JWT_SECRET is required"),
});

export const env = schema.parse({
  CRUD_FS_TS_NEXTJS_JWT_SECRET: process.env.CRUD_FS_TS_NEXTJS_JWT_SECRET,
});
