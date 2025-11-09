//! CLI tests for mmlabc-to-smf binary

use std::fs;
use std::path::Path;
use std::process::Command;

/// Test that the CLI runs successfully with --no-play flag
#[test]
fn test_cli_no_play() {
    let output = Command::new("cargo")
        .args(&["run", "--", "cde", "--no-play", "-o", "/tmp/test_cli_output.mid"])
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
