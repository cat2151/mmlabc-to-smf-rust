//! Tests for chord functionality (semicolon operator)

use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};

#[test]
fn test_parse_simple_chord() {
    let tokens = pass1_parser::parse_mml("c;e;g");
    assert_eq!(tokens.len(), 3);

    // All tokens should have chord_group assigned
    assert_eq!(tokens[0].chord_group, Some(0));
    assert_eq!(tokens[1].chord_group, Some(1));
    assert_eq!(tokens[2].chord_group, Some(2));
}

#[test]
fn test_chord_to_ast() {
    let tokens = pass1_parser::parse_mml("c;e;g");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);

    // Each note should have different channel
    assert_eq!(ast.notes[0].channel, Some(0));
    assert_eq!(ast.notes[1].channel, Some(1));
    assert_eq!(ast.notes[2].channel, Some(2));

    // Verify MIDI pitches
    assert_eq!(ast.notes[0].pitch, 60); // C
    assert_eq!(ast.notes[1].pitch, 64); // E
    assert_eq!(ast.notes[2].pitch, 67); // G
}

#[test]
fn test_chord_events_simultaneous() {
    let tokens = pass1_parser::parse_mml("c;e;g");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    // Should have 6 events (3 note_on + 3 note_off)
    assert_eq!(events.len(), 6);

    // All note_on events should be at time 0
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);
    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 0);
    assert_eq!(note_on_events[2].time, 0);

    // Each note_on should be on different channel
    assert_eq!(note_on_events[0].channel, 0);
    assert_eq!(note_on_events[1].channel, 1);
    assert_eq!(note_on_events[2].channel, 2);
}

#[test]
fn test_chord_events_channels() {
    let tokens = pass1_parser::parse_mml("c;e;g");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    // Verify note pitches and channels
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // C on channel 0 (channel 1 in 1-based)
    assert_eq!(note_on_events[0].note, 60);
    assert_eq!(note_on_events[0].channel, 0);

    // E on channel 1 (channel 2 in 1-based)
    assert_eq!(note_on_events[1].note, 64);
    assert_eq!(note_on_events[1].channel, 1);

    // G on channel 2 (channel 3 in 1-based)
    assert_eq!(note_on_events[2].note, 67);
    assert_eq!(note_on_events[2].channel, 2);
}

#[test]
fn test_chord_midi_format() {
    let tokens = pass1_parser::parse_mml("c;e;g");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);
    let midi_data = pass4_midi::events_to_midi(&events).unwrap();

    // MIDI file should be created successfully
    assert!(!midi_data.is_empty());
    assert_eq!(&midi_data[0..4], b"MThd");
}

#[test]
fn test_full_pipeline_chord() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let test_dir = env::temp_dir().join("test_chord");
    fs::create_dir_all(&test_dir).unwrap();

    // Pass 1
    let pass1_file = test_dir.join("pass1.json");
    let tokens = pass1_parser::process_pass1("c;e;g", pass1_file.to_str().unwrap()).unwrap();
    assert_eq!(tokens.len(), 3);

    // Pass 2
    let pass2_file = test_dir.join("pass2.json");
    let ast = pass2_ast::process_pass2(&tokens, pass2_file.to_str().unwrap()).unwrap();
    assert_eq!(ast.notes.len(), 3);

    // Pass 3
    let pass3_file = test_dir.join("pass3.json");
    let events = pass3_events::process_pass3(&ast, pass3_file.to_str().unwrap()).unwrap();
    assert_eq!(events.len(), 6);

    // Pass 4
    let output_file = test_dir.join("output_chord.mid");
    let midi_data = pass4_midi::process_pass4(&events, output_file.to_str().unwrap()).unwrap();
    assert!(Path::new(&output_file).exists());
    assert!(!midi_data.is_empty());

    // Verify all debug JSONs exist
    assert!(Path::new(&pass1_file).exists());
    assert!(Path::new(&pass2_file).exists());
    assert!(Path::new(&pass3_file).exists());

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_two_note_chord() {
    let tokens = pass1_parser::parse_mml("c;e");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    assert_eq!(tokens.len(), 2);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(events.len(), 4); // 2 notes * 2 events each

    // Both notes should be at time 0 on different channels
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 0);
    assert_eq!(note_on_events[0].channel, 0);
    assert_eq!(note_on_events[1].channel, 1);
}

#[test]
fn test_sequential_notes_unchanged() {
    // Verify that sequential notes (without semicolons) still work as before
    let tokens = pass1_parser::parse_mml("cde");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    // All tokens should have chord_group None
    assert_eq!(tokens[0].chord_group, None);
    assert_eq!(tokens[1].chord_group, None);
    assert_eq!(tokens[2].chord_group, None);

    // All notes should have channel None
    assert_eq!(ast.notes[0].channel, None);
    assert_eq!(ast.notes[1].channel, None);
    assert_eq!(ast.notes[2].channel, None);

    // Notes should be sequential, not simultaneous
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 480);
    assert_eq!(note_on_events[2].time, 960);
}
