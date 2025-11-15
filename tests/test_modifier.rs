//! Unit tests for + and - modifiers (sharp and flat)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::types::Token;

#[test]
fn test_parse_sharp_modifier() {
    let tokens = parse_mml("c+");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[0].modifier, Some("+".to_string()));
}

#[test]
fn test_parse_flat_modifier() {
    let tokens = parse_mml("c-");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[0].modifier, Some("-".to_string()));
}

#[test]
fn test_parse_multiple_notes_with_modifiers() {
    let tokens = parse_mml("c+de-f");
    assert_eq!(tokens.len(), 4);

    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[0].modifier, Some("+".to_string()));

    assert_eq!(tokens[1].value, "d");
    assert_eq!(tokens[1].modifier, None);

    assert_eq!(tokens[2].value, "e");
    assert_eq!(tokens[2].modifier, Some("-".to_string()));

    assert_eq!(tokens[3].value, "f");
    assert_eq!(tokens[3].modifier, None);
}

#[test]
fn test_sharp_increases_pitch_by_one() {
    let tokens = vec![Token {
        token_type: "note".to_string(),
        value: "c".to_string(),
        channel_group: None,
        chord_id: None,
        modifier: Some("+".to_string()),
        note_length: None,
        dots: None,
    }];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 61); // C=60, C+=61
}

#[test]
fn test_flat_decreases_pitch_by_one() {
    let tokens = vec![Token {
        token_type: "note".to_string(),
        value: "c".to_string(),
        channel_group: None,
        chord_id: None,
        modifier: Some("-".to_string()),
        note_length: None,
        dots: None,
    }];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].pitch, 59); // C=60, C-=59
}

#[test]
fn test_modifier_affects_only_preceding_note() {
    let tokens = vec![
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
            modifier: Some("+".to_string()),
            note_length: None,
            dots: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "d".to_string(),
            channel_group: None,
            chord_id: None,
            modifier: None,
            note_length: None,
            dots: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "e".to_string(),
            channel_group: None,
            chord_id: None,
            modifier: Some("-".to_string()),
            note_length: None,
            dots: None,
        },
    ];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 61); // C+ = 61
    assert_eq!(ast.notes[1].pitch, 62); // D = 62 (unaffected)
    assert_eq!(ast.notes[2].pitch, 63); // E- = 63
}

#[test]
fn test_all_notes_with_sharp() {
    let tokens = parse_mml("c+d+e+f+g+a+b+");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 7);
    assert_eq!(ast.notes[0].pitch, 61); // C+ = 61
    assert_eq!(ast.notes[1].pitch, 63); // D+ = 63
    assert_eq!(ast.notes[2].pitch, 65); // E+ = 65
    assert_eq!(ast.notes[3].pitch, 66); // F+ = 66
    assert_eq!(ast.notes[4].pitch, 68); // G+ = 68
    assert_eq!(ast.notes[5].pitch, 70); // A+ = 70
    assert_eq!(ast.notes[6].pitch, 72); // B+ = 72
}

#[test]
fn test_all_notes_with_flat() {
    let tokens = parse_mml("c-d-e-f-g-a-b-");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 7);
    assert_eq!(ast.notes[0].pitch, 59); // C- = 59
    assert_eq!(ast.notes[1].pitch, 61); // D- = 61
    assert_eq!(ast.notes[2].pitch, 63); // E- = 63
    assert_eq!(ast.notes[3].pitch, 64); // F- = 64
    assert_eq!(ast.notes[4].pitch, 66); // G- = 66
    assert_eq!(ast.notes[5].pitch, 68); // A- = 68
    assert_eq!(ast.notes[6].pitch, 70); // B- = 70
}

#[test]
fn test_modifier_with_octave_change() {
    let tokens = parse_mml("<c+>d-");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 73); // Octave 6, C+ = 73
    assert_eq!(ast.notes[1].pitch, 61); // Octave 4, D- = 61
}

#[test]
fn test_case_insensitive_with_modifiers() {
    let tokens_upper = parse_mml("C+D-E+");
    let tokens_lower = parse_mml("c+d-e+");

    let ast_upper = tokens_to_ast(&tokens_upper);
    let ast_lower = tokens_to_ast(&tokens_lower);

    assert_eq!(ast_upper.notes.len(), ast_lower.notes.len());
    for (upper, lower) in ast_upper.notes.iter().zip(ast_lower.notes.iter()) {
        assert_eq!(upper.pitch, lower.pitch);
    }
}
