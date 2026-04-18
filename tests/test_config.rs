//! Tests for TOML configuration functionality.

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn write_toml_config(dir: &Path, player: &str) {
    fs::write(
        dir.join("mmlabc-to-smf-rust.toml"),
        format!("external_smf_player = \"{player}\"\n"),
    )
    .unwrap();
}

/// Test that --no-play suppresses external player launch even when TOML config is present.
#[test]
fn test_cli_no_play_with_toml_config() {
    let dir = tempdir().expect("Failed to create temp dir");
    let mid_path = dir.path().join("output.mid");
    write_toml_config(dir.path(), "definitely-missing-player-command");

    let output = Command::new(env!("CARGO_BIN_EXE_mmlabc-to-smf"))
        .args(&["cde", "-o", mid_path.to_str().unwrap(), "--no-play"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(mid_path.exists());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(!combined.contains("Attempting to play"));
    assert!(!combined.contains("Failed to start"));
    assert!(!combined.contains("definitely-missing-player-command"));
}

/// Test that help text mentions TOML configuration
#[test]
fn test_cli_help_mentions_toml() {
    let output = Command::new(env!("CARGO_BIN_EXE_mmlabc-to-smf"))
        .args(&["--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("mmlabc-to-smf-rust.toml"),
        "Help text should mention TOML config file"
    );
}
