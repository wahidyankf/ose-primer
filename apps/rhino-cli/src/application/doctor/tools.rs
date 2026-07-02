//! Port of `apps/rhino-cli/internal/doctor/tools.go`.
//!
//! Defines [`ToolDef`] (the per-tool check configuration) and
//! [`build_tool_defs`] (the ordered list of all known tools), together with
//! their install-step factories and version readers.
//!
//! The 19-tool list mirrors ose-primer's own polyglot demo-app portfolio
//! (`apps/crud-be-*`, `apps/crud-fe-*`): Node/npm via Volta, the JVM stack
//! (Java/Maven), Go, Python, Rust, the BEAM stack (Elixir/Erlang), .NET,
//! Clojure, Dart/Flutter, plus the cross-cutting infra tools (git, Docker,
//! jq, Playwright). Do not narrow this list to match ose-public's own
//! (much smaller) app portfolio — the two repos intentionally diverge here.

use std::path::{Path, PathBuf};

use super::ToolStatus;
use super::checker::{
    compare_exact, compare_gte, compare_major, compare_major_gte, compare_playwright,
    parse_cargo_llvm_cov, parse_clojure_version, parse_dart_version, parse_docker_version,
    parse_dotnet_version, parse_elixir_version, parse_erlang_version, parse_flutter_version,
    parse_java_version, parse_jq_version, parse_line_word, parse_playwright_version,
    parse_python_version, parse_rust_version, parse_trim_version, read_dart_sdk_version,
    read_dotnet_version, read_flutter_version, read_go_version, read_java_version,
    read_node_version, read_npm_version, read_python_version, read_rust_version,
    read_tool_versions_entry,
};

/// A single step in an auto-install sequence.
pub struct InstallStep {
    /// Short description shown to the user (e.g. `"Install Node.js 24.11.1 via Volta"`).
    pub description: String,
    /// Command to run (e.g. `"volta"`).
    pub command: String,
    /// Arguments passed to `command`.
    pub args: Vec<String>,
}

/// Function pointer that returns platform-specific install steps.
///
/// `required` is the version string from the project config; `platform` is
/// `"darwin"`, `"linux"`, or another `std::env::consts::OS` value.
/// Returns an empty `Vec` when auto-install is not supported on `platform`.
pub type InstallFunc = fn(required: &str, platform: &str) -> Vec<InstallStep>;

/// Complete specification for checking one tool.
pub struct ToolDef {
    /// Human-readable name (e.g. `"node"`).
    pub name: String,
    /// Executable name passed to the runner (e.g. `"node"`, `"go"`).
    pub binary: String,
    /// Config file that provides the required version (for display only).
    pub source: String,
    /// Arguments appended to `binary` when querying the installed version.
    pub args: Vec<String>,
    /// When `true`, version information is parsed from stderr instead of stdout.
    pub use_stderr: bool,
    /// Extracts the version string from raw command output.
    pub parse_ver: fn(&str) -> String,
    /// Compares the installed and required versions and returns a status + note.
    pub compare: fn(&str, &str) -> (ToolStatus, String),
    /// Reads the required version from the project config.
    pub read_req: fn() -> String,
    /// Optional install function; `None` means auto-install is unavailable.
    pub install_cmd: Option<InstallFunc>,
}

// --- ToolDef builders ---

/// Returns an empty string indicating no version requirement for this tool.
fn no_req() -> String {
    String::new()
}

/// Extracts the Git version from `git --version` output
/// (e.g. `"git version 2.42.0"`).
fn parse_git_version(s: &str) -> String {
    parse_line_word(s, "git version ", 2, "")
}

/// Extracts the Maven version from `mvn --version` output
/// (e.g. `"Apache Maven 3.9.9 ..."`).
fn parse_maven_version(s: &str) -> String {
    parse_line_word(s, "Apache Maven ", 2, "")
}

/// Extracts the Go version from `go version` output
/// (e.g. `"go version go1.25.0 darwin/arm64"` → `"1.25.0"`).
fn parse_golang_version(s: &str) -> String {
    parse_line_word(s, "go version ", 2, "go")
}

// Per-binary readers using a path captured in a static OnceLock.
// Go's closures capture repo_root; in Rust we precompute paths and stash them via static
// once-locks keyed off PID-stable build_tool_defs(repo_root) call.
use std::sync::OnceLock;

/// Process-wide cached collection of config-file paths derived from the repo root.
static PATHS: OnceLock<Paths> = OnceLock::new();

/// Pre-computed absolute paths to project config files used by version readers.
struct Paths {
    /// Path to the root `package.json` (for `volta.node` / `volta.npm`).
    package_json: PathBuf,
    /// Path to `apps/crud-be-fsharp-giraffe-jasb/pom.xml` (for `<java.version>`).
    pom_xml: PathBuf,
    /// Path to the root `go.work` (for the `go` directive).
    go_work: PathBuf,
    /// Path to `apps/crud-be-python-fastapi/.python-version`.
    python_version: PathBuf,
    /// Path to the root `.tool-versions` (for Elixir / Erlang).
    tool_versions: PathBuf,
    /// Path to `apps/crud-be-fsharp-giraffe/global.json` (for .NET `sdk.version`).
    global_json: PathBuf,
    /// Path to `apps/crud-fe-dart-flutterweb/pubspec.yaml` (for Dart SDK / Flutter versions).
    pubspec: PathBuf,
    /// Path to `apps/crud-be-rust-axum/Cargo.toml` (for `rust-version`).
    cargo_toml: PathBuf,
}

/// Initialises [`PATHS`] from `repo_root`.
///
/// The [`OnceLock`] guarantees only the first call has any effect; subsequent
/// calls with a different root are silently ignored.
fn set_paths(repo_root: &Path) {
    let p = Paths {
        package_json: repo_root.join("package.json"),
        pom_xml: repo_root
            .join("apps")
            .join("crud-be-fsharp-giraffe-jasb")
            .join("pom.xml"),
        go_work: repo_root.join("go.work"),
        python_version: repo_root
            .join("apps")
            .join("crud-be-python-fastapi")
            .join(".python-version"),
        tool_versions: repo_root.join(".tool-versions"),
        global_json: repo_root
            .join("apps")
            .join("crud-be-fsharp-giraffe")
            .join("global.json"),
        pubspec: repo_root
            .join("apps")
            .join("crud-fe-dart-flutterweb")
            .join("pubspec.yaml"),
        cargo_toml: repo_root
            .join("apps")
            .join("crud-be-rust-axum")
            .join("Cargo.toml"),
    };
    // OnceLock — only the first writer wins. For tests we reset via reset_paths.
    let _ = PATHS.set(p);
}

/// Returns a reference to the global [`Paths`] instance.
///
/// # Panics
///
/// Panics when [`set_paths`] has not been called (i.e. [`PATHS`] is still
/// uninitialised), which should never happen in normal usage because
/// [`build_tool_defs`] always calls [`set_paths`] first.
fn p() -> &'static Paths {
    PATHS.get().expect("PATHS not initialized")
}

/// Reads the `node` version from the cached `package.json`.
fn read_node_v() -> String {
    read_node_version(&p().package_json).unwrap_or_default()
}
/// Reads the `npm` version from the cached `package.json`.
fn read_npm_v() -> String {
    read_npm_version(&p().package_json).unwrap_or_default()
}
/// Reads the Java version from the cached `pom.xml`.
fn read_java_v() -> String {
    read_java_version(&p().pom_xml).unwrap_or_default()
}
/// Reads the Go version from the cached `go.work`.
fn read_go_v() -> String {
    read_go_version(&p().go_work).unwrap_or_default()
}
/// Reads the Python version from the cached `.python-version` file.
fn read_python_v() -> String {
    read_python_version(&p().python_version).unwrap_or_default()
}
/// Reads the `rust-version` (MSRV) from the cached `Cargo.toml`.
fn read_rust_v() -> String {
    read_rust_version(&p().cargo_toml).unwrap_or_default()
}
/// Reads the Elixir version from the cached `.tool-versions`, stripping any
/// `-otp-XX` suffix (e.g. `"1.19.5-otp-27"` → `"1.19.5"`).
fn read_elixir_v() -> String {
    let v = read_tool_versions_entry(&p().tool_versions, "elixir").unwrap_or_default();
    if let Some(idx) = v.find("-otp-") {
        v[..idx].to_string()
    } else {
        v
    }
}
/// Reads the Erlang/OTP version from the cached `.tool-versions`.
fn read_erlang_v() -> String {
    read_tool_versions_entry(&p().tool_versions, "erlang").unwrap_or_default()
}
/// Reads the .NET SDK version from the cached `global.json`.
fn read_dotnet_v() -> String {
    read_dotnet_version(&p().global_json).unwrap_or_default()
}
/// Reads the Dart SDK minimum version from the cached `pubspec.yaml`.
fn read_dart_v() -> String {
    read_dart_sdk_version(&p().pubspec).unwrap_or_default()
}
/// Reads the Flutter minimum version from the cached `pubspec.yaml`.
fn read_flutter_v() -> String {
    read_flutter_version(&p().pubspec).unwrap_or_default()
}

// --- Install commands ---

/// Returns install steps for `git`.
///
/// On macOS: `xcode-select --install`.
/// On Linux: `sudo apt-get install -y git`.
fn install_git(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Xcode Command Line Tools".into(),
            command: "xcode-select".into(),
            args: vec!["--install".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install git".into(),
            command: "sudo".into(),
            args: vec![
                "apt-get".into(),
                "install".into(),
                "-y".into(),
                "git".into(),
            ],
        }]
    }
}

/// Returns install steps for Volta (the Node.js version manager).
fn install_volta(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install Volta".into(),
        command: "bash".into(),
        args: vec!["-c".into(), "curl https://get.volta.sh | bash".into()],
    }]
}

/// Returns install steps for Node.js via `volta install node@<req>`.
fn install_node(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Node.js {req} via Volta"),
        command: "volta".into(),
        args: vec!["install".into(), format!("node@{req}")],
    }]
}

/// Returns install steps for npm via `volta install npm@<req>`.
fn install_npm(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install npm {req} via Volta"),
        command: "volta".into(),
        args: vec!["install".into(), format!("npm@{req}")],
    }]
}

/// Returns install steps for Java via SDKMAN.
fn install_java(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Java {req} via SDKMAN"),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            format!("source \"$HOME/.sdkman/bin/sdkman-init.sh\" && sdk install java {req}-tem"),
        ],
    }]
}

/// Returns install steps for Maven via SDKMAN.
fn install_maven(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install Maven via SDKMAN".into(),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            "source \"$HOME/.sdkman/bin/sdkman-init.sh\" && sdk install maven".into(),
        ],
    }]
}

/// Returns install steps for Go.
///
/// On macOS: `brew install go`.
/// On Linux: downloads the pinned tarball from go.dev.
fn install_golang(req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Go via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "go".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install Go from go.dev".into(),
            command: "bash".into(),
            args: vec![
                "-c".into(),
                format!(
                    "curl -L https://go.dev/dl/go{req}.linux-amd64.tar.gz | sudo tar -xz -C /usr/local"
                ),
            ],
        }]
    }
}

/// Returns install steps for Python via pyenv.
fn install_python(req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![
            InstallStep {
                description: "Install pyenv via Homebrew".into(),
                command: "brew".into(),
                args: vec!["install".into(), "pyenv".into()],
            },
            InstallStep {
                description: format!("Install Python {req}"),
                command: "bash".into(),
                args: vec![
                    "-c".into(),
                    format!("pyenv install {req} && pyenv global {req}"),
                ],
            },
        ]
    } else {
        vec![
            InstallStep {
                description: "Install pyenv".into(),
                command: "bash".into(),
                args: vec!["-c".into(), "curl https://pyenv.run | bash".into()],
            },
            InstallStep {
                description: format!("Install Python {req}"),
                command: "bash".into(),
                args: vec![
                    "-c".into(),
                    format!("pyenv install {req} && pyenv global {req}"),
                ],
            },
        ]
    }
}

/// Returns install steps for Rust via `rustup`.
fn install_rust(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install Rust via rustup".into(),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y".into(),
        ],
    }]
}

/// Returns install steps for `cargo-llvm-cov` via `cargo install`.
fn install_cargo_llvm_cov(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install cargo-llvm-cov".into(),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            "source \"$HOME/.cargo/env\" && cargo install cargo-llvm-cov".into(),
        ],
    }]
}

/// Returns install steps for Elixir via asdf.
fn install_elixir(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Elixir {req} via asdf"),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            format!(
                "asdf plugin add elixir 2>/dev/null; asdf install elixir {req} && asdf global elixir {req}"
            ),
        ],
    }]
}

/// Returns install steps for Erlang via asdf.
fn install_erlang(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Erlang {req} via asdf"),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            format!(
                "asdf plugin add erlang 2>/dev/null; asdf install erlang {req} && asdf global erlang {req}"
            ),
        ],
    }]
}

/// Returns install steps for .NET SDK.
///
/// On macOS: `brew install dotnet`.
/// On Linux: `sudo snap install dotnet-sdk --classic --channel=10.0`.
fn install_dotnet(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install .NET via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "dotnet".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install .NET via snap".into(),
            command: "sudo".into(),
            args: vec![
                "snap".into(),
                "install".into(),
                "dotnet-sdk".into(),
                "--classic".into(),
                "--channel=10.0".into(),
            ],
        }]
    }
}

/// Returns install steps for Clojure CLI.
///
/// On macOS: `brew install clojure/tools/clojure`.
/// On Linux: the official install script.
fn install_clojure(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Clojure via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "clojure/tools/clojure".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install Clojure CLI".into(),
            command: "bash".into(),
            args: vec![
                "-c".into(),
                "curl -L -O https://github.com/clojure/brew-install/releases/latest/download/linux-install.sh && chmod +x linux-install.sh && sudo ./linux-install.sh && rm linux-install.sh".into(),
            ],
        }]
    }
}

/// Returns install steps for Flutter (which bundles the Dart SDK).
///
/// On macOS: `brew install --cask flutter`.
/// On Linux: `sudo snap install flutter --classic`.
fn install_flutter(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Flutter via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "--cask".into(), "flutter".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install Flutter via snap".into(),
            command: "sudo".into(),
            args: vec![
                "snap".into(),
                "install".into(),
                "flutter".into(),
                "--classic".into(),
            ],
        }]
    }
}

/// Returns install steps for Docker.
///
/// On macOS: returns an empty `Vec` (Docker Desktop must be installed manually).
/// On Linux: `sudo apt-get install -y docker.io docker-compose-v2`.
fn install_docker(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        // Docker Desktop must be installed manually on macOS.
        Vec::new()
    } else {
        vec![InstallStep {
            description: "Install Docker".into(),
            command: "sudo".into(),
            args: vec![
                "apt-get".into(),
                "install".into(),
                "-y".into(),
                "docker.io".into(),
                "docker-compose-v2".into(),
            ],
        }]
    }
}

/// Returns install steps for `jq`.
///
/// On macOS: `brew install jq`.
/// On Linux: `sudo apt-get install -y jq`.
fn install_jq(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install jq via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "jq".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install jq".into(),
            command: "sudo".into(),
            args: vec!["apt-get".into(), "install".into(), "-y".into(), "jq".into()],
        }]
    }
}

/// Returns install steps for Playwright browsers.
///
/// On macOS: `npx playwright install`.
/// On Linux: `npx playwright install` followed by `npx playwright install-deps`.
fn install_playwright(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Playwright browsers".into(),
            command: "npx".into(),
            args: vec!["playwright".into(), "install".into()],
        }]
    } else {
        vec![
            InstallStep {
                description: "Install Playwright browsers".into(),
                command: "npx".into(),
                args: vec!["playwright".into(), "install".into()],
            },
            InstallStep {
                description: "Install Playwright system deps".into(),
                command: "npx".into(),
                args: vec!["playwright".into(), "install-deps".into()],
            },
        ]
    }
}

/// Build the ordered list of tool defs for the given repo root.
pub fn build_tool_defs(repo_root: &Path) -> Vec<ToolDef> {
    // PATHS is a OnceLock — only set once per process. Tests use isolated runners.
    set_paths(repo_root);
    let mut defs = tool_defs_core();
    defs.extend(tool_defs_system_lang());
    defs.extend(tool_defs_rust());
    defs.extend(tool_defs_beam());
    defs.extend(tool_defs_dotnet());
    defs.extend(tool_defs_clojure_dart());
    defs.extend(tool_defs_infra());
    defs
}

/// Returns the core tool definitions: `git`, `volta`, `node`, `npm`, `java`, `maven`.
fn tool_defs_core() -> Vec<ToolDef> {
    let mut defs = tool_defs_vcs_node();
    defs.extend(tool_defs_jvm());
    defs
}

/// Returns tool definitions for: `git`, `volta`, `node`, `npm`.
fn tool_defs_vcs_node() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "git".into(),
            binary: "git".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_git_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_git),
        },
        ToolDef {
            name: "volta".into(),
            binary: "volta".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_trim_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_volta),
        },
        ToolDef {
            name: "node".into(),
            binary: "node".into(),
            source: "package.json → volta.node".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_trim_version,
            compare: compare_exact,
            read_req: read_node_v,
            install_cmd: Some(install_node),
        },
        ToolDef {
            name: "npm".into(),
            binary: "npm".into(),
            source: "package.json → volta.npm".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_trim_version,
            compare: compare_exact,
            read_req: read_npm_v,
            install_cmd: Some(install_npm),
        },
    ]
}

/// Returns tool definitions for the JVM stack: `java`, `maven`.
fn tool_defs_jvm() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "java".into(),
            binary: "java".into(),
            source: "apps/crud-be-fsharp-giraffe-jasb/pom.xml → <java.version>".into(),
            args: vec!["-version".into()],
            use_stderr: true,
            parse_ver: parse_java_version,
            compare: compare_major,
            read_req: read_java_v,
            install_cmd: Some(install_java),
        },
        ToolDef {
            name: "maven".into(),
            binary: "mvn".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_maven_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_maven),
        },
    ]
}

/// Returns tool definitions for: `golang`, `python`.
fn tool_defs_system_lang() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "golang".into(),
            binary: "go".into(),
            source: "go.work → go directive".into(),
            args: vec!["version".into()],
            use_stderr: false,
            parse_ver: parse_golang_version,
            compare: compare_gte,
            read_req: read_go_v,
            install_cmd: Some(install_golang),
        },
        ToolDef {
            name: "python".into(),
            binary: "python3".into(),
            source: "apps/crud-be-python-fastapi/.python-version".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_python_version,
            compare: compare_gte,
            read_req: read_python_v,
            install_cmd: Some(install_python),
        },
    ]
}

/// Returns tool definitions for Rust: `rust`, `cargo-llvm-cov`.
fn tool_defs_rust() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "rust".into(),
            binary: "rustc".into(),
            source: "apps/crud-be-rust-axum/Cargo.toml → rust-version".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_rust_version,
            compare: compare_gte,
            read_req: read_rust_v,
            install_cmd: Some(install_rust),
        },
        ToolDef {
            name: "cargo-llvm-cov".into(),
            binary: "cargo".into(),
            source: "(no config file)".into(),
            args: vec!["llvm-cov".into(), "--version".into()],
            use_stderr: false,
            parse_ver: parse_cargo_llvm_cov,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_cargo_llvm_cov),
        },
    ]
}

/// Returns tool definitions for the BEAM stack: `elixir`, `erlang`.
fn tool_defs_beam() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "elixir".into(),
            binary: "elixir".into(),
            source: ".tool-versions → elixir".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_elixir_version,
            compare: compare_gte,
            read_req: read_elixir_v,
            install_cmd: Some(install_elixir),
        },
        ToolDef {
            name: "erlang".into(),
            binary: "erl".into(),
            source: ".tool-versions → erlang".into(),
            args: vec![
                "-noshell".into(),
                "-eval".into(),
                "io:format(\"~s\",[erlang:system_info(otp_release)]),halt().".into(),
            ],
            use_stderr: false,
            parse_ver: parse_erlang_version,
            compare: compare_major_gte,
            read_req: read_erlang_v,
            install_cmd: Some(install_erlang),
        },
    ]
}

/// Returns tool definitions for .NET: `dotnet`.
fn tool_defs_dotnet() -> Vec<ToolDef> {
    vec![ToolDef {
        name: "dotnet".into(),
        binary: "dotnet".into(),
        source: "apps/crud-be-fsharp-giraffe/global.json → sdk.version".into(),
        args: vec!["--version".into()],
        use_stderr: false,
        parse_ver: parse_dotnet_version,
        compare: compare_major_gte,
        read_req: read_dotnet_v,
        install_cmd: Some(install_dotnet),
    }]
}

/// Returns tool definitions for: `clojure`, `dart`, `flutter`.
fn tool_defs_clojure_dart() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "clojure".into(),
            binary: "clj".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_clojure_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_clojure),
        },
        ToolDef {
            name: "dart".into(),
            binary: "dart".into(),
            source: "apps/crud-fe-dart-flutterweb/pubspec.yaml → environment.sdk".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_dart_version,
            compare: compare_gte,
            read_req: read_dart_v,
            install_cmd: None, // Installed as part of Flutter.
        },
        ToolDef {
            name: "flutter".into(),
            binary: "flutter".into(),
            source: "apps/crud-fe-dart-flutterweb/pubspec.yaml → environment.flutter".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_flutter_version,
            compare: compare_gte,
            read_req: read_flutter_v,
            install_cmd: Some(install_flutter),
        },
    ]
}

/// Returns tool definitions for infrastructure: `docker`, `jq`, `playwright`.
fn tool_defs_infra() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "docker".into(),
            binary: "docker".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_docker_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_docker),
        },
        ToolDef {
            name: "jq".into(),
            binary: "jq".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_jq_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_jq),
        },
        ToolDef {
            name: "playwright".into(),
            binary: "npx".into(),
            source: "node_modules (npx playwright)".into(),
            args: vec!["playwright".into(), "--version".into()],
            use_stderr: false,
            parse_ver: parse_playwright_version,
            compare: compare_playwright,
            read_req: no_req,
            install_cmd: Some(install_playwright),
        },
    ]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    /// Regression lock (§ ose-primer doctor tool-list restoration): a
    /// cross-repo source sync from ose-public silently narrowed this list
    /// from 19 tools to 13, dropping every tool used exclusively by
    /// ose-primer's own polyglot demo apps (Java, Go, Python, Elixir,
    /// Erlang, Clojure, Dart, Flutter) while gaining ose-public-only tools
    /// (shellcheck, hadolint, actionlint) that ose-primer's own doctor
    /// gherkin spec never asked for. `specs/apps/rhino/behavior/rhino-cli/
    /// gherkin/system/doctor.feature` (via `tests/doctor.rs`) is the
    /// intentional contract for this list — keep this test and that spec in
    /// sync.
    #[test]
    fn build_returns_all_19_polyglot_tools_in_order() {
        let dir = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(dir.path());
        let names: Vec<&str> = defs.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "git",
                "volta",
                "node",
                "npm",
                "java",
                "maven",
                "golang",
                "python",
                "rust",
                "cargo-llvm-cov",
                "elixir",
                "erlang",
                "dotnet",
                "clojure",
                "dart",
                "flutter",
                "docker",
                "jq",
                "playwright",
            ]
        );
    }

    #[test]
    fn dart_has_no_install_command() {
        let dir = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(dir.path());
        assert!(
            defs.iter()
                .find(|d| d.name == "dart")
                .unwrap()
                .install_cmd
                .is_none()
        );
        assert!(
            defs.iter()
                .find(|d| d.name == "git")
                .unwrap()
                .install_cmd
                .is_some()
        );
    }

    #[test]
    fn java_uses_stderr_others_stdout() {
        let dir = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(dir.path());
        assert!(defs.iter().find(|d| d.name == "java").unwrap().use_stderr);
        assert!(!defs.iter().find(|d| d.name == "node").unwrap().use_stderr);
    }

    #[test]
    fn install_git_macos() {
        let steps = install_git("", "darwin");
        assert_eq!(steps[0].command, "xcode-select");
    }

    #[test]
    fn install_git_linux() {
        let steps = install_git("", "linux");
        assert_eq!(steps[0].command, "sudo");
        assert!(steps[0].args.contains(&"git".to_string()));
    }

    #[test]
    fn install_docker_macos_empty() {
        assert!(install_docker("", "darwin").is_empty());
    }

    #[test]
    fn install_node_formats_required() {
        let s = install_node("24.11.1", "darwin");
        assert_eq!(s[0].args[1], "node@24.11.1");
    }

    #[test]
    fn install_golang_differs_by_platform() {
        assert_eq!(install_golang("1.24.0", "darwin")[0].command, "brew");
        let linux = install_golang("1.24.0", "linux");
        assert_eq!(linux[0].command, "bash");
        assert!(linux[0].args[1].contains("go1.24.0.linux-amd64"));
    }

    #[test]
    fn install_java_formats_required() {
        let steps = install_java("21", "darwin");
        assert!(steps[0].args[1].contains("sdk install java 21-tem"));
    }
}
