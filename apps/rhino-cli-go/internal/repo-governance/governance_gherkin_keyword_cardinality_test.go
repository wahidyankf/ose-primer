package repogovernance

import (
	"os"
	"path/filepath"
	"testing"
)

func TestGherkinCardinality_FlagsRepeatedWhen(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Scenario: Double when offender\n" +
		"    Given a start\n" +
		"    When the first action runs\n" +
		"    When the second action runs\n" +
		"    Then the outcome is checked\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 1 {
		t.Fatalf("expected 1 finding, got %d: %+v", len(findings), findings)
	}
	f := findings[0]
	if f.Path != "x.feature" {
		t.Errorf("expected path x.feature, got %q", f.Path)
	}
	if f.Line != 3 {
		t.Errorf("expected line 3 (scenario header), got %d", f.Line)
	}
	if f.Scenario != "Double when offender" {
		t.Errorf("expected scenario name, got %q", f.Scenario)
	}
	if f.Detail != "2 When" {
		t.Errorf("expected detail %q, got %q", "2 When", f.Detail)
	}
}

func TestGherkinCardinality_ExemptsBackground(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Background:\n" +
		"    Given one precondition\n" +
		"    Given another precondition\n\n" +
		"  Scenario: Conforming\n" +
		"    Given a thing\n" +
		"    When it acts\n" +
		"    Then it is checked\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 0 {
		t.Fatalf("expected 0 findings, got %d: %+v", len(findings), findings)
	}
}

func TestGherkinCardinality_ExemptsScenarioOutlineExamples(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Scenario Outline: Outline body obeys the rule\n" +
		"    Given a value <v>\n" +
		"    When it is processed\n" +
		"    Then it succeeds\n\n" +
		"    Examples:\n" +
		"      | v |\n" +
		"      | 1 |\n" +
		"      | 2 |\n" +
		"      | 3 |\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 0 {
		t.Fatalf("expected 0 findings, got %d: %+v", len(findings), findings)
	}
}

func TestGherkinCardinality_IgnoresDocstringsAndComments(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Scenario: Docstring and comment heavy\n" +
		"    Given a setup\n" +
		"    When something runs with this payload\n" +
		"      \"\"\"\n" +
		"      When this line is data, not a step\n" +
		"      Then neither is this one\n" +
		"      \"\"\"\n" +
		"    # Then this comment line is ignored\n" +
		"    Then the result is checked\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 0 {
		t.Fatalf("expected 0 findings, got %d: %+v", len(findings), findings)
	}
}

func TestGherkinCardinality_SortsFindingsByPathAndLine(t *testing.T) {
	tmp := t.TempDir()
	violating := "Feature: F\n\n" +
		"  Scenario: Late offender\n" +
		"    Given a start\n" +
		"    Then one outcome\n" +
		"    Then another outcome\n\n" +
		"  Scenario: Early offender on second file\n" +
		"    Given a start\n" +
		"    When one action\n" +
		"    When another action\n" +
		"    Then an outcome\n"
	write := func(rel, content string) {
		p := filepath.Join(tmp, rel)
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write("bbb/late.feature", violating)
	write("aaa/early.feature", violating)

	findings, err := WalkFeatures(tmp)
	if err != nil {
		t.Fatalf("WalkFeatures: %v", err)
	}
	if len(findings) != 4 {
		t.Fatalf("expected 4 findings, got %d: %+v", len(findings), findings)
	}
	for i := 1; i < len(findings); i++ {
		prev, cur := findings[i-1], findings[i]
		if prev.Path > cur.Path || (prev.Path == cur.Path && prev.Line > cur.Line) {
			t.Errorf("findings not sorted by (path, line): %+v before %+v", prev, cur)
		}
	}
	if !startsWithPath(findings[0].Path, filepath.Join(tmp, "aaa")) {
		t.Errorf("expected first finding under aaa/, got %q", findings[0].Path)
	}
}

func startsWithPath(path, prefix string) bool {
	return len(path) >= len(prefix) && path[:len(prefix)] == prefix
}

func TestGherkinCardinality_ConformingFilePasses(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Scenario: Conforming chained scenario\n" +
		"    Given a start\n" +
		"    And another precondition\n" +
		"    When the action runs\n" +
		"    Then the outcome is checked\n" +
		"    And a second outcome is checked\n" +
		"    But a third outcome is absent\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 0 {
		t.Fatalf("expected 0 findings, got %d: %+v", len(findings), findings)
	}
}

func TestGherkinCardinality_CombinedDetailInKeywordOrder(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Scenario: Multi offender\n" +
		"    Given a start\n" +
		"    Given another start\n" +
		"    When one action\n" +
		"    When another action\n" +
		"    Then one outcome\n" +
		"    Then another outcome\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 1 {
		t.Fatalf("expected 1 finding, got %d: %+v", len(findings), findings)
	}
	if findings[0].Detail != "2 Given, 2 When, 2 Then" {
		t.Errorf("expected combined detail, got %q", findings[0].Detail)
	}
}

func TestGherkinCardinality_IgnoresBacktickDocstrings(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Scenario: Backtick docstring\n" +
		"    Given a setup\n" +
		"    When something runs with this payload\n" +
		"      ```\n" +
		"      When this line is data\n" +
		"      ```\n" +
		"    Then the result is checked\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 0 {
		t.Fatalf("expected 0 findings, got %d: %+v", len(findings), findings)
	}
}

func TestGherkinCardinality_ExampleAndRuleHeadersHandled(t *testing.T) {
	content := "Feature: F\n\n" +
		"  Rule: A rule\n\n" +
		"  Example: Offending example block\n" +
		"    Given a start\n" +
		"    When one action\n" +
		"    When another action\n" +
		"    Then an outcome\n"
	findings := ScanFeatureContent("x.feature", content)
	if len(findings) != 1 {
		t.Fatalf("expected 1 finding, got %d: %+v", len(findings), findings)
	}
	if findings[0].Scenario != "Offending example block" {
		t.Errorf("expected Example block name, got %q", findings[0].Scenario)
	}
}

func TestGherkinCardinality_WalkSkipsExcludedDirsAndFixtureTrees(t *testing.T) {
	violating := "Feature: F\n\n" +
		"  Scenario: Offender\n" +
		"    Given a start\n" +
		"    When one action\n" +
		"    When another action\n" +
		"    Then an outcome\n"
	tmp := t.TempDir()
	write := func(rel, content string) {
		p := filepath.Join(tmp, rel)
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	for _, rel := range []string{
		"node_modules/dep.feature",
		"worktrees/w/x.feature",
		"archived/old.feature",
		"libs/elixir-cabbage/test/features/self.feature",
		"libs/elixir-gherkin/test/fixtures/self.feature",
	} {
		write(rel, violating)
	}
	write("specs/bad.feature", violating)
	write("specs/notes.txt", violating)

	findings, err := WalkFeatures(tmp)
	if err != nil {
		t.Fatalf("WalkFeatures: %v", err)
	}
	if len(findings) != 1 {
		t.Fatalf("expected exactly 1 finding, got %d: %+v", len(findings), findings)
	}
	if filepath.ToSlash(findings[0].Path) != filepath.ToSlash(filepath.Join(tmp, "specs", "bad.feature")) {
		t.Errorf("expected finding in specs/bad.feature, got %q", findings[0].Path)
	}
}

func TestGherkinCardinality_WalkMissingRootIsEmpty(t *testing.T) {
	missing := filepath.Join(t.TempDir(), "does-not-exist")
	findings, err := WalkFeatures(missing)
	if err != nil {
		t.Fatalf("WalkFeatures: %v", err)
	}
	if len(findings) != 0 {
		t.Fatalf("expected 0 findings, got %d: %+v", len(findings), findings)
	}
}

func TestGherkinCardinality_ScanFeatureFileReadsDisk(t *testing.T) {
	tmp := t.TempDir()
	p := filepath.Join(tmp, "doc.feature")
	content := "Feature: F\n\n" +
		"  Scenario: Offender\n" +
		"    Given a start\n" +
		"    Then one outcome\n" +
		"    Then another outcome\n"
	if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
	findings, err := ScanFeatureFile(p)
	if err != nil {
		t.Fatalf("ScanFeatureFile: %v", err)
	}
	if len(findings) != 1 {
		t.Fatalf("expected 1 finding, got %d: %+v", len(findings), findings)
	}
	if findings[0].Detail != "2 Then" {
		t.Errorf("expected detail %q, got %q", "2 Then", findings[0].Detail)
	}
}

func TestGherkinCardinality_ScanFeatureFileMissingErrors(t *testing.T) {
	if _, err := ScanFeatureFile(filepath.Join(t.TempDir(), "missing.feature")); err == nil {
		t.Fatal("expected read error for missing file")
	}
}

func TestGherkinCardinality_PrimaryKeywordClassification(t *testing.T) {
	cases := map[string]string{
		"Given a thing":        "Given",
		"When acted":           "When",
		"Then checked":         "Then",
		"And chained":          "",
		"But negated":          "",
		"* bulleted":           "",
		"Whenever it happens":  "",
		"Given":                "",
		"| table | row |":      "",
		"Some plain prose row": "",
	}
	for line, want := range cases {
		if got := primaryKeyword(line); got != want {
			t.Errorf("primaryKeyword(%q) = %q, want %q", line, got, want)
		}
	}
}
