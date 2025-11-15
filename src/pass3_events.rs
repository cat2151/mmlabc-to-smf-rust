//! Pass 3: Convert AST to MIDI events
//! Outputs debug JSON.

use crate::types::{Ast, MidiEvent};
use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

/// Convert AST to MIDI event list
///
/// # Arguments
/// * `ast` - AST structure from Pass 2
///
/// # Returns
/// List of MIDI event structures with channel assignments
pub fn ast_to_events(ast: &Ast) -> Vec<MidiEvent> {
    let mut events = Vec::new();
    let mut time = 0;
    let duration = 480; // Default duration in ticks (quarter note at 480 ticks per beat)

    // Check if notes have channel assignments (multi-channel mode)
    // When notes have channel assignments, each note plays on a different channel (0, 1, 2, etc.)
    let has_multiple_channels = ast.notes.iter().any(|n| n.channel.is_some());

    // Check if we have any chords
    let has_chords = ast.notes.iter().any(|n| n.chord_id.is_some());

    if has_multiple_channels {
        // Multi-channel mode: notes within each channel are sequential
        // Track time separately for each channel
        let mut channel_times: std::collections::HashMap<u8, u32> =
            std::collections::HashMap::new();

        for note in &ast.notes {
            let channel = note.channel.unwrap_or(0);
            let current_time = channel_times.get(&channel).copied().unwrap_or(0);

            // Note on event
            events.push(MidiEvent {
                event_type: "note_on".to_string(),
                time: current_time,
                note: note.pitch,
                velocity: 64,
                channel,
            });

            // Note off event
            events.push(MidiEvent {
                event_type: "note_off".to_string(),
                time: current_time + duration,
                note: note.pitch,
                velocity: 0,
                channel,
            });

            // Advance time for this channel
            channel_times.insert(channel, current_time + duration);
        }
    } else if has_chords {
        // Chord mode: notes with the same chord_id play simultaneously on the same channel
        // Track the last chord_id processed
        let mut last_chord_id: Option<usize> = None;

        for note in &ast.notes {
            // Determine if this note is part of a chord
            let is_chord_note = note.chord_id.is_some();
            let current_chord_id = note.chord_id;

            // If this is a different chord or a non-chord note after a chord, advance time
            if last_chord_id.is_some() && last_chord_id != current_chord_id {
                time += duration;
            }

            // Note on event
            events.push(MidiEvent {
                event_type: "note_on".to_string(),
                time,
                note: note.pitch,
                velocity: 64,
                channel: 0,
            });

            // Note off event
            events.push(MidiEvent {
                event_type: "note_off".to_string(),
                time: time + duration,
                note: note.pitch,
                velocity: 0,
                channel: 0,
            });

            // If this is not a chord note, advance time
            if !is_chord_note {
                time += duration;
                last_chord_id = None;
            } else {
                last_chord_id = current_chord_id;
            }
        }
    } else {
        // Sequential notes (single channel)
        for note in &ast.notes {
            // Note on event
            events.push(MidiEvent {
                event_type: "note_on".to_string(),
                time,
                note: note.pitch,
                velocity: 64,
                channel: 0,
            });

            // Note off event
            events.push(MidiEvent {
                event_type: "note_off".to_string(),
                time: time + duration,
                note: note.pitch,
                velocity: 0,
                channel: 0,
            });

            time += duration;
        }
    }

    events
}

#[derive(Serialize)]
struct EventOutput {
    pass: u8,
    description: String,
    events: Vec<MidiEvent>,
}

/// Save events to JSON file for debugging
///
/// # Arguments
/// * `events` - List of MIDI event structures
/// * `filepath` - Output JSON file path
pub fn save_events_to_json(events: &[MidiEvent], filepath: &str) -> Result<()> {
    let output = EventOutput {
        pass: 3,
        description: "MIDI events".to_string(),
        events: events.to_vec(),
    };

    let json = serde_json::to_string_pretty(&output)?;
    let mut file = File::create(filepath)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Execute Pass 3: Create MIDI events from AST
///
/// # Arguments
/// * `ast` - AST structure from Pass 2
/// * `output_json` - Output JSON file path
///
/// # Returns
/// List of MIDI events
pub fn process_pass3(ast: &Ast, output_json: &str) -> Result<Vec<MidiEvent>> {
    let events = ast_to_events(ast);
    save_events_to_json(&events, output_json)?;
    Ok(events)
}
