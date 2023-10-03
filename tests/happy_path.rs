use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command; // Run programs

#[test]
fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minigrep")?;

    cmd.arg("./etc/poem.txt").arg("nobody");
    cmd.assert().success();

    Ok(())
}
