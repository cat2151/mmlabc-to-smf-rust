//! Common type definitions for the MML to SMF converter

use serde::{Deserialize, Serialize};

/// A token representing a parsed element from MML string
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "type")]
    pub token_type: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chord_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dots: Option<u32>,
}

/// A note in the Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstNote {
    #[serde(rename = "type")]
    pub note_type: String,
    pub pitch: u8,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chord_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dots: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<u8>,
}

/// Abstract Syntax Tree representing the musical structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ast {
    #[serde(rename = "type")]
    pub ast_type: String,
    pub notes: Vec<AstNote>,
    /// Channel groups that contain @128 (should be mapped to drum channel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drum_channel_groups: Option<Vec<usize>>,
}

/// A MIDI event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MidiEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub time: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<u8>,
    pub channel: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tempo: Option<u32>,
}
