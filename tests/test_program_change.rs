//! Unit tests for program change command (@)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;
use mmlabc_to_smf::pass4_midi::*;

#[test]
fn test_parse_program_change() {
    let tokens = parse_mml("@0c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "program_change");
    assert_eq!(tokens[0].value, "@0");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_parse_program_change_127() {
    let tokens = parse_mml("@127c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "program_change");
    assert_eq!(tokens[0].value, "@127");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_program_change_to_ast() {
    let tokens = parse_mml("@0c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);

    // First item is program change
    assert_eq!(ast.notes[0].note_type, "program_change");
    assert_eq!(ast.notes[0].pitch, 0); // Program 0
    assert_eq!(ast.notes[0].name, "@0");

    // Second item is the note
    assert_eq!(ast.notes[1].note_type, "note");
    assert_eq!(ast.notes[1].pitch, 60); // C5
}

#[test]
fn test_program_change_at_127() {
    let tokens = parse_mml("@127c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "program_change");
    assert_eq!(ast.notes[0].pitch, 127); // Program 127
}

#[test]
fn test_program_change_to_events() {
    let tokens = parse_mml("@0c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Should have: program_change, note_on, note_off
    assert_eq!(events.len(), 3);

    // Program change event at time 0
    assert_eq!(events[0].event_type, "program_change");
    assert_eq!(events[0].time, 0);
    assert_eq!(events[0].program, Some(0));
    assert_eq!(events[0].channel, 0);

    // Note on at time 0 (program change doesn't advance time)
    assert_eq!(events[1].event_type, "note_on");
    assert_eq!(events[1].time, 0);
    assert_eq!(events[1].note, Some(60));

    // Note off at time 480 (quarter note duration)
    assert_eq!(events[2].event_type, "note_off");
    assert_eq!(events[2].time, 480);
}

#[test]
fn test_program_change_doesnt_advance_time() {
    // @1 should not affect timing of notes
    let tokens = parse_mml("c@1d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Find the program change event
    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    assert_eq!(program_change_events.len(), 1);
    // Program change happens at time 480 (after first note)
    assert_eq!(program_change_events[0].time, 480);

    // Second note should start right after program change (same time)
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[1].time, 480); // D starts at same time as program change
}

#[test]
fn test_multiple_program_changes() {
    let tokens = parse_mml("@0c@1d@127e");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    assert_eq!(program_change_events.len(), 3);
    assert_eq!(program_change_events[0].program, Some(0));
    assert_eq!(program_change_events[1].program, Some(1));
    assert_eq!(program_change_events[2].program, Some(127));
}

#[test]
fn test_program_change_with_octave() {
    let tokens = parse_mml("@0o4c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "program_change");
    assert_eq!(ast.notes[0].pitch, 0);
    assert_eq!(ast.notes[1].pitch, 48); // C4
}

#[test]
fn test_program_change_with_length() {
    let tokens = parse_mml("@0l8c");
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
fn test_program_change_in_multi_channel() {
    // Channel 0: @0c (program 0, note C)
    // Channel 1: @1e (program 1, note E)
    let tokens = parse_mml("@0c;@1e");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    assert_eq!(program_change_events.len(), 2);

    // Channel 0 program change
    assert_eq!(program_change_events[0].channel, 0);
    assert_eq!(program_change_events[0].program, Some(0));

    // Channel 1 program change
    assert_eq!(program_change_events[1].channel, 1);
    assert_eq!(program_change_events[1].program, Some(1));
}

#[test]
fn test_program_change_to_midi() {
    let tokens = parse_mml("@0c");
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
fn test_full_pipeline_with_program_change() {
    use std::env;
    use std::fs;

    // Create a test directory
    let test_dir = env::temp_dir().join("test_program_change");
    fs::create_dir_all(&test_dir).unwrap();

    let mml = "@0c@1d@127e";

    // Pass 1: Parse
    let tokens = parse_mml(mml);
    let pass1_json = test_dir.join("pass1_program_change.json");
    save_tokens_to_json(&tokens, pass1_json.to_str().unwrap()).unwrap();

    // Pass 2: Create AST
    let ast = tokens_to_ast(&tokens);
    let pass2_json = test_dir.join("pass2_program_change.json");
    save_ast_to_json(&ast, pass2_json.to_str().unwrap()).unwrap();

    // Pass 3: Generate MIDI events
    let events = ast_to_events(&ast);
    let pass3_json = test_dir.join("pass3_program_change.json");
    save_events_to_json(&events, pass3_json.to_str().unwrap()).unwrap();

    // Pass 4: Create MIDI file
    let midi_file = test_dir.join("output_program_change.mid");
    process_pass4(&events, midi_file.to_str().unwrap()).unwrap();

    // Verify MIDI file was created
    assert!(midi_file.exists());

    // Verify we have program change events
    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();
    assert_eq!(program_change_events.len(), 3);

    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_program_change_between_notes() {
    let tokens = parse_mml("c@5d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Find events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    // C should start at 0
    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[0].note, Some(60)); // C

    // Program change should happen at 480 (after C)
    assert_eq!(program_change_events[0].time, 480);
    assert_eq!(program_change_events[0].program, Some(5));

    // D should start at 480 (same time as program change)
    assert_eq!(note_on_events[1].time, 480);
    assert_eq!(note_on_events[1].note, Some(62)); // D
}

#[test]
fn test_program_change_with_modifier() {
    let tokens = parse_mml("@10c+");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "program_change");
    assert_eq!(ast.notes[0].pitch, 10);
    assert_eq!(ast.notes[1].pitch, 61); // C# (60 + 1)
}

#[test]
fn test_program_change_with_chord() {
    let tokens = parse_mml("@0'ceg'");
    let ast = tokens_to_ast(&tokens);

    // Should have program change + 3 chord notes
    assert_eq!(ast.notes.len(), 4);
    assert_eq!(ast.notes[0].note_type, "program_change");

    // Check that chord notes all have the same chord_id
    assert_eq!(ast.notes[1].chord_id, ast.notes[2].chord_id);
    assert_eq!(ast.notes[2].chord_id, ast.notes[3].chord_id);
}

#[test]
fn test_program_change_zero() {
    // @0 should set program to 0 (often Acoustic Grand Piano)
    let tokens = parse_mml("@0c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    assert_eq!(program_change_events[0].program, Some(0));
}

#[test]
fn test_program_change_boundaries() {
    // Test boundary values: 0 and 127
    let tokens = parse_mml("@0c@127d");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes[0].pitch, 0); // Program 0
    assert_eq!(ast.notes[2].pitch, 127); // Program 127
}
