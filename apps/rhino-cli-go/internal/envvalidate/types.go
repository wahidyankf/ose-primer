// Package envvalidate compares declared env vars (.env.example) against
// what each app's source code actually reads, using line-oriented regex
// extractors per language.
//
// Byte-for-byte port target: apps/rhino-cli-rust/src/internal/envvalidate/
package envvalidate

// SurfaceResult holds the validation result for one app surface.
type SurfaceResult struct {
	App             string
	DeclaredNotRead []string // declared in .env.example but not read by source
	ReadNotDeclared []string // read by source but not declared in .env.example
}

// IsOK returns true when there are no violations.
func (s *SurfaceResult) IsOK() bool {
	return len(s.DeclaredNotRead) == 0 && len(s.ReadNotDeclared) == 0
}

// ValidateResult holds validation results for all surfaces.
type ValidateResult struct {
	Surfaces []*SurfaceResult
}

// IsOK returns true when all surfaces are clean.
func (r *ValidateResult) IsOK() bool {
	for _, s := range r.Surfaces {
		if !s.IsOK() {
			return false
		}
	}
	return true
}

// ViolationCount returns the total number of violated surfaces.
func (r *ValidateResult) ViolationCount() int {
	n := 0
	for _, s := range r.Surfaces {
		if !s.IsOK() {
			n++
		}
	}
	return n
}
