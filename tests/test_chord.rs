//! Tests for chord functionality (apostrophe operator)

use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};

#[test]
fn test_parse_simple_chord() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    assert_eq!(tokens.len(), 3);

    // All tokens should have the same chord_id
    assert_eq!(tokens[0].chord_id, Some(0));
    assert_eq!(tokens[1].chord_id, Some(0));
    assert_eq!(tokens[2].chord_id, Some(0));

    // All should be note tokens
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[2].token_type, "note");

    // Verify note values
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[1].value, "e");
    assert_eq!(tokens[2].value, "g");
}

#[test]
fn test_chord_to_ast() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);

    // All notes should have the same chord_id
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(0));
    assert_eq!(ast.notes[2].chord_id, Some(0));

    // All notes should be on the same channel (None = default channel 0)
    assert_eq!(ast.notes[0].channel, None);
    assert_eq!(ast.notes[1].channel, None);
    assert_eq!(ast.notes[2].channel, None);

    // Verify MIDI pitches (C major chord)
    assert_eq!(ast.notes[0].pitch, 60); // C
    assert_eq!(ast.notes[1].pitch, 64); // E
    assert_eq!(ast.notes[2].pitch, 67); // G
}

#[test]
fn test_chord_events_simultaneous() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    // Should have 6 events (3 note_on + 3 note_off)
    assert_eq!(events.len(), 6);

    // All note_on events should be at time 0 (simultaneous)
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);
    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 0);
    assert_eq!(note_on_events[2].time, 0);

    // All notes should be on the same channel (channel 0)
    assert_eq!(note_on_events[0].channel, 0);
    assert_eq!(note_on_events[1].channel, 0);
    assert_eq!(note_on_events[2].channel, 0);
}

#[test]
fn test_chord_midi_format() {
    let tokens = pass1_parser::parse_mml("'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);
    let midi_data = pass4_midi::events_to_midi(&events).unwrap();

    // MIDI file should be created successfully
    assert!(!midi_data.is_empty());
    assert_eq!(&midi_data[0..4], b"MThd");
}

#[test]
fn test_sequential_notes_then_chord() {
    let tokens = pass1_parser::parse_mml("cd'eg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 4);
    assert_eq!(ast.notes.len(), 4);

    // First two notes should not be in a chord
    assert_eq!(ast.notes[0].chord_id, None);
    assert_eq!(ast.notes[1].chord_id, None);

    // Last two notes should be in the same chord
    assert_eq!(ast.notes[2].chord_id, Some(0));
    assert_eq!(ast.notes[3].chord_id, Some(0));

    // Get note_on events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // c at time 0, d at time 240, e and g both at time 480
    assert_eq!(note_on_events[0].time, 0); // c
    assert_eq!(note_on_events[1].time, 240); // d
    assert_eq!(note_on_events[2].time, 480); // e (chord)
    assert_eq!(note_on_events[3].time, 480); // g (chord)
}

#[test]
fn test_chord_then_sequential_notes() {
    let tokens = pass1_parser::parse_mml("'ceg'de");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 5);
    assert_eq!(ast.notes.len(), 5);

    // First three notes should be in a chord
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(0));
    assert_eq!(ast.notes[2].chord_id, Some(0));

    // Last two notes should not be in a chord
    assert_eq!(ast.notes[3].chord_id, None);
    assert_eq!(ast.notes[4].chord_id, None);

    // Get note_on events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // c, e, g all at time 0 (chord), d at time 240, e at time 480
    assert_eq!(note_on_events[0].time, 0); // c (chord)
    assert_eq!(note_on_events[1].time, 0); // e (chord)
    assert_eq!(note_on_events[2].time, 0); // g (chord)
    assert_eq!(note_on_events[3].time, 240); // d (sequential)
    assert_eq!(note_on_events[4].time, 480); // e (sequential)
}

#[test]
fn test_multiple_chords() {
    let tokens = pass1_parser::parse_mml("'ce''df'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 4);
    assert_eq!(ast.notes.len(), 4);

    // First chord: c and e with chord_id 0
    assert_eq!(ast.notes[0].chord_id, Some(0));
    assert_eq!(ast.notes[1].chord_id, Some(0));

    // Second chord: d and f with chord_id 1
    assert_eq!(ast.notes[2].chord_id, Some(1));
    assert_eq!(ast.notes[3].chord_id, Some(1));

    // Get note_on events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // First chord at time 0
    assert_eq!(note_on_events[0].time, 0); // c
    assert_eq!(note_on_events[1].time, 0); // e

    // Second chord at time 240
    assert_eq!(note_on_events[2].time, 240); // d
    assert_eq!(note_on_events[3].time, 240); // f
}

#[test]
fn test_two_note_chord() {
    let tokens = pass1_parser::parse_mml("'ce'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    assert_eq!(tokens.len(), 2);
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(events.len(), 4); // 2 notes * 2 events each

    // Both notes should be at time 0 on the same channel
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 0);
    assert_eq!(note_on_events[0].channel, 0);
    assert_eq!(note_on_events[1].channel, 0);
}

#[test]
fn test_sequential_notes_without_chords_unchanged() {
    // Verify that sequential notes (without chords) still work as before
    let tokens = pass1_parser::parse_mml("cde");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    // All tokens should have chord_id None
    assert_eq!(tokens[0].chord_id, None);
    assert_eq!(tokens[1].chord_id, None);
    assert_eq!(tokens[2].chord_id, None);

    // All notes should have chord_id None
    assert_eq!(ast.notes[0].chord_id, None);
    assert_eq!(ast.notes[1].chord_id, None);
    assert_eq!(ast.notes[2].chord_id, None);

    // Notes should be sequential, not simultaneous
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events[0].time, 0);
    assert_eq!(note_on_events[1].time, 240);
    assert_eq!(note_on_events[2].time, 480);
}

#[test]
fn test_full_pipeline_chord() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let test_dir = env::temp_dir().join("test_chord");
    fs::create_dir_all(&test_dir).unwrap();

    // Pass 1
    let pass1_file = test_dir.join("pass1.json");
    let tokens = pass1_parser::process_pass1("'ceg'", pass1_file.to_str().unwrap()).unwrap();
    assert_eq!(tokens.len(), 3);

    // Pass 2
    let pass2_file = test_dir.join("pass2.json");
    let ast = pass2_ast::process_pass2(&tokens, pass2_file.to_str().unwrap()).unwrap();
    assert_eq!(ast.notes.len(), 3);

    // Pass 3
    let pass3_file = test_dir.join("pass3.json");
    let events = pass3_events::process_pass3(&ast, pass3_file.to_str().unwrap(), true).unwrap();
    assert_eq!(events.len(), 6);

    // Pass 4
    let output_file = test_dir.join("output_chord.mid");
    let midi_data = pass4_midi::process_pass4(&events, output_file.to_str().unwrap()).unwrap();
    assert!(Path::new(&output_file).exists());
    assert!(!midi_data.is_empty());

    // Verify all debug JSONs exist
    assert!(Path::new(&pass1_file).exists());
    assert!(Path::new(&pass2_file).exists());
    assert!(Path::new(&pass3_file).exists());

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_chord_case_insensitive() {
    let tokens_lower = pass1_parser::parse_mml("'ceg'");
    let tokens_upper = pass1_parser::parse_mml("'CEG'");
    let tokens_mixed = pass1_parser::parse_mml("'CeG'");

    assert_eq!(tokens_lower.len(), tokens_upper.len());
    assert_eq!(tokens_lower.len(), tokens_mixed.len());

    // All should produce the same note values (lowercase)
    for i in 0..3 {
        assert_eq!(tokens_lower[i].value, tokens_upper[i].value);
        assert_eq!(tokens_lower[i].value, tokens_mixed[i].value);
    }
}

#[test]
fn test_chord_with_octave() {
    // Test that octave commands work with chords
    let tokens = pass1_parser::parse_mml("<'ceg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);

    // All notes in the chord should be one octave higher
    assert_eq!(ast.notes[0].pitch, 72); // C5 (60 + 12)
    assert_eq!(ast.notes[1].pitch, 76); // E5 (64 + 12)
    assert_eq!(ast.notes[2].pitch, 79); // G5 (67 + 12)
}

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
fn test_octave_only_chord_syntax_is_not_treated_as_a_chord() {
    let tokens = pass1_parser::parse_mml("'<>'");

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "octave_up");
    assert_eq!(tokens[1].token_type, "octave_down");
    assert!(tokens.iter().all(|token| token.chord_id.is_none()));
}

#[test]
fn test_chord_first_note_length_propagates_within_chord() {
    // 'c8eg' – only the first note has an explicit length (8).
    // The second and third notes should inherit that length.
    let tokens = pass1_parser::parse_mml("'c8eg'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(8)); // c8 – explicit
    assert_eq!(ast.notes[1].length, Some(8)); // e – inherits from c8
    assert_eq!(ast.notes[2].length, Some(8)); // g – inherits from c8

    let events = pass3_events::ast_to_events(&ast, true);
    // All chord notes start at 0 and end at 240 (eighth-note = 240 ticks)
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 240); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 240); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 240); // G off
}

#[test]
fn test_chord_then_different_length_note() {
    // 'c8eg'd4 – chord is eighth notes, following note is a quarter note.
    // After the chord (duration 240) the next note should start at tick 240.
    let tokens = pass1_parser::parse_mml("'c8eg'd4");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 4);
    assert_eq!(note_on_events[0].time, 0); // c (chord)
    assert_eq!(note_on_events[1].time, 0); // e (chord)
    assert_eq!(note_on_events[2].time, 0); // g (chord)
    assert_eq!(note_on_events[3].time, 240); // d – starts after the chord ends
}

#[test]
fn test_chord_last_note_length_propagates_within_chord() {
    // 'ceg2.' – the length is on the LAST note; all notes should inherit it.
    // dotted half note: 1920/2 * 1.5 = 1440 ticks
    let tokens = pass1_parser::parse_mml("'ceg2.'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(2)); // c – inherits from g2.
    assert_eq!(ast.notes[1].length, Some(2)); // e – inherits from g2.
    assert_eq!(ast.notes[2].length, Some(2)); // g2. – explicit

    // All dots should be 1
    assert_eq!(ast.notes[0].dots, Some(1));
    assert_eq!(ast.notes[1].dots, Some(1));
    assert_eq!(ast.notes[2].dots, Some(1));

    let events = pass3_events::ast_to_events(&ast, true);
    // All chord notes on at 0, off at 1440 (dotted half note)
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 1440); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 1440); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 1440); // G off
}

#[test]
fn test_chord_middle_note_length_propagates_within_chord() {
    // 'ce8g' – the length is on a middle note; all notes should inherit it.
    // eighth note: 1920/8 = 240 ticks
    let tokens = pass1_parser::parse_mml("'ce8g'");
    let ast = pass2_ast::tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].length, Some(8)); // c – inherits from e8
    assert_eq!(ast.notes[1].length, Some(8)); // e8 – explicit
    assert_eq!(ast.notes[2].length, Some(8)); // g – inherits from e8

    let events = pass3_events::ast_to_events(&ast, true);
    // All chord notes on at 0, off at 240 (eighth note)
    assert_eq!(events[0].time, 0); // C on
    assert_eq!(events[1].time, 240); // C off
    assert_eq!(events[2].time, 0); // E on
    assert_eq!(events[3].time, 240); // E off
    assert_eq!(events[4].time, 0); // G on
    assert_eq!(events[5].time, 240); // G off
}

#[test]
fn test_two_chords_different_lengths() {
    // 'c8eg''d4fa' – first chord is eighth notes, second is quarter notes.
    // The second chord should start at tick 240 (end of first chord).
    let tokens = pass1_parser::parse_mml("'c8eg''d4fa'");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 6);
    // First chord at tick 0
    assert_eq!(note_on_events[0].time, 0); // c
    assert_eq!(note_on_events[1].time, 0); // e
    assert_eq!(note_on_events[2].time, 0); // g
                                           // Second chord starts after first chord (240 ticks)
    assert_eq!(note_on_events[3].time, 240); // d
    assert_eq!(note_on_events[4].time, 240); // f
    assert_eq!(note_on_events[5].time, 240); // a
}

#[test]
fn test_chord_and_channel_simultaneous() {
    // 'cg';e – C and G as a chord on channel 0, E on channel 1.
    // All three should sound simultaneously (C major chord spread across channels).
    let tokens = pass1_parser::parse_mml("'cg';e");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);

    // All three notes should start at time 0
    assert_eq!(note_on_events[0].time, 0); // c (chord, ch 0)
    assert_eq!(note_on_events[1].time, 0); // g (chord, ch 0)
    assert_eq!(note_on_events[2].time, 0); // e (ch 1)

    // C and G on channel 0, E on channel 1
    assert_eq!(note_on_events[0].channel, 0); // c
    assert_eq!(note_on_events[1].channel, 0); // g
    assert_eq!(note_on_events[2].channel, 1); // e
}
