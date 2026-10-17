use std::error::Error;

#[test]
#[ignore = "Expansion tests are prone to breaking due to formatting issues and warnings, so they \
            should be run separately."]
fn expansion() -> Result<(), Box<dyn Error>> {
    // Call the conditionally compiled expansion test.
    // Modules are used to avoid dead code warnings.
    test::expansion()
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use std::collections::{HashMap, HashSet};
    use std::fs::DirEntry;
    use std::io::Write;
    use std::path::Path;
    use std::process::{Command, Output, Stdio};
    use std::{env, fs};

    pub(super) fn expansion() -> Result<(), Box<dyn super::Error>> {
        let should_overwrite = env::var("OXIP_EXPANSION").ok() == Some("overwrite".to_string());
        let expected_destination = Path::new("tests/expansion/expected/");
        let actual_destination = Path::new("tests/expansion/actual/");
        clear_directory_except_gitignore(actual_destination)?;
        let mut mismatched = 0;
        let mut expected_paths = HashSet::new();
        let test_directory = Path::new("tests/integration/");

        let mut mods = HashMap::new();
        extract_mods(&mut mods)?;

        check_test_expansions(
            &mut mods,
            test_directory,
            String::from(""),
            expected_destination,
            should_overwrite,
            actual_destination,
            &mut expected_paths,
            &mut mismatched,
            false,
        )?;

        // Check if there are any leftover files from deleted test files.
        check_for_leftover_expected_expansions(
            expected_destination,
            should_overwrite,
            &mut expected_paths,
            &mut mismatched,
        )?;

        for (name, _code) in mods {
            mismatched += 1;
            writeln!(std::io::stdout(), "{name} ... unused",)?;
        }

        if mismatched > 0 && !should_overwrite {
            Err("One or more expansions test results were mismatched or missing")?;
        }

        Ok(())
    }

    fn clear_directory_except_gitignore(
        actual_destination: &Path,
    ) -> Result<(), Box<dyn super::Error>> {
        for entry in fs::read_dir(actual_destination)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            // Delete directories and their contents.
            if file_type.is_dir() {
                clear_directory_except_gitignore(&entry.path())?;
                fs::remove_dir(entry.path())?;
                continue;
            }

            // Skip `.gitignore`.
            if entry.file_name() == ".gitignore" {
                continue;
            }

            fs::remove_file(entry.path())?;
        }

        Ok(())
    }

    fn extract_mods(mods: &mut HashMap<String, String>) -> Result<(), Box<dyn super::Error>> {
        let Output {
            status,
            stdout,
            stderr,
        } = Command::new("cargo")
            .args(["--color", "never", "expand", "--test", "integration"])
            .output()?;
        if !status.success() {
            Err(String::from_utf8_lossy(&stderr))?;
        }

        let expansion = String::from_utf8_lossy(&stdout);

        extract_mods_from_module("", &expansion, 0, mods)?;

        Ok(())
    }

    fn extract_mods_from_module<'a>(
        path: &str,
        code: &'a str,
        indentation_level: usize,
        mods: &mut HashMap<String, String>,
    ) -> Result<(), Box<dyn super::Error>> {
        let mod_pattern = format!("\n{}mod ", "    ".repeat(indentation_level));
        let mut mods_iterator = code.split(&mod_pattern).into_iter();

        // Top-level module has test-related boilerplate in it
        // and the rest start with `{`
        // that should be ignored.
        mods_iterator.next();

        // Build patterns before loop to avoid unnecessary work.
        let inner_mod_pattern = format!("\n{}mod ", "    ".repeat(indentation_level + 1));

        for code in &mut mods_iterator {
            // Capture the name by finding the next space.
            // `mod ` has already been removed by split above,
            // leaving just `module_name + ' ' + module_block`.
            let space_pos = code.find(' ').expect("` ` should always be present");
            let (name, code) = code.split_at(space_pos);

            // Skip to just after the opening `{`
            let open_bracket = code.find('{').expect("`{` should always be present");
            let code = code.split_at(open_bracket + 1).1;

            let path = if path.is_empty() {
                name.to_owned()
            } else {
                format!("{path}::{name}")
            };

            // If this module contains other modules,
            // extract all of them instead of this module.
            if code.starts_with(&inner_mod_pattern) {
                extract_mods_from_module(&path, code, indentation_level + 1, mods)?;
                continue;
            }

            let code = normalize(code, indentation_level)?;

            mods.insert(path, code);
        }

        Ok(())
    }

    fn normalize(
        mut code: &str,
        indentation_level: usize,
    ) -> Result<String, Box<dyn super::Error>> {
        let mut is_leading_newline = true;

        let mut normalized_code = String::new();
        let newline_with_indentation = format!("\n{}", "    ".repeat(indentation_level + 1));
        loop {
            let newline_pos = code.find('\n');
            let quote_pos = code.find('"');

            // Discard indentation after newline for current indentation level.
            if newline_pos < quote_pos || quote_pos.is_none() {
                if let Some(newline_pos) = newline_pos {
                    let (normalized, new_code) = code.split_at(newline_pos);
                    normalized_code += normalized;
                    code = new_code;

                    // `mod [...] {` and `}` should be at the same indentation level.
                    // Break out of the loop if end is encountered.
                    let mod_end_pattern = format!("\n{}}}", "    ".repeat(indentation_level));
                    if code.starts_with(&mod_end_pattern) {
                        normalized_code += "\n";
                        break;
                    }

                    assert_eq!(
                        code.find(&newline_with_indentation),
                        Some(0),
                        "Newline and indentation expected to match {:?}, found:\n\n{:?}",
                        newline_with_indentation,
                        code
                    );
                    code = code.split_at(newline_with_indentation.len()).1;

                    if is_leading_newline {
                        is_leading_newline = false;
                    } else {
                        normalized_code += "\n";
                    }

                    continue;
                }
            }

            is_leading_newline = false;

            // Add raw strings as-is.
            let r_pos = code.find('r');
            if r_pos < quote_pos {
                if let Some(r_pos) = r_pos {
                    if let Some(quote_pos) = quote_pos {
                        let hashes = "#".repeat(quote_pos - r_pos - 1);
                        let raw_string_start = format!("r{}\"", hashes);
                        if let Some(raw_string_start_pos) = code.find(&raw_string_start) {
                            if raw_string_start_pos == r_pos {
                                // Jump into the string to avoid matching `r#"#`.
                                let (normalized, new_code) =
                                    code.split_at(raw_string_start_pos + raw_string_start.len());
                                normalized_code += normalized;
                                code = new_code;

                                let raw_string_end = format!("\"{}", hashes);
                                let Some(end) = code.find(&raw_string_end) else {
                                    panic!(
                                        "Raw string should always be closed. Expected `{}` \
                                         in:\n\n{}",
                                        raw_string_end, code
                                    );
                                };
                                let (normalized, new_code) =
                                    code.split_at(end + raw_string_end.len());
                                normalized_code += normalized;
                                code = new_code;
                                continue;
                            }
                        }
                    }
                }
            }

            // Add strings as-is.
            if let Some(quote_pos) = quote_pos {
                let (normalized, new_code) = code.split_at(quote_pos + 1);
                normalized_code += normalized;
                code = new_code;

                loop {
                    let escaped_quote_pos = code.find("\\\"");
                    let quote_pos = code.find('"');

                    let Some(quote_pos) = quote_pos else {
                        panic!("Quotes should always be closed");
                    };

                    // Ignore escaped quotes.
                    if let Some(escaped_quote_pos) = escaped_quote_pos {
                        if escaped_quote_pos + 1 == quote_pos {
                            let (normalized, new_code) = code.split_at(quote_pos + 1);
                            normalized_code += normalized;
                            code = new_code;
                            continue;
                        }
                    }

                    let (normalized, new_code) = code.split_at(quote_pos + 1);
                    normalized_code += normalized;
                    code = new_code;
                    break;
                }

                continue;
            }

            // Add remaining code and break out of the loop.
            normalized_code += code;
            break;
        }

        // Try to use formatted code from `rustfmt`.
        let mut rustfmt = Command::new("rustfmt")
            .args([
                "--color",
                "never",
                "--edition",
                "2024",
                "--style-edition",
                "2024",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        rustfmt
            .stdin
            .as_mut()
            .expect("Failed to get mutable STDIN")
            .write_all(normalized_code.as_bytes())?;
        let output = rustfmt.wait_with_output()?;
        if output.status.success() {
            normalized_code = String::from_utf8(output.stdout)?;
        }

        Ok(normalized_code)
    }

    fn check_test_expansions(
        mods: &mut HashMap<String, String>,
        test_directory: &Path,
        mod_prefix: String,
        expected_destination: &Path,
        should_overwrite: bool,
        actual_destination: &Path,
        expected_paths: &mut HashSet<String>,
        mismatched: &mut usize,
        mut create_actual_destination_dir: bool,
    ) -> Result<(), Box<dyn super::Error>> {
        writeln!(
            std::io::stdout(),
            "checking {}",
            test_directory.to_string_lossy()
        )?;

        for entry in entries(test_directory)? {
            let expected_expansion_path = expected_destination.join(entry.file_name());
            let actual_expansion_path = if should_overwrite {
                expected_destination
            } else {
                actual_destination
            }
            .join(entry.file_name());

            // Check directories and skip symlinks.
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                // Use raw idents for keywords.
                let mut mod_name = entry.file_name().to_string_lossy().into_owned();
                if is_keyword(&mod_name) {
                    mod_name = format!("r#{mod_name}");
                }

                let mod_prefix = if mod_prefix.is_empty() {
                    mod_name
                } else {
                    format!("{mod_prefix}::{mod_name}")
                };

                check_test_expansions(
                    mods,
                    &test_directory.join(format!("{}/", entry.file_name().to_string_lossy())),
                    mod_prefix,
                    &expected_destination.join(format!("{}/", entry.file_name().to_string_lossy())),
                    should_overwrite,
                    &actual_destination.join(format!("{}/", entry.file_name().to_string_lossy())),
                    expected_paths,
                    mismatched,
                    true,
                )?;

                continue;
            } else if file_type.is_symlink() {
                // Ignore symlinks
                continue;
            }

            // Ignore non-rust files
            if !entry.file_name().to_string_lossy().ends_with(".rs") {
                writeln!(
                    std::io::stdout(),
                    "skipping non-rust file `{}`",
                    entry.file_name().to_string_lossy()
                )?;
                continue;
            }

            expected_paths.insert(expected_expansion_path.to_string_lossy().into_owned());

            let test_name_path = entry.path().with_extension("");
            let mut test_name = test_name_path
                .file_name()
                .ok_or("failed to read filename")?
                .to_str()
                .ok_or("failed to convert filename to a str")?
                .to_owned();

            // Ignore `mod.rs` and `main.rs`.
            if matches!(test_name.as_str(), "mod" | "main") {
                continue;
            }

            // Use raw idents for keywords.
            if is_keyword(&test_name) {
                test_name = format!("r#{test_name}");
            }

            let mod_name = if mod_prefix.is_empty() {
                test_name
            } else {
                format!("{mod_prefix}::{test_name}")
            };

            let Some(actual_expansion) = mods.remove(&mod_name) else {
                writeln!(
                    std::io::stdout(),
                    "expansion of {mod_name} ... not generated",
                )?;
                *mismatched += 1;
                continue;
            };

            if let Ok(expected_expansion) = std::fs::read_to_string(&expected_expansion_path) {
                let same = actual_expansion.lines().eq(expected_expansion.lines());

                if same {
                    writeln!(std::io::stdout(), "expansion of {mod_name} ... ok")?;
                } else {
                    writeln!(
                        std::io::stdout(),
                        "expansion of {mod_name} ... {}",
                        if should_overwrite {
                            "updated"
                        } else {
                            "mismatched"
                        }
                    )?;
                    std::fs::write(actual_expansion_path.clone(), actual_expansion.as_bytes())?;
                    *mismatched += 1;

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
                if create_actual_destination_dir {
                    // The parent directory will not have been created
                    // if it doesn't contain any mismatches
                    // or they haven't been found yet.
                    std::fs::create_dir_all(
                        actual_expansion_path
                            .parent()
                            .expect("Parent of actual expansion path should exist"),
                    )?;
                    create_actual_destination_dir = false;
                }

                std::fs::write(&actual_expansion_path, actual_expansion.as_bytes())?;
                *mismatched += 1;
                writeln!(
                    std::io::stdout(),
                    "expansion of {mod_name} ... {}",
                    if should_overwrite {
                        "created"
                    } else {
                        "missing"
                    }
                )?;
            }
        }

        Ok(())
    }

    fn entries(path: &Path) -> Result<Vec<DirEntry>, Box<dyn super::Error>> {
        let mut entries: Vec<_> = Vec::new();

        for entry in fs::read_dir(path)? {
            entries.push(entry?);
        }

        // Sort so files appear first
        // and each group is in alphabetical order.
        entries.sort_by(|a, b| {
            let a_is_file = if let Ok(file_type) = a.file_type() {
                file_type.is_file()
            } else {
                false
            };
            let b_is_file = if let Ok(file_type) = b.file_type() {
                file_type.is_file()
            } else {
                false
            };
            if a_is_file && !b_is_file {
                return Ordering::Less;
            } else if !a_is_file && b_is_file {
                return Ordering::Greater;
            }

            a.file_name().cmp(&b.file_name())
        });

        Ok(entries)
    }

    fn is_keyword(mod_name: &str) -> bool {
        match mod_name {
            // Keywords from <https://doc.rust-lang.org/reference/keywords.html>.
            // Prefix with `r#` so Rust will accept them as idents.
            "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const"
            | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false"
            | "final" | "fn" | "for" | "gen" | "if" | "impl" | "in" | "let" | "loop" | "macro"
            | "macro_rules" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub"
            | "ref" | "return" | "static" | "struct" | "trait" | "true" | "try" | "type"
            | "typeof" | "union" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
            | "yield" => true,
            _ => false,
        }
    }

    fn check_for_leftover_expected_expansions(
        expected_destination: &Path,
        should_overwrite: bool,
        expected_paths: &mut HashSet<String>,
        mismatched: &mut usize,
    ) -> Result<(), Box<dyn super::Error>> {
        for entry in entries(expected_destination)? {
            // Check directories and skip symlinks.
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                check_for_leftover_expected_expansions(
                    &expected_destination.join(format!("{}/", entry.file_name().to_string_lossy())),
                    should_overwrite,
                    expected_paths,
                    mismatched,
                )?;

                continue;
            } else if file_type.is_symlink() {
                // Ignore symlinks
                continue;
            }

            // Skip expected paths.
            if expected_paths.contains(&entry.path().to_string_lossy().to_string()) {
                continue;
            }

            if should_overwrite {
                fs::remove_file(entry.path())?;
            }

            *mismatched += 1;
            writeln!(
                std::io::stdout(),
                "{} ... {}",
                entry.path().to_string_lossy(),
                if should_overwrite {
                    "deleted"
                } else {
                    "unexpected"
                }
            )?;
        }

        Ok(())
    }
}
