package envvalidate

import (
	"sort"
	"strings"
	"unicode"
)

// ExtractRust extracts env var names from Rust source (env::var("KEY")).
func ExtractRust(content string) []string {
	return extractPattern(content, []string{`env::var("`}, "//")
}

// ExtractGo extracts env var names from Go source:
// os.Getenv("KEY"), os.LookupEnv("KEY"), struct tags `env:"KEY"`.
func ExtractGo(content string) []string {
	keys := make(map[string]struct{})
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "//") {
			continue
		}
		for _, pat := range []string{`os.Getenv("`, `os.LookupEnv("`} {
			rest := trimmed
			for {
				pos := strings.Index(rest, pat)
				if pos < 0 {
					break
				}
				after := rest[pos+len(pat):]
				end := strings.Index(after, `"`)
				if end >= 0 {
					k := after[:end]
					if isEnvKey(k) {
						keys[k] = struct{}{}
					}
				}
				rest = rest[pos+len(pat):]
			}
		}
		// struct tags: `env:"KEY"` or `env:"KEY,..."`
		rest := trimmed
		for {
			pos := strings.Index(rest, "`env:\"")
			if pos < 0 {
				break
			}
			after := rest[pos+6:]
			end := strings.Index(after, `"`)
			if end >= 0 {
				tagVal := after[:end]
				k := strings.SplitN(tagVal, ",", 2)[0]
				if isEnvKey(k) {
					keys[k] = struct{}{}
				}
			}
			rest = rest[pos+6:]
		}
	}
	return sortedKeys(keys)
}

// ExtractTypeScript extracts env var names from TypeScript/JavaScript source:
// process.env.KEY, process.env["KEY"], Config.string/integer/number/boolean("KEY").
func ExtractTypeScript(content string) []string {
	keys := make(map[string]struct{})
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "//") {
			continue
		}
		// process.env.KEY
		rest := trimmed
		for {
			pos := strings.Index(rest, "process.env.")
			if pos < 0 {
				break
			}
			after := rest[pos+12:]
			var sb strings.Builder
			for _, c := range after {
				if unicode.IsUpper(c) || c == '_' || unicode.IsDigit(c) {
					sb.WriteRune(c)
				} else {
					break
				}
			}
			k := sb.String()
			if isEnvKey(k) {
				keys[k] = struct{}{}
			}
			rest = rest[pos+12:]
		}
		// process.env["KEY"]
		rest = trimmed
		for {
			pos := strings.Index(rest, `process.env["`)
			if pos < 0 {
				break
			}
			after := rest[pos+13:]
			end := strings.Index(after, `"`)
			if end >= 0 {
				k := after[:end]
				if isEnvKey(k) {
					keys[k] = struct{}{}
				}
			}
			rest = rest[pos+13:]
		}
		// Config.string/integer/number/boolean("KEY")
		for _, variant := range []string{`Config.string("`, `Config.integer("`, `Config.number("`, `Config.boolean("`} {
			rest := trimmed
			for {
				pos := strings.Index(rest, variant)
				if pos < 0 {
					break
				}
				after := rest[pos+len(variant):]
				end := strings.Index(after, `"`)
				if end >= 0 {
					k := after[:end]
					if isEnvKey(k) {
						keys[k] = struct{}{}
					}
				}
				rest = rest[pos+len(variant):]
			}
		}
	}
	return sortedKeys(keys)
}

// ExtractClojure extracts env var names from Clojure source.
// Matches both (System/getenv "KEY") and (getenv "KEY") (local wrapper form).
func ExtractClojure(content string) []string {
	keys := make(map[string]struct{})
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, ";") {
			continue
		}
		rest := trimmed
		for {
			pos := strings.Index(rest, `getenv "`)
			if pos < 0 {
				break
			}
			after := rest[pos+8:]
			end := strings.Index(after, `"`)
			if end >= 0 {
				k := after[:end]
				if isEnvKey(k) {
					keys[k] = struct{}{}
				}
			}
			rest = rest[pos+8:]
		}
	}
	return sortedKeys(keys)
}

// ExtractCSharp extracts env var names from C# source:
// Configuration["KEY"], Environment.GetEnvironmentVariable("KEY").
func ExtractCSharp(content string) []string {
	return extractPattern(content, []string{`Configuration["`, `GetEnvironmentVariable("`}, "//")
}

// ExtractElixir extracts env var names from Elixir source (System.get_env("KEY")).
func ExtractElixir(content string) []string {
	return extractPattern(content, []string{`System.get_env("`}, "#")
}

// ExtractFSharp extracts env var names from F# source (Environment.GetEnvironmentVariable("KEY")).
func ExtractFSharp(content string) []string {
	return extractPattern(content, []string{`GetEnvironmentVariable("`}, "//")
}

// ExtractJava extracts env var names from Java source (System.getenv("KEY"))
// or Spring YAML (${KEY} without default).
func ExtractJava(content string, isYAML bool) []string {
	keys := make(map[string]struct{})
	if isYAML {
		for _, line := range strings.Split(content, "\n") {
			trimmed := strings.TrimSpace(line)
			if strings.HasPrefix(trimmed, "#") {
				continue
			}
			rest := trimmed
			for {
				pos := strings.Index(rest, "${")
				if pos < 0 {
					break
				}
				after := rest[pos+2:]
				hasDefault := false
				end := -1
				for i, c := range after {
					if c == ':' {
						hasDefault = true
						break
					}
					if c == '}' {
						end = i
						break
					}
				}
				if !hasDefault && end >= 0 {
					k := after[:end]
					if isEnvKey(k) {
						keys[k] = struct{}{}
					}
				}
				rest = rest[pos+2:]
			}
		}
	} else {
		for _, line := range strings.Split(content, "\n") {
			trimmed := strings.TrimSpace(line)
			if strings.HasPrefix(trimmed, "//") {
				continue
			}
			rest := trimmed
			for {
				pos := strings.Index(rest, `System.getenv("`)
				if pos < 0 {
					break
				}
				after := rest[pos+15:]
				end := strings.Index(after, `"`)
				if end >= 0 {
					k := after[:end]
					if isEnvKey(k) {
						keys[k] = struct{}{}
					}
				}
				rest = rest[pos+15:]
			}
		}
	}
	return sortedKeys(keys)
}

// ExtractKotlin extracts env var names from Kotlin source (System.getenv("KEY")).
func ExtractKotlin(content string) []string {
	return extractPattern(content, []string{`System.getenv("`}, "//")
}

// ExtractPython extracts env var names from Python pydantic-settings source
// and os.getenv/os.environ.get calls.
func ExtractPython(content string) []string {
	keys := make(map[string]struct{})
	inSettingsClass := false
	classIndent := -1

	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)

		// Detect class Settings(BaseSettings):
		if strings.HasPrefix(trimmed, "class ") && strings.Contains(trimmed, "BaseSettings") {
			inSettingsClass = true
			classIndent = -1
			continue
		}

		if !inSettingsClass {
			// Extract os.environ.get, os.environ[, os.getenv
			for _, pat := range []string{`os.environ.get("`, `os.environ["`, `os.getenv("`} {
				rest := trimmed
				for {
					pos := strings.Index(rest, pat)
					if pos < 0 {
						break
					}
					after := rest[pos+len(pat):]
					end := strings.Index(after, `"`)
					if end >= 0 {
						k := after[:end]
						if isEnvKey(k) {
							keys[k] = struct{}{}
						}
					}
					rest = rest[pos+len(pat):]
				}
			}
			continue
		}

		// Inside class body
		if trimmed == "" || strings.HasPrefix(trimmed, "#") {
			continue
		}
		indent := len(line) - len(strings.TrimLeft(line, " \t"))

		// Set class body indent on first non-empty line
		if classIndent < 0 && indent > 0 {
			classIndent = indent
		}

		// End of class: dedented past body indent
		if classIndent >= 0 && indent < classIndent {
			inSettingsClass = false
			continue
		}

		// Skip model_config lines
		if strings.HasPrefix(trimmed, "model_config") {
			continue
		}

		// Match field: identifier: type (with or without = default)
		colonPos := strings.Index(trimmed, ":")
		if colonPos > 0 {
			namePart := strings.TrimSpace(trimmed[:colonPos])
			if isValidPythonIdent(namePart) {
				k := strings.ToUpper(namePart)
				if isEnvKey(k) {
					keys[k] = struct{}{}
				}
			}
		}
	}
	return sortedKeys(keys)
}

// isValidPythonIdent returns true for a bare Python identifier (no spaces, starts with alpha/underscore).
func isValidPythonIdent(s string) bool {
	if s == "" {
		return false
	}
	for i, c := range s {
		if i == 0 {
			if !unicode.IsLetter(c) && c != '_' {
				return false
			}
		} else {
			if !unicode.IsLetter(c) && !unicode.IsDigit(c) && c != '_' {
				return false
			}
		}
	}
	return true
}

// extractPattern is a helper for extractors that share the same single-string-arg pattern.
// commentPrefix: skip lines starting with this prefix.
func extractPattern(content string, patterns []string, commentPrefix string) []string {
	keys := make(map[string]struct{})
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if commentPrefix != "" && strings.HasPrefix(trimmed, commentPrefix) {
			continue
		}
		for _, pat := range patterns {
			rest := trimmed
			for {
				pos := strings.Index(rest, pat)
				if pos < 0 {
					break
				}
				after := rest[pos+len(pat):]
				end := strings.Index(after, `"`)
				if end >= 0 {
					k := after[:end]
					if isEnvKey(k) {
						keys[k] = struct{}{}
					}
				}
				rest = rest[pos+len(pat):]
			}
		}
	}
	return sortedKeys(keys)
}

// isEnvKey returns true for a valid ALL_CAPS env var key (>=2 chars, starts with uppercase letter).
func isEnvKey(s string) bool {
	if len(s) < 2 {
		return false
	}
	for i, c := range s {
		if i == 0 {
			if !unicode.IsUpper(c) {
				return false
			}
		} else {
			if !unicode.IsUpper(c) && !unicode.IsDigit(c) && c != '_' {
				return false
			}
		}
	}
	return true
}

func sortedKeys(m map[string]struct{}) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}
