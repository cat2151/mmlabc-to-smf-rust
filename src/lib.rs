//! mmlabc-to-smf Library
//!
//! This library provides functionality to convert Music Macro Language (MML)
//! strings to Standard MIDI Files (SMF) using a 4-pass architecture.

pub mod attachment_json;
pub mod config;
pub mod mml_preprocessor;
pub mod parse_tree_tokens;
#[cfg(feature = "cli")]
pub mod pass1_parser;
pub mod pass2_ast;
pub mod pass3_events;
pub mod pass4_midi;
#[cfg(feature = "cli")]
pub mod tree_sitter_mml;
pub mod types;
