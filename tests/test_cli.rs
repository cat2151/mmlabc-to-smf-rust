//! CLI tests for mmlabc-to-smf binary

use std::fs;
use std::path::Path;
use std::process::Command;

/// Test that the CLI runs successfully with --no-play flag
#[test]
fn test_cli_no_play() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "cde",
            "--no-play",
            "-o",
            "/tmp/test_cli_output.mid",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(Path::new("/tmp/test_cli_output.mid").exists());

    // Check that output doesn't contain playback-related messages
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("Attempting to play"));

    // Cleanup
    let _ = fs::remove_file("/tmp/test_cli_output.mid");
}

/// Test that the CLI attempts to play by default
#[test]
fn test_cli_auto_play_attempt() {
    let output = Command::new("cargo")
        .args(&["run", "--", "cde", "-o", "/tmp/test_cli_autoplay.mid"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(Path::new("/tmp/test_cli_autoplay.mid").exists());

    // Check that output contains playback-related messages
    // It should either succeed or show warning about cat-play-mml not being available
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.contains("Attempting to play") || combined.contains("cat-play-mml"),
        "Expected playback attempt message"
    );

    // Cleanup
    let _ = fs::remove_file("/tmp/test_cli_autoplay.mid");
}

/// Test help output includes --no-play option
#[test]
fn test_cli_help_includes_no_play() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
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
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "@1cde",
            "--no-play",
            "-o",
            "/tmp/test_cli_attachment.mid",
            "--attachment-output",
            "/tmp/test_cli_attachment.json",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(Path::new("/tmp/test_cli_attachment.mid").exists());
    assert!(Path::new("/tmp/test_cli_attachment.json").exists());

    // Verify the attachment JSON is valid and contains the expected structure
    let json_content =
        fs::read_to_string("/tmp/test_cli_attachment.json").expect("Failed to read JSON file");
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

    // Cleanup
    let _ = fs::remove_file("/tmp/test_cli_attachment.mid");
    let _ = fs::remove_file("/tmp/test_cli_attachment.json");
}

/// Test that omitting --attachment-output does not create a JSON file
#[test]
fn test_cli_no_attachment_output_by_default() {
    let json_path = "/tmp/test_cli_no_attachment.json";
    // Ensure file doesn't exist before the test
    let _ = fs::remove_file(json_path);

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "@1cde",
            "--no-play",
            "-o",
            "/tmp/test_cli_no_attachment.mid",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    // No attachment JSON should be created
    assert!(!Path::new(json_path).exists());

    // Cleanup
    let _ = fs::remove_file("/tmp/test_cli_no_attachment.mid");
}
