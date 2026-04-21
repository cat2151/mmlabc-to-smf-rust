//! mmlabc-to-smf Library
//!
//! This library provides functionality to convert Music Macro Language (MML)
//! strings to Standard MIDI Files (SMF) using a 4-pass architecture.

use anyhow::Result;

pub mod attachment_json;
pub mod config;
pub mod mml_preprocessor;
pub mod parse_tree_tokens;
#[cfg(feature = "parser")]
pub mod pass1_parser;
pub mod pass2_ast;
pub mod pass3_events;
pub mod pass4_midi;
#[cfg(feature = "parser")]
pub mod tree_sitter_mml;
pub mod types;

/// Options for converting MML or tokens to SMF bytes without writing debug files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmfConversionOptions {
    /// Use MIDI channel 9 (0-based) for channel groups containing `@128`.
    pub use_drum_channel_for_128: bool,
}

impl Default for SmfConversionOptions {
    fn default() -> Self {
        Self {
            use_drum_channel_for_128: true,
        }
    }
}

/// Convert already-tokenized MML to SMF bytes without writing intermediate files.
pub fn tokens_to_smf_bytes(tokens: &[types::Token]) -> Result<Vec<u8>> {
    tokens_to_smf_bytes_with_options(tokens, SmfConversionOptions::default())
}

/// Convert already-tokenized MML to SMF bytes without writing intermediate files.
pub fn tokens_to_smf_bytes_with_options(
    tokens: &[types::Token],
    options: SmfConversionOptions,
) -> Result<Vec<u8>> {
    let ast = pass2_ast::tokens_to_ast(tokens);
    let events = pass3_events::ast_to_events(&ast, options.use_drum_channel_for_128);
    pass4_midi::events_to_midi(&events)
}

/// Convert an MML string to SMF bytes without writing intermediate files.
///
/// If the string starts with an embedded attachment JSON block, the JSON block is
/// stripped before parsing the MML.
#[cfg(feature = "parser")]
pub fn mml_to_smf_bytes(mml: &str) -> Result<Vec<u8>> {
    mml_to_smf_bytes_with_options(mml, SmfConversionOptions::default())
}

/// Convert an MML string to SMF bytes without writing intermediate files.
///
/// If the string starts with an embedded attachment JSON block, the JSON block is
/// stripped before parsing the MML.
#[cfg(feature = "parser")]
pub fn mml_to_smf_bytes_with_options(mml: &str, options: SmfConversionOptions) -> Result<Vec<u8>> {
    let preprocessed = mml_preprocessor::extract_embedded_json(mml);
    raw_mml_to_smf_bytes_with_options(&preprocessed.remaining_mml, options)
}

/// Convert an MML string with no embedded attachment JSON prefix to SMF bytes.
#[cfg(feature = "parser")]
pub fn raw_mml_to_smf_bytes(mml: &str) -> Result<Vec<u8>> {
    raw_mml_to_smf_bytes_with_options(mml, SmfConversionOptions::default())
}

/// Convert an MML string with no embedded attachment JSON prefix to SMF bytes.
#[cfg(feature = "parser")]
pub fn raw_mml_to_smf_bytes_with_options(
    mml: &str,
    options: SmfConversionOptions,
) -> Result<Vec<u8>> {
    let tokens = pass1_parser::parse_mml(mml);
    tokens_to_smf_bytes_with_options(&tokens, options)
}
