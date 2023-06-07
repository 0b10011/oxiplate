use std::io::Write;
use std::{
    error::Error,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    process::{Command, Output},
};

#[test]
#[ignore]
fn expansion() -> Result<(), Box<dyn Error>> {
    let destination = Path::new("tests/expansion/");
    let mut mismatched = 0;
    for entry in fs::read_dir("tests")? {
        let entry = entry?;
        let expansion_path = destination.join(entry.file_name());
        if !entry.file_type()?.is_file() || entry.path().to_str() == Some(file!()) {
            continue;
        }
        if !file_changed(&expansion_path, &entry)? {
            continue;
        }
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

        if let Ok(expected_expansion) = std::fs::read_to_string(&expansion_path) {
            let same = actual_expansion.lines().eq(expected_expansion.lines());

            if !same {
                writeln!(
                    std::io::stderr(),
                    "expansion of {} ... mismatched",
                    test_name
                )?;
                mismatched += 1;
            } else {
                writeln!(std::io::stdout(), "expansion of {} ... ok", test_name)?;
            }
        } else {
            std::fs::write(expansion_path, actual_expansion.as_bytes())?;
            writeln!(std::io::stdout(), "expansion of {} ... written", test_name)?;
        }
    }

    if mismatched > 0 {
        Err("mismatched")?;
    }

    Ok(())
}

fn file_changed(expansion_path: &PathBuf, entry: &DirEntry) -> Result<bool, Box<dyn Error>> {
    let Ok(expansion_file) = fs::File::open(expansion_path) else { return Ok(true) };
    let expansion_metadata = expansion_file.metadata()?;
    let entry_metadata = entry.metadata()?;
    if expansion_metadata.modified()? < entry_metadata.modified()? {
        return Ok(true);
    }
    if expansion_metadata.len() != entry_metadata.len() {
        return Ok(true);
    }

    Ok(false)
}
