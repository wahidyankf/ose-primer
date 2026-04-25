package mermaid

import "math"

// ValidateOptions configures the thresholds used during validation.
type ValidateOptions struct {
	MaxLabelLen int
	MaxWidth    int
	MaxDepth    int
}

// DefaultValidateOptions returns the standard validation thresholds.
func DefaultValidateOptions() ValidateOptions {
	return ValidateOptions{MaxLabelLen: 30, MaxWidth: 4, MaxDepth: math.MaxInt}
}

// ValidateBlocks validates a slice of MermaidBlocks against the given options.
// It applies three rules:
//  1. Node labels must not exceed MaxLabelLen runes.
//  2. The horizontal dimension (direction-aware) must not exceed MaxWidth unless
//     the vertical dimension also exceeds MaxDepth.
//  3. A block must not contain more than one flowchart/graph header.
//
// Horizontal dimension is direction-aware:
//   - graph LR / RL → horizontal = depth (rank columns), vertical = span
//   - graph TD / TB / BT → horizontal = span (nodes per rank), vertical = depth
//
// Rule 2 special case: when BOTH horizontal > MaxWidth AND vertical > MaxDepth,
// a Warning is emitted instead of a Violation. With default MaxDepth=math.MaxInt
// this branch is inactive unless --max-depth N is passed explicitly.
func ValidateBlocks(blocks []MermaidBlock, opts ValidateOptions) ValidationResult {
	filesSeen := make(map[string]bool)
	var violations []Violation
	var warnings []Warning

	for _, block := range blocks {
		filesSeen[block.FilePath] = true

		diagram, count, _ := ParseDiagram(block)

		// Rule 3: multiple diagrams in one block.
		if count > 1 {
			violations = append(violations, Violation{
				Kind:       ViolationMultipleDiagrams,
				FilePath:   block.FilePath,
				BlockIndex: block.BlockIndex,
				StartLine:  block.StartLine,
			})
		}

		// Non-flowchart: skip Rule 1 and Rule 2.
		if count == 0 {
			continue
		}

		// Rule 1: label length.
		// EffectiveLabelLen handles <br/> multi-line labels by checking the longest line.
		for _, node := range diagram.Nodes {
			labelLen := EffectiveLabelLen(node.Label)
			if labelLen > opts.MaxLabelLen {
				violations = append(violations, Violation{
					Kind:        ViolationLabelTooLong,
					FilePath:    block.FilePath,
					BlockIndex:  block.BlockIndex,
					StartLine:   block.StartLine,
					NodeID:      node.ID,
					LabelText:   node.Label,
					LabelLen:    labelLen,
					MaxLabelLen: opts.MaxLabelLen,
				})
			}
		}

		// Rule 2: direction-aware width/depth check.
		span := MaxWidth(diagram.Nodes, diagram.Edges)
		depth := Depth(diagram.Nodes, diagram.Edges)

		// LR/RL: rank columns flow horizontally → depth is the horizontal dimension.
		// TD/TB/BT: nodes per rank flow horizontally → span is the horizontal dimension.
		var horizontal, vertical int
		switch diagram.Direction {
		case DirectionLR, DirectionRL:
			horizontal, vertical = depth, span
		case DirectionTD, DirectionTB, DirectionBT:
			horizontal, vertical = span, depth
		}

		if horizontal > opts.MaxWidth && vertical > opts.MaxDepth {
			// Both exceeded → warning only.
			warnings = append(warnings, Warning{
				Kind:        WarningComplexDiagram,
				FilePath:    block.FilePath,
				BlockIndex:  block.BlockIndex,
				StartLine:   block.StartLine,
				ActualWidth: span,
				ActualDepth: depth,
				MaxWidth:    opts.MaxWidth,
				MaxDepth:    opts.MaxDepth,
			})
		} else if horizontal > opts.MaxWidth {
			// Horizontal exceeded alone → violation.
			violations = append(violations, Violation{
				Kind:        ViolationWidthExceeded,
				FilePath:    block.FilePath,
				BlockIndex:  block.BlockIndex,
				StartLine:   block.StartLine,
				ActualWidth: span,
				MaxWidth:    opts.MaxWidth,
			})
		}
		// Vertical exceeded alone → no output.
	}

	return ValidationResult{
		FilesScanned:  len(filesSeen),
		BlocksScanned: len(blocks),
		Violations:    violations,
		Warnings:      warnings,
	}
}
