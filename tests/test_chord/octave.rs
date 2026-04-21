use mmlabc_to_smf::{pass1_parser, pass2_ast};

#[test]
fn test_chord_with_internal_octave_commands_tokenized() {
    let tokens = pass1_parser::parse_mml("'c<eg'");

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[1].token_type, "octave_up");
    assert_eq!(tokens[2].token_type, "note");
    assert_eq!(tokens[3].token_type, "note");
    assert!(tokens.iter().all(|token| token.chord_id == Some(0)));
}

#[test]
fn test_chord_with_leading_and_trailing_octave_commands_tokenized() {
    let tokens = pass1_parser::parse_mml("'<c>'");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, "octave_up");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[2].token_type, "octave_down");
    assert!(tokens.iter().all(|token| token.chord_id == Some(0)));
}

#[test]
fn test_chord_with_internal_octave_commands_affect_following_chord_notes() {
    let tokens = pass1_parser::parse_mml("'c<e>g'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert!(ast.notes.iter().all(|note| note.chord_id == Some(0)));
    assert_eq!(ast.notes[0].pitch, 60); // C4 in the default chord octave
    assert_eq!(ast.notes[1].pitch, 76); // E one octave up after <
    assert_eq!(ast.notes[2].pitch, 67); // G back to the original octave after >
}

#[test]
fn test_two_note_chord_with_internal_octave_command_affects_following_note() {
    let tokens = pass1_parser::parse_mml("'c<g'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert!(ast.notes.iter().all(|note| note.chord_id == Some(0)));
    assert_eq!(ast.notes[0].pitch, 60); // C4 in the default chord octave
    assert_eq!(ast.notes[1].pitch, 79); // G one octave up after <
}

#[test]
fn test_trailing_chord_octave_command_does_not_affect_next_chord() {
    let tokens = pass1_parser::parse_mml("'c<''c'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(1));
    assert_eq!(ast.notes[0].pitch, 60); // First C in the default chord octave
    assert_eq!(ast.notes[1].pitch, ast.notes[0].pitch); // Next chord C stays in the same octave
}

#[test]
fn test_octave_only_chord_syntax_is_not_treated_as_a_chord() {
    let tokens = pass1_parser::parse_mml("'<>'");

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "octave_up");
    assert_eq!(tokens[1].token_type, "octave_down");
    assert!(tokens.iter().all(|token| token.chord_id.is_none()));
}
