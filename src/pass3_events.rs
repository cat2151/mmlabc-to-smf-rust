//! Pass 3: Convert AST to MIDI events
//! Outputs debug JSON.

use crate::types::{Ast, MidiEvent};
use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

/// Calculate duration in ticks for a given note length with optional dots
///
/// # Arguments
/// * `length` - Note length (1=whole, 4=quarter, 8=eighth, etc.)
/// * `dots` - Number of dots (0=none, 1=dotted, 2=double-dotted, etc.)
///
/// # Returns
/// Duration in ticks
fn calculate_duration(length: u32, dots: u32) -> u32 {
    // Whole note = 1920 ticks (480 ticks per quarter note * 4 beats)
    // For any note length n: base_duration = 1920 / n
    let base_duration = 1920 / length;

    // For dotted notes, each dot adds half of the previous value
    // 1 dot: duration = base + base/2 = base * 1.5
    // 2 dots: duration = base + base/2 + base/4 = base * 1.75
    // 3 dots: duration = base + base/2 + base/4 + base/8 = base * 1.875
    // Formula: duration = base * (2 - 1/(2^dots))
    if dots > 0 {
        let mut total_duration = base_duration;
        let mut additional = base_duration / 2;
        for _ in 0..dots {
            total_duration += additional;
            additional /= 2;
        }
        total_duration
    } else {
        base_duration
    }
}

/// Convert AST to MIDI event list
///
/// # Arguments
/// * `ast` - AST structure from Pass 2
/// * `use_drum_channel_for_128` - Whether to map @128 tracks to MIDI channel 9
///
/// # Returns
/// List of MIDI event structures with channel assignments
pub fn ast_to_events(ast: &Ast, use_drum_channel_for_128: bool) -> Vec<MidiEvent> {
    let mut events = Vec::new();
    let mut time = 0;
    let default_duration = 480; // Default duration in ticks (quarter note at 480 ticks per beat)

    // Check if notes have channel assignments (multi-channel mode)
    // When notes have channel assignments, each note plays on a different channel (0, 1, 2, etc.)
    let has_multiple_channels = ast.notes.iter().any(|n| n.channel.is_some());

    // Check if we have any chords
    let has_chords = ast.notes.iter().any(|n| n.chord_id.is_some());

    // Helper function to map channel to drum channel if needed
    let map_channel = |channel: u8| -> u8 {
        if use_drum_channel_for_128 {
            if let Some(ref drum_groups) = ast.drum_channel_groups {
                if drum_groups.contains(&(channel as usize)) {
                    return 9; // Map to MIDI channel 9 (0-based) for drums
                }
            }
        }
        channel
    };

    if has_multiple_channels {
        // Multi-channel mode: notes within each channel are sequential
        // Track time separately for each channel
        let mut channel_times: std::collections::HashMap<u8, u32> =
            std::collections::HashMap::new();

        for note in &ast.notes {
            let channel = note.channel.unwrap_or(0);
            let mapped_channel = map_channel(channel);
            let current_time = channel_times.get(&channel).copied().unwrap_or(0);
            let duration = if let Some(length) = note.length {
                calculate_duration(length, note.dots.unwrap_or(0))
            } else {
                default_duration
            };

            if note.note_type == "rest" {
                // Rest: just advance time without generating events
                channel_times.insert(channel, current_time + duration);
            } else if note.note_type == "program_change" {
                // Program change: generate program_change event without advancing time
                events.push(MidiEvent {
                    event_type: "program_change".to_string(),
                    time: current_time,
                    note: None,
                    velocity: None,
                    channel: mapped_channel,
                    program: Some(note.pitch),
                    tempo: None,
                });
            } else if note.note_type == "tempo_set" {
                // Tempo change: generate tempo_set event without advancing time
                // Convert BPM to microseconds per beat: usec_per_beat = 60,000,000 / BPM
                let bpm = note.pitch as u32;
                let usec_per_beat = 60_000_000 / bpm;
                events.push(MidiEvent {
                    event_type: "tempo_set".to_string(),
                    time: current_time,
                    note: None,
                    velocity: None,
                    channel: mapped_channel,
                    program: None,
                    tempo: Some(usec_per_beat),
                });
            } else {
                // Note on event
                events.push(MidiEvent {
                    event_type: "note_on".to_string(),
                    time: current_time,
                    note: Some(note.pitch),
                    velocity: Some(note.velocity.unwrap_or(127)), // Use note's velocity or default to 127
                    channel: mapped_channel,
                    program: None,
                    tempo: None,
                });

                // Note off event
                events.push(MidiEvent {
                    event_type: "note_off".to_string(),
                    time: current_time + duration,
                    note: Some(note.pitch),
                    velocity: Some(0),
                    channel: mapped_channel,
                    program: None,
                    tempo: None,
                });

                // Advance time for this channel
                channel_times.insert(channel, current_time + duration);
            }
        }
    } else if has_chords {
        // Chord mode: notes with the same chord_id play simultaneously on the same channel
        // Track the last chord_id processed
        let mut last_chord_id: Option<usize> = None;

        for note in &ast.notes {
            let duration = if let Some(length) = note.length {
                calculate_duration(length, note.dots.unwrap_or(0))
            } else {
                default_duration
            };

            // Determine if this note is part of a chord
            let is_chord_note = note.chord_id.is_some();
            let current_chord_id = note.chord_id;

            // If this is a different chord or a non-chord note after a chord, advance time
            if last_chord_id.is_some() && last_chord_id != current_chord_id {
                time += duration;
            }

            if note.note_type == "rest" {
                // Rest: just advance time without generating events
                time += duration;
                last_chord_id = None;
            } else if note.note_type == "program_change" {
                // Program change: generate program_change event without advancing time
                events.push(MidiEvent {
                    event_type: "program_change".to_string(),
                    time,
                    note: None,
                    velocity: None,
                    channel: 0,
                    program: Some(note.pitch),
                    tempo: None,
                });
                last_chord_id = None;
            } else if note.note_type == "tempo_set" {
                // Tempo change: generate tempo_set event without advancing time
                let bpm = note.pitch as u32;
                let usec_per_beat = 60_000_000 / bpm;
                events.push(MidiEvent {
                    event_type: "tempo_set".to_string(),
                    time,
                    note: None,
                    velocity: None,
                    channel: 0,
                    program: None,
                    tempo: Some(usec_per_beat),
                });
                last_chord_id = None;
            } else {
                // Note on event
                events.push(MidiEvent {
                    event_type: "note_on".to_string(),
                    time,
                    note: Some(note.pitch),
                    velocity: Some(note.velocity.unwrap_or(127)), // Use note's velocity or default to 127
                    channel: 0,
                    program: None,
                    tempo: None,
                });

                // Note off event
                events.push(MidiEvent {
                    event_type: "note_off".to_string(),
                    time: time + duration,
                    note: Some(note.pitch),
                    velocity: Some(0),
                    channel: 0,
                    program: None,
                    tempo: None,
                });

                // If this is not a chord note, advance time
                if !is_chord_note {
                    time += duration;
                    last_chord_id = None;
                } else {
                    last_chord_id = current_chord_id;
                }
            }
        }
    } else {
        // Sequential notes (single channel)
        for note in &ast.notes {
            let duration = if let Some(length) = note.length {
                calculate_duration(length, note.dots.unwrap_or(0))
            } else {
                default_duration
            };

            if note.note_type == "rest" {
                // Rest: just advance time without generating events
                time += duration;
            } else if note.note_type == "program_change" {
                // Program change: generate program_change event without advancing time
                events.push(MidiEvent {
                    event_type: "program_change".to_string(),
                    time,
                    note: None,
                    velocity: None,
                    channel: 0,
                    program: Some(note.pitch),
                    tempo: None,
                });
            } else if note.note_type == "tempo_set" {
                // Tempo change: generate tempo_set event without advancing time
                let bpm = note.pitch as u32;
                let usec_per_beat = 60_000_000 / bpm;
                events.push(MidiEvent {
                    event_type: "tempo_set".to_string(),
                    time,
                    note: None,
                    velocity: None,
                    channel: 0,
                    program: None,
                    tempo: Some(usec_per_beat),
                });
            } else {
                // Note on event
                events.push(MidiEvent {
                    event_type: "note_on".to_string(),
                    time,
                    note: Some(note.pitch),
                    velocity: Some(note.velocity.unwrap_or(127)), // Use note's velocity or default to 127
                    channel: 0,
                    program: None,
                    tempo: None,
                });

                // Note off event
                events.push(MidiEvent {
                    event_type: "note_off".to_string(),
                    time: time + duration,
                    note: Some(note.pitch),
                    velocity: Some(0),
                    channel: 0,
                    program: None,
                    tempo: None,
                });

                time += duration;
            }
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
/// * `use_drum_channel_for_128` - Whether to map @128 tracks to MIDI channel 9
///
/// # Returns
/// List of MIDI events
pub fn process_pass3(
    ast: &Ast,
    output_json: &str,
    use_drum_channel_for_128: bool,
) -> Result<Vec<MidiEvent>> {
    let events = ast_to_events(ast, use_drum_channel_for_128);
    save_events_to_json(&events, output_json)?;
    Ok(events)
}
