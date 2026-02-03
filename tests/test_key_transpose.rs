//! Unit tests for key transpose command (kt)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;
use mmlabc_to_smf::types::Token;

#[test]
fn test_parse_key_transpose_positive() {
    let tokens = parse_mml("kt1c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "key_transpose");
    assert_eq!(tokens[0].value, "kt1");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_parse_key_transpose_negative() {
    let tokens = parse_mml("kt-1c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "key_transpose");
    assert_eq!(tokens[0].value, "kt-1");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_parse_key_transpose_uppercase() {
    let tokens = parse_mml("KT2c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "key_transpose");
    assert_eq!(tokens[0].value, "KT2");
}

#[test]
fn test_key_transpose_up_1() {
    // kt1 c should be note number 61 (60 + 1)
    let tokens = parse_mml("kt1c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
}

#[test]
fn test_key_transpose_down_1() {
    // kt-1 c should be note number 59 (60 - 1)
    let tokens = parse_mml("kt-1c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 59); // C5 (60) - 1 = 59
}

#[test]
fn test_key_transpose_manual_token() {
    // Test with manually constructed tokens
    let tokens = vec![
        Token {
            token_type: "key_transpose".to_string(),
            value: "kt1".to_string(),
            channel_group: None,
            chord_id: None,
            modifier: None,
            note_length: None,
            dots: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
            modifier: None,
            note_length: None,
            dots: None,
        },
    ];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
}

#[test]
fn test_key_transpose_affects_multiple_notes() {
    let tokens = parse_mml("kt2cde");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 62); // C5 (60) + 2 = 62
    assert_eq!(ast.notes[1].pitch, 64); // D5 (62) + 2 = 64
    assert_eq!(ast.notes[2].pitch, 66); // E5 (64) + 2 = 66
}

#[test]
fn test_key_transpose_negative_multiple_notes() {
    let tokens = parse_mml("kt-2cde");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 58); // C5 (60) - 2 = 58
    assert_eq!(ast.notes[1].pitch, 60); // D5 (62) - 2 = 60
    assert_eq!(ast.notes[2].pitch, 62); // E5 (64) - 2 = 62
}

#[test]
fn test_key_transpose_with_octave() {
    let tokens = parse_mml("o4kt1c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 49); // C4 (48) + 1 = 49
}

#[test]
fn test_key_transpose_with_modifier() {
    // kt1 c+ should be 60 + 1 (kt) + 1 (modifier) = 62
    let tokens = parse_mml("kt1c+");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 62); // C5 (60) + 1 (kt) + 1 (sharp) = 62
}

#[test]
fn test_key_transpose_with_flat_modifier() {
    // kt1 d- should be 62 + 1 (kt) - 1 (modifier) = 62
    let tokens = parse_mml("kt1d-");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 62); // D5 (62) + 1 (kt) - 1 (flat) = 62
}

#[test]
fn test_multiple_key_transpose_changes() {
    let tokens = parse_mml("kt1ckt2dkt-1e");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
    assert_eq!(ast.notes[1].pitch, 64); // D5 (62) + 2 = 64
    assert_eq!(ast.notes[2].pitch, 63); // E5 (64) - 1 = 63
}

#[test]
fn test_key_transpose_large_value() {
    let tokens = parse_mml("kt12c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 72); // C5 (60) + 12 = 72 (one octave up)
}

#[test]
fn test_key_transpose_large_negative_value() {
    let tokens = parse_mml("kt-12c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 48); // C5 (60) - 12 = 48 (one octave down)
}

#[test]
fn test_key_transpose_independent_per_channel() {
    // Channel 0: kt1c (C5 + 1 = 61)
    // Channel 1: kt-1e (E5 - 1 = 63)
    let tokens = parse_mml("kt1c;kt-1e");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);

    // Channel 0 note
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
    assert_eq!(ast.notes[0].channel, Some(0));

    // Channel 1 note (should not be affected by channel 0's transpose)
    assert_eq!(ast.notes[1].pitch, 63); // E5 (64) - 1 = 63
    assert_eq!(ast.notes[1].channel, Some(1));
}

#[test]
fn test_key_transpose_with_chord() {
    let tokens = parse_mml("kt2'ceg'");
    let ast = tokens_to_ast(&tokens);

    // Should have 3 chord notes, all transposed
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 62); // C5 (60) + 2 = 62
    assert_eq!(ast.notes[1].pitch, 66); // E5 (64) + 2 = 66
    assert_eq!(ast.notes[2].pitch, 69); // G5 (67) + 2 = 69

    // Check that all notes have the same chord_id
    assert_eq!(ast.notes[0].chord_id, ast.notes[1].chord_id);
    assert_eq!(ast.notes[1].chord_id, ast.notes[2].chord_id);
}

#[test]
fn test_key_transpose_with_rest() {
    let tokens = parse_mml("kt1cr");
    let ast = tokens_to_ast(&tokens);

    // Should have a note and a rest
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "note");
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
    assert_eq!(ast.notes[1].note_type, "rest");
}

#[test]
fn test_key_transpose_zero() {
    // kt0 should have no effect
    let tokens = parse_mml("kt0c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 60); // C5 (no transpose)
}

#[test]
fn test_default_no_transpose() {
    // Without kt command, transpose should be 0
    let tokens = parse_mml("cde");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 60); // C5
    assert_eq!(ast.notes[1].pitch, 62); // D5
    assert_eq!(ast.notes[2].pitch, 64); // E5
}

#[test]
fn test_key_transpose_with_octave_up() {
    let tokens = parse_mml("kt1c<c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
    assert_eq!(ast.notes[1].pitch, 73); // C6 (72) + 1 = 73
}

#[test]
fn test_key_transpose_with_octave_down() {
    let tokens = parse_mml("kt1c>c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
    assert_eq!(ast.notes[1].pitch, 49); // C4 (48) + 1 = 49
}

#[test]
fn test_key_transpose_with_velocity() {
    let tokens = parse_mml("kt1v8c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
    assert_eq!(ast.notes[0].velocity, Some(68)); // v8 = 68
}

#[test]
fn test_key_transpose_to_events() {
    let tokens = parse_mml("kt1c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Should have: note_on, note_off
    assert_eq!(events.len(), 2);

    // Note on at time 0 with note 61
    assert_eq!(events[0].event_type, "note_on");
    assert_eq!(events[0].time, 0);
    assert_eq!(events[0].note, Some(61)); // C5 + 1

    // Note off at time 480 (quarter note duration)
    assert_eq!(events[1].event_type, "note_off");
    assert_eq!(events[1].time, 480);
    assert_eq!(events[1].note, Some(61)); // C5 + 1
}

#[test]
fn test_key_transpose_with_length() {
    let tokens = parse_mml("kt1l8c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Note off should be at 240 ticks (eighth note)
    let note_off_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_off")
        .collect();

    assert_eq!(note_off_events[0].time, 240); // 240 ticks for eighth note
    assert_eq!(note_off_events[0].note, Some(61)); // C5 + 1
}

#[test]
fn test_key_transpose_complex_sequence() {
    let tokens = parse_mml("o4kt2c<kt-1d>kt0e");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);

    // First note: o4 (C4=48) + kt2 = 50
    assert_eq!(ast.notes[0].pitch, 50);

    // Second note: < makes it C5 (60) + kt-1 = 61... wait, D not C
    // After <, we're in octave 5, D5 = 62, kt-1 = 61
    assert_eq!(ast.notes[1].pitch, 61);

    // Third note: > makes it back to octave 4, E4 = 52, kt0 = 52
    assert_eq!(ast.notes[2].pitch, 52);
}

#[test]
fn test_key_transpose_saturation_bounds() {
    // Test that transpose doesn't overflow/underflow MIDI note range
    let tokens = parse_mml("o8kt10c"); // C8 (96) + 10 = 106
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    // Should saturate at 127 or handle gracefully
    assert!(ast.notes[0].pitch <= 127);
}

#[test]
fn test_uppercase_kt() {
    let tokens = parse_mml("KT1c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 61); // C5 (60) + 1 = 61
}
