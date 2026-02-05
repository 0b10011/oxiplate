use std::error::Error;
use std::path::Path;
use std::{env, fs};

use colored::Colorize;

#[test]
fn stderr_matches() -> Result<(), Box<dyn Error>> {
    let expected_destination = Path::new("../../config/expected/");
    let crates_destination = Path::new("../../config/crates/");

    assert!(
        expected_destination.is_dir(),
        "Expected destination {expected_destination:?} is not a directory. Current directory: {:?}",
        env::current_dir()
    );
    assert!(
        crates_destination.is_dir(),
        "Crates destination {crates_destination:?} is not a directory. Current directory: {:?}",
        env::current_dir()
    );

    let mut mismatched = 0;
    for entry in fs::read_dir(expected_destination)? {
        let entry = entry?;

        // Skip non-files (directories) and non-stderr files
        if !entry.file_type()?.is_file()
            || !entry
                .file_name()
                .to_str()
                .unwrap_or("")
                .ends_with(".stderr")
        {
            continue;
        }

        let base_name = entry.path().with_extension("");
        let base_name = base_name
            .file_name()
            .ok_or("failed to read filename")?
            .to_str()
            .ok_or("failed to convert filename to a str")?;
        let package_dir = crates_destination.join(base_name);

        let original = fs::read_to_string(entry.path()).ok();
        let generated = fs::read_to_string(
            package_dir
                .join("tests")
                .join("broken")
                .join(format!("{base_name}.stderr")),
        )
        .ok();

        print!("{base_name}.stderr ... ");
        if original == generated {
            println!("{}", "ok".green());
        } else if generated.is_none() {
            println!("{}", "missing".yellow());
            mismatched += 1;
        } else {
            println!("{}", "mismatched".red());
            mismatched += 1;
        }
    }

    if mismatched > 0 {
        panic!(
            "1 or more mismatched stderr files. Try running `just build-config-test-crates` to \
             fix."
        );
    }

    Ok(())
}

#[test]
fn config_matches() -> Result<(), Box<dyn Error>> {
    let expected_destination = Path::new("../../config/expected/");
    let crates_destination = Path::new("../../config/crates/");

    assert!(
        expected_destination.is_dir(),
        "Expected destination {expected_destination:?} is not a directory. Current directory: {:?}",
        env::current_dir()
    );
    assert!(
        crates_destination.is_dir(),
        "Crates destination {crates_destination:?} is not a directory. Current directory: {:?}",
        env::current_dir()
    );

    let mut mismatched = 0;
    for entry in fs::read_dir(expected_destination)? {
        let entry = entry?;

        // Skip non-files (directories) and non-toml files
        if !entry.file_type()?.is_file()
            || !entry.file_name().to_str().unwrap_or("").ends_with(".toml")
        {
            continue;
        }

        let base_name = entry.path().with_extension("");
        let base_name = base_name
            .file_name()
            .ok_or("failed to read filename")?
            .to_str()
            .ok_or("failed to convert filename to a str")?;
        let package_dir = crates_destination.join(base_name);

        let original = fs::read_to_string(entry.path()).ok();
        let generated = fs::read_to_string(package_dir.join("oxiplate.toml")).ok();

        print!("{base_name}.toml ... ");
        if generated.is_none() {
            println!("{}", "missing".yellow());
            mismatched += 1;
        } else if original == generated {
            println!("{}", "ok".green());
        } else {
            println!("{}", "mismatched".red());
            mismatched += 1;
        }
    }

    if mismatched > 0 {
        panic!(
            "1 or more mismatched oxiplate.toml files. Try running `just \
             build-config-test-crates` to fix."
        );
    }

    Ok(())
}
