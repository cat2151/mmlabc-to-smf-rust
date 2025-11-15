//! Unit tests for velocity command (v)

use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;
use mmlabc_to_smf::pass4_midi::*;

#[test]
fn test_parse_velocity_set() {
    let tokens = parse_mml("v15c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "velocity_set");
    assert_eq!(tokens[0].value, "v15");
    assert_eq!(tokens[1].token_type, "note");
    assert_eq!(tokens[1].value, "c");
}

#[test]
fn test_parse_velocity_1() {
    let tokens = parse_mml("v1c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "velocity_set");
    assert_eq!(tokens[0].value, "v1");
}

#[test]
fn test_parse_velocity_8() {
    let tokens = parse_mml("v8c");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, "velocity_set");
    assert_eq!(tokens[0].value, "v8");
}

#[test]
fn test_velocity_to_ast() {
    let tokens = parse_mml("v15c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 1);

    // Note should have velocity 127 (v15 = max)
    assert_eq!(ast.notes[0].note_type, "note");
    assert_eq!(ast.notes[0].pitch, 60); // C5
    assert_eq!(ast.notes[0].velocity, Some(127)); // v15 = 127
}

#[test]
fn test_velocity_1_to_ast() {
    let tokens = parse_mml("v1c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 1);
    // v1 should map to approximately 8 (1 * 127 / 15 = 8.47, rounded to 8)
    assert_eq!(ast.notes[0].velocity, Some(8));
}

#[test]
fn test_velocity_8_to_ast() {
    let tokens = parse_mml("v8c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 1);
    // v8 should map to approximately 68 (8 * 127 / 15 = 67.73, rounded to 68)
    assert_eq!(ast.notes[0].velocity, Some(68));
}

#[test]
fn test_velocity_to_events() {
    let tokens = parse_mml("v15c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Should have: note_on, note_off
    assert_eq!(events.len(), 2);

    // Note on at time 0 with velocity 127
    assert_eq!(events[0].event_type, "note_on");
    assert_eq!(events[0].time, 0);
    assert_eq!(events[0].note, Some(60));
    assert_eq!(events[0].velocity, Some(127)); // v15 = 127

    // Note off at time 480 (quarter note duration)
    assert_eq!(events[1].event_type, "note_off");
    assert_eq!(events[1].time, 480);
}

#[test]
fn test_velocity_1_conversion() {
    let tokens = parse_mml("v1c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 1);
    assert_eq!(note_on_events[0].velocity, Some(8)); // v1 ≈ 8
}

#[test]
fn test_velocity_affects_subsequent_notes() {
    let tokens = parse_mml("v8cde");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // All three notes should have velocity 68 (v8)
    assert_eq!(note_on_events.len(), 3);
    assert_eq!(note_on_events[0].velocity, Some(68)); // C
    assert_eq!(note_on_events[1].velocity, Some(68)); // D
    assert_eq!(note_on_events[2].velocity, Some(68)); // E
}

#[test]
fn test_multiple_velocity_changes() {
    let tokens = parse_mml("v1cv8dv15e");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);
    assert_eq!(note_on_events[0].velocity, Some(8)); // v1
    assert_eq!(note_on_events[1].velocity, Some(68)); // v8
    assert_eq!(note_on_events[2].velocity, Some(127)); // v15
}

#[test]
fn test_velocity_with_octave() {
    let tokens = parse_mml("v15o4c");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].velocity, Some(127));
    assert_eq!(ast.notes[0].pitch, 48); // C4
}

#[test]
fn test_velocity_with_length() {
    let tokens = parse_mml("v15l8c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Note should have eighth note length (240 ticks)
    let note_off_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_off")
        .collect();

    assert_eq!(note_off_events[0].time, 240); // 240 ticks for eighth note

    // And velocity should be 127
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();
    assert_eq!(note_on_events[0].velocity, Some(127));
}

#[test]
fn test_velocity_to_midi() {
    let tokens = parse_mml("v15c");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let result = events_to_midi(&events);
    assert!(result.is_ok());

    let midi_data = result.unwrap();
    assert!(!midi_data.is_empty());

    // MIDI file should start with "MThd" header
    assert_eq!(&midi_data[0..4], b"MThd");
}

#[test]
fn test_full_pipeline_with_velocity() {
    use std::env;
    use std::fs;

    // Create a test directory
    let test_dir = env::temp_dir().join("test_velocity");
    fs::create_dir_all(&test_dir).unwrap();

    let mml = "v1cv8dv15e";

    // Pass 1: Parse
    let tokens = parse_mml(mml);
    let pass1_json = test_dir.join("pass1_velocity.json");
    save_tokens_to_json(&tokens, pass1_json.to_str().unwrap()).unwrap();

    // Pass 2: Create AST
    let ast = tokens_to_ast(&tokens);
    let pass2_json = test_dir.join("pass2_velocity.json");
    save_ast_to_json(&ast, pass2_json.to_str().unwrap()).unwrap();

    // Pass 3: Generate MIDI events
    let events = ast_to_events(&ast);
    let pass3_json = test_dir.join("pass3_velocity.json");
    save_events_to_json(&events, pass3_json.to_str().unwrap()).unwrap();

    // Pass 4: Create MIDI file
    let midi_file = test_dir.join("output_velocity.mid");
    process_pass4(&events, midi_file.to_str().unwrap()).unwrap();

    // Verify MIDI file was created
    assert!(midi_file.exists());

    // Verify we have note events with different velocities
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();
    assert_eq!(note_on_events.len(), 3);

    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_velocity_between_notes() {
    let tokens = parse_mml("cv8d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    // Find events
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // C should have default velocity 127
    assert_eq!(note_on_events[0].velocity, Some(127));

    // D should have velocity 68 (v8)
    assert_eq!(note_on_events[1].velocity, Some(68));
}

#[test]
fn test_velocity_with_modifier() {
    let tokens = parse_mml("v15c+");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes.len(), 1);
    assert_eq!(ast.notes[0].velocity, Some(127));
    assert_eq!(ast.notes[0].pitch, 61); // C# (60 + 1)
}

#[test]
fn test_velocity_with_chord() {
    let tokens = parse_mml("v8'ceg'");
    let ast = tokens_to_ast(&tokens);

    // Should have 3 chord notes with the same velocity
    assert_eq!(ast.notes.len(), 3);

    // Check that all chord notes have the same velocity
    assert_eq!(ast.notes[0].velocity, Some(68)); // v8
    assert_eq!(ast.notes[1].velocity, Some(68)); // v8
    assert_eq!(ast.notes[2].velocity, Some(68)); // v8

    // Check that chord notes all have the same chord_id
    assert_eq!(ast.notes[0].chord_id, ast.notes[1].chord_id);
    assert_eq!(ast.notes[1].chord_id, ast.notes[2].chord_id);
}

#[test]
fn test_velocity_in_multi_channel() {
    // Channel 0: v8c
    // Channel 1: v15e
    let tokens = parse_mml("v8c;v15e");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast);

    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    // Should have 2 note events
    assert_eq!(note_on_events.len(), 2);

    // Channel 0 should have velocity 68 (v8)
    assert_eq!(note_on_events[0].channel, 0);
    assert_eq!(note_on_events[0].velocity, Some(68));

    // Channel 1 should have velocity 127 (v15)
    assert_eq!(note_on_events[1].channel, 1);
    assert_eq!(note_on_events[1].velocity, Some(127));
}

#[test]
fn test_velocity_value_boundaries() {
    // Test boundary values
    let tokens = parse_mml("v1cv15d");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes[0].velocity, Some(8)); // v1 ≈ 8
    assert_eq!(ast.notes[1].velocity, Some(127)); // v15 = 127
}

#[test]
fn test_default_velocity() {
    // Without velocity command, should default to 127 (v15)
    let tokens = parse_mml("cde");
    let ast = tokens_to_ast(&tokens);

    assert_eq!(ast.notes[0].velocity, Some(127));
    assert_eq!(ast.notes[1].velocity, Some(127));
    assert_eq!(ast.notes[2].velocity, Some(127));
}

#[test]
fn test_velocity_with_rest() {
    let tokens = parse_mml("v8cr");
    let ast = tokens_to_ast(&tokens);

    // Note should have velocity, rest should not
    assert_eq!(ast.notes[0].note_type, "note");
    assert_eq!(ast.notes[0].velocity, Some(68));

    assert_eq!(ast.notes[1].note_type, "rest");
    assert_eq!(ast.notes[1].velocity, None);
}

#[test]
fn test_velocity_with_program_change() {
    let tokens = parse_mml("v8@0c");
    let ast = tokens_to_ast(&tokens);

    // Should have program change (no velocity) and note (with velocity)
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "program_change");
    assert_eq!(ast.notes[0].velocity, None);

    assert_eq!(ast.notes[1].note_type, "note");
    assert_eq!(ast.notes[1].velocity, Some(68));
}

#[test]
fn test_velocity_with_tempo() {
    let tokens = parse_mml("v8t120c");
    let ast = tokens_to_ast(&tokens);

    // Should have tempo change (no velocity) and note (with velocity)
    assert_eq!(ast.notes.len(), 2);
    assert_eq!(ast.notes[0].note_type, "tempo_set");
    assert_eq!(ast.notes[0].velocity, None);

    assert_eq!(ast.notes[1].note_type, "note");
    assert_eq!(ast.notes[1].velocity, Some(68));
}

#[test]
fn test_velocity_linear_scaling() {
    // Test linear scaling formula: velocity = (v_value * 127 + 7) / 15
    let test_cases = vec![
        (1, 8),   // (1*127+7)/15 = 134/15 = 8.93 → 8
        (2, 17),  // (2*127+7)/15 = 261/15 = 17.4 → 17
        (3, 25),  // (3*127+7)/15 = 388/15 = 25.87 → 25
        (4, 34),  // (4*127+7)/15 = 515/15 = 34.33 → 34
        (5, 42),  // (5*127+7)/15 = 642/15 = 42.8 → 42
        (6, 51),  // (6*127+7)/15 = 769/15 = 51.27 → 51
        (7, 59),  // (7*127+7)/15 = 896/15 = 59.73 → 59
        (8, 68),  // (8*127+7)/15 = 1023/15 = 68.2 → 68
        (9, 76),  // (9*127+7)/15 = 1150/15 = 76.67 → 76
        (10, 85), // (10*127+7)/15 = 1277/15 = 85.13 → 85
        (11, 93), // (11*127+7)/15 = 1404/15 = 93.6 → 93
        (12, 102), // (12*127+7)/15 = 1531/15 = 102.07 → 102
        (13, 110), // (13*127+7)/15 = 1658/15 = 110.53 → 110
        (14, 119), // (14*127+7)/15 = 1785/15 = 119 → 119
        (15, 127), // (15*127+7)/15 = 1912/15 = 127.47 → 127
    ];

    for (v_value, expected_velocity) in test_cases {
        let mml = format!("v{}c", v_value);
        let tokens = parse_mml(&mml);
        let ast = tokens_to_ast(&tokens);

        assert_eq!(
            ast.notes[0].velocity,
            Some(expected_velocity),
            "v{} should map to velocity {}",
            v_value,
            expected_velocity
        );
    }
}
