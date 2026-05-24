use std::path::PathBuf;
use std::time::Duration;

/// Options controlling a spec-coverage scan. Mirrors Go `ScanOptions`.
#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    /// Absolute path to repository root.
    pub repo_root: PathBuf,
    /// Absolute path to specs directory.
    pub specs_dir: PathBuf,
    /// Absolute path to app directory.
    pub app_dir: PathBuf,
    pub verbose: bool,
    pub quiet: bool,
    /// Skip file matching, validate steps across ALL files.
    pub shared_steps: bool,
    /// Directory names to exclude from spec walking.
    pub exclude_dirs: Vec<String>,
}

/// File-level coverage gap: a `.feature` file with no matching test file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageGap {
    pub spec_file: String,
    pub stem: String,
}

/// Scenario-level gap: a scenario title with no matching test implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioGap {
    pub spec_file: String,
    pub scenario_title: String,
}

/// Step-level gap: a Gherkin step with no matching step definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepGap {
    pub spec_file: String,
    pub scenario_title: String,
    pub step_keyword: String,
    pub step_text: String,
}

/// Aggregate result of a spec-coverage scan. Mirrors Go `CheckResult`.
#[derive(Debug, Clone, Default)]
pub struct CheckResult {
    pub total_specs: usize,
    pub total_scenarios: usize,
    pub total_steps: usize,
    pub gaps: Vec<CoverageGap>,
    pub scenario_gaps: Vec<ScenarioGap>,
    pub step_gaps: Vec<StepGap>,
    pub duration: Duration,
}
