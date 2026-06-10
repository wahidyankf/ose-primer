package envvalidate

import (
	"fmt"
	"sort"
	"strings"
)

// FormatText formats env-validate results as human-readable text.
func FormatText(result *ValidateResult) string {
	if result.IsOK() {
		return "✓ Env validate passed! All app surfaces clean.\n"
	}
	var sb strings.Builder
	for _, s := range result.Surfaces {
		if s.IsOK() {
			continue
		}
		fmt.Fprintf(&sb, "✗ %s\n", s.App)
		for _, k := range s.DeclaredNotRead {
			fmt.Fprintf(&sb, "  declared-but-unread: %s\n", k)
		}
		for _, k := range s.ReadNotDeclared {
			fmt.Fprintf(&sb, "  read-but-undeclared: %s\n", k)
		}
	}
	v := result.ViolationCount()
	suffix := "s"
	if v == 1 {
		suffix = ""
	}
	fmt.Fprintf(&sb, "\nEnv validate FAILED: %d violation%s\n", v, suffix)
	return sb.String()
}

// FormatJSON formats env-validate results as compact JSON.
func FormatJSON(result *ValidateResult) (string, error) {
	var surfaces []string
	for _, s := range result.Surfaces {
		dnr := jsonStringArray(s.DeclaredNotRead)
		rnd := jsonStringArray(s.ReadNotDeclared)
		surfaces = append(surfaces, fmt.Sprintf(
			`{"app":%q,"declared_not_read":[%s],"read_not_declared":[%s]}`,
			s.App, dnr, rnd,
		))
	}
	ok := result.IsOK()
	return fmt.Sprintf(
		`{"ok":%t,"violations":%d,"surfaces":[%s]}`,
		ok, result.ViolationCount(), strings.Join(surfaces, ","),
	), nil
}

func jsonStringArray(keys []string) string {
	sorted := make([]string, len(keys))
	copy(sorted, keys)
	sort.Strings(sorted)
	quoted := make([]string, len(sorted))
	for i, k := range sorted {
		quoted[i] = fmt.Sprintf("%q", k)
	}
	return strings.Join(quoted, ",")
}
