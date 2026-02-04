//! Unit tests for note-specific lengths (e.g., c8, d4, e2)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;

#[test]
fn test_parse_note_with_length() {
    let tokens = parse_mml("c8");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[0].note_length, Some(8));
}

#[test]
fn test_note_specific_length_overrides_l() {
    // l4 sets default to quarter note, but c8 should be eighth note
    let tokens = parse_mml("l4c8");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(8));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 240); // Eighth note duration
}

#[test]
fn test_note_uses_default_length_when_no_specific_length() {
    // l8 sets default, c should use it
    let tokens = parse_mml("l8c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(8));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 240); // Eighth note duration
}

#[test]
fn test_multiple_notes_with_different_lengths() {
    let tokens = parse_mml("c4d8e2");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(4)); // Quarter note
    assert_eq!(ast.notes[1].length, Some(8)); // Eighth note
    assert_eq!(ast.notes[2].length, Some(2)); // Half note

    let events = ast_to_events(&ast, true);
    // First note (c4): 0-480
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 480);
    // Second note (d8): 480-720
    assert_eq!(events[2].time, 480);
    assert_eq!(events[3].time, 720);
    // Third note (e2): 720-1680
    assert_eq!(events[4].time, 720);
    assert_eq!(events[5].time, 1680);
}

#[test]
fn test_note_specific_length_with_l_command() {
    // l4 sets default to quarter, c8 uses eighth, d uses default quarter
    let tokens = parse_mml("l4c8d");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].length, Some(8)); // c8
    assert_eq!(ast.notes[1].length, Some(4)); // d uses l4

    let events = ast_to_events(&ast, true);
    // First note (c8): 0-240
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 240);
    // Second note (d4): 240-720
    assert_eq!(events[2].time, 240);
    assert_eq!(events[3].time, 720);
}

#[test]
fn test_whole_note_specific_length() {
    let tokens = parse_mml("c1");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(1));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 1920); // Whole note duration
}

#[test]
fn test_sixteenth_note_specific_length() {
    let tokens = parse_mml("c16");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(16));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 120); // Sixteenth note duration
}

#[test]
fn test_note_length_with_modifier() {
    let tokens = parse_mml("c+8d-4");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 61); // C#
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[1].pitch, 61); // Db
    assert_eq!(ast.notes[1].length, Some(4));
}

#[test]
fn test_note_length_with_octave() {
    let tokens = parse_mml("o5c8o4d4");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 60); // C5
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[1].pitch, 50); // D4
    assert_eq!(ast.notes[1].length, Some(4));
}

#[test]
fn test_rest_with_specific_length() {
    let tokens = parse_mml("r8c4r2");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].note_type, "rest");
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[1].note_type, "note");
    assert_eq!(ast.notes[1].length, Some(4));
    assert_eq!(ast.notes[2].note_type, "rest");
    assert_eq!(ast.notes[2].length, Some(2));

    let events = ast_to_events(&ast, true);
    // First rest (r8): 0-240 (no events)
    // Note (c4): 240-720
    assert_eq!(events[0].time, 240);
    assert_eq!(events[1].time, 720);
    // Second rest (r2): 720-1680 (no events)
}

#[test]
fn test_note_length_in_chord() {
    let tokens = parse_mml("'c8e8g8'");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert!(ast.notes[0].chord_id.is_some());
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[1].length, Some(8));
    assert_eq!(ast.notes[2].length, Some(8));

    let events = ast_to_events(&ast, true);
    // All chord notes start at 0 and end at 240
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 240); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 240); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 240); // G off
}

#[test]
fn test_note_length_independent_per_channel() {
    // Channel 0: c8 (eighth note)
    // Channel 1: e4 (quarter note)
    let tokens = parse_mml("c8;e4");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);

    // Channel 0 note
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[0].channel, Some(0));

    // Channel 1 note
    assert_eq!(ast.notes[1].length, Some(4));
    assert_eq!(ast.notes[1].channel, Some(1));
}
