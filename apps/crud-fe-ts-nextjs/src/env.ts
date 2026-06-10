import { z } from "zod";

const schema = z.object({
  BACKEND_URL: z.string().default("http://localhost:8201"),
});

export const env = schema.parse({
  BACKEND_URL: process.env.BACKEND_URL,
});
