use std::error::Error;
use std::fs;
use std::path::Path;

pub fn main() -> Result<(), Box<dyn Error>> {
    let expected_destination = Path::new("oxiplate-derive/tests/config/expected/");
    let template_dir = Path::new("oxiplate-derive/tests/config/crates/.template/");
    let crates_destination = Path::new("oxiplate-derive/tests/config/crates/");

    assert!(
        expected_destination.is_dir(),
        "Expected destination {expected_destination:?} is not a directory"
    );
    assert!(
        template_dir.is_dir(),
        "Template location {template_dir:?} is not a directory"
    );
    assert!(
        crates_destination.is_dir(),
        "Crates destination {crates_destination:?} is not a directory"
    );

    clear_directory_except_readme_and_template(crates_destination)?;

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
        let toml = expected_destination.join(format!("{base_name}.toml"));
        let stderr = expected_destination.join(format!("{base_name}.stderr"));
        let package_dir = crates_destination.join(base_name);

        copy_directory(template_dir, &package_dir)
            .expect("Copying template dir to package dir should succeed");

        // Replace `oxiplate.toml` with entry's version
        fs::remove_file(package_dir.join("oxiplate.toml"))?;
        fs::copy(toml, package_dir.join("oxiplate.toml"))?;

        // Replace `oxiplate.toml` with entry's version
        fs::rename(
            package_dir.join("tests/broken/config.rs"),
            package_dir.join(format!("tests/broken/{base_name}.rs")),
        )?;

        // Replace `config.stderr` with entry's version
        fs::remove_file(package_dir.join("tests/broken/config.stderr"))?;
        if stderr.exists() {
            fs::copy(
                stderr,
                package_dir.join(format!("tests/broken/{base_name}.stderr")),
            )?;
        }

        // Update package name
        let package_config = fs::read_to_string(package_dir.join("Cargo.toml"))?.replace(
            "oxiplate-derive-test-config-template",
            &format!("oxiplate-derive-test-config-{base_name}"),
        );
        fs::write(package_dir.join("Cargo.toml"), package_config)?;
    }

    println!("Config crates updated! Run tests to make sure output is still valid.");

    Ok(())
}

fn clear_directory_except_readme_and_template(destination: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(destination)? {
        let entry = entry?;

        // Skip `README.md` and `.template` directory
        if matches!(entry.file_name().to_str(), Some("README.md" | ".template")) {
            continue;
        }

        if entry.file_type()?.is_file() {
            fs::remove_file(entry.path())?;
        } else {
            fs::remove_dir_all(entry.path())?;
        }
    }

    Ok(())
}

fn copy_directory(from: &Path, to: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir(to).expect("Creating directory should succeed");
    for entry in fs::read_dir(from).expect("Failed to read `from` dir") {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_directory(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.join(entry.file_name()))
                .expect("Copying file should succeed");
        }
    }
    Ok(())
}
