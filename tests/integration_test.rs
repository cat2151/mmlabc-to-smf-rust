//! Integration tests for complete MML to SMF conversion

use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};
use std::fs;
use std::path::Path;

#[test]
fn test_full_pipeline_cde() {
    use std::env;

    let test_dir = env::temp_dir().join("test_integration");
    fs::create_dir_all(&test_dir).unwrap();

    // Pass 1
    let pass1_file = test_dir.join("pass1.json");
    let tokens = pass1_parser::process_pass1("cde", pass1_file.to_str().unwrap()).unwrap();
    assert_eq!(tokens.len(), 3);

    // Pass 2
    let pass2_file = test_dir.join("pass2.json");
    let ast = pass2_ast::process_pass2(&tokens, pass2_file.to_str().unwrap()).unwrap();
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 60); // C
    assert_eq!(ast.notes[1].pitch, 62); // D
    assert_eq!(ast.notes[2].pitch, 64); // E

    // Pass 3
    let pass3_file = test_dir.join("pass3.json");
    let events = pass3_events::process_pass3(&ast, pass3_file.to_str().unwrap(), true).unwrap();
    assert_eq!(events.len(), 6); // 3 notes * 2 events each

    // Pass 4
    let output_file = test_dir.join("output.mid");
    let midi_data = pass4_midi::process_pass4(&events, output_file.to_str().unwrap()).unwrap();
    assert!(Path::new(&output_file).exists());
    assert!(!midi_data.is_empty());

    // Verify MIDI file structure
    assert_eq!(&midi_data[0..4], b"MThd");

    // Verify all debug JSONs exist
    assert!(Path::new(&pass1_file).exists());
    assert!(Path::new(&pass2_file).exists());
    assert!(Path::new(&pass3_file).exists());

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_notes_60_62_64() {
    use std::env;

    let test_dir = env::temp_dir().join("test_notes");
    fs::create_dir_all(&test_dir).unwrap();

    let pass1_file = test_dir.join("pass1.json");
    let pass2_file = test_dir.join("pass2.json");
    let pass3_file = test_dir.join("pass3.json");

    let tokens = pass1_parser::process_pass1("cde", pass1_file.to_str().unwrap()).unwrap();
    let ast = pass2_ast::process_pass2(&tokens, pass2_file.to_str().unwrap()).unwrap();
    let events = pass3_events::process_pass3(&ast, pass3_file.to_str().unwrap(), true).unwrap();

    // Check that we have the right notes
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);
    assert_eq!(note_on_events[0].note, Some(60));
    assert_eq!(note_on_events[1].note, Some(62));
    assert_eq!(note_on_events[2].note, Some(64));

    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_output_file_option() {
    use std::env;

    let test_dir = env::temp_dir().join("test_custom_output");
    fs::create_dir_all(&test_dir).unwrap();

    let pass1_file = test_dir.join("pass1.json");
    let pass2_file = test_dir.join("pass2.json");
    let pass3_file = test_dir.join("pass3.json");

    let tokens = pass1_parser::process_pass1("c", pass1_file.to_str().unwrap()).unwrap();
    let ast = pass2_ast::process_pass2(&tokens, pass2_file.to_str().unwrap()).unwrap();
    let events = pass3_events::process_pass3(&ast, pass3_file.to_str().unwrap(), true).unwrap();

    let custom_output = test_dir.join("custom_song.mid");
    pass4_midi::process_pass4(&events, custom_output.to_str().unwrap()).unwrap();

    assert!(Path::new(&custom_output).exists());

    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}
