//! Pass 2: Convert tokens to AST (Abstract Syntax Tree)
//! Outputs debug JSON.

use crate::types::{Ast, AstNote, Token};
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Convert tokens to AST structure
///
/// # Arguments
/// * `tokens` - List of token structures
///
/// # Returns
/// AST structure with note events (with channel assignments for multi-channel notes)
pub fn tokens_to_ast(tokens: &[Token]) -> Ast {
    // Map note names to their offset within an octave (C=0, D=2, E=4, etc.)
    let note_to_offset: HashMap<&str, u8> = [
        ("c", 0),
        ("d", 2),
        ("e", 4),
        ("f", 5),
        ("g", 7),
        ("a", 9),
        ("b", 11),
    ]
    .iter()
    .cloned()
    .collect();

    let mut notes = Vec::new();
    // Track current octave per channel (channel_group)
    // Default octave is 5 (where C5 = MIDI 60)
    // Use None as key for single-channel mode, Some(n) for multi-channel mode
    let mut current_octaves: HashMap<Option<usize>, u8> = HashMap::new();

    for token in tokens {
        if token.token_type == "note" {
            let note_offset = note_to_offset
                .get(token.value.as_str())
                .copied()
                .unwrap_or(0);

            // Get current octave for this channel (default to 5)
            let octave = *current_octaves.get(&token.channel_group).unwrap_or(&5);

            // Calculate MIDI note: octave * 12 + note_offset
            let midi_note = octave * 12 + note_offset;

            // Assign channel based on channel_group
            // If channel_group is present, notes are assigned to separate channels
            // Each channel group gets its own channel (0-based in channel_group)
            let channel = token.channel_group.map(|g| g as u8);

            notes.push(AstNote {
                note_type: "note".to_string(),
                pitch: midi_note,
                name: token.value.clone(),
                channel,
                chord_id: token.chord_id,
            });
        } else if token.token_type == "octave_up" {
            // < means octave up
            let octave = current_octaves.entry(token.channel_group).or_insert(5);
            *octave = octave.saturating_add(1);
        } else if token.token_type == "octave_down" {
            // > means octave down
            let octave = current_octaves.entry(token.channel_group).or_insert(5);
            *octave = octave.saturating_sub(1);
        } else if token.token_type == "octave_set" {
            // o command sets absolute octave (e.g., "o5" sets octave to 5)
            if let Some(octave_str) = token.value.strip_prefix('o') {
                if let Ok(octave_value) = octave_str.parse::<u8>() {
                    current_octaves.insert(token.channel_group, octave_value);
                }
            }
        }
    }

    Ast {
        ast_type: "sequence".to_string(),
        notes,
    }
}

#[derive(Serialize)]
struct AstOutput {
    pass: u8,
    description: String,
    ast: Ast,
}

/// Save AST to JSON file for debugging
///
/// # Arguments
/// * `ast` - AST structure
/// * `filepath` - Output JSON file path
pub fn save_ast_to_json(ast: &Ast, filepath: &str) -> Result<()> {
    let output = AstOutput {
        pass: 2,
        description: "Abstract Syntax Tree".to_string(),
        ast: ast.clone(),
    };

    let json = serde_json::to_string_pretty(&output)?;
    let mut file = File::create(filepath)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Execute Pass 2: Create AST from tokens
///
/// # Arguments
/// * `tokens` - List of tokens from Pass 1
/// * `output_json` - Output JSON file path
///
/// # Returns
/// AST structure
pub fn process_pass2(tokens: &[Token], output_json: &str) -> Result<Ast> {
    let ast = tokens_to_ast(tokens);
    save_ast_to_json(&ast, output_json)?;
    Ok(ast)
}
