package envvalidate_test

import (
	"testing"

	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/envvalidate"
)

func assertContains(t *testing.T, keys []string, want string) {
	t.Helper()
	for _, k := range keys {
		if k == want {
			return
		}
	}
	t.Errorf("expected %q in %v", want, keys)
}

func assertNotContains(t *testing.T, keys []string, notWant string) {
	t.Helper()
	for _, k := range keys {
		if k == notWant {
			t.Errorf("expected %q NOT in %v", notWant, keys)
			return
		}
	}
}

func TestExtractRust(t *testing.T) {
	src := `
let s = env::var("FIXTURE_JWT_SECRET").context("required")?;
let p = env::var("FIXTURE_PORT").unwrap_or("8080".into());
// let c = env::var("COMMENTED");
`
	got := envvalidate.ExtractRust(src)
	assertContains(t, got, "FIXTURE_JWT_SECRET")
	assertContains(t, got, "FIXTURE_PORT")
	assertNotContains(t, got, "COMMENTED")
}

func TestExtractGo(t *testing.T) {
	src := `
secret := os.Getenv("CRUD_BE_GOLANG_GIN_JWT_SECRET")
port, _ := os.LookupEnv("CRUD_BE_GOLANG_GIN_PORT")
// ignored := os.Getenv("COMMENTED")
type Config struct {
	Secret string ` + "`env:\"CRUD_BE_GOLANG_GIN_JWT_SECRET\"`" + `
}
`
	got := envvalidate.ExtractGo(src)
	assertContains(t, got, "CRUD_BE_GOLANG_GIN_JWT_SECRET")
	assertContains(t, got, "CRUD_BE_GOLANG_GIN_PORT")
	assertNotContains(t, got, "COMMENTED")
}

func TestExtractTypeScript(t *testing.T) {
	src := `
const url = process.env.BACKEND_URL;
const secret = process.env["CRUD_FS_TS_NEXTJS_JWT_SECRET"];
const jwt = yield* Config.string("CRUD_BE_TS_EFFECT_JWT_SECRET");
const port = yield* Config.integer("CRUD_BE_TS_EFFECT_PORT");
`
	got := envvalidate.ExtractTypeScript(src)
	assertContains(t, got, "BACKEND_URL")
	assertContains(t, got, "CRUD_FS_TS_NEXTJS_JWT_SECRET")
	assertContains(t, got, "CRUD_BE_TS_EFFECT_JWT_SECRET")
	assertContains(t, got, "CRUD_BE_TS_EFFECT_PORT")
}

func TestExtractClojure(t *testing.T) {
	src := `
(getenv "CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET")
; (System/getenv "COMMENTED")
`
	got := envvalidate.ExtractClojure(src)
	assertContains(t, got, "CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET")
	assertNotContains(t, got, "COMMENTED")
}

func TestExtractClojureSystemGetenv(t *testing.T) {
	src := `(System/getenv "ENABLE_TEST_API")`
	got := envvalidate.ExtractClojure(src)
	assertContains(t, got, "ENABLE_TEST_API")
}

func TestExtractCSharp(t *testing.T) {
	src := `
builder.Configuration["CRUD_BE_CSHARP_ASPNETCORE_JWT_SECRET"]
Environment.GetEnvironmentVariable("DATABASE_URL")
`
	got := envvalidate.ExtractCSharp(src)
	assertContains(t, got, "CRUD_BE_CSHARP_ASPNETCORE_JWT_SECRET")
	assertContains(t, got, "DATABASE_URL")
}

func TestExtractElixir(t *testing.T) {
	src := `
System.get_env("CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET")
# System.get_env("COMMENTED")
`
	got := envvalidate.ExtractElixir(src)
	assertContains(t, got, "CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET")
	assertNotContains(t, got, "COMMENTED")
}

func TestExtractFSharp(t *testing.T) {
	src := `Environment.GetEnvironmentVariable("CRUD_BE_FSHARP_GIRAFFE_JWT_SECRET")`
	got := envvalidate.ExtractFSharp(src)
	assertContains(t, got, "CRUD_BE_FSHARP_GIRAFFE_JWT_SECRET")
}

func TestExtractJavaYAML(t *testing.T) {
	src := `
secret: ${CRUD_BE_JAVA_SPRINGBOOT_JWT_SECRET}
enabled: ${ENABLE_TEST_API:false}
`
	got := envvalidate.ExtractJava(src, true)
	assertContains(t, got, "CRUD_BE_JAVA_SPRINGBOOT_JWT_SECRET")
	assertNotContains(t, got, "ENABLE_TEST_API")
}

func TestExtractJavaSource(t *testing.T) {
	src := `String secret = System.getenv("CRUD_BE_JAVA_VERTX_JWT_SECRET");`
	got := envvalidate.ExtractJava(src, false)
	assertContains(t, got, "CRUD_BE_JAVA_VERTX_JWT_SECRET")
}

func TestExtractKotlin(t *testing.T) {
	src := `val secret = System.getenv("CRUD_BE_KOTLIN_KTOR_JWT_SECRET")`
	got := envvalidate.ExtractKotlin(src)
	assertContains(t, got, "CRUD_BE_KOTLIN_KTOR_JWT_SECRET")
}

func TestExtractPython(t *testing.T) {
	src := `
class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env")
    database_url: str = "sqlite:///:memory:"
    crud_be_python_fastapi_jwt_secret: str
    app_jwt_issuer: str = "crud-be"

settings = Settings()
`
	got := envvalidate.ExtractPython(src)
	assertContains(t, got, "DATABASE_URL")
	assertContains(t, got, "CRUD_BE_PYTHON_FASTAPI_JWT_SECRET")
	assertContains(t, got, "APP_JWT_ISSUER")
}

func TestExtractPythonOsGetenv(t *testing.T) {
	src := `
import os
x = os.getenv("ENABLE_TEST_API")
y = os.environ.get("DATABASE_URL", "")
`
	got := envvalidate.ExtractPython(src)
	assertContains(t, got, "ENABLE_TEST_API")
	assertContains(t, got, "DATABASE_URL")
}
