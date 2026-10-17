use std::collections::HashSet;
use std::io::{self, BufRead as _, Write as _};
use std::path::Path;
#[cfg(test)]
use std::{fs, path::PathBuf};

#[test]
fn ensure_all_modules_are_included() {
    let integration_tests_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("integration");
    let file = integration_tests_dir.join("main.rs");

    assert!(
        integration_tests_dir.exists(),
        "{integration_tests_dir:?} should exist"
    );
    assert!(file.exists(), "{file:?} should exist");

    let mut missing_mods = Vec::new();

    check_dir_for_missing_mods(integration_tests_dir, Some(file), &mut missing_mods)
        .expect("Checking directory for missing modules failed");

    if !missing_mods.is_empty() {
        let mut message = String::new();
        for (path, mut missing_mods) in missing_mods {
            missing_mods.sort_unstable_by(|a, b| {
                a.strip_prefix("r#")
                    .unwrap_or(a)
                    .cmp(b.strip_prefix("r#").unwrap_or(b))
            });

            message.push_str(&format!(
                "{path:?}\nmod {};\n\n",
                missing_mods.join(";\nmod ")
            ));
        }
        panic!("Some test modules are not properly included:\n\n{message}")
    }
}

#[cfg(test)]
fn check_dir_for_missing_mods(
    dir: PathBuf,
    mod_root: Option<PathBuf>,
    missing_mods: &mut Vec<(PathBuf, Vec<String>)>,
) -> io::Result<()> {
    let mod_root = if let Some(mod_root) = mod_root {
        mod_root
    } else {
        dir.join("mod.rs")
    };

    assert!(
        dir.exists(),
        "{:?} directory expected in {:?}",
        dir.file_name(),
        dir.parent(),
    );
    assert!(
        mod_root.exists(),
        "{:?} not found in {:?}",
        mod_root.file_name(),
        mod_root.parent()
    );

    let expected_mods = parse_expected_mods(&mod_root)?;

    let mut missing_mods_in_file: Vec<String> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;

        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            panic!("Invalid file name `{:?}`", entry.file_name())
        };

        // Handle directories and symlinks.
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let mod_name = file_name;
            if !expected_mods.contains(mod_name) {
                let mod_name = if is_keyword(mod_name) {
                    format!("r#{mod_name}")
                } else {
                    mod_name.to_owned()
                };

                missing_mods_in_file.push(mod_name);
            }

            check_dir_for_missing_mods(entry.path(), None, missing_mods)?;
            continue;
        } else if file_type.is_symlink() {
            panic!(
                "Symlinks are not expected in the `integrations` directory. Found one at: {:?}",
                entry.path()
            );
        }

        // `mod.rs` is handled before the loop.
        if matches!(file_name, "mod.rs" | "main.rs") {
            continue;
        }

        // Ignore non-rust files
        if !file_name.ends_with(".rs") {
            writeln!(io::stdout(), "skipping non-rust file `{file_name}`",)?;
            continue;
        }

        let mod_name = file_name.trim_end_matches(".rs");
        if !expected_mods.contains(mod_name) {
            let mod_name = if is_keyword(mod_name) {
                format!("r#{mod_name}")
            } else {
                mod_name.to_owned()
            };

            missing_mods_in_file.push(mod_name);
        }
    }

    if !missing_mods_in_file.is_empty() {
        missing_mods.push((mod_root, missing_mods_in_file));
    }

    Ok(())
}

fn is_keyword(mod_name: &str) -> bool {
    match mod_name {
        // Keywords from <https://doc.rust-lang.org/reference/keywords.html>.
        // Prefix with `r#` so Rust will accept them as idents.
        "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const"
        | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false" | "final"
        | "fn" | "for" | "gen" | "if" | "impl" | "in" | "let" | "loop" | "macro"
        | "macro_rules" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub"
        | "ref" | "return" | "static" | "struct" | "trait" | "true" | "try" | "type" | "typeof"
        | "union" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" => true,
        _ => false,
    }
}

fn parse_expected_mods(mod_root: &PathBuf) -> io::Result<HashSet<String>> {
    let mut expected_mods = HashSet::new();
    let mut reader = io::BufReader::new(fs::File::open(mod_root)?);
    let mut line_buffer = String::new();

    while reader.read_line(&mut line_buffer)? > 0 {
        if !line_buffer.starts_with("mod ") || !line_buffer.ends_with(";\n") {
            line_buffer.clear();
            continue;
        }

        let mod_name = line_buffer[4..line_buffer.len() - 2]
            .trim_start_matches("r#")
            .to_owned();
        line_buffer.clear();

        expected_mods.insert(mod_name);
    }

    Ok(expected_mods)
}
