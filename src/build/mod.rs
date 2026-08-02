mod rustc;

use crate::version::RustVersion;
use crate::VersionDetectionError;
use std::env;
use std::ffi::OsString;
use std::iter;
use std::process::Command;

pub fn determine_version() -> Result<RustVersion, VersionDetectionError> {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let rustc_wrapper = env::var_os("RUSTC_WRAPPER").filter(|wrapper| !wrapper.is_empty());
    let wrapped_rustc = rustc_wrapper.iter().chain(iter::once(&rustc));

    let mut is_clippy_driver = false;
    let mut is_mirai = false;
    loop {
        let mut command;
        if is_mirai {
            command = Command::new(&rustc);
        } else {
            let mut wrapped_rustc = wrapped_rustc.clone();
            command = Command::new(wrapped_rustc.next().unwrap());
            command.args(wrapped_rustc);
        }
        if is_clippy_driver {
            command.arg("--rustc");
        }
        command.arg("--version");

        // Allow wrapper scripts or alternate compilers to tell that this is
        // `rustversion` running --version, so that they can stick to rustc's
        // version format. https://github.com/dtolnay/rustversion/issues/67
        command.env("RUSTVERSION", "1");

        let output = match command.output() {
            Ok(output) => output,
            Err(e) => {
                let rustc = rustc.to_string_lossy();
                return Err(VersionDetectionError::with_cause(
                    format!("Error: failed to run `{} --version`", rustc),
                    e,
                ));
            }
        };

        let string = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => {
                let rustc = rustc.to_string_lossy();
                return Err(crate::VersionDetectionError::new(format!(
                    "Error: Invalid UTF8 in output of `{} --version`",
                    rustc
                )));
            }
        };

        return match rustc::parse(&string) {
            rustc::ParseResult::Success(version) => Ok(version),
            rustc::ParseResult::OopsClippy if !is_clippy_driver => {
                is_clippy_driver = true;
                continue;
            },
            rustc::ParseResult::OopsMirai if !is_mirai && rustc_wrapper.is_some() => {
                is_mirai = true;
                continue;
            },
            rustc::ParseResult::Unrecognized(desc)
            | rustc::ParseResult::OopsClippy
            | rustc::ParseResult::OopsMirai => {
                return Err(crate::VersionDetectionError::new(format!(
                    "Error: unexpected output from `rustc --version`: {:?}\n\n\
                    Please file an issue in https://github.com/Techcable/rustversion-detect",
                    string
                )));
            }
        };
    }
}
