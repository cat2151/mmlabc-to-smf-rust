//! Unit tests for Pass 3: MIDI Event Generation

use mmlabc_to_smf::pass3_events::*;
use mmlabc_to_smf::types::{Ast, AstNote};

#[test]
fn test_ast_to_events() {
    let ast = Ast {
        ast_type: "sequence".to_string(),
        notes: vec![
            AstNote {
                note_type: "note".to_string(),
                pitch: 60,
                name: "c".to_string(),
                channel: None,
            },
            AstNote {
                note_type: "note".to_string(),
                pitch: 62,
                name: "d".to_string(),
                channel: None,
            },
            AstNote {
                note_type: "note".to_string(),
                pitch: 64,
                name: "e".to_string(),
                channel: None,
            },
        ],
    };

    let events = ast_to_events(&ast);
    assert_eq!(events.len(), 6); // 3 notes * 2 events each
}

#[test]
fn test_event_types() {
    let ast = Ast {
        ast_type: "sequence".to_string(),
        notes: vec![AstNote {
            note_type: "note".to_string(),
            pitch: 60,
            name: "c".to_string(),
            channel: None,
        }],
    };

    let events = ast_to_events(&ast);
    assert_eq!(events[0].event_type, "note_on");
    assert_eq!(events[1].event_type, "note_off");
}

#[test]
fn test_event_timing() {
    let ast = Ast {
        ast_type: "sequence".to_string(),
        notes: vec![
            AstNote {
                note_type: "note".to_string(),
                pitch: 60,
                name: "c".to_string(),
                channel: None,
            },
            AstNote {
                note_type: "note".to_string(),
                pitch: 62,
                name: "d".to_string(),
                channel: None,
            },
        ],
    };

    let events = ast_to_events(&ast);
    // First note on at time 0
    assert_eq!(events[0].time, 0);
    // First note off at time 480
    assert_eq!(events[1].time, 480);
    // Second note on at time 480
    assert_eq!(events[2].time, 480);
    // Second note off at time 960
    assert_eq!(events[3].time, 960);
}

#[test]
fn test_note_properties() {
    let ast = Ast {
        ast_type: "sequence".to_string(),
        notes: vec![AstNote {
            note_type: "note".to_string(),
            pitch: 60,
            name: "c".to_string(),
            channel: None,
        }],
    };

    let events = ast_to_events(&ast);
    assert_eq!(events[0].note, 60);
    assert_eq!(events[0].velocity, 64);
    assert_eq!(events[1].note, 60);
    assert_eq!(events[1].velocity, 0);
}

#[test]
fn test_empty_ast() {
    let ast = Ast {
        ast_type: "sequence".to_string(),
        notes: vec![],
    };

    let events = ast_to_events(&ast);
    assert_eq!(events.len(), 0);
}

#[test]
fn test_save_events_to_json() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let ast = Ast {
        ast_type: "sequence".to_string(),
        notes: vec![AstNote {
            note_type: "note".to_string(),
            pitch: 60,
            name: "c".to_string(),
            channel: None,
        }],
    };
    let events = ast_to_events(&ast);
    let filepath = env::temp_dir().join("test_pass3_events.json");

    let result = save_events_to_json(&events, filepath.to_str().unwrap());
    assert!(result.is_ok());
    assert!(Path::new(&filepath).exists());

    // Clean up
    let _ = fs::remove_file(filepath);
}
