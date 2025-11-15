//! Unit tests for tempo command (t)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;
use mmlabc_to_smf::pass4_midi::*;

#[test]
fn test_parse_tempo_set() {
    let tokens = parse_mml("t120c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "tempo_set");
    assert_eq!(tokens[0].value, "t120");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_parse_tempo_60() {
    let tokens = parse_mml("t60c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "tempo_set");
    assert_eq!(tokens[0].value, "t60");
}

#[test]
fn test_parse_tempo_150() {
    let tokens = parse_mml("t150c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "tempo_set");
    assert_eq!(tokens[0].value, "t150");
}

#[test]
fn test_tempo_to_ast() {
    let tokens = parse_mml("t120c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);

    // First item is tempo_set
    assert_eq!(ast.notes[0].note_type, "tempo_set");
    assert_eq!(ast.notes[0].pitch, 120); // BPM 120
    assert_eq!(ast.notes[0].name, "t120");

    // Second item is the note
    assert_eq!(ast.notes[1].note_type, "note");
    assert_eq!(ast.notes[1].pitch, 60); // C5
}

#[test]
fn test_tempo_60_to_ast() {
    let tokens = parse_mml("t60c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "tempo_set");
    assert_eq!(ast.notes[0].pitch, 60); // BPM 60
}

#[test]
fn test_tempo_150_to_ast() {
    let tokens = parse_mml("t150c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "tempo_set");
    assert_eq!(ast.notes[0].pitch, 150); // BPM 150
}

#[test]
fn test_tempo_to_events() {
    let tokens = parse_mml("t120c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Should have: tempo_set, note_on, note_off
    assert_eq!(events.len(), 3);

    // Tempo change event at time 0
    assert_eq!(events[0].event_type, "tempo_set");
    assert_eq!(events[0].time, 0);
    // BPM 120 = 500000 usec per beat (60,000,000 / 120)
    assert_eq!(events[0].tempo, Some(500000));
    assert_eq!(events[0].channel, 0);

    // Note on at time 0 (tempo change doesn't advance time)
    assert_eq!(events[1].event_type, "note_on");
    assert_eq!(events[1].time, 0);
    assert_eq!(events[1].note, Some(60));

    // Note off at time 480 (quarter note duration)
    assert_eq!(events[2].event_type, "note_off");
    assert_eq!(events[2].time, 480);
}

#[test]
fn test_tempo_60_conversion() {
    // t60 should convert to 1000000 usec per beat (60,000,000 / 60)
    let tokens = parse_mml("t60c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();

    assert_eq!(tempo_events.len(), 1);
    assert_eq!(tempo_events[0].tempo, Some(1000000));
}

#[test]
fn test_tempo_150_conversion() {
    // t150 should convert to 400000 usec per beat (60,000,000 / 150)
    let tokens = parse_mml("t150c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();

    assert_eq!(tempo_events.len(), 1);
    assert_eq!(tempo_events[0].tempo, Some(400000));
}

#[test]
fn test_tempo_doesnt_advance_time() {
    // t120 should not affect timing of notes
    let tokens = parse_mml("ct120d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Find the tempo change event
    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();

    assert_eq!(tempo_events.len(), 1);
    // Tempo change happens at time 480 (after first note)
    assert_eq!(tempo_events[0].time, 480);

    // Second note should start right after tempo change (same time)
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[1].time, 480); // D starts at same time as tempo change
}

#[test]
fn test_multiple_tempo_changes() {
    let tokens = parse_mml("t60ct120dt150e");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();

    assert_eq!(tempo_events.len(), 3);
    assert_eq!(tempo_events[0].tempo, Some(1000000)); // BPM 60
    assert_eq!(tempo_events[1].tempo, Some(500000)); // BPM 120
    assert_eq!(tempo_events[2].tempo, Some(400000)); // BPM 150
}

#[test]
fn test_tempo_with_octave() {
    let tokens = parse_mml("t120o4c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "tempo_set");
    assert_eq!(ast.notes[0].pitch, 120);
    assert_eq!(ast.notes[1].pitch, 48); // C4
}

#[test]
fn test_tempo_with_length() {
    let tokens = parse_mml("t120l8c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Note should have eighth note length (240 ticks)
    let note_off_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_off")
        .collect();

    assert_eq!(note_off_events[0].time, 240); // 240 ticks for eighth note
}

#[test]
fn test_tempo_to_midi() {
    let tokens = parse_mml("t120c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let result = events_to_midi(&events);
    assert!(result.is_ok());

    let midi_data = result.unwrap();
    assert!(!midi_data.is_empty());

    // MIDI file should start with "MThd" header
    assert_eq!(&midi_data[0..4], b"MThd");
}

#[test]
fn test_full_pipeline_with_tempo() {
    use std::env;
    use std::fs;

    // Create a test directory
    let test_dir = env::temp_dir().join("test_tempo");
    fs::create_dir_all(&test_dir).unwrap();

    let mml = "t60ct120dt150e";

    // Pass 1: Parse
    let tokens = parse_mml(mml);
    let pass1_json = test_dir.join("pass1_tempo.json");
    save_tokens_to_json(&tokens, pass1_json.to_str().unwrap()).unwrap();

    // Pass 2: Create AST
    let ast = tokens_to_ast(&tokens);
    let pass2_json = test_dir.join("pass2_tempo.json");
    save_ast_to_json(&ast, pass2_json.to_str().unwrap()).unwrap();

    // Pass 3: Generate MIDI events
    let events = ast_to_events(&ast);
    let pass3_json = test_dir.join("pass3_tempo.json");
    save_events_to_json(&events, pass3_json.to_str().unwrap()).unwrap();

    // Pass 4: Create MIDI file
    let midi_file = test_dir.join("output_tempo.mid");
    process_pass4(&events, midi_file.to_str().unwrap()).unwrap();

    // Verify MIDI file was created
    assert!(midi_file.exists());

    // Verify we have tempo change events
    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();
    assert_eq!(tempo_events.len(), 3);

    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_tempo_between_notes() {
    let tokens = parse_mml("ct60d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Find events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();

    // C should start at 0
    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[0].note, Some(60)); // C

    // Tempo change should happen at 480 (after C)
    assert_eq!(tempo_events[0].time, 480);
    assert_eq!(tempo_events[0].tempo, Some(1000000)); // BPM 60

    // D should start at 480 (same time as tempo change)
    assert_eq!(note_on_events[1].time, 480);
    assert_eq!(note_on_events[1].note, Some(62)); // D
}

#[test]
fn test_tempo_with_modifier() {
    let tokens = parse_mml("t120c+");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "tempo_set");
    assert_eq!(ast.notes[0].pitch, 120);
    assert_eq!(ast.notes[1].pitch, 61); // C# (60 + 1)
}

#[test]
fn test_tempo_with_chord() {
    let tokens = parse_mml("t120'ceg'");
    let ast = tokens_to_ast(&tokens);

    // Should have tempo_set + 3 chord notes
    assert_eq!(ast.notes.len(), 4);
    assert_eq!(ast.notes[0].note_type, "tempo_set");

    // Check that chord notes all have the same chord_id
    assert_eq!(ast.notes[1].chord_id, ast.notes[2].chord_id);
    assert_eq!(ast.notes[2].chord_id, ast.notes[3].chord_id);
}

#[test]
fn test_tempo_in_multi_channel() {
    // Channel 0: t60c
    // Channel 1: t120e
    let tokens = parse_mml("t60c;t120e");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();

    // Should have 2 tempo events
    assert_eq!(tempo_events.len(), 2);

    // Both tempo events should be at channel 0 and 1 respectively
    assert_eq!(tempo_events[0].channel, 0);
    assert_eq!(tempo_events[0].tempo, Some(1000000)); // BPM 60

    assert_eq!(tempo_events[1].channel, 1);
    assert_eq!(tempo_events[1].tempo, Some(500000)); // BPM 120
}

#[test]
fn test_tempo_value_boundaries() {
    // Test boundary values
    let tokens = parse_mml("t1ct255d");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes[0].pitch, 1); // BPM 1
    assert_eq!(ast.notes[2].pitch, 255); // BPM 255 (max for u8)
}
