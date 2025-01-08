use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};

#[test]
#[ignore]
fn expansion() -> Result<(), Box<dyn Error>> {
    let expected_destination = Path::new("tests/expansion/expected/");
    let actual_destination = Path::new("tests/expansion/actual/");
    clear_directory_except_gitignore(actual_destination)?;
    let mut mismatched = 0;
    let mut expected_paths = HashSet::new();
    for entry in fs::read_dir("tests")? {
        let entry = entry?;
        let expected_expansion_path = expected_destination.join(entry.file_name());
        let actual_expansion_path = actual_destination.join(entry.file_name());

        // Skip non-files (directories)
        if !entry.file_type()?.is_file() || entry.file_name().to_str() == Some("expansion.rs") {
            continue;
        }

        expected_paths.insert(expected_expansion_path.to_string_lossy().into_owned());

        let test_name_path = entry.path().with_extension("");
        let test_name = test_name_path
            .file_name()
            .ok_or("failed to read filename")?
            .to_str()
            .ok_or("failed to convert filename to a str")?;
        let Output {
            status,
            stdout,
            stderr,
        } = Command::new("cargo")
            .args(["expand", "--test", test_name])
            .output()?;
        if !status.success() {
            Err(String::from_utf8_lossy(&stderr))?;
        }

        let actual_expansion = String::from_utf8_lossy(&stdout);

        if let Ok(expected_expansion) = std::fs::read_to_string(&expected_expansion_path) {
            let same = actual_expansion.lines().eq(expected_expansion.lines());

            if same {
                writeln!(std::io::stdout(), "expansion of {test_name} ... ok")?;
            } else {
                writeln!(std::io::stdout(), "expansion of {test_name} ... mismatched")?;
                std::fs::write(actual_expansion_path.clone(), actual_expansion.as_bytes())?;
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
                        expected_expansion_path.to_str().unwrap(),
                        &actual_expansion_path.to_string_lossy(),
                    ])
                    .output()?;
                writeln!(std::io::stdout(), "\n{}", String::from_utf8_lossy(&stdout))?;
            }
        } else {
            std::fs::write(actual_expansion_path, actual_expansion.as_bytes())?;
            mismatched += 1;
            writeln!(
                std::io::stdout(),
                "expansion of {test_name} ... expected/{test_name}.rs is missing"
            )?;
        }
    }

    // Check if there are any leftover files from deleted test files.
    for entry in fs::read_dir(expected_destination)? {
        let entry = entry?;

        // Skip non-files (directories) and expected paths
        if !entry.file_type()?.is_file()
            || expected_paths.contains(&entry.path().to_string_lossy().to_string())
        {
            continue;
        }

        mismatched += 1;
        writeln!(
            std::io::stdout(),
            "found expected/{}, but no associated test exists or expansion is intentionally \
             ignored for it; delete it?",
            entry.file_name().to_string_lossy()
        )?;
    }

    if mismatched > 0 {
        Err("One or more expansions test results were mismatched or missing")?;
    }

    Ok(())
}

fn clear_directory_except_gitignore(actual_destination: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(actual_destination)? {
        let entry = entry?;

        // Skip non-files (directories) and the `.gitignore`
        if !entry.file_type()?.is_file() || entry.file_name() == ".gitignore" {
            continue;
        }

        fs::remove_file(entry.path())?;
    }

    Ok(())
}
