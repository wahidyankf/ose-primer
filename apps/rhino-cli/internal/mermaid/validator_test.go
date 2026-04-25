package mermaid

import (
	"testing"
)

// makeFlowchartBlock builds a MermaidBlock with the given mermaid source.
func makeFlowchartBlock(source string) MermaidBlock {
	return MermaidBlock{
		FilePath:   "test.md",
		BlockIndex: 0,
		Source:     source,
		StartLine:  1,
	}
}

// span4depth6Source produces a TD flowchart with span=4 and depth=6.
// Ranks: A=0, B/C/D/E=1, F=2, G=3, H=4, I=5 → 6 distinct ranks, max at rank 1 = 4.
const span4depth6Source = `flowchart TD
A --> B
A --> C
A --> D
A --> E
B --> F
F --> G
G --> H
H --> I`

// span4depth4Source produces a TD flowchart with span=4 and depth=4.
// Ranks: A=0, B/C/D/E=1, F=2, G=3 → 4 distinct ranks, max at rank 1 = 4.
const span4depth4Source = `flowchart TD
A --> B
A --> C
A --> D
A --> E
B --> F
F --> G`

// span2depth6Source produces a TD flowchart with span=2 and depth=6.
// A=0, B=1, G=1, C=2, D=3, E=4, F=5 → depth=6, max-width=2.
const span2depth6Source = `flowchart TD
A --> B
A --> G
B --> C
C --> D
D --> E
E --> F`

// span5depth3Source produces a TD flowchart with span=5 and depth=3.
// Ranks: A=0, B/C/D/E/F=1 (span=5), G=2 → depth=3.
const span5depth3Source = `flowchart TD
A --> B
A --> C
A --> D
A --> E
A --> F
B --> G`

// lrDepth6Span2Source produces a LR graph with depth=6 and span=2.
// Chain A→B→C→D→E→F, plus A→G: ranks A=0, B=1 G=1, C=2, D=3, E=4, F=5 → depth=6, span=2.
const lrDepth6Span2Source = `graph LR
A --> B
A --> G
B --> C
C --> D
D --> E
E --> F`

// lrSpan5Depth2Source produces a LR graph with span=5 and depth=2.
// A fans out to 5 children: A=0, B/C/D/E/F=1 → depth=2, span=5.
const lrSpan5Depth2Source = `graph LR
A --> B
A --> C
A --> D
A --> E
A --> F`

// lrDepth4Span6Source produces a LR graph with span=6 and depth=4.
// 6 nodes converge on B: A/C/D/E/F/G=0 (span=6), B=1, H=2, I=3 → depth=4, span=6.
const lrDepth4Span6Source = `graph LR
A --> B
C --> B
D --> B
E --> B
F --> B
G --> B
B --> H
H --> I`

func TestValidateBlocks(t *testing.T) {
	defaultOpts := DefaultValidateOptions()

	tests := []struct {
		name           string
		blocks         []MermaidBlock
		opts           ValidateOptions
		wantViolations int
		wantWarnings   int
		violationKind  ViolationKind
		warningKind    WarningKind
	}{
		{
			name: "clean block short labels width within limit",
			blocks: []MermaidBlock{
				makeFlowchartBlock("flowchart TD\nA[Short] --> B[Label]"),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "label exactly at limit no violation",
			blocks: []MermaidBlock{
				makeFlowchartBlock("flowchart TD\nA[" + repeat30() + "]"),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "label at limit+1 violation",
			blocks: []MermaidBlock{
				makeFlowchartBlock("flowchart TD\nA[" + repeat31() + "]"),
			},
			opts:           defaultOpts,
			wantViolations: 1,
			wantWarnings:   0,
			violationKind:  ViolationLabelTooLong,
		},
		{
			name: "width exactly at limit no violation",
			// A → B, A → C, A → D: span=3 at rank 1 — well within MaxWidth=4
			blocks: []MermaidBlock{
				makeFlowchartBlock("flowchart TD\nA --> B\nA --> C\nA --> D"),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "width exactly at new limit 4 no violation",
			// span=4 = MaxWidth=4; 4 > 4 is false — passes
			blocks: []MermaidBlock{
				makeFlowchartBlock(span4depth4Source),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "width at limit+1 violation",
			// span5depth3Source: TD span=5 > MaxWidth=4, depth=3 ≤ MaxDepth=∞ → violation
			blocks: []MermaidBlock{
				makeFlowchartBlock(span5depth3Source),
			},
			opts:           defaultOpts,
			wantViolations: 1,
			wantWarnings:   0,
			violationKind:  ViolationWidthExceeded,
		},
		{
			name: "non-flowchart block no violations",
			blocks: []MermaidBlock{
				makeFlowchartBlock("sequenceDiagram\nA->>B: hello"),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "multiple diagrams block violation",
			blocks: []MermaidBlock{
				makeFlowchartBlock("flowchart TD\nA --> B\nflowchart LR\nC --> D"),
			},
			opts:           defaultOpts,
			wantViolations: 1,
			violationKind:  ViolationMultipleDiagrams,
		},
		{
			name: "custom opts respected",
			// With MaxLabelLen=40 a 31-char label is fine.
			blocks: []MermaidBlock{
				makeFlowchartBlock("flowchart TD\nA[" + repeat31() + "]"),
			},
			opts:           ValidateOptions{MaxLabelLen: 40, MaxWidth: 5, MaxDepth: 10},
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "both exceeded warning only",
			// span4depth6Source: TD span=4, depth=6; with explicit {MaxWidth:3,MaxDepth:5}:
			// horizontal=span=4>3, vertical=depth=6>5 → both exceeded → warning
			blocks: []MermaidBlock{
				makeFlowchartBlock(span4depth6Source),
			},
			opts:           ValidateOptions{MaxWidth: 3, MaxDepth: 5},
			wantViolations: 0,
			wantWarnings:   1,
			warningKind:    WarningComplexDiagram,
		},
		{
			name: "width only exceeded violation",
			// span4depth4Source: TD span=4, depth=4; with explicit {MaxWidth:3,MaxDepth:5}:
			// horizontal=span=4>3, vertical=depth=4 NOT>5 → violation only
			blocks: []MermaidBlock{
				makeFlowchartBlock(span4depth4Source),
			},
			opts:           ValidateOptions{MaxWidth: 3, MaxDepth: 5},
			wantViolations: 1,
			wantWarnings:   0,
			violationKind:  ViolationWidthExceeded,
		},
		{
			name: "depth only exceeded no output",
			// span2depth6Source: TD span=2, depth=6; horizontal=span=2 ≤ MaxWidth=4 → no output
			blocks: []MermaidBlock{
				makeFlowchartBlock(span2depth6Source),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		// Direction-aware tests: LR graphs check depth (horizontal rank columns).
		{
			name: "LR direction depth exceeded triggers width_exceeded",
			// lrDepth6Span2Source: LR depth=6, span=2; horizontal=depth=6>4 → violation
			blocks: []MermaidBlock{
				makeFlowchartBlock(lrDepth6Span2Source),
			},
			opts:           defaultOpts,
			wantViolations: 1,
			wantWarnings:   0,
			violationKind:  ViolationWidthExceeded,
		},
		{
			name: "LR direction large span does not trigger width_exceeded",
			// lrSpan5Depth2Source: LR span=5, depth=2; horizontal=depth=2 ≤ 4 → no violation
			blocks: []MermaidBlock{
				makeFlowchartBlock(lrSpan5Depth2Source),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "TD direction large span triggers width_exceeded",
			// span5depth3Source: TD span=5, depth=3; horizontal=span=5>4 → violation
			blocks: []MermaidBlock{
				makeFlowchartBlock(span5depth3Source),
			},
			opts:           defaultOpts,
			wantViolations: 1,
			wantWarnings:   0,
			violationKind:  ViolationWidthExceeded,
		},
		{
			name: "TD direction large depth does not trigger width_exceeded",
			// span2depth6Source: TD span=2, depth=6; horizontal=span=2 ≤ 4 → no violation
			blocks: []MermaidBlock{
				makeFlowchartBlock(span2depth6Source),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "TD span exactly at MaxWidth 4 no violation",
			// span4depth4Source: TD span=4; 4 > 4 is false → no violation
			blocks: []MermaidBlock{
				makeFlowchartBlock(span4depth4Source),
			},
			opts:           defaultOpts,
			wantViolations: 0,
			wantWarnings:   0,
		},
		{
			name: "LR both dimensions exceeded emits complex_diagram warning",
			// lrDepth4Span6Source: LR depth=4, span=6; with {MaxWidth:3,MaxDepth:5}:
			// horizontal=depth=4>3, vertical=span=6>5 → both exceeded → warning
			blocks: []MermaidBlock{
				makeFlowchartBlock(lrDepth4Span6Source),
			},
			opts:           ValidateOptions{MaxWidth: 3, MaxDepth: 5},
			wantViolations: 0,
			wantWarnings:   1,
			warningKind:    WarningComplexDiagram,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ValidateBlocks(tt.blocks, tt.opts)
			if len(result.Violations) != tt.wantViolations {
				t.Errorf("violations = %d, want %d; violations: %+v", len(result.Violations), tt.wantViolations, result.Violations)
			}
			if len(result.Warnings) != tt.wantWarnings {
				t.Errorf("warnings = %d, want %d; warnings: %+v", len(result.Warnings), tt.wantWarnings, result.Warnings)
			}
			if tt.violationKind != "" && len(result.Violations) > 0 {
				if result.Violations[0].Kind != tt.violationKind {
					t.Errorf("violation kind = %q, want %q", result.Violations[0].Kind, tt.violationKind)
				}
			}
			if tt.warningKind != "" && len(result.Warnings) > 0 {
				if result.Warnings[0].Kind != tt.warningKind {
					t.Errorf("warning kind = %q, want %q", result.Warnings[0].Kind, tt.warningKind)
				}
			}
		})
	}
}

func TestValidateBlocks_FilesAndBlocksScanned(t *testing.T) {
	blocks := []MermaidBlock{
		{FilePath: "a.md", BlockIndex: 0, Source: "flowchart TD\nA --> B", StartLine: 1},
		{FilePath: "a.md", BlockIndex: 1, Source: "flowchart TD\nC --> D", StartLine: 10},
		{FilePath: "b.md", BlockIndex: 0, Source: "flowchart TD\nE --> F", StartLine: 1},
	}
	result := ValidateBlocks(blocks, DefaultValidateOptions())
	if result.FilesScanned != 2 {
		t.Errorf("FilesScanned = %d, want 2", result.FilesScanned)
	}
	if result.BlocksScanned != 3 {
		t.Errorf("BlocksScanned = %d, want 3", result.BlocksScanned)
	}
}

// repeat30 returns a string of exactly 30 characters.
func repeat30() string {
	return "123456789012345678901234567890"
}

// repeat31 returns a string of exactly 31 characters.
func repeat31() string {
	return "1234567890123456789012345678901"
}
