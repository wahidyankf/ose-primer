//! ENV-VALIDATE CONFIG: hardcoded constants (single stable app surface in ose-primer;
//! no env-contract.yaml needed — all 15 surfaces are known at compile time).
//!
//! Byte-for-byte port target: `apps/rhino-cli-go/internal/envvalidate/contract.go`.

/// A configured app surface for env-validate.
pub struct AppSurface {
    /// Basename of both `infra/dev/<app>/` and `apps/<app>/`.
    pub app: &'static str,
    /// File extensions to scan (without leading dot).
    pub source_exts: &'static [&'static str],
    /// Source subdirectory relative to `apps/<app>/` root. Empty = scan whole app dir.
    pub source_subdir: &'static str,
    /// Per-app allowlist: keys exempt from both declared-but-unread AND read-but-undeclared.
    pub allowlist: &'static [&'static str],
}

/// Global allowlist applied to every surface.
pub const GLOBAL_ALLOWLIST: &[&str] = &[
    "POSTGRES_USER",
    "POSTGRES_PASSWORD",
    "ENABLE_TEST_API",
    "DATABASE_URL",
];

/// All 15 app surfaces in ose-primer (one per infra/dev/<app>/).
pub const SURFACES: &[AppSurface] = &[
    AppSurface {
        app: "crud-be-clojure-pedestal",
        source_exts: &["clj", "cljs"],
        source_subdir: "src",
        allowlist: &["CRUD_BE_CLOJURE_PEDESTAL_PORT"],
    },
    AppSurface {
        app: "crud-be-csharp-aspnetcore",
        source_exts: &["cs"],
        source_subdir: "src",
        allowlist: &["CRUD_BE_CSHARP_ASPNETCORE_PORT"],
    },
    AppSurface {
        app: "crud-be-elixir-phoenix",
        source_exts: &["ex", "exs"],
        source_subdir: "",
        allowlist: &[
            "CRUD_BE_ELIXIR_PHOENIX_PORT",
            "PHX_SERVER",
            "POOL_SIZE",
            "ECTO_IPV6",
        ],
    },
    AppSurface {
        app: "crud-be-fsharp-giraffe",
        source_exts: &["fs", "fsx"],
        source_subdir: "src",
        allowlist: &[],
    },
    AppSurface {
        app: "crud-be-golang-gin",
        source_exts: &["go"],
        source_subdir: "",
        allowlist: &["CRUD_BE_GOLANG_GIN_PORT"],
    },
    AppSurface {
        app: "crud-be-java-springboot",
        source_exts: &["java", "yml", "yaml"],
        source_subdir: "src",
        allowlist: &[
            "SPRING_PROFILES_ACTIVE",
            "JAVA_OPTS",
            "MAVEN_OPTS",
            "SPRING_DATASOURCE_URL",
            "SPRING_DATASOURCE_USERNAME",
            "SPRING_DATASOURCE_PASSWORD",
        ],
    },
    AppSurface {
        app: "crud-be-java-vertx",
        source_exts: &["java"],
        source_subdir: "src",
        allowlist: &["CRUD_BE_JAVA_VERTX_PORT"],
    },
    AppSurface {
        app: "crud-be-kotlin-ktor",
        source_exts: &["kt"],
        source_subdir: "src",
        allowlist: &[
            "CRUD_BE_KOTLIN_KTOR_PORT",
            "DATABASE_USER",
            "DATABASE_PASSWORD",
        ],
    },
    AppSurface {
        app: "crud-be-python-fastapi",
        source_exts: &["py"],
        source_subdir: "src",
        allowlist: &[
            "APP_JWT_ISSUER",
            "MAX_FAILED_LOGIN_ATTEMPTS",
            "MAX_ATTACHMENT_SIZE_BYTES",
        ],
    },
    AppSurface {
        app: "crud-be-rust-axum",
        source_exts: &["rs"],
        source_subdir: "src",
        allowlist: &["CRUD_BE_RUST_AXUM_PORT"],
    },
    AppSurface {
        app: "crud-be-ts-effect",
        source_exts: &["ts", "tsx"],
        source_subdir: "src",
        allowlist: &["CRUD_BE_TS_EFFECT_PORT"],
    },
    AppSurface {
        app: "crud-fe-dart-flutterweb",
        source_exts: &["dart"],
        source_subdir: "lib",
        allowlist: &["CRUD_BE_GOLANG_GIN_JWT_SECRET"],
    },
    AppSurface {
        app: "crud-fe-ts-nextjs",
        source_exts: &["ts", "tsx"],
        source_subdir: "src",
        allowlist: &["CRUD_BE_GOLANG_GIN_JWT_SECRET", "BACKEND_URL"],
    },
    AppSurface {
        app: "crud-fe-ts-tanstack-start",
        source_exts: &["ts", "tsx"],
        source_subdir: "src",
        allowlist: &["CRUD_BE_GOLANG_GIN_JWT_SECRET", "BACKEND_URL"],
    },
    AppSurface {
        app: "crud-fs-ts-nextjs",
        source_exts: &["ts", "tsx"],
        source_subdir: "src",
        allowlist: &[],
    },
];
