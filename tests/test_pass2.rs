//! Unit tests for Pass 2: AST Generation

use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::types::Token;

#[test]
fn test_tokens_to_ast() {
    let tokens = vec![
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "d".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "e".to_string(),
            channel_group: None,
            chord_id: None,
        },
    ];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.ast_type, "sequence");
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 60); // C
    assert_eq!(ast.notes[1].pitch, 62); // D
    assert_eq!(ast.notes[2].pitch, 64); // E
}

#[test]
fn test_note_to_midi_mapping() {
    let tokens = vec![
        Token {
            token_type: "note".to_string(),
            value: "c".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "d".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "e".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "f".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "g".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "a".to_string(),
            channel_group: None,
            chord_id: None,
        },
        Token {
            token_type: "note".to_string(),
            value: "b".to_string(),
            channel_group: None,
            chord_id: None,
        },
    ];

    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes[0].pitch, 60); // C
    assert_eq!(ast.notes[1].pitch, 62); // D
    assert_eq!(ast.notes[2].pitch, 64); // E
    assert_eq!(ast.notes[3].pitch, 65); // F
    assert_eq!(ast.notes[4].pitch, 67); // G
    assert_eq!(ast.notes[5].pitch, 69); // A
    assert_eq!(ast.notes[6].pitch, 71); // B
}

#[test]
fn test_empty_tokens() {
    let tokens = vec![];
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 0);
}

#[test]
fn test_save_ast_to_json() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let tokens = vec![Token {
        token_type: "note".to_string(),
        value: "c".to_string(),
        channel_group: None,
        chord_id: None,
    }];
    let ast = tokens_to_ast(&tokens);
    let filepath = env::temp_dir().join("test_pass2_ast.json");

    let result = save_ast_to_json(&ast, filepath.to_str().unwrap());
    assert!(result.is_ok());
    assert!(Path::new(&filepath).exists());

    // Clean up
    let _ = fs::remove_file(filepath);
}
