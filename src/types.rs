//! Common type definitions for the MML to SMF converter

use serde::{Deserialize, Serialize};

/// A token representing a parsed element from MML string
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "type")]
    pub token_type: String,
    pub value: String,
}

/// A note in the Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstNote {
    #[serde(rename = "type")]
    pub note_type: String,
    pub pitch: u8,
    pub name: String,
}

/// Abstract Syntax Tree representing the musical structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ast {
    #[serde(rename = "type")]
    pub ast_type: String,
    pub notes: Vec<AstNote>,
}

/// A MIDI event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MidiEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub time: u32,
    pub note: u8,
    pub velocity: u8,
}
