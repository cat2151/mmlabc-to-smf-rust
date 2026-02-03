//! Configuration module for mmlabc-to-smf
//!
//! Reads configuration from mmlabc-to-smf-rust.toml in the current directory.

use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Configuration structure for mmlabc-to-smf
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// External SMF player command (e.g., "timidity", "fluidsynth output.mid")
    pub external_smf_player: Option<String>,
    /// Use MIDI channel 9 (0-based) for tracks containing @128 (drum channel)
    /// Default: true (enabled)
    #[serde(default = "default_use_drum_channel_for_128")]
    pub use_drum_channel_for_128: bool,
}

fn default_use_drum_channel_for_128() -> bool {
    true
}

impl Config {
    /// Load configuration from mmlabc-to-smf-rust.toml in the current directory
    ///
    /// Returns a default configuration if the file doesn't exist or can't be read.
    pub fn load() -> Result<Self> {
        let config_path = Path::new("mmlabc-to-smf-rust.toml");

        if !config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(Config::default());
        }

        let config_str = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    /// Get the SMF player command to use
    ///
    /// Returns the configured external player if set, otherwise returns "cat-play-mml"
    pub fn get_player_command(&self) -> &str {
        self.external_smf_player
            .as_deref()
            .unwrap_or("cat-play-mml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.get_player_command(), "cat-play-mml");
    }

    #[test]
    fn test_load_nonexistent_config() {
        // Save current directory
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory where config doesn't exist
        let temp_dir = std::env::temp_dir();
        std::env::set_current_dir(&temp_dir).unwrap();

        let config = Config::load().unwrap();
        assert_eq!(config.get_player_command(), "cat-play-mml");

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_load_config_with_player() {
        // Create a temporary directory for this test
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("mmlabc-to-smf-rust.toml");

        // Write test config
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "external_smf_player = \"timidity\"").unwrap();

        // Save current directory and change to temp
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Load config
        let config = Config::load().unwrap();
        assert_eq!(config.get_player_command(), "timidity");

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }
}
