//! CLI tests for mmlabc-to-smf binary

use std::fs;
use std::process::Command;
use tempfile::tempdir;

/// Test that the CLI runs successfully with --no-play flag
#[test]
fn test_cli_no_play() {
    let dir = tempdir().expect("Failed to create temp dir");
    let mid_path = dir.path().join("output.mid");

    let output = Command::new(env!("CARGO_BIN_EXE_mmlabc-to-smf"))
        .args(&["cde", "--no-play", "-o", mid_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(mid_path.exists());

    // Check that output doesn't contain playback-related messages
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("Attempting to play"));
}

/// Test help output includes --no-play option
#[test]
fn test_cli_help_includes_no_play() {
    let output = Command::new(env!("CARGO_BIN_EXE_mmlabc-to-smf"))
        .args(&["--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--no-play"));
    assert!(stdout.contains("cat-play-mml"));
}

/// Test that --attachment-output generates a JSON file
#[test]
fn test_cli_attachment_output() {
    let dir = tempdir().expect("Failed to create temp dir");
    let mid_path = dir.path().join("output.mid");
    let json_path = dir.path().join("attachment.json");

    let output = Command::new(env!("CARGO_BIN_EXE_mmlabc-to-smf"))
        .args(&[
            "@1cde",
            "--no-play",
            "-o",
            mid_path.to_str().unwrap(),
            "--attachment-output",
            json_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(mid_path.exists());
    assert!(json_path.exists());

    // Verify the attachment JSON is valid and contains the expected structure
    let json_content = fs::read_to_string(&json_path).expect("Failed to read JSON file");
    let parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Attachment JSON is not valid JSON");
    assert!(parsed.is_array());
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["ProgramChange"], 1);
    assert!(arr[0]["Tone"]["events"].is_array());

    // Check stdout mentions the attachment JSON
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("attachment JSON"));
}

/// Test that omitting --attachment-output does not create a JSON file
#[test]
fn test_cli_no_attachment_output_by_default() {
    let dir = tempdir().expect("Failed to create temp dir");
    let mid_path = dir.path().join("output.mid");
    let json_path = dir.path().join("attachment.json");

    let output = Command::new(env!("CARGO_BIN_EXE_mmlabc-to-smf"))
        .args(&["@1cde", "--no-play", "-o", mid_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    // No attachment JSON should be created
    assert!(!json_path.exists());
}
