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
    let note_to_midi: HashMap<&str, u8> = [
        ("c", 60), // Middle C (C4)
        ("d", 62),
        ("e", 64),
        ("f", 65),
        ("g", 67),
        ("a", 69),
        ("b", 71),
    ]
    .iter()
    .cloned()
    .collect();

    let mut notes = Vec::new();
    // Track octave offset per channel (channel_group)
    // Use None as key for single-channel mode, Some(n) for multi-channel mode
    let mut octave_offsets: HashMap<Option<usize>, i8> = HashMap::new();

    for token in tokens {
        if token.token_type == "note" {
            let base_midi_note = note_to_midi
                .get(token.value.as_str())
                .copied()
                .unwrap_or(60);

            // Get octave offset for this channel (default to 0)
            let octave_offset = *octave_offsets.get(&token.channel_group).unwrap_or(&0);

            // Apply octave offset to the note
            let midi_note = (base_midi_note as i8 + octave_offset) as u8;

            // Assign channel based on channel_group
            // If channel_group is present, notes are assigned to separate channels
            // Each channel group gets its own channel (0-based in channel_group)
            let channel = token.channel_group.map(|g| g as u8);

            notes.push(AstNote {
                note_type: "note".to_string(),
                pitch: midi_note,
                name: token.value.clone(),
                channel,
            });
        } else if token.token_type == "octave_up" {
            // < means octave up (add 12 semitones)
            let offset = octave_offsets.entry(token.channel_group).or_insert(0);
            *offset += 12;
        } else if token.token_type == "octave_down" {
            // > means octave down (subtract 12 semitones)
            let offset = octave_offsets.entry(token.channel_group).or_insert(0);
            *offset -= 12;
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
