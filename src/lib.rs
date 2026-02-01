//! mmlabc-to-smf Library
//!
//! This library provides functionality to convert Music Macro Language (MML)
//! strings to Standard MIDI Files (SMF) using a 4-pass architecture.
//!
//! ## WASM Support
//!
//! This library supports WASM compilation using WASI Reactor pattern for Passes 2-4.
//! Pass 1 (tree-sitter parsing) cannot be compiled to WASM due to POSIX dependencies.
//! See WASI_REACTOR_VERIFICATION.md for verification details.

pub mod config;
#[cfg(not(target_family = "wasm"))]
pub mod pass1_parser;
pub mod pass2_ast;
pub mod pass3_events;
pub mod pass4_midi;
#[cfg(not(target_family = "wasm"))]
pub mod tree_sitter_mml;
pub mod types;

// WASM-compatible interface (Passes 2-4 only, requires external parsing)
#[cfg(target_family = "wasm")]
pub mod wasm_compat;
