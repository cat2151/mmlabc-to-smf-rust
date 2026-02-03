//! Unit tests for @128 drum channel support (Issue #37)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;

#[test]
fn test_parse_128_program_change() {
    let tokens = parse_mml("@128c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "program_change");
    assert_eq!(tokens[0].value, "@128");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_128_to_ast() {
    let tokens = parse_mml("@128c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 2);

    // First item is program change
    assert_eq!(ast.notes[0].note_type, "program_change");
    assert_eq!(ast.notes[0].pitch, 128); // Program 128
    assert_eq!(ast.notes[0].name, "@128");

    // Second item is the note
    assert_eq!(ast.notes[1].note_type, "note");
    assert_eq!(ast.notes[1].pitch, 60); // C5

    // AST should NOT have drum_channel_groups for single-channel mode (no semicolons)
    assert_eq!(ast.drum_channel_groups, None);
}

#[test]
fn test_128_single_channel_no_mapping() {
    // In single-channel mode (no semicolons), @128 should not trigger drum channel mapping
    let tokens = parse_mml("@128c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    // Find the program change event
    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    assert_eq!(program_change_events.len(), 1);
    // In single-channel mode, channel should remain 0 (not mapped to 9)
    assert_eq!(program_change_events[0].channel, 0);
    assert_eq!(program_change_events[0].program, Some(128));
}

#[test]
fn test_128_multi_channel_maps_to_channel_9() {
    // Channel 0: @0c (normal channel)
    // Channel 1: @128d (should map to channel 9)
    // Channel 2: @1e (normal channel)
    let tokens = parse_mml("@0c;@128d;@1e");
    let ast = tokens_to_ast(&tokens);

    // AST should identify channel group 1 (index 1) as having @128
    assert_eq!(ast.drum_channel_groups, Some(vec![1]));

    let events = ast_to_events(&ast, true);

    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    assert_eq!(program_change_events.len(), 3);

    // Channel 0 program change (normal channel)
    assert_eq!(program_change_events[0].channel, 0);
    assert_eq!(program_change_events[0].program, Some(0));

    // Channel 1 program change should be mapped to channel 9
    assert_eq!(program_change_events[1].channel, 9);
    assert_eq!(program_change_events[1].program, Some(128));

    // Channel 2 program change (normal channel)
    assert_eq!(program_change_events[2].channel, 2);
    assert_eq!(program_change_events[2].program, Some(1));

    // Check that notes are also mapped correctly
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // Note on channel 0
    assert_eq!(note_on_events[0].channel, 0);
    // Note on channel 1 (mapped to 9)
    assert_eq!(note_on_events[1].channel, 9);
    // Note on channel 2
    assert_eq!(note_on_events[2].channel, 2);
}

#[test]
fn test_128_can_be_disabled() {
    // Channel 0: @0c (normal channel)
    // Channel 1: @128d (would map to channel 9, but disabled)
    let tokens = parse_mml("@0c;@128d");
    let ast = tokens_to_ast(&tokens);

    // With use_drum_channel_for_128 = false, no mapping should occur
    let events = ast_to_events(&ast, false);

    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();

    // Channel 1 should remain as 1, not mapped to 9
    assert_eq!(program_change_events[1].channel, 1);
    assert_eq!(program_change_events[1].program, Some(128));

    // Check notes too
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // Note should remain on channel 1
    assert_eq!(note_on_events[1].channel, 1);
}

#[test]
fn test_multiple_tracks_with_128() {
    // Channel 0: @128c (drum)
    // Channel 1: @1d (normal)
    // Channel 2: @128e (drum)
    let tokens = parse_mml("@128c;@1d;@128e");
    let ast = tokens_to_ast(&tokens);

    // AST should identify channel groups 0 and 2 as having @128
    assert_eq!(ast.drum_channel_groups, Some(vec![0, 2]));

    let events = ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // Channel 0 mapped to 9
    assert_eq!(note_on_events[0].channel, 9);
    // Channel 1 stays as 1
    assert_eq!(note_on_events[1].channel, 1);
    // Channel 2 mapped to 9
    assert_eq!(note_on_events[2].channel, 9);
}

#[test]
fn test_128_with_other_commands() {
    // Test that @128 works with octave, length, velocity, etc.
    let tokens = parse_mml("@128o4l8v10cde");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    // Should have program change and 3 notes
    let program_change_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "program_change")
        .collect();
    assert_eq!(program_change_events.len(), 1);
    assert_eq!(program_change_events[0].program, Some(128));

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();
    assert_eq!(note_on_events.len(), 3);

    // Check octave o4 (C4 = 48)
    assert_eq!(note_on_events[0].note, Some(48)); // C4

    // Check velocity v10 (should be scaled appropriately)
    // v10 * 127 / 15 ≈ 84.67 → 85
    assert_eq!(note_on_events[0].velocity, Some(85));

    // Check eighth note length (240 ticks)
    let note_off_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_off")
        .collect();
    assert_eq!(note_off_events[0].time, 240); // 240 ticks for eighth note
}

#[test]
fn test_128_in_multi_channel_preserves_timing() {
    // Each channel should have independent timing
    let tokens = parse_mml("@0c4;@128d8;@1e2");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let note_off_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_off")
        .collect();

    // Channel 0: quarter note = 480 ticks
    assert_eq!(note_off_events[0].channel, 0);
    assert_eq!(note_off_events[0].time, 480);

    // Channel 1 (mapped to 9): eighth note = 240 ticks
    assert_eq!(note_off_events[1].channel, 9);
    assert_eq!(note_off_events[1].time, 240);

    // Channel 2: half note = 960 ticks
    assert_eq!(note_off_events[2].channel, 2);
    assert_eq!(note_off_events[2].time, 960);
}

#[test]
fn test_128_only_applies_to_its_track() {
    // @128 in channel 1 should not affect other channels
    let tokens = parse_mml("@0cde;@128cde;@1cde");
    let ast = tokens_to_ast(&tokens);

    // Only channel 1 should be marked as drum
    assert_eq!(ast.drum_channel_groups, Some(vec![1]));

    let events = ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // Channel 0 notes (3 notes)
    for i in 0..3 {
        assert_eq!(note_on_events[i].channel, 0);
    }

    // Channel 1 notes (3 notes, mapped to 9)
    for i in 3..6 {
        assert_eq!(note_on_events[i].channel, 9);
    }

    // Channel 2 notes (3 notes)
    for i in 6..9 {
        assert_eq!(note_on_events[i].channel, 2);
    }
}

#[test]
fn test_128_with_tempo_change() {
    // Tempo changes should also respect channel mapping
    let tokens = parse_mml("@0t120c;@128t60d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let tempo_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "tempo_set")
        .collect();

    assert_eq!(tempo_events.len(), 2);

    // Channel 0 tempo
    assert_eq!(tempo_events[0].channel, 0);

    // Channel 1 tempo (mapped to 9)
    assert_eq!(tempo_events[1].channel, 9);
}

#[test]
fn test_128_with_rest() {
    // Rests should work correctly in drum channel
    let tokens = parse_mml("@0c;@128cr8d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on" && e.channel == 9)
        .collect();

    // Should have 2 notes on channel 9 (c and d)
    assert_eq!(note_on_events.len(), 2);

    // First note at time 0
    assert_eq!(note_on_events[0].time, 0);

    // Second note after quarter note (480) + eighth rest (240) = 720
    assert_eq!(note_on_events[1].time, 720);
}
