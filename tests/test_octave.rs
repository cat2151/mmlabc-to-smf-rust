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
        },
        Token {
            token_type: "octave_up".to_string(),
            value: "<".to_string(),
            channel_group: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
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
        },
        Token {
            token_type: "octave_down".to_string(),
            value: ">".to_string(),
            channel_group: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
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
