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

    // Track current length per channel
    // Default length is 4 (quarter note)
    let mut current_lengths: HashMap<Option<usize>, u32> = HashMap::new();

    // Track current dots per channel
    // Default dots is 0 (no dots)
    let mut current_dots: HashMap<Option<usize>, u32> = HashMap::new();

    // Track current velocity per channel
    // Default velocity is v15 which maps to MIDI velocity 127
    let mut current_velocities: HashMap<Option<usize>, u8> = HashMap::new();

    // Track key transpose per channel
    // Default transpose is 0 (no transposition)
    let mut current_transposes: HashMap<Option<usize>, i8> = HashMap::new();

    for token in tokens {
        if token.token_type == "note" {
            let note_offset = note_to_offset
                .get(token.value.as_str())
                .copied()
                .unwrap_or(0);

            // Get current octave for this channel (default to 5)
            let octave = *current_octaves.get(&token.channel_group).unwrap_or(&5);

            // Get current length for this channel (default to 4 = quarter note)
            // Use note-specific length if present, otherwise use current length
            let length = token
                .note_length
                .or_else(|| current_lengths.get(&token.channel_group).copied())
                .unwrap_or(4);

            // Get current dots for this channel (default to 0)
            // Use note-specific dots if present, otherwise use current dots
            let dots = token
                .dots
                .or_else(|| current_dots.get(&token.channel_group).copied())
                .unwrap_or(0);

            // Get current velocity for this channel (default to 127 = v15)
            let velocity = *current_velocities.get(&token.channel_group).unwrap_or(&127);

            // Get current transpose for this channel (default to 0)
            let transpose = *current_transposes.get(&token.channel_group).unwrap_or(&0);

            // Calculate MIDI note: octave * 12 + note_offset
            let mut midi_note = octave * 12 + note_offset;

            // Apply modifier if present (+ for sharp, - for flat)
            if let Some(modifier) = &token.modifier {
                match modifier.as_str() {
                    "+" => midi_note = midi_note.saturating_add(1),
                    "-" => midi_note = midi_note.saturating_sub(1),
                    _ => {}
                }
            }

            // Apply key transpose
            if transpose >= 0 {
                midi_note = midi_note.saturating_add(transpose as u8);
            } else {
                midi_note = midi_note.saturating_sub((-transpose) as u8);
            }

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
                length: Some(length),
                dots: Some(dots),
                velocity: Some(velocity),
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
        } else if token.token_type == "length_set" {
            // l command sets default note length (e.g., "l8" sets to eighth note)
            // Also supports dots (e.g., "l4." sets to dotted quarter note)
            if let Some(length_value) = token.note_length {
                current_lengths.insert(token.channel_group, length_value);
            }
            if let Some(dots_value) = token.dots {
                current_dots.insert(token.channel_group, dots_value);
            } else {
                // Reset dots to 0 when length_set has no dots
                current_dots.insert(token.channel_group, 0);
            }
        } else if token.token_type == "velocity_set" {
            // v command sets velocity (e.g., "v15" sets to MIDI velocity 127)
            // Linear scaling: v_value * (127/15) = velocity
            // v1 = 1 * (127/15) ≈ 8.47 → 8
            // v15 = 15 * (127/15) ≈ 127 → 127
            if let Some(velocity_str) = token.value.strip_prefix('v') {
                if let Ok(v_value) = velocity_str.parse::<u32>() {
                    // Clamp v_value to 1-15 range
                    let v_clamped = v_value.clamp(1, 15);
                    // Calculate MIDI velocity: round(v_value * 127 / 15)
                    let velocity = ((v_clamped * 127 + 7) / 15) as u8; // +7 for rounding
                    current_velocities.insert(token.channel_group, velocity);
                }
            }
        } else if token.token_type == "rest" {
            // Rest command - add a special rest note
            // Assign channel based on channel_group
            let channel = token.channel_group.map(|g| g as u8);

            // Get current length for this channel (default to 4 = quarter note)
            // Use rest-specific length if present, otherwise use current length
            let length = token
                .note_length
                .or_else(|| current_lengths.get(&token.channel_group).copied())
                .unwrap_or(4);

            // Get current dots for this channel (default to 0)
            // Use rest-specific dots if present, otherwise use current dots
            let dots = token
                .dots
                .or_else(|| current_dots.get(&token.channel_group).copied())
                .unwrap_or(0);

            notes.push(AstNote {
                note_type: "rest".to_string(),
                pitch: 0, // Pitch is not used for rests
                name: "r".to_string(),
                channel,
                chord_id: token.chord_id,
                length: Some(length),
                dots: Some(dots),
                velocity: None, // Velocity is not used for rests
            });
        } else if token.token_type == "program_change" {
            // @ command sets MIDI program (instrument)
            if let Some(program_str) = token.value.strip_prefix('@') {
                if let Ok(program_value) = program_str.parse::<u8>() {
                    // Assign channel based on channel_group
                    let channel = token.channel_group.map(|g| g as u8);

                    notes.push(AstNote {
                        note_type: "program_change".to_string(),
                        pitch: program_value, // Store program number in pitch field
                        name: token.value.clone(),
                        channel,
                        chord_id: None,
                        length: None,
                        dots: None,
                        velocity: None, // Velocity is not used for program changes
                    });
                }
            }
        } else if token.token_type == "tempo_set" {
            // t command sets tempo (BPM)
            if let Some(tempo_str) = token.value.strip_prefix('t') {
                if let Ok(tempo_value) = tempo_str.parse::<u32>() {
                    // Assign channel based on channel_group
                    let channel = token.channel_group.map(|g| g as u8);

                    // Store BPM value in pitch field (we'll convert to usec/beat in pass3)
                    // Clamp to reasonable BPM range (1-255)
                    let bpm = tempo_value.clamp(1, 255) as u8;

                    notes.push(AstNote {
                        note_type: "tempo_set".to_string(),
                        pitch: bpm, // Store BPM in pitch field
                        name: token.value.clone(),
                        channel,
                        chord_id: None,
                        length: None,
                        dots: None,
                        velocity: None, // Velocity is not used for tempo changes
                    });
                }
            }
        } else if token.token_type == "key_transpose" {
            // kt command sets key transpose (e.g., "kt1" transposes up 1 semitone, "kt-1" transposes down 1)
            // Format: kt[+/-]number where + is optional
            if let Some(kt_str) = token
                .value
                .strip_prefix("kt")
                .or_else(|| token.value.strip_prefix("KT"))
            {
                // Parse the transpose value (can be negative)
                if let Ok(transpose_value) = kt_str.parse::<i8>() {
                    current_transposes.insert(token.channel_group, transpose_value);
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
