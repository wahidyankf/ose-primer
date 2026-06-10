package envvalidate

// AppSurface is a configured app surface for env-validate.
type AppSurface struct {
	// App is the basename of both infra/dev/<app>/ and apps/<app>/.
	App string
	// SourceExts are the file extensions to scan (without leading dot).
	SourceExts []string
	// SourceSubdir is the source subdirectory relative to apps/<app>/. Empty = scan whole app dir.
	SourceSubdir string
	// Allowlist contains per-app keys exempt from both violation directions.
	Allowlist []string
}

// GlobalAllowlist is applied to every surface.
var GlobalAllowlist = []string{
	"POSTGRES_USER",
	"POSTGRES_PASSWORD",
	"ENABLE_TEST_API",
	"DATABASE_URL",
}

// Surfaces lists all 15 app surfaces in ose-primer.
var Surfaces = []*AppSurface{
	{
		App:          "crud-be-clojure-pedestal",
		SourceExts:   []string{"clj", "cljs"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_CLOJURE_PEDESTAL_PORT"},
	},
	{
		App:          "crud-be-csharp-aspnetcore",
		SourceExts:   []string{"cs"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_CSHARP_ASPNETCORE_PORT"},
	},
	{
		App:          "crud-be-elixir-phoenix",
		SourceExts:   []string{"ex", "exs"},
		SourceSubdir: "",
		Allowlist:    []string{"CRUD_BE_ELIXIR_PHOENIX_PORT", "PHX_SERVER", "POOL_SIZE", "ECTO_IPV6"},
	},
	{
		App:          "crud-be-fsharp-giraffe",
		SourceExts:   []string{"fs", "fsx"},
		SourceSubdir: "src",
		Allowlist:    []string{},
	},
	{
		App:          "crud-be-golang-gin",
		SourceExts:   []string{"go"},
		SourceSubdir: "",
		Allowlist:    []string{"CRUD_BE_GOLANG_GIN_PORT"},
	},
	{
		App:          "crud-be-java-springboot",
		SourceExts:   []string{"java", "yml", "yaml"},
		SourceSubdir: "src",
		Allowlist:    []string{"SPRING_PROFILES_ACTIVE", "JAVA_OPTS", "MAVEN_OPTS", "SPRING_DATASOURCE_URL", "SPRING_DATASOURCE_USERNAME", "SPRING_DATASOURCE_PASSWORD"},
	},
	{
		App:          "crud-be-java-vertx",
		SourceExts:   []string{"java"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_JAVA_VERTX_PORT"},
	},
	{
		App:          "crud-be-kotlin-ktor",
		SourceExts:   []string{"kt"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_KOTLIN_KTOR_PORT", "DATABASE_USER", "DATABASE_PASSWORD"},
	},
	{
		App:          "crud-be-python-fastapi",
		SourceExts:   []string{"py"},
		SourceSubdir: "src",
		Allowlist:    []string{"APP_JWT_ISSUER", "MAX_FAILED_LOGIN_ATTEMPTS", "MAX_ATTACHMENT_SIZE_BYTES"},
	},
	{
		App:          "crud-be-rust-axum",
		SourceExts:   []string{"rs"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_RUST_AXUM_PORT"},
	},
	{
		App:          "crud-be-ts-effect",
		SourceExts:   []string{"ts", "tsx"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_TS_EFFECT_PORT"},
	},
	{
		App:          "crud-fe-dart-flutterweb",
		SourceExts:   []string{"dart"},
		SourceSubdir: "lib",
		Allowlist:    []string{"CRUD_BE_GOLANG_GIN_JWT_SECRET"},
	},
	{
		App:          "crud-fe-ts-nextjs",
		SourceExts:   []string{"ts", "tsx"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_GOLANG_GIN_JWT_SECRET", "BACKEND_URL"},
	},
	{
		App:          "crud-fe-ts-tanstack-start",
		SourceExts:   []string{"ts", "tsx"},
		SourceSubdir: "src",
		Allowlist:    []string{"CRUD_BE_GOLANG_GIN_JWT_SECRET", "BACKEND_URL"},
	},
	{
		App:          "crud-fs-ts-nextjs",
		SourceExts:   []string{"ts", "tsx"},
		SourceSubdir: "src",
		Allowlist:    []string{},
	},
}
