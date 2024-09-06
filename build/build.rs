mod rustc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::iter;
use std::path::Path;
use std::process::{self, Command};

fn main() {
    println!("cargo:rerun-if-changed=build/build.rs");

    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let rustc_wrapper = env::var_os("RUSTC_WRAPPER").filter(|wrapper| !wrapper.is_empty());
    let wrapped_rustc = rustc_wrapper.iter().chain(iter::once(&rustc));

    let mut is_clippy_driver = false;
    let version = loop {
        let mut wrapped_rustc = wrapped_rustc.clone();
        let mut command = Command::new(wrapped_rustc.next().unwrap());
        command.args(wrapped_rustc);
        if is_clippy_driver {
            command.arg("--rustc");
        }
        command.arg("--version");

        let output = match command.output() {
            Ok(output) => output,
            Err(e) => {
                let rustc = rustc.to_string_lossy();
                eprintln!("Error: failed to run `{} --version`: {}", rustc, e);
                process::exit(1);
            }
        };

        let string = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(e) => {
                let rustc = rustc.to_string_lossy();
                eprintln!(
                    "Error: failed to parse output of `{} --version`: {}",
                    rustc, e,
                );
                process::exit(1);
            }
        };

        break match rustc::parse(&string) {
            rustc::ParseResult::Success(version) => version,
            rustc::ParseResult::OopsClippy if !is_clippy_driver => {
                is_clippy_driver = true;
                continue;
            }
            rustc::ParseResult::Unrecognized | rustc::ParseResult::OopsClippy => {
                eprintln!(
                    "Error: unexpected output from `rustc --version`: {:?}\n\n\
                    Please file an issue in https://github.com/Techcable/rustversion-detect",
                    string
                );
                process::exit(1);
            }
        };
    };

    if version.major != 1 {
        eprintln!(
            "Error: Only major version 1.0 supported (got {:?})",
            version
        );
        process::exit(1);
    }

    // NOTE: sorted by version.minor

    if version.minor >= 32 {
        // Support for $x:literal
        println!("cargo:rustc-cfg=supports_macro_literal");
    }

    if version.minor >= 40 {
        println!("cargo:rustc-cfg=has_non_exhaustive")
    }

    if version.minor >= 46 {
        println!("cargo:rustc-cfg=has_const_match");
        println!("cargo:rustc-cfg=has_track_caller")
    }

    if version.minor >= 57 {
        println!("cargo:rustc-cfg=has_const_panic")
    }

    if version.minor >= 80 {
        println!("cargo:rustc-check-cfg=cfg(supports_macro_literal)");
        println!("cargo:rustc-check-cfg=cfg(has_non_exhaustive)");
        println!("cargo:rustc-check-cfg=cfg(has_const_match)");
        println!("cargo:rustc-check-cfg=cfg(has_track_caller)");
        println!("cargo:rustc-check-cfg=cfg(has_const_panic)");
        println!("cargo:rustc-check-cfg=cfg(host_os, values(\"windows\"))");
    }

    let version = format!("{:#?}\n", version);
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_file = Path::new(&out_dir).join("version.expr");
    fs::write(out_file, version).expect("failed to write version.expr");

    let host = env::var_os("HOST").expect("HOST not set");
    if let Some("windows") = host.to_str().unwrap().split('-').nth(2) {
        println!("cargo:rustc-cfg=host_os=\"windows\"");
    }
}
