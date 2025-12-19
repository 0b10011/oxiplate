use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};

#[test]
fn clippy() -> Result<(), Box<dyn Error>> {
    // Build before running tests for consistent test output
    let Output {
        status,
        stdout: _stdout,
        stderr,
    } = Command::new("cargo")
        .args(["build", "--manifest-path", "tests/clippy/Cargo.toml"])
        .output()?;
    if !status.success() {
        Err(format!(
            "Failed to build clippy tests. STDERR: {}",
            String::from_utf8_lossy(&stderr),
        ))?;
    }

    let expected_destination = Path::new("tests/clippy/expected/");
    let actual_destination = Path::new("tests/clippy/actual/");
    let mut mismatched = 0;
    for entry in fs::read_dir("tests/clippy/src/bin")? {
        let entry = entry?;
        let expected_output_path =
            expected_destination.join(format!("{}.stderr", entry.file_name().to_string_lossy()));
        let actual_output_path =
            actual_destination.join(format!("{}.stderr", entry.file_name().to_string_lossy()));

        // Skip non-files (directories)
        if !entry.file_type()?.is_file() || entry.path().to_str() == Some(file!()) {
            continue;
        }

        // Ignore non-rust files
        if !entry.file_name().to_string_lossy().ends_with(".rs") {
            writeln!(
                std::io::stdout(),
                "Skipping non-rust file `{}`",
                entry.file_name().to_string_lossy()
            )?;
            continue;
        }

        let test_name_path = entry.path().with_extension("");
        let test_name = test_name_path
            .file_name()
            .ok_or("failed to read filename")?
            .to_string_lossy();
        let Output {
            status,
            stdout,
            stderr,
        } = Command::new("cargo")
            .args([
                "--color",
                "never",
                "clippy",
                "--bin",
                &test_name,
                "--manifest-path",
                "tests/clippy/Cargo.toml",
            ])
            .output()?;
        if status.success() {
            Err(format!(
                "Clippy passed for '{}' when it shouldn't have. STDOUT: {}",
                test_name,
                String::from_utf8_lossy(&stdout),
            ))?;
        }

        // Actual expansion without the "Checking clippy-tests v0.0.0 ($path)" line
        let mut actual_expansion = String::from_utf8_lossy(&stderr)
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");
        actual_expansion.push('\n');

        if let Ok(expected_expansion) = std::fs::read_to_string(&expected_output_path) {
            if actual_expansion == expected_expansion {
                writeln!(std::io::stdout(), "expansion of {test_name} ... ok")?;
            } else {
                writeln!(std::io::stdout(), "expansion of {test_name} ... mismatched")?;
                std::fs::write(actual_output_path.clone(), actual_expansion.as_bytes())?;
                mismatched += 1;

                let Output {
                    status: _,
                    stdout,
                    stderr: _,
                } = Command::new("git")
                    .args([
                        "diff",
                        "--color",
                        "--no-index",
                        "--",
                        expected_output_path.to_str().unwrap(),
                        &actual_output_path.to_string_lossy(),
                    ])
                    .output()?;
                writeln!(std::io::stdout(), "\n{}", String::from_utf8_lossy(&stdout))?;
            }
        } else {
            std::fs::write(actual_output_path, actual_expansion.as_bytes())?;
            mismatched += 1;
            writeln!(
                std::io::stdout(),
                "expansion of {test_name} ... tests/clippy/expected/{test_name}.rs.stderr is \
                 missing"
            )?;
        }
    }

    if mismatched > 0 {
        Err("One or more expansions test results were mismatched or missing")?;
    }

    Ok(())
}
