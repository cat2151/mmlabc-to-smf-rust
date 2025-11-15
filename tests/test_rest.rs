//! Tests for rest command (r)

use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events};

#[test]
fn test_parse_simple_rest() {
    let tokens = pass1_parser::parse_mml("r");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "rest");
    assert_eq!(tokens[0].value, "r");
}

#[test]
fn test_parse_crc() {
    let tokens = pass1_parser::parse_mml("crc");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[1].token_type, "rest");
    assert_eq!(tokens[1].value, "r");
    assert_eq!(tokens[2].token_type, "note");
    assert_eq!(tokens[2].value, "c");
}

#[test]
fn test_parse_rest_uppercase() {
    let tokens = pass1_parser::parse_mml("R");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "rest");
    assert_eq!(tokens[0].value, "r");
}

#[test]
fn test_rest_to_ast() {
    let tokens = pass1_parser::parse_mml("crc");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);

    // First note (C)
    assert_eq!(ast.notes[0].note_type, "note");
    assert_eq!(ast.notes[0].pitch, 60);
    assert_eq!(ast.notes[0].name, "c");

    // Rest
    assert_eq!(ast.notes[1].note_type, "rest");
    assert_eq!(ast.notes[1].name, "r");

    // Second note (C)
    assert_eq!(ast.notes[2].note_type, "note");
    assert_eq!(ast.notes[2].pitch, 60);
    assert_eq!(ast.notes[2].name, "c");
}

#[test]
fn test_rest_events_timing() {
    let tokens = pass1_parser::parse_mml("crc");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    // Should have 4 events: 2 notes * 2 events each (note_on + note_off)
    // No events for the rest
    assert_eq!(events.len(), 4);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 2);

    // First C starts at time 0
    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[0].note, 60);

    // Second C starts at time 960 (after first note duration 480 + rest duration 480)
    assert_eq!(note_on_events[1].time, 960);
    assert_eq!(note_on_events[1].note, 60);
}

#[test]
fn test_multiple_rests() {
    let tokens = pass1_parser::parse_mml("crrrc");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    // Should have 4 events: 2 notes * 2 events each
    assert_eq!(events.len(), 4);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // First C at time 0
    assert_eq!(note_on_events[0].time, 0);
    // Second C at time 1920 (480 for first note + 480*3 for three rests)
    assert_eq!(note_on_events[1].time, 1920);
}

#[test]
fn test_rest_between_different_notes() {
    let tokens = pass1_parser::parse_mml("crdre");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    // Should have 6 events: 3 notes * 2 events each
    assert_eq!(events.len(), 6);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);

    // C at time 0
    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[0].note, 60); // C

    // D at time 960 (480 for C + 480 for rest)
    assert_eq!(note_on_events[1].time, 960);
    assert_eq!(note_on_events[1].note, 62); // D

    // E at time 1920 (480 for C + 480 for rest + 480 for D + 480 for rest)
    assert_eq!(note_on_events[2].time, 1920);
    assert_eq!(note_on_events[2].note, 64); // E
}

#[test]
fn test_rest_with_octave_change() {
    let tokens = pass1_parser::parse_mml("c<r>c");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    // First C should be in octave 5 (MIDI 60)
    assert_eq!(ast.notes[0].pitch, 60);

    // Rest in the middle
    assert_eq!(ast.notes[1].note_type, "rest");

    // Last C should be back in octave 5 (MIDI 60)
    // Octave up then octave down
    assert_eq!(ast.notes[2].pitch, 60);
}

#[test]
fn test_rest_in_multi_channel() {
    let tokens = pass1_parser::parse_mml("crc;ere");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);

    // Should have 8 events: 4 notes * 2 events each
    assert_eq!(events.len(), 8);

    // Filter events by channel
    let channel_0_events: Vec<_> = events
        .iter()
        .filter(|e| e.channel == 0 && e.event_type == "note_on")
        .collect();

    let channel_1_events: Vec<_> = events
        .iter()
        .filter(|e| e.channel == 1 && e.event_type == "note_on")
        .collect();

    assert_eq!(channel_0_events.len(), 2);
    assert_eq!(channel_1_events.len(), 2);

    // Channel 0: C at 0, C at 960
    assert_eq!(channel_0_events[0].time, 0);
    assert_eq!(channel_0_events[1].time, 960);

    // Channel 1: E at 0, E at 960
    assert_eq!(channel_1_events[0].time, 0);
    assert_eq!(channel_1_events[1].time, 960);
}

#[test]
fn test_full_pipeline_with_rest() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let test_dir = env::temp_dir().join("test_rest_integration");
    fs::create_dir_all(&test_dir).unwrap();

    // Pass 1
    let pass1_file = test_dir.join("pass1.json");
    let tokens = pass1_parser::process_pass1("crc", pass1_file.to_str().unwrap()).unwrap();
    assert_eq!(tokens.len(), 3);

    // Pass 2
    let pass2_file = test_dir.join("pass2.json");
    let ast = pass2_ast::process_pass2(&tokens, pass2_file.to_str().unwrap()).unwrap();
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].note_type, "note");
    assert_eq!(ast.notes[1].note_type, "rest");
    assert_eq!(ast.notes[2].note_type, "note");

    // Pass 3
    let pass3_file = test_dir.join("pass3.json");
    let events = pass3_events::process_pass3(&ast, pass3_file.to_str().unwrap()).unwrap();
    assert_eq!(events.len(), 4); // 2 notes * 2 events each

    // Verify all debug JSONs exist
    assert!(Path::new(&pass1_file).exists());
    assert!(Path::new(&pass2_file).exists());
    assert!(Path::new(&pass3_file).exists());

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}
