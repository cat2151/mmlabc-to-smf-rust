//! Unit tests for length command (l)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;

#[test]
fn test_parse_length_set() {
    let tokens = parse_mml("l8c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "length_set");
    assert_eq!(tokens[0].value, "l8");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_length_set_l1() {
    // l1 means whole note (1920 ticks)
    let tokens = parse_mml("l1c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(1));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 1920); // Whole note duration
}

#[test]
fn test_length_set_l4() {
    // l4 means quarter note (480 ticks)
    let tokens = parse_mml("l4c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 480); // Quarter note duration
}

#[test]
fn test_length_set_l8() {
    // l8 means eighth note (240 ticks)
    let tokens = parse_mml("l8c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(8));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 240); // Eighth note duration
}

#[test]
fn test_length_set_l16() {
    // l16 means sixteenth note (120 ticks)
    let tokens = parse_mml("l16c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(16));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 120); // Sixteenth note duration
}

#[test]
fn test_length_affects_all_following_notes() {
    let tokens = parse_mml("l8cde");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[1].length, Some(8));
    assert_eq!(ast.notes[2].length, Some(8));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 240);
    assert_eq!(events[2].time, 240);
    assert_eq!(events[3].time, 480);
    assert_eq!(events[4].time, 480);
    assert_eq!(events[5].time, 720);
}

#[test]
fn test_multiple_length_sets() {
    let tokens = parse_mml("l8cl4cl1c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(8)); // Eighth note
    assert_eq!(ast.notes[1].length, Some(4)); // Quarter note
    assert_eq!(ast.notes[2].length, Some(1)); // Whole note

    let events = ast_to_events(&ast, true);
    // First note (l8): 0-240
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 240);
    // Second note (l4): 240-720
    assert_eq!(events[2].time, 240);
    assert_eq!(events[3].time, 720);
    // Third note (l1): 720-2640
    assert_eq!(events[4].time, 720);
    assert_eq!(events[5].time, 2640);
}

#[test]
fn test_length_with_octave() {
    let tokens = parse_mml("l8o4c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 48); // C4
    assert_eq!(ast.notes[0].length, Some(8));
}

#[test]
fn test_length_with_modifier() {
    let tokens = parse_mml("l8c+");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 61); // C#
    assert_eq!(ast.notes[0].length, Some(8));
}

#[test]
fn test_length_independent_per_channel() {
    // Channel 0: l8c (eighth note C)
    // Channel 1: l4e (quarter note E)
    let tokens = parse_mml("l8c;l4e");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);

    // Channel 0 note
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[0].channel, Some(0));

    // Channel 1 note (should not be affected by channel 0's length)
    assert_eq!(ast.notes[1].length, Some(4));
    assert_eq!(ast.notes[1].channel, Some(1));
}

#[test]
fn test_default_length_is_quarter_note() {
    // Without any length command, default should be quarter note (4)
    let tokens = parse_mml("c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4)); // Default quarter note

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 480); // Quarter note duration
}

#[test]
fn test_length_with_rest() {
    let tokens = parse_mml("l8rcr");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);

    // All should have eighth note length
    assert_eq!(ast.notes[0].length, Some(8)); // Rest
    assert_eq!(ast.notes[1].length, Some(8)); // Note
    assert_eq!(ast.notes[2].length, Some(8)); // Rest

    let events = ast_to_events(&ast, true);
    // First rest: 0-240 (no events)
    // Note: 240-480
    assert_eq!(events[0].time, 240);
    assert_eq!(events[1].time, 480);
    // Second rest: 480-720 (no events)
}

#[test]
fn test_length_with_chord() {
    let tokens = parse_mml("l8'ceg'");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);

    // All chord notes should have eighth note length
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[1].length, Some(8));
    assert_eq!(ast.notes[2].length, Some(8));

    let events = ast_to_events(&ast, true);
    // All notes start at 0 and end at 240
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 240); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 240); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 240); // G off
}

#[test]
fn test_length_l2() {
    // l2 means half note (960 ticks)
    let tokens = parse_mml("l2c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(2));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 960); // Half note duration
}

#[test]
fn test_length_preservation_across_multiple_channels() {
    // Each channel maintains its own length setting
    let tokens = parse_mml("l8cd;l4ef");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 4);

    // Channel 0: eighth notes
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[1].length, Some(8));

    // Channel 1: quarter notes
    assert_eq!(ast.notes[2].length, Some(4));
    assert_eq!(ast.notes[3].length, Some(4));
}
