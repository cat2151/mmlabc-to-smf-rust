//! Unit tests for dotted notes (e.g., c4., d8.., e1....)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;

#[test]
fn test_parse_dotted_note() {
    let tokens = parse_mml("c4.");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[0].note_length, Some(4));
    assert_eq!(tokens[0].dots, Some(1));
}

#[test]
fn test_parse_double_dotted_note() {
    let tokens = parse_mml("c4..");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[0].note_length, Some(4));
    assert_eq!(tokens[0].dots, Some(2));
}

#[test]
fn test_parse_quadruple_dotted_note() {
    let tokens = parse_mml("c1....");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[0].note_length, Some(1));
    assert_eq!(tokens[0].dots, Some(4));
}

#[test]
fn test_dotted_quarter_note_duration() {
    // c4. = quarter note + eighth note = 480 + 240 = 720 ticks
    let tokens = parse_mml("c4.");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 720); // 480 + 240
}

#[test]
fn test_dotted_half_note_duration() {
    // c2. = half note + quarter note = 960 + 480 = 1440 ticks
    let tokens = parse_mml("c2.");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(2));
    assert_eq!(ast.notes[0].dots, Some(1));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 1440); // 960 + 480
}

#[test]
fn test_dotted_eighth_note_duration() {
    // c8. = eighth note + sixteenth note = 240 + 120 = 360 ticks
    let tokens = parse_mml("c8.");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(8));
    assert_eq!(ast.notes[0].dots, Some(1));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 360); // 240 + 120
}

#[test]
fn test_double_dotted_quarter_note_duration() {
    // c4.. = quarter + eighth + sixteenth = 480 + 240 + 120 = 840 ticks
    let tokens = parse_mml("c4..");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(2));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 840); // 480 + 240 + 120
}

#[test]
fn test_triple_dotted_quarter_note_duration() {
    // c4... = quarter + eighth + sixteenth + 32nd = 480 + 240 + 120 + 60 = 900 ticks
    let tokens = parse_mml("c4...");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(3));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 900); // 480 + 240 + 120 + 60
}

#[test]
fn test_quadruple_dotted_whole_note_duration() {
    // c1.... = whole + half + quarter + eighth + sixteenth
    // = 1920 + 960 + 480 + 240 + 120 = 3720 ticks
    // This is 2 measures (3840 ticks) minus a sixteenth note (120 ticks)
    let tokens = parse_mml("c1....");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(1));
    assert_eq!(ast.notes[0].dots, Some(4));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 3720); // 1920 + 960 + 480 + 240 + 120
}

#[test]
fn test_l_command_with_dot() {
    // l4. sets default to dotted quarter note
    let tokens = parse_mml("l4.c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 720); // Dotted quarter note
}

#[test]
fn test_l_command_with_double_dot() {
    // l4.. sets default to double-dotted quarter note
    let tokens = parse_mml("l4..c");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(2));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 840); // Double-dotted quarter note
}

#[test]
fn test_note_specific_dot_overrides_l() {
    // l4 sets default to quarter note (no dot), but c4. should be dotted
    let tokens = parse_mml("l4c4.");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));

    let events = ast_to_events(&ast, true);
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 720); // Dotted quarter note
}

#[test]
fn test_note_uses_default_dots_from_l() {
    // l4. sets default to dotted quarter, c should use it
    let tokens = parse_mml("l4.cd");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[1].length, Some(4));
    assert_eq!(ast.notes[1].dots, Some(1));

    let events = ast_to_events(&ast, true);
    // First note: 0-720
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 720);
    // Second note: 720-1440
    assert_eq!(events[2].time, 720);
    assert_eq!(events[3].time, 1440);
}

#[test]
fn test_l_resets_dots_when_no_dots() {
    // l4. sets dotted, l8 resets to no dots
    let tokens = parse_mml("l4.cl8d");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[1].length, Some(8));
    assert_eq!(ast.notes[1].dots, Some(0));

    let events = ast_to_events(&ast, true);
    // First note (c4.): 0-720
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 720);
    // Second note (d8): 720-960
    assert_eq!(events[2].time, 720);
    assert_eq!(events[3].time, 960);
}

#[test]
fn test_dotted_note_with_modifier() {
    let tokens = parse_mml("c+4.d-8.");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].pitch, 61); // C#
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[1].pitch, 61); // Db
    assert_eq!(ast.notes[1].length, Some(8));
    assert_eq!(ast.notes[1].dots, Some(1));
}

#[test]
fn test_dotted_rest() {
    let tokens = parse_mml("r4.c8");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "rest");
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[1].note_type, "note");
    assert_eq!(ast.notes[1].length, Some(8));

    let events = ast_to_events(&ast, true);
    // First rest (r4.): 0-720 (no events)
    // Note (c8): 720-960
    assert_eq!(events[0].time, 720);
    assert_eq!(events[1].time, 960);
}

#[test]
fn test_dotted_note_in_chord() {
    let tokens = parse_mml("'c4.e4.g4.'");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 3);
    assert!(ast.notes[0].chord_id.is_some());
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[1].length, Some(4));
    assert_eq!(ast.notes[1].dots, Some(1));
    assert_eq!(ast.notes[2].length, Some(4));
    assert_eq!(ast.notes[2].dots, Some(1));

    let events = ast_to_events(&ast, true);
    // All chord notes start at 0 and end at 720
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 720); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 720); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 720); // G off
}

#[test]
fn test_mixed_dotted_and_non_dotted_notes() {
    let tokens = parse_mml("c4.d8e4.f8");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 4);
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1)); // c4.
    assert_eq!(ast.notes[1].length, Some(8));
    assert_eq!(ast.notes[1].dots, Some(0)); // d8 - no dots means Some(0)
    assert_eq!(ast.notes[2].length, Some(4));
    assert_eq!(ast.notes[2].dots, Some(1)); // e4.
    assert_eq!(ast.notes[3].length, Some(8));
    assert_eq!(ast.notes[3].dots, Some(0)); // f8 - no dots means Some(0)

    let events = ast_to_events(&ast, true);
    // c4.: 0-720
    assert_eq!(events[0].time, 0);
    assert_eq!(events[1].time, 720);
    // d8: 720-960
    assert_eq!(events[2].time, 720);
    assert_eq!(events[3].time, 960);
    // e4.: 960-1680
    assert_eq!(events[4].time, 960);
    assert_eq!(events[5].time, 1680);
    // f8: 1680-1920
    assert_eq!(events[6].time, 1680);
    assert_eq!(events[7].time, 1920);
}

#[test]
fn test_dotted_notes_independent_per_channel() {
    // Channel 0: c4. (dotted quarter)
    // Channel 1: e8. (dotted eighth)
    let tokens = parse_mml("c4.;e8.");
    let ast = tokens_to_ast(&tokens);
    assert_eq!(ast.notes.len(), 2);

    // Channel 0 note
    assert_eq!(ast.notes[0].length, Some(4));
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[0].channel, Some(0));

    // Channel 1 note
    assert_eq!(ast.notes[1].length, Some(8));
    assert_eq!(ast.notes[1].dots, Some(1));
    assert_eq!(ast.notes[1].channel, Some(1));
}
