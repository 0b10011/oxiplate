#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::io::Write;
use std::{error::Error, fs::self, path::Path, process::{Command, Output}};
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "expansion"]
#[doc(hidden)]
pub const expansion: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expansion"),
        ignore: true,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\expansion.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(expansion())),
};
#[ignore]
fn expansion() -> Result<(), Box<dyn Error>> {
    let expected_destination = Path::new("tests/expansion/expected/");
    let actual_destination = Path::new("tests/expansion/actual/");
    let mut mismatched = 0;
    for entry in fs::read_dir("tests")? {
        let entry = entry?;
        let expected_expansion_path = expected_destination.join(entry.file_name());
        let actual_expansion_path = actual_destination.join(entry.file_name());
        if !entry.file_type()?.is_file()
            || entry.path().to_str() == Some("oxiplate-derive\\tests\\expansion.rs")
        {
            continue;
        }
        let test_name_path = entry.path().with_extension("");
        let test_name = test_name_path
            .file_name()
            .ok_or("failed to read filename")?
            .to_str()
            .ok_or("failed to convert filename to a str")?;
        let Output { status, stdout, stderr } = Command::new("cargo")
            .args(["expand", "--test", test_name])
            .output()?;
        if !status.success() {
            Err(String::from_utf8_lossy(&stderr))?;
        }
        let actual_expansion = String::from_utf8_lossy(&stdout);
        if let Ok(expected_expansion)
            = std::fs::read_to_string(&expected_expansion_path) {
            let same = actual_expansion.lines().eq(expected_expansion.lines());
            if same {
                std::io::stdout()
                    .write_fmt(format_args!("expansion of {0} ... ok\n", test_name))?;
            } else {
                std::io::stdout()
                    .write_fmt(
                        format_args!("expansion of {0} ... mismatched\n", test_name),
                    )?;
                std::fs::write(
                    actual_expansion_path.clone(),
                    actual_expansion.as_bytes(),
                )?;
                mismatched += 1;
                let Output { status: _, stdout, stderr: _ } = Command::new("git")
                    .args([
                        "diff",
                        "--color",
                        "--no-index",
                        "--",
                        expected_expansion_path.to_str().unwrap(),
                        &actual_expansion_path.to_string_lossy(),
                    ])
                    .output()?;
                std::io::stdout()
                    .write_fmt(
                        format_args!("\n{0}\n", String::from_utf8_lossy(& stdout)),
                    )?;
            }
        } else {
            std::fs::write(actual_expansion_path, actual_expansion.as_bytes())?;
            mismatched += 1;
            std::io::stdout()
                .write_fmt(
                    format_args!(
                        "expansion of {0} ... expected/{0}.rs is missing\n", test_name
                    ),
                )?;
        }
    }
    if mismatched > 0 {
        Err("One or more expansions test results were mismatched or missing")?;
    }
    Ok(())
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&expansion])
}
