import { Config, ConfigError, Effect } from "effect";

export interface AppConfig {
  readonly databaseUrl: string;
  readonly jwtSecret: string;
  readonly port: number;
}

export const loadConfig = (): Effect.Effect<AppConfig, ConfigError.ConfigError> =>
  Effect.gen(function* () {
    const databaseUrl = yield* Config.withDefault(Config.string("DATABASE_URL"), "sqlite::memory:");
    const jwtSecret = yield* Config.string("CRUD_BE_TS_EFFECT_JWT_SECRET");
    const port = yield* Config.withDefault(Config.integer("CRUD_BE_TS_EFFECT_PORT"), 8201);
    return { databaseUrl, jwtSecret, port };
  });
