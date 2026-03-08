//! Attachment JSON generation
//!
//! Generates an "attached JSON" (添付JSON) alongside the SMF file.
//! The attached JSON describes per-ProgramChange settings (Tone, Portamento, LFO, etc.)
//! in a self-describing format that is separate from the SMF.
//!
//! The format is compatible with smf-to-ym2151log-rust's attachment input:
//! ```json
//! [
//!   {
//!     "ProgramChange": 0,
//!     "Tone": { "events": [] }
//!   }
//! ]
//! ```

use crate::types::MidiEvent;
use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeSet;

/// A single YM2151 register event (for Tone data)
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Ym2151RegisterEvent {
    pub time: u32,
    pub addr: String,
    pub data: String,
}

/// Tone data for a program
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ToneData {
    pub events: Vec<Ym2151RegisterEvent>,
}

/// A single entry in the attachment JSON array
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AttachmentEntry {
    #[serde(rename = "ProgramChange")]
    pub program_change: u8,
    #[serde(rename = "Tone")]
    pub tone: ToneData,
}

/// Collect unique program numbers from MIDI events.
/// Returns a sorted set of unique program numbers.
fn collect_program_numbers(events: &[MidiEvent]) -> BTreeSet<u8> {
    events
        .iter()
        .filter_map(|e| {
            if e.event_type == "program_change" {
                e.program
            } else {
                None
            }
        })
        .collect()
}

/// Generate the attachment JSON entries from MIDI events.
///
/// For each unique ProgramChange found in the events, an entry is created
/// with an empty Tone (placeholder for the user to fill in).
/// If no ProgramChange events are found, a single entry for program 0 is returned.
pub fn generate_attachment_entries(events: &[MidiEvent]) -> Vec<AttachmentEntry> {
    let programs = collect_program_numbers(events);
    if programs.is_empty() {
        vec![AttachmentEntry {
            program_change: 0,
            tone: ToneData { events: vec![] },
        }]
    } else {
        programs
            .into_iter()
            .map(|p| AttachmentEntry {
                program_change: p,
                tone: ToneData { events: vec![] },
            })
            .collect()
    }
}

/// Serialize attachment entries to a pretty-printed JSON string.
pub fn attachment_entries_to_json(entries: &[AttachmentEntry]) -> Result<String> {
    Ok(serde_json::to_string_pretty(entries)?)
}

/// Generate the attachment JSON string from MIDI events.
///
/// Convenience wrapper that calls [`generate_attachment_entries`] and
/// [`attachment_entries_to_json`] in one step.
pub fn generate_attachment_json(events: &[MidiEvent]) -> Result<String> {
    let entries = generate_attachment_entries(events);
    attachment_entries_to_json(&entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MidiEvent;

    fn make_program_change(program: u8) -> MidiEvent {
        MidiEvent {
            event_type: "program_change".to_string(),
            time: 0,
            note: None,
            velocity: None,
            channel: 0,
            program: Some(program),
            tempo: None,
        }
    }

    fn make_note_on() -> MidiEvent {
        MidiEvent {
            event_type: "note_on".to_string(),
            time: 0,
            note: Some(60),
            velocity: Some(127),
            channel: 0,
            program: None,
            tempo: None,
        }
    }

    #[test]
    fn test_no_program_change_defaults_to_program_0() {
        let events = vec![make_note_on()];
        let entries = generate_attachment_entries(&events);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].program_change, 0);
        assert!(entries[0].tone.events.is_empty());
    }

    #[test]
    fn test_single_program_change() {
        let events = vec![make_program_change(1), make_note_on()];
        let entries = generate_attachment_entries(&events);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].program_change, 1);
        assert!(entries[0].tone.events.is_empty());
    }

    #[test]
    fn test_multiple_program_changes_sorted() {
        let events = vec![
            make_program_change(3),
            make_program_change(1),
            make_program_change(2),
        ];
        let entries = generate_attachment_entries(&events);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].program_change, 1);
        assert_eq!(entries[1].program_change, 2);
        assert_eq!(entries[2].program_change, 3);
    }

    #[test]
    fn test_duplicate_program_changes_deduplicated() {
        let events = vec![
            make_program_change(0),
            make_note_on(),
            make_program_change(0),
        ];
        let entries = generate_attachment_entries(&events);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].program_change, 0);
    }

    #[test]
    fn test_generate_attachment_json_output() {
        let events = vec![make_program_change(0)];
        let json = generate_attachment_json(&events).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array());
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["ProgramChange"], 0);
        assert!(arr[0]["Tone"]["events"].is_array());
        assert_eq!(arr[0]["Tone"]["events"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_generate_attachment_json_no_events() {
        let events: Vec<MidiEvent> = vec![];
        let json = generate_attachment_json(&events).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["ProgramChange"], 0);
    }
}
