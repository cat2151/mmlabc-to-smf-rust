//! Tests for mapping text positions to sounding units.

use mmlabc_to_smf::sounding_units::{
    sounding_units, unit_at, unit_covering, units_in, SoundingUnit, UnitKind,
};
use mmlabc_to_smf::{pass1_parser, pass2_ast};

fn shapes(mml: &str) -> Vec<(&str, UnitKind)> {
    sounding_units(mml)
        .into_iter()
        .map(|unit| (&mml[unit.byte_range], unit.kind))
        .collect()
}

/// Pitches of the AST notes a unit points at.
fn pitches(mml: &str, unit: &SoundingUnit) -> Vec<u8> {
    let ast = pass2_ast::tokens_to_ast(&pass1_parser::parse_mml(mml));
    unit.note_indices
        .iter()
        .map(|index| ast.notes[*index].pitch)
        .collect()
}

// --- splitting ---

#[test]
fn splits_a_line_into_one_unit_per_musical_event() {
    let mml = "o5 l8 'ceg' r4 <d+4. c";
    assert_eq!(
        shapes(mml),
        vec![
            ("o5", UnitKind::Command),
            ("l8", UnitKind::Command),
            ("'ceg'", UnitKind::Chord),
            ("r4", UnitKind::Rest),
            ("<", UnitKind::Command),
            ("d+4.", UnitKind::Note),
            ("c", UnitKind::Note),
        ]
    );
}

#[test]
fn a_note_unit_covers_its_modifier_length_and_dots() {
    let units = sounding_units("d+4.");
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].byte_range, 0..4);
}

#[test]
fn only_notes_and_chords_are_sounding() {
    assert!(UnitKind::Note.is_sounding());
    assert!(UnitKind::Chord.is_sounding());
    assert!(!UnitKind::Rest.is_sounding());
    assert!(!UnitKind::Command.is_sounding());
}

#[test]
fn whitespace_belongs_to_no_unit() {
    let units = sounding_units("c d");
    assert_eq!(unit_covering(&units, 1), None);
}

#[test]
fn text_with_no_musical_event_has_no_units() {
    assert!(sounding_units("").is_empty());
    assert!(sounding_units("   ").is_empty());
}

// --- chords ---

#[test]
fn a_chord_is_one_unit_covering_both_quotes() {
    let units = sounding_units("'ceg'");
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].byte_range, 0..5);
    assert_eq!(units[0].kind, UnitKind::Chord);
    assert!(!units[0].incomplete);
}

#[test]
fn the_cursor_on_a_closing_quote_is_still_in_the_chord() {
    let units = sounding_units("'ceg'");
    // byte 4 is the closing quote, byte 5 is just past it.
    assert_eq!(unit_at(&units, 4), Some(0));
    assert_eq!(unit_at(&units, 5), Some(0));
}

#[test]
fn an_unfinished_chord_still_sounds_and_is_marked_incomplete() {
    let units = sounding_units("'ceg");
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].byte_range, 0..4);
    assert!(units[0].incomplete);
    assert_eq!(pitches("'ceg", &units[0]), vec![60, 64, 67]);
}

#[test]
fn closing_a_chord_changes_the_unit_even_though_the_notes_do_not() {
    // Cursor at the end of each: the notes are identical, so only the range
    // tells the two apart.
    let open = sounding_units("'ceg");
    let closed = sounding_units("'ceg'");
    assert_eq!(pitches("'ceg", &open[0]), pitches("'ceg'", &closed[0]));
    assert_ne!(open[0].byte_range, closed[0].byte_range);
}

#[test]
fn octave_shifts_inside_a_chord_stay_in_the_chords_unit() {
    let units = sounding_units("'c<e g'");
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].byte_range, 0..7);
    assert_eq!(units[0].kind, UnitKind::Chord);
}

// --- ast mapping ---

#[test]
fn a_chord_points_at_every_note_it_produced() {
    let mml = "'ceg'";
    let units = sounding_units(mml);
    assert_eq!(pitches(mml, &units[0]), vec![60, 64, 67]);
}

#[test]
fn a_rest_is_its_own_unit_and_points_at_its_ast_entry() {
    let mml = "cr";
    let units = sounding_units(mml);
    assert_eq!(
        shapes(mml),
        vec![("c", UnitKind::Note), ("r", UnitKind::Rest)]
    );

    let ast = pass2_ast::tokens_to_ast(&pass1_parser::parse_mml(mml));
    assert_eq!(units[1].note_indices, vec![1]);
    assert_eq!(ast.notes[1].note_type, "rest");
}

#[test]
fn commands_that_do_not_sound_point_at_no_notes() {
    let units = sounding_units("o5 l8 <");
    assert!(units.iter().all(|unit| unit.note_indices.is_empty()));
}

#[test]
fn channel_groups_keep_source_order() {
    let mml = "c;e;g";
    let units = sounding_units(mml);
    assert_eq!(
        units
            .iter()
            .map(|unit| unit.note_indices.clone())
            .collect::<Vec<_>>(),
        vec![vec![0], vec![1], vec![2]]
    );

    let ast = pass2_ast::tokens_to_ast(&pass1_parser::parse_mml(mml));
    let channels: Vec<_> = units
        .iter()
        .map(|unit| ast.notes[unit.note_indices[0]].channel)
        .collect();
    assert_eq!(channels, vec![Some(0), Some(1), Some(2)]);
}

#[test]
fn program_changes_between_channel_groups_stay_matched() {
    let mml = "@1cd;@128e";
    let units = sounding_units(mml);
    assert_eq!(
        shapes(mml),
        vec![
            ("@1", UnitKind::Command),
            ("c", UnitKind::Note),
            ("d", UnitKind::Note),
            ("@128", UnitKind::Command),
            ("e", UnitKind::Note),
        ]
    );

    let ast = pass2_ast::tokens_to_ast(&pass1_parser::parse_mml(mml));
    let names: Vec<_> = units
        .iter()
        .flat_map(|unit| unit.note_indices.iter())
        .map(|index| ast.notes[*index].name.as_str())
        .collect();
    assert_eq!(names, vec!["@1", "c", "d", "@128", "e"]);
}

// --- cursor lookup ---

#[test]
fn the_cursor_reports_the_unit_it_just_left() {
    let units = sounding_units("cde");
    assert_eq!(unit_at(&units, 1), Some(0));
    assert_eq!(unit_at(&units, 2), Some(1));
    assert_eq!(unit_at(&units, 3), Some(2));
}

#[test]
fn the_cursor_takes_the_unit_on_its_right_when_nothing_is_on_its_left() {
    let units = sounding_units("cde");
    assert_eq!(unit_at(&units, 0), Some(0));

    // ...but only when it actually touches one.
    let padded = sounding_units("  cde");
    assert_eq!(unit_at(&padded, 0), None);
    assert_eq!(unit_at(&padded, 1), None);
    assert_eq!(unit_at(&padded, 2), Some(0));
}

#[test]
fn the_cursor_past_the_end_of_the_text_reports_nothing() {
    let units = sounding_units("c ");
    assert_eq!(unit_at(&units, 2), None);
}

#[test]
fn unit_covering_ignores_the_cursor_rule() {
    let units = sounding_units("cde");
    assert_eq!(unit_covering(&units, 0), Some(0));
    assert_eq!(unit_covering(&units, 1), Some(1));
    assert_eq!(unit_covering(&units, 3), None);
}

// --- selections ---

#[test]
fn a_selection_takes_every_unit_it_touches() {
    let mml = "c 'deg' r4";
    let units = sounding_units(mml);
    // Selecting from inside the chord to inside the rest.
    assert_eq!(units_in(&units, 4..9), vec![1, 2]);
    assert_eq!(units_in(&units, 0..mml.len()), vec![0, 1, 2]);
}

#[test]
fn an_empty_selection_takes_nothing() {
    let units = sounding_units("cde");
    assert!(units_in(&units, 1..1).is_empty());
}

// --- embedded attachment json ---

#[test]
fn ranges_stay_relative_to_the_text_that_was_passed_in() {
    let mml = r#"[{"ProgramChange":1}]@1cde"#;
    let units = sounding_units(mml);
    assert_eq!(
        shapes(mml),
        vec![
            ("@1", UnitKind::Command),
            ("c", UnitKind::Note),
            ("d", UnitKind::Note),
            ("e", UnitKind::Note),
        ]
    );
    assert_eq!(units[0].byte_range.start, 21);
}

// --- input being typed ---

#[test]
fn half_written_mml_does_not_panic() {
    for mml in ["'ce", "c+", "o", "kt-", "l", "@", "'", ";", ">>>", "xyz"] {
        let units = sounding_units(mml);
        for byte in 0..=mml.len() {
            unit_at(&units, byte);
            unit_covering(&units, byte);
        }
        units_in(&units, 0..mml.len());
    }
}

#[test]
fn a_note_being_lengthened_stays_one_growing_unit() {
    for (mml, end) in [("c", 1), ("c4", 2), ("c4.", 3)] {
        let units = sounding_units(mml);
        assert_eq!(units.len(), 1, "{mml}");
        assert_eq!(units[0].byte_range, 0..end, "{mml}");
        assert_eq!(units[0].kind, UnitKind::Note, "{mml}");
    }
}
