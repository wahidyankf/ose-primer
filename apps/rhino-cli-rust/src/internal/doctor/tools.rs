//! Tool-check definitions.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/doctor/tools.go`. Each
//! [`ToolDef`] describes how to probe a tool, parse its version, compare it
//! against the requirement, read the requirement source, and (optionally)
//! install it. The ordering of the returned slice IS the report ordering.

use std::path::Path;

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
use super::types::ToolStatus;

/// A single installation command. Mirrors Go `InstallStep`.
pub struct InstallStep {
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
}

/// Closure that returns install steps for a tool on a platform (`"darwin"` or
/// `"linux"`). Empty result means "cannot install on this platform". Mirrors Go
/// `InstallFunc`. `None` (the field is `Option`) means "cannot auto-install".
type InstallFn = Box<dyn Fn(&str, &str) -> Vec<InstallStep>>;

/// Version parser closure: raw command output → parsed version string.
type ParseFn = Box<dyn Fn(&str) -> String>;

/// Comparator closure: `(installed, required, browsers_present) -> (status, note)`.
type CompareFn = Box<dyn Fn(&str, &str, bool) -> (ToolStatus, String)>;

/// Requirement reader closure: returns the required version (`""` when none).
type ReadReqFn = Box<dyn Fn() -> String>;

/// One tool-check definition. Mirrors Go `toolDef`.
pub struct ToolDef {
    pub name: &'static str,
    pub binary: &'static str,
    pub source: &'static str,
    pub args: Vec<&'static str>,
    pub use_stderr: bool,
    pub parse_ver: ParseFn,
    pub compare: CompareFn,
    pub read_req: ReadReqFn,
    pub install_cmd: Option<InstallFn>,
}

/// Helper: a comparator that ignores `browsers_present`.
fn plain(f: fn(&str, &str) -> (ToolStatus, String)) -> CompareFn {
    Box::new(move |i, r, _| f(i, r))
}

/// Builds the ordered list of tool definitions for `repo_root`. Mirrors Go
/// `buildToolDefs`.
pub fn build_tool_defs(repo_root: &Path) -> Vec<ToolDef> {
    let package_json = repo_root.join("package.json");
    let pom_xml = repo_root
        .join("apps")
        .join("crud-be-fsharp-giraffe-jasb")
        .join("pom.xml");
    let go_mod = repo_root.join("apps").join("rhino-cli").join("go.mod");
    let python_version = repo_root
        .join("apps")
        .join("crud-be-python-fastapi")
        .join(".python-version");
    let tool_versions = repo_root.join(".tool-versions");
    let global_json = repo_root
        .join("apps")
        .join("crud-be-fsharp-giraffe")
        .join("global.json");
    let pubspec = repo_root
        .join("apps")
        .join("crud-fe-dart-flutterweb")
        .join("pubspec.yaml");
    let cargo_toml = repo_root
        .join("apps")
        .join("crud-be-rust-axum")
        .join("Cargo.toml");

    let mut defs = build_core_tools(&package_json, &pom_xml);
    defs.extend(build_system_lang_tools(
        &go_mod,
        &python_version,
        &cargo_toml,
    ));
    defs.extend(build_beam_tools(&tool_versions));
    defs.extend(build_platform_tools(&global_json, &pubspec));
    defs.extend(build_infra_tools());
    defs
}

/// git, volta, node, npm, java, maven.
fn build_core_tools(package_json: &std::path::Path, pom_xml: &std::path::Path) -> Vec<ToolDef> {
    let mut defs = build_vcs_node_tools(package_json);
    defs.extend(build_jvm_tools(pom_xml));
    defs
}

/// git, volta, node, npm.
fn build_vcs_node_tools(package_json: &std::path::Path) -> Vec<ToolDef> {
    let no_req = || Box::new(String::new) as Box<dyn Fn() -> String>;
    let mut defs: Vec<ToolDef> = Vec::new();
    defs.push(ToolDef {
        name: "git",
        binary: "git",
        source: "(no config file)",
        args: vec!["--version"],
        use_stderr: false,
        parse_ver: Box::new(|s| parse_line_word(s, "git version ", 2, "")),
        compare: plain(compare_exact),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, platform| {
            if platform == "darwin" {
                vec![InstallStep {
                    description: "Install Xcode Command Line Tools".to_string(),
                    command: "xcode-select".to_string(),
                    args: vec!["--install".to_string()],
                }]
            } else {
                vec![InstallStep {
                    description: "Install git".to_string(),
                    command: "sudo".to_string(),
                    args: vec![
                        "apt-get".to_string(),
                        "install".to_string(),
                        "-y".to_string(),
                        "git".to_string(),
                    ],
                }]
            }
        })),
    });
    defs.push(ToolDef {
        name: "volta",
        binary: "volta",
        source: "(no config file)",
        args: vec!["--version"],
        use_stderr: false,
        parse_ver: Box::new(parse_trim_version),
        compare: plain(compare_exact),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, _platform| {
            vec![InstallStep {
                description: "Install Volta".to_string(),
                command: "bash".to_string(),
                args: vec![
                    "-c".to_string(),
                    "curl https://get.volta.sh | bash".to_string(),
                ],
            }]
        })),
    });
    {
        let pj = package_json.to_path_buf();
        defs.push(ToolDef {
            name: "node",
            binary: "node",
            source: "package.json \u{2192} volta.node",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_trim_version),
            compare: plain(compare_exact),
            read_req: Box::new(move || read_node_version(&pj).unwrap_or_default()),
            install_cmd: Some(Box::new(|req, _platform| {
                vec![InstallStep {
                    description: format!("Install Node.js {req} via Volta"),
                    command: "volta".to_string(),
                    args: vec!["install".to_string(), format!("node@{req}")],
                }]
            })),
        });
    }
    {
        let pj = package_json.to_path_buf();
        defs.push(ToolDef {
            name: "npm",
            binary: "npm",
            source: "package.json \u{2192} volta.npm",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_trim_version),
            compare: plain(compare_exact),
            read_req: Box::new(move || read_npm_version(&pj).unwrap_or_default()),
            install_cmd: Some(Box::new(|req, _platform| {
                vec![InstallStep {
                    description: format!("Install npm {req} via Volta"),
                    command: "volta".to_string(),
                    args: vec!["install".to_string(), format!("npm@{req}")],
                }]
            })),
        });
    }
    defs
}

/// java, maven.
fn build_jvm_tools(pom_xml: &std::path::Path) -> Vec<ToolDef> {
    let no_req = || Box::new(String::new) as Box<dyn Fn() -> String>;
    let mut defs: Vec<ToolDef> = Vec::new();
    {
        let pom = pom_xml.to_path_buf();
        defs.push(ToolDef {
            name: "java",
            binary: "java",
            source: "apps/crud-be-fsharp-giraffe-jasb/pom.xml \u{2192} <java.version>",
            args: vec!["-version"],
            use_stderr: true,
            parse_ver: Box::new(parse_java_version),
            compare: plain(compare_major),
            read_req: Box::new(move || read_java_version(&pom).unwrap_or_default()),
            install_cmd: Some(Box::new(|req, _platform| {
                vec![InstallStep {
                    description: format!("Install Java {req} via SDKMAN"),
                    command: "bash".to_string(),
                    args: vec![
                        "-c".to_string(),
                        format!(
                            "source \"$HOME/.sdkman/bin/sdkman-init.sh\" && sdk install java {req}-tem"
                        ),
                    ],
                }]
            })),
        });
    }
    defs.push(ToolDef {
        name: "maven",
        binary: "mvn",
        source: "(no config file)",
        args: vec!["--version"],
        use_stderr: false,
        parse_ver: Box::new(|s| parse_line_word(s, "Apache Maven ", 2, "")),
        compare: plain(compare_exact),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, _platform| {
            vec![InstallStep {
                description: "Install Maven via SDKMAN".to_string(),
                command: "bash".to_string(),
                args: vec![
                    "-c".to_string(),
                    "source \"$HOME/.sdkman/bin/sdkman-init.sh\" && sdk install maven".to_string(),
                ],
            }]
        })),
    });
    defs
}

/// golang, python, rust, cargo-llvm-cov.
fn build_system_lang_tools(
    go_mod: &std::path::Path,
    python_version: &std::path::Path,
    cargo_toml: &std::path::Path,
) -> Vec<ToolDef> {
    let mut defs = build_go_python_tools(go_mod, python_version);
    defs.extend(build_rust_tools(cargo_toml));
    defs
}

/// golang, python.
fn build_go_python_tools(
    go_mod: &std::path::Path,
    python_version: &std::path::Path,
) -> Vec<ToolDef> {
    let mut defs: Vec<ToolDef> = Vec::new();

    {
        let gm = go_mod.to_path_buf();
        defs.push(ToolDef {
            name: "golang",
            binary: "go",
            source: "apps/rhino-cli/go.mod \u{2192} go directive",
            args: vec!["version"],
            use_stderr: false,
            parse_ver: Box::new(|s| parse_line_word(s, "go version ", 2, "go")),
            compare: plain(compare_gte),
            read_req: Box::new(move || read_go_version(&gm).unwrap_or_default()),
            install_cmd: Some(Box::new(|req, platform| {
                if platform == "darwin" {
                    vec![InstallStep {
                        description: "Install Go via Homebrew".to_string(),
                        command: "brew".to_string(),
                        args: vec!["install".to_string(), "go".to_string()],
                    }]
                } else {
                    vec![InstallStep {
                        description: "Install Go from go.dev".to_string(),
                        command: "bash".to_string(),
                        args: vec![
                            "-c".to_string(),
                            format!(
                                "curl -L https://go.dev/dl/go{req}.linux-amd64.tar.gz | sudo tar -xz -C /usr/local"
                            ),
                        ],
                    }]
                }
            })),
        });
    }
    {
        let pv = python_version.to_path_buf();
        defs.push(ToolDef {
            name: "python",
            binary: "python3",
            source: "apps/crud-be-python-fastapi/.python-version",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_python_version),
            compare: plain(compare_gte),
            read_req: Box::new(move || read_python_version(&pv).unwrap_or_default()),
            install_cmd: Some(Box::new(|req, platform| {
                if platform == "darwin" {
                    vec![
                        InstallStep {
                            description: "Install pyenv via Homebrew".to_string(),
                            command: "brew".to_string(),
                            args: vec!["install".to_string(), "pyenv".to_string()],
                        },
                        InstallStep {
                            description: format!("Install Python {req}"),
                            command: "bash".to_string(),
                            args: vec![
                                "-c".to_string(),
                                format!("pyenv install {req} && pyenv global {req}"),
                            ],
                        },
                    ]
                } else {
                    vec![
                        InstallStep {
                            description: "Install pyenv".to_string(),
                            command: "bash".to_string(),
                            args: vec![
                                "-c".to_string(),
                                "curl https://pyenv.run | bash".to_string(),
                            ],
                        },
                        InstallStep {
                            description: format!("Install Python {req}"),
                            command: "bash".to_string(),
                            args: vec![
                                "-c".to_string(),
                                format!("pyenv install {req} && pyenv global {req}"),
                            ],
                        },
                    ]
                }
            })),
        });
    }
    defs
}

/// rust, cargo-llvm-cov.
fn build_rust_tools(cargo_toml: &std::path::Path) -> Vec<ToolDef> {
    let no_req = || Box::new(String::new) as Box<dyn Fn() -> String>;
    let mut defs: Vec<ToolDef> = Vec::new();
    {
        let ct = cargo_toml.to_path_buf();
        defs.push(ToolDef {
            name: "rust",
            binary: "rustc",
            source: "apps/crud-be-rust-axum/Cargo.toml \u{2192} rust-version",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_rust_version),
            compare: plain(compare_gte),
            read_req: Box::new(move || read_rust_version(&ct).unwrap_or_default()),
            install_cmd: Some(Box::new(|_req, _platform| {
                vec![InstallStep {
                    description: "Install Rust via rustup".to_string(),
                    command: "bash".to_string(),
                    args: vec![
                        "-c".to_string(),
                        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
                            .to_string(),
                    ],
                }]
            })),
        });
    }
    defs.push(ToolDef {
        name: "cargo-llvm-cov",
        binary: "cargo",
        source: "(no config file)",
        args: vec!["llvm-cov", "--version"],
        use_stderr: false,
        parse_ver: Box::new(parse_cargo_llvm_cov),
        compare: plain(compare_exact),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, _platform| {
            vec![InstallStep {
                description: "Install cargo-llvm-cov".to_string(),
                command: "bash".to_string(),
                args: vec![
                    "-c".to_string(),
                    "source \"$HOME/.cargo/env\" && cargo install cargo-llvm-cov".to_string(),
                ],
            }]
        })),
    });
    defs
}

/// elixir, erlang.
fn build_beam_tools(tool_versions: &std::path::Path) -> Vec<ToolDef> {
    let mut defs: Vec<ToolDef> = Vec::new();
    {
        let tv = tool_versions.to_path_buf();
        defs.push(ToolDef {
            name: "elixir",
            binary: "elixir",
            source: ".tool-versions \u{2192} elixir",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_elixir_version),
            compare: plain(compare_gte),
            read_req: Box::new(move || {
                let v = read_tool_versions_entry(&tv, "elixir").unwrap_or_default();
                // Strip -otp-XX suffix: "1.19.5-otp-27" → "1.19.5".
                if let Some(idx) = v.find("-otp-") { v[..idx].to_string() } else { v }
            }),
            install_cmd: Some(Box::new(|req, _platform| {
                vec![InstallStep {
                    description: format!("Install Elixir {req} via asdf"),
                    command: "bash".to_string(),
                    args: vec![
                        "-c".to_string(),
                        format!(
                            "asdf plugin add elixir 2>/dev/null; asdf install elixir {req} && asdf global elixir {req}"
                        ),
                    ],
                }]
            })),
        });
    }
    {
        let tv = tool_versions.to_path_buf();
        defs.push(ToolDef {
            name: "erlang",
            binary: "erl",
            source: ".tool-versions \u{2192} erlang",
            args: vec![
                "-noshell",
                "-eval",
                "io:format(\"~s\",[erlang:system_info(otp_release)]),halt().",
            ],
            use_stderr: false,
            parse_ver: Box::new(parse_erlang_version),
            compare: plain(compare_major_gte),
            read_req: Box::new(move || {
                read_tool_versions_entry(&tv, "erlang").unwrap_or_default()
            }),
            install_cmd: Some(Box::new(|req, _platform| {
                vec![InstallStep {
                    description: format!("Install Erlang {req} via asdf"),
                    command: "bash".to_string(),
                    args: vec![
                        "-c".to_string(),
                        format!(
                            "asdf plugin add erlang 2>/dev/null; asdf install erlang {req} && asdf global erlang {req}"
                        ),
                    ],
                }]
            })),
        });
    }
    defs
}

/// dotnet, clojure, dart, flutter.
fn build_platform_tools(global_json: &std::path::Path, pubspec: &std::path::Path) -> Vec<ToolDef> {
    let mut defs = build_dotnet_tool(global_json);
    defs.extend(build_clojure_dart_tools(pubspec));
    defs
}

/// dotnet.
fn build_dotnet_tool(global_json: &std::path::Path) -> Vec<ToolDef> {
    let mut defs: Vec<ToolDef> = Vec::new();
    {
        let gj = global_json.to_path_buf();
        defs.push(ToolDef {
            name: "dotnet",
            binary: "dotnet",
            source: "apps/crud-be-fsharp-giraffe/global.json \u{2192} sdk.version",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_dotnet_version),
            compare: plain(compare_major_gte),
            read_req: Box::new(move || read_dotnet_version(&gj).unwrap_or_default()),
            install_cmd: Some(Box::new(|_req, platform| {
                if platform == "darwin" {
                    vec![InstallStep {
                        description: "Install .NET via Homebrew".to_string(),
                        command: "brew".to_string(),
                        args: vec!["install".to_string(), "dotnet".to_string()],
                    }]
                } else {
                    vec![InstallStep {
                        description: "Install .NET via snap".to_string(),
                        command: "sudo".to_string(),
                        args: vec![
                            "snap".to_string(),
                            "install".to_string(),
                            "dotnet-sdk".to_string(),
                            "--classic".to_string(),
                            "--channel=10.0".to_string(),
                        ],
                    }]
                }
            })),
        });
    }
    defs
}

/// clojure, dart, flutter.
fn build_clojure_dart_tools(pubspec: &std::path::Path) -> Vec<ToolDef> {
    let no_req = || Box::new(String::new) as Box<dyn Fn() -> String>;
    let mut defs: Vec<ToolDef> = Vec::new();
    defs.push(ToolDef {
        name: "clojure",
        binary: "clj",
        source: "(no config file)",
        args: vec!["--version"],
        use_stderr: false,
        parse_ver: Box::new(parse_clojure_version),
        compare: plain(compare_exact),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, platform| {
            if platform == "darwin" {
                vec![InstallStep {
                    description: "Install Clojure via Homebrew".to_string(),
                    command: "brew".to_string(),
                    args: vec!["install".to_string(), "clojure/tools/clojure".to_string()],
                }]
            } else {
                vec![InstallStep {
                    description: "Install Clojure CLI".to_string(),
                    command: "bash".to_string(),
                    args: vec![
                        "-c".to_string(),
                        "curl -L -O https://github.com/clojure/brew-install/releases/latest/download/linux-install.sh && chmod +x linux-install.sh && sudo ./linux-install.sh && rm linux-install.sh".to_string(),
                    ],
                }]
            }
        })),
    });
    {
        let ps = pubspec.to_path_buf();
        defs.push(ToolDef {
            name: "dart",
            binary: "dart",
            source: "apps/crud-fe-dart-flutterweb/pubspec.yaml \u{2192} environment.sdk",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_dart_version),
            compare: plain(compare_gte),
            read_req: Box::new(move || read_dart_sdk_version(&ps).unwrap_or_default()),
            install_cmd: None, // Installed as part of Flutter.
        });
    }
    {
        let ps = pubspec.to_path_buf();
        defs.push(ToolDef {
            name: "flutter",
            binary: "flutter",
            source: "apps/crud-fe-dart-flutterweb/pubspec.yaml \u{2192} environment.flutter",
            args: vec!["--version"],
            use_stderr: false,
            parse_ver: Box::new(parse_flutter_version),
            compare: plain(compare_gte),
            read_req: Box::new(move || read_flutter_version(&ps).unwrap_or_default()),
            install_cmd: Some(Box::new(|_req, platform| {
                if platform == "darwin" {
                    vec![InstallStep {
                        description: "Install Flutter via Homebrew".to_string(),
                        command: "brew".to_string(),
                        args: vec![
                            "install".to_string(),
                            "--cask".to_string(),
                            "flutter".to_string(),
                        ],
                    }]
                } else {
                    vec![InstallStep {
                        description: "Install Flutter via snap".to_string(),
                        command: "sudo".to_string(),
                        args: vec![
                            "snap".to_string(),
                            "install".to_string(),
                            "flutter".to_string(),
                            "--classic".to_string(),
                        ],
                    }]
                }
            })),
        });
    }
    defs
}

/// docker, jq, playwright.
fn build_infra_tools() -> Vec<ToolDef> {
    let no_req = || Box::new(String::new) as Box<dyn Fn() -> String>;
    let mut defs: Vec<ToolDef> = Vec::new();
    defs.push(ToolDef {
        name: "docker",
        binary: "docker",
        source: "(no config file)",
        args: vec!["--version"],
        use_stderr: false,
        parse_ver: Box::new(parse_docker_version),
        compare: plain(compare_exact),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, platform| {
            if platform == "darwin" {
                Vec::new() // Docker Desktop must be installed manually on macOS.
            } else {
                vec![InstallStep {
                    description: "Install Docker".to_string(),
                    command: "sudo".to_string(),
                    args: vec![
                        "apt-get".to_string(),
                        "install".to_string(),
                        "-y".to_string(),
                        "docker.io".to_string(),
                        "docker-compose-v2".to_string(),
                    ],
                }]
            }
        })),
    });
    defs.push(ToolDef {
        name: "jq",
        binary: "jq",
        source: "(no config file)",
        args: vec!["--version"],
        use_stderr: false,
        parse_ver: Box::new(parse_jq_version),
        compare: plain(compare_exact),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, platform| {
            if platform == "darwin" {
                vec![InstallStep {
                    description: "Install jq via Homebrew".to_string(),
                    command: "brew".to_string(),
                    args: vec!["install".to_string(), "jq".to_string()],
                }]
            } else {
                vec![InstallStep {
                    description: "Install jq".to_string(),
                    command: "sudo".to_string(),
                    args: vec![
                        "apt-get".to_string(),
                        "install".to_string(),
                        "-y".to_string(),
                        "jq".to_string(),
                    ],
                }]
            }
        })),
    });
    defs.push(ToolDef {
        name: "playwright",
        binary: "npx",
        source: "node_modules (npx playwright)",
        args: vec!["playwright", "--version"],
        use_stderr: false,
        parse_ver: Box::new(parse_playwright_version),
        compare: Box::new(compare_playwright),
        read_req: no_req(),
        install_cmd: Some(Box::new(|_req, platform| {
            if platform == "darwin" {
                vec![InstallStep {
                    description: "Install Playwright browsers".to_string(),
                    command: "npx".to_string(),
                    args: vec!["playwright".to_string(), "install".to_string()],
                }]
            } else {
                vec![
                    InstallStep {
                        description: "Install Playwright browsers".to_string(),
                        command: "npx".to_string(),
                        args: vec!["playwright".to_string(), "install".to_string()],
                    },
                    InstallStep {
                        description: "Install Playwright system deps".to_string(),
                        command: "npx".to_string(),
                        args: vec!["playwright".to_string(), "install-deps".to_string()],
                    },
                ]
            }
        })),
    });
    defs
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn build_defs_has_19_tools_in_order() {
        let tmp = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(tmp.path());
        let names: Vec<&str> = defs.iter().map(|d| d.name).collect();
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
    fn source_strings_match_go() {
        let tmp = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(tmp.path());
        let by = |n: &str| defs.iter().find(|d| d.name == n).unwrap();
        assert_eq!(by("node").source, "package.json \u{2192} volta.node");
        assert_eq!(
            by("java").source,
            "apps/crud-be-fsharp-giraffe-jasb/pom.xml \u{2192} <java.version>"
        );
        assert_eq!(
            by("golang").source,
            "apps/rhino-cli/go.mod \u{2192} go directive"
        );
        assert!(by("git").source == "(no config file)");
        assert_eq!(by("playwright").source, "node_modules (npx playwright)");
    }

    #[test]
    fn java_uses_stderr_others_stdout() {
        let tmp = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(tmp.path());
        assert!(defs.iter().find(|d| d.name == "java").unwrap().use_stderr);
        assert!(!defs.iter().find(|d| d.name == "node").unwrap().use_stderr);
    }

    #[test]
    fn dart_has_no_install_command() {
        let tmp = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(tmp.path());
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
    fn install_steps_differ_by_platform() {
        let tmp = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(tmp.path());
        let go = defs.iter().find(|d| d.name == "golang").unwrap();
        let cmd = go.install_cmd.as_ref().unwrap();
        assert_eq!(cmd("1.24.0", "darwin")[0].command, "brew");
        let linux = cmd("1.24.0", "linux");
        assert_eq!(linux[0].command, "bash");
        assert!(linux[0].args[1].contains("go1.24.0.linux-amd64"));
        // docker on darwin yields no steps.
        let docker = defs.iter().find(|d| d.name == "docker").unwrap();
        assert!(docker.install_cmd.as_ref().unwrap()("", "darwin").is_empty());
    }

    #[test]
    fn read_req_reads_node_from_config() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("package.json"),
            "{\"volta\":{\"node\":\"24.11.1\",\"npm\":\"11.0.0\"}}",
        )
        .unwrap();
        let defs = build_tool_defs(tmp.path());
        let node = defs.iter().find(|d| d.name == "node").unwrap();
        assert_eq!((node.read_req)(), "24.11.1");
        // no-config tools return empty.
        let git = defs.iter().find(|d| d.name == "git").unwrap();
        assert_eq!((git.read_req)(), "");
    }

    #[test]
    fn elixir_read_req_strips_otp_suffix() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join(".tool-versions"),
            "elixir 1.19.5-otp-27\nerlang 27.3\n",
        )
        .unwrap();
        let defs = build_tool_defs(tmp.path());
        let elixir = defs.iter().find(|d| d.name == "elixir").unwrap();
        assert_eq!((elixir.read_req)(), "1.19.5");
        let erlang = defs.iter().find(|d| d.name == "erlang").unwrap();
        assert_eq!((erlang.read_req)(), "27.3");
    }

    #[test]
    fn parse_ver_closures_work() {
        let tmp = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(tmp.path());
        let git = defs.iter().find(|d| d.name == "git").unwrap();
        assert_eq!((git.parse_ver)("git version 2.43.0"), "2.43.0");
        let pw = defs.iter().find(|d| d.name == "playwright").unwrap();
        assert_eq!((pw.compare)("", "", true).0, ToolStatus::Ok);
        assert_eq!((pw.compare)("", "", false).0, ToolStatus::Warning);
    }

    /// Exercises every tool's install-step closure on both platforms (the
    /// darwin/linux branches), and confirms the per-platform command choices
    /// match the Go reference. This drives the large install closures that the
    /// black-box doctor cucumber test cannot reach deterministically.
    #[test]
    fn install_commands_cover_all_tools_both_platforms() {
        let tmp = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(tmp.path());
        for d in &defs {
            if let Some(cmd) = d.install_cmd.as_ref() {
                for platform in ["darwin", "linux"] {
                    let steps = cmd("1.2.3", platform);
                    // Every produced step must carry a command + description.
                    for s in &steps {
                        assert!(!s.command.is_empty(), "{} on {platform}", d.name);
                        assert!(!s.description.is_empty(), "{} on {platform}", d.name);
                    }
                }
            }
        }

        let by = |n: &str| defs.iter().find(|d| d.name == n).unwrap();
        // Spot-check the per-platform divergences from the Go reference.
        assert_eq!(
            by("git").install_cmd.as_ref().unwrap()("", "darwin")[0].command,
            "xcode-select"
        );
        assert_eq!(
            by("git").install_cmd.as_ref().unwrap()("", "linux")[0].command,
            "sudo"
        );
        assert_eq!(
            by("python").install_cmd.as_ref().unwrap()("3.13", "darwin").len(),
            2
        );
        assert_eq!(
            by("python").install_cmd.as_ref().unwrap()("3.13", "linux").len(),
            2
        );
        assert_eq!(
            by("playwright").install_cmd.as_ref().unwrap()("", "linux").len(),
            2
        );
        assert_eq!(
            by("playwright").install_cmd.as_ref().unwrap()("", "darwin").len(),
            1
        );
        assert_eq!(
            by("clojure").install_cmd.as_ref().unwrap()("", "darwin")[0].command,
            "brew"
        );
        assert_eq!(
            by("dotnet").install_cmd.as_ref().unwrap()("", "linux")[0].command,
            "sudo"
        );
        assert_eq!(
            by("flutter").install_cmd.as_ref().unwrap()("", "darwin")[0].args,
            vec![
                "install".to_string(),
                "--cask".to_string(),
                "flutter".to_string()
            ]
        );
        // jq, maven, volta, rust, cargo-llvm-cov, elixir, erlang, npm, node, java.
        assert!(
            by("jq").install_cmd.as_ref().unwrap()("", "linux")[0]
                .args
                .contains(&"jq".to_string())
        );
        assert!(
            by("maven").install_cmd.as_ref().unwrap()("", "darwin")[0].args[1]
                .contains("sdk install maven")
        );
        assert!(
            by("volta").install_cmd.as_ref().unwrap()("", "linux")[0].args[1]
                .contains("get.volta.sh")
        );
        assert!(
            by("rust").install_cmd.as_ref().unwrap()("", "linux")[0].args[1]
                .contains("sh.rustup.rs")
        );
        assert!(
            by("cargo-llvm-cov").install_cmd.as_ref().unwrap()("", "linux")[0].args[1]
                .contains("cargo install cargo-llvm-cov")
        );
        assert!(
            by("elixir").install_cmd.as_ref().unwrap()("1.19", "linux")[0].args[1]
                .contains("asdf install elixir 1.19")
        );
        assert!(
            by("erlang").install_cmd.as_ref().unwrap()("27", "linux")[0].args[1]
                .contains("asdf install erlang 27")
        );
        assert_eq!(
            by("npm").install_cmd.as_ref().unwrap()("11.0.0", "darwin")[0].args,
            vec!["install".to_string(), "npm@11.0.0".to_string()]
        );
        assert!(
            by("java").install_cmd.as_ref().unwrap()("21", "darwin")[0].args[1]
                .contains("sdk install java 21-tem")
        );
    }
}
