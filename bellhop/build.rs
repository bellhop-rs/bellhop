use std::env::var_os;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

static REDOC_INPUT: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/openapi/bellhop.yaml");

enum GenResult {
    Missing,
    Failure,
    Success,
}

/// Try to run `redoc-cli` to generate the API documentation from the swagger
/// spec.
fn generate_redoc() -> GenResult {
    let redoc_cli: PathBuf = var_os("REDOC_CLI")
        .unwrap_or(OsString::from("redoc-cli"))
        .into();

    let mut redoc_output: PathBuf = var_os("OUT_DIR").unwrap().into();
    redoc_output.push("redoc-static.html");

    let output = Command::new(redoc_cli)
        .arg("bundle")
        .arg(REDOC_INPUT)
        .arg("--output")
        .arg(redoc_output)
        .status();

    println!("cargo:rerun-if-changed={}", REDOC_INPUT);
    println!("cargo:rerun-if-env-changed=REDOC_CLI");

    let status = match output {
        Ok(o) => o,
        Err(_) => {
            println!(
                "cargo:warning=Unable to execute redoc-cli. API documentation won't be generated."
            );
            println!(
                "cargo:warning=Use npm to install redoc-cli or set the env. variable REDOC_CLI."
            );
            return GenResult::Missing;
        }
    };

    if status.success() {
        GenResult::Success
    } else {
        println!("cargo:warning=redoc-cli returned a non-zero exit status");
        GenResult::Failure
    }
}

static MISSING: &'static str = r#"pub static HTML: &'static str = "API documentation missing";"#;
static INCLUDE: &'static str =
    r#"pub static HTML: &'static str = include_str!("redoc-static.html");"#;

fn generate_stub(txt: &str) {
    let mut output: PathBuf = var_os("OUT_DIR").unwrap().into();
    output.push("redoc_static.rs");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .expect("unable to open stub file for API docs");

    file.write_all(txt.as_bytes()).unwrap();
}

fn main() {
    match generate_redoc() {
        GenResult::Missing => generate_stub(MISSING),
        GenResult::Success => generate_stub(INCLUDE),
        GenResult::Failure => std::process::exit(1),
    }
}
