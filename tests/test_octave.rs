//! Unit tests for octave up/down commands

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::types::Token;

#[test]
fn test_parse_octave_up() {
    let tokens = parse_mml("c<c");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[1].token_type, "octave_up");
    assert_eq!(tokens[1].value, "<");
    assert_eq!(tokens[2].token_type, "note");
    assert_eq!(tokens[2].value, "c");
}

#[test]
fn test_parse_octave_down() {
    let tokens = parse_mml("c>c");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[1].token_type, "octave_down");
    assert_eq!(tokens[1].value, ">");
    assert_eq!(tokens[2].token_type, "note");
    assert_eq!(tokens[2].value, "c");
}

#[test]
fn test_octave_up_effect() {
    let tokens = vec![
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "octave_up".to_string(),
            value: "<".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
        },
    ];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[1].pitch, 72); // C5 (60 + 12)
}

#[test]
fn test_octave_down_effect() {
    let tokens = vec![
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "octave_down".to_string(),
            value: ">".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
        },
    ];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[1].pitch, 48); // C3 (60 - 12)
}

#[test]
fn test_octave_up_then_down() {
    let tokens = parse_mml("c<c>c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[1].pitch, 72); // C5 (after <)
    assert_eq!(ast.notes[2].pitch, 60); // C4 (after >)
}

#[test]
fn test_multiple_octave_up() {
    let tokens = parse_mml("c<<c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[1].pitch, 84); // C6 (60 + 24)
}

#[test]
fn test_multiple_octave_down() {
    let tokens = parse_mml("c>>c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[1].pitch, 36); // C2 (60 - 24)
}

#[test]
fn test_octave_affects_all_following_notes() {
    let tokens = parse_mml("c<cde");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 4);
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[1].pitch, 72); // C5 (after <)
    assert_eq!(ast.notes[2].pitch, 74); // D5 (still in higher octave)
    assert_eq!(ast.notes[3].pitch, 76); // E5 (still in higher octave)
}

#[test]
fn test_octave_with_different_notes() {
    let tokens = parse_mml("cdefgab<cdefgab>cdefgab");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 21);
    // First octave (normal)
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[6].pitch, 71); // B4
                                        // Second octave (one up)
    assert_eq!(ast.notes[7].pitch, 72); // C5
    assert_eq!(ast.notes[13].pitch, 83); // B5
                                         // Third octave (back to normal)
    assert_eq!(ast.notes[14].pitch, 60); // C4
    assert_eq!(ast.notes[20].pitch, 71); // B4
}

#[test]
fn test_octave_command_alone() {
    let tokens = parse_mml("<>");
    let ast = tokens_to_ast(&tokens);

    // Should have no notes, only octave commands
    assert_eq!(ast.notes.len(), 0);
}

#[test]
fn test_complex_octave_sequence() {
    let tokens = parse_mml("c<c<<c>>c>c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 5);
    assert_eq!(ast.notes[0].pitch, 60); // C4 (starting)
    assert_eq!(ast.notes[1].pitch, 72); // C5 (after <)
    assert_eq!(ast.notes[2].pitch, 96); // C7 (after <<, total +24)
    assert_eq!(ast.notes[3].pitch, 72); // C5 (after >>, back to +12)
    assert_eq!(ast.notes[4].pitch, 60); // C4 (after >, back to 0)
}

#[test]
fn test_octave_independent_per_channel() {
    // Channel 0: c<c (C4, C5)
    // Channel 1: e>e (E4, E3)
    let tokens = parse_mml("c<c;e>e");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 4);

    // Channel 0 notes
    assert_eq!(ast.notes[0].pitch, 60); // C4
    assert_eq!(ast.notes[0].channel, Some(0));
    assert_eq!(ast.notes[1].pitch, 72); // C5 (after < in channel 0)
    assert_eq!(ast.notes[1].channel, Some(0));

    // Channel 1 notes (should start with default octave, not affected by channel 0)
    assert_eq!(ast.notes[2].pitch, 64); // E4 (starts at default octave)
    assert_eq!(ast.notes[2].channel, Some(1));
    assert_eq!(ast.notes[3].pitch, 52); // E3 (after > in channel 1)
    assert_eq!(ast.notes[3].channel, Some(1));
}

// Tests for octave set command (o)

#[test]
fn test_parse_octave_set() {
    let tokens = parse_mml("o5c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "octave_set");
    assert_eq!(tokens[0].value, "o5");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_octave_set_o5() {
    // o5 means C = MIDI note 60
    let tokens = parse_mml("o5c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 60); // C5 (octave 5)
}

#[test]
fn test_octave_set_o4() {
    // o4 means C = MIDI note 48
    let tokens = parse_mml("o4c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 48); // C4 (octave 4)
}

#[test]
fn test_octave_set_o6() {
    // o6 means C = MIDI note 72
    let tokens = parse_mml("o6c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 72); // C6 (octave 6)
}

#[test]
fn test_octave_set_affects_all_notes() {
    let tokens = parse_mml("o4cde");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 48); // C4
    assert_eq!(ast.notes[1].pitch, 50); // D4
    assert_eq!(ast.notes[2].pitch, 52); // E4
}

#[test]
fn test_octave_set_with_octave_up() {
    let tokens = parse_mml("o4c<c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 48); // C4
    assert_eq!(ast.notes[1].pitch, 60); // C5 (after < from octave 4)
}

#[test]
fn test_octave_set_with_octave_down() {
    let tokens = parse_mml("o6c>c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 72); // C6
    assert_eq!(ast.notes[1].pitch, 60); // C5 (after > from octave 6)
}

#[test]
fn test_multiple_octave_sets() {
    let tokens = parse_mml("o4co5co6c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 48); // C4
    assert_eq!(ast.notes[1].pitch, 60); // C5
    assert_eq!(ast.notes[2].pitch, 72); // C6
}

#[test]
fn test_octave_set_independent_per_channel() {
    // Channel 0: o4c (C4)
    // Channel 1: o6e (E6)
    let tokens = parse_mml("o4c;o6e");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);

    // Channel 0 note
    assert_eq!(ast.notes[0].pitch, 48); // C4
    assert_eq!(ast.notes[0].channel, Some(0));

    // Channel 1 note (should not be affected by channel 0's octave)
    assert_eq!(ast.notes[1].pitch, 76); // E6
    assert_eq!(ast.notes[1].channel, Some(1));
}

#[test]
fn test_octave_set_with_all_notes() {
    let tokens = parse_mml("o5cdefgab");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 7);
    assert_eq!(ast.notes[0].pitch, 60); // C5
    assert_eq!(ast.notes[1].pitch, 62); // D5
    assert_eq!(ast.notes[2].pitch, 64); // E5
    assert_eq!(ast.notes[3].pitch, 65); // F5
    assert_eq!(ast.notes[4].pitch, 67); // G5
    assert_eq!(ast.notes[5].pitch, 69); // A5
    assert_eq!(ast.notes[6].pitch, 71); // B5
}

#[test]
fn test_default_octave_is_5() {
    // Without any octave command, default should be octave 5 (C=60)
    let tokens = parse_mml("c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 60); // C5 (default octave 5)
}

#[test]
fn test_octave_set_lower_octaves() {
    let tokens = parse_mml("o3co2co1c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 36); // C3
    assert_eq!(ast.notes[1].pitch, 24); // C2
    assert_eq!(ast.notes[2].pitch, 12); // C1
}

#[test]
fn test_octave_set_higher_octaves() {
    let tokens = parse_mml("o7co8c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 84); // C7
    assert_eq!(ast.notes[1].pitch, 96); // C8
}
