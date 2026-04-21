use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events};

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
