use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()?;
    let hash = String::from_utf8(out.stdout)?;

    println!("cargo:rustc-env=HASH={}", hash);
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
