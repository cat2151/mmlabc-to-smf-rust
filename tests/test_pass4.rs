//! Unit tests for Pass 4: MIDI File Creation

use mmlabc_to_smf::pass4_midi::*;
use mmlabc_to_smf::types::MidiEvent;

#[test]
fn test_events_to_midi() {
    let events = vec![
        MidiEvent {
            event_type: "note_on".to_string(),
            time: 0,
            note: 60,
            velocity: 64,
        },
        MidiEvent {
            event_type: "note_off".to_string(),
            time: 480,
            note: 60,
            velocity: 0,
        },
    ];

    let result = events_to_midi(&events);
    assert!(result.is_ok());
    let midi_data = result.unwrap();
    assert!(!midi_data.is_empty());
}

#[test]
fn test_midi_file_structure() {
    let events = vec![
        MidiEvent {
            event_type: "note_on".to_string(),
            time: 0,
            note: 60,
            velocity: 64,
        },
        MidiEvent {
            event_type: "note_off".to_string(),
            time: 480,
            note: 60,
            velocity: 0,
        },
    ];

    let midi_data = events_to_midi(&events).unwrap();

    // MIDI file should start with "MThd" header
    assert_eq!(&midi_data[0..4], b"MThd");
}

#[test]
fn test_save_midi_file() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let events = vec![
        MidiEvent {
            event_type: "note_on".to_string(),
            time: 0,
            note: 60,
            velocity: 64,
        },
        MidiEvent {
            event_type: "note_off".to_string(),
            time: 480,
            note: 60,
            velocity: 0,
        },
    ];

    let midi_data = events_to_midi(&events).unwrap();
    let filepath = env::temp_dir().join("test_output.mid");

    let result = save_midi_file(&midi_data, filepath.to_str().unwrap());
    assert!(result.is_ok());
    assert!(Path::new(&filepath).exists());

    // Clean up
    let _ = fs::remove_file(filepath);
}

#[test]
fn test_process_pass4() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let events = vec![
        MidiEvent {
            event_type: "note_on".to_string(),
            time: 0,
            note: 60,
            velocity: 64,
        },
        MidiEvent {
            event_type: "note_off".to_string(),
            time: 480,
            note: 60,
            velocity: 0,
        },
    ];

    let filepath = env::temp_dir().join("test_process_pass4.mid");
    let result = process_pass4(&events, filepath.to_str().unwrap());

    assert!(result.is_ok());
    assert!(Path::new(&filepath).exists());

    // Clean up
    let _ = fs::remove_file(filepath);
}

#[test]
fn test_empty_events() {
    let events = vec![];
    let result = events_to_midi(&events);
    assert!(result.is_ok());
}
