//! `rkb`, the benchmark harness of rukernel.
//!
//! The harness runs each workload on rukernel and on Linux on the same machine, with the same
//! mitigations. It compares the results with a Mann-Whitney test and applies the gate of the class.
//! The design is in spec/19-rukernel-bench.md in tamnd/rukernel.

#![forbid(unsafe_code)]

mod data;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "\
rkb, the benchmark harness of rukernel

usage: rkb <command>

commands that work now:
  check     examine the machines and the workloads for errors
  version   show the version of rkb
  help      show this text

commands that come with the milestones of tamnd/rukernel:
  run       run the workloads on rukernel and on Linux (R4)
  compare   compare two runs with the Mann-Whitney test (R4)
  gate      apply the gate of each class and show the red rows (R4)
  report    write the bench table and the charts (R4)
  profile   record a profile of both kernels for one workload (R4)";

/// The benchmark classes and their gates, per spec/19-rukernel-bench.md section 19.1. A gate is
/// the largest loss against Linux, in percent, that a class can have at 1.0.
const CLASSES: &[(&str, u32)] = &[
    ("entry", 5),
    ("sched", 5),
    ("sched-p99", 10),
    ("mem", 10),
    ("storage", 5),
    ("fsmeta", 10),
    ("net", 5),
    ("net-p99", 10),
    ("apps", 5),
];

/// The keys that each workload must have.
const WORKLOAD_KEYS: &[&str] =
    &["name", "class", "command", "metric", "unit", "direction", "gate", "runs"];

fn main() -> ExitCode {
    let command = std::env::args().nth(1).unwrap_or_else(|| String::from("help"));
    match command.as_str() {
        "check" => match check(Path::new(".")) {
            Ok(count) => {
                println!("rkb check: ok, {count} workloads");
                ExitCode::SUCCESS
            }
            Err(errors) => {
                eprintln!("rkb check: {} problems\n{}", errors.len(), errors.join("\n"));
                ExitCode::FAILURE
            }
        },
        "version" | "--version" | "-V" => {
            println!("rkb {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        "help" | "--help" | "-h" => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        "run" | "compare" | "gate" | "report" | "profile" => {
            eprintln!("rkb {command}: this command is not in this version. Refer to `rkb help`.");
            ExitCode::from(2)
        }
        _ => {
            eprintln!("rkb: unknown command `{command}`\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// Examines `machines.toml` and each `bench.toml` under `workloads/`. Gives the number of
/// workloads, or each error.
fn check(root: &Path) -> Result<usize, Vec<String>> {
    let mut errors = Vec::new();

    match read_tables(&root.join("machines.toml"), "machines.toml") {
        Ok(tables) => {
            for t in tables.iter().skip(1) {
                if t.name != "machine" {
                    errors.push(format!(
                        "machines.toml:{}: [[{}]] is not [[machine]]",
                        t.line, t.name
                    ));
                    continue;
                }
                errors.extend(t.require(
                    "machines.toml",
                    &["id", "cpu", "memory", "microcode", "mitigations", "owner"],
                ));
            }
        }
        Err(e) => errors.push(e),
    }

    let mut files = Vec::new();
    find_workloads(&root.join("workloads"), &mut files, &mut errors);
    files.sort();
    for path in &files {
        let shown = path.strip_prefix(root).unwrap_or(path).display().to_string();
        let tables = match read_tables(path, &shown) {
            Ok(tables) => tables,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };
        let top = &tables[0];
        errors.extend(top.require(&shown, WORKLOAD_KEYS));
        let class = top.values.get("class").map(String::as_str).unwrap_or("");
        match CLASSES.iter().find(|(c, _)| *c == class) {
            None => errors.push(format!("{shown}: unknown class `{class}`")),
            Some((_, limit)) => {
                let gate = top.values.get("gate").and_then(|g| g.parse::<u32>().ok());
                if gate.is_some_and(|g| g > *limit) {
                    errors.push(format!(
                        "{shown}: gate is wider than the {limit}% of class `{class}`"
                    ));
                }
            }
        }
        if let Some(d) = top.values.get("direction")
            && d != "lower-is-better"
            && d != "higher-is-better"
        {
            errors.push(format!(
                "{shown}: direction `{d}` is not lower-is-better or higher-is-better"
            ));
        }
        if top.values.get("runs").and_then(|r| r.parse::<u32>().ok()).is_some_and(|r| r < 10) {
            errors.push(format!("{shown}: runs must be 10 or more, per spec/19-rukernel-bench.md"));
        }
    }

    if errors.is_empty() { Ok(files.len()) } else { Err(errors) }
}

fn find_workloads(dir: &Path, files: &mut Vec<PathBuf>, errors: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            errors.push(format!("{}: {e}", dir.display()));
            return;
        }
    };
    for path in entries.filter_map(|e| e.ok()).map(|e| e.path()) {
        if path.is_dir() {
            find_workloads(&path, files, errors);
        } else if path.file_name().is_some_and(|n| n == "bench.toml") {
            files.push(path);
        }
    }
}

fn read_tables(path: &Path, shown: &str) -> Result<Vec<data::Table>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{shown}: {e}"))?;
    data::parse(shown, &text)
}

#[cfg(test)]
mod tests {
    use super::check;
    use std::path::Path;

    #[test]
    fn the_data_in_this_repository_is_correct() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        assert_eq!(check(&root).map_err(|e| e.join("\n")), Ok(2));
    }
}
