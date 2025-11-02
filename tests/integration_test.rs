//! Integration tests for complete MML to SMF conversion

use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};
use std::fs;
use std::path::Path;

#[test]
fn test_full_pipeline_cde() {
    let test_dir = "/tmp/test_integration";
    fs::create_dir_all(test_dir).unwrap();

    // Pass 1
    let tokens = pass1_parser::process_pass1("cde", &format!("{}/pass1.json", test_dir)).unwrap();
    assert_eq!(tokens.len(), 3);

    // Pass 2
    let ast = pass2_ast::process_pass2(&tokens, &format!("{}/pass2.json", test_dir)).unwrap();
    assert_eq!(ast.notes.len(), 3);
    assert_eq!(ast.notes[0].pitch, 60); // C
    assert_eq!(ast.notes[1].pitch, 62); // D
    assert_eq!(ast.notes[2].pitch, 64); // E

    // Pass 3
    let events = pass3_events::process_pass3(&ast, &format!("{}/pass3.json", test_dir)).unwrap();
    assert_eq!(events.len(), 6); // 3 notes * 2 events each

    // Pass 4
    let output_file = format!("{}/output.mid", test_dir);
    let midi_data = pass4_midi::process_pass4(&events, &output_file).unwrap();
    assert!(Path::new(&output_file).exists());
    assert!(!midi_data.is_empty());

    // Verify MIDI file structure
    assert_eq!(&midi_data[0..4], b"MThd");

    // Verify all debug JSONs exist
    assert!(Path::new(&format!("{}/pass1.json", test_dir)).exists());
    assert!(Path::new(&format!("{}/pass2.json", test_dir)).exists());
    assert!(Path::new(&format!("{}/pass3.json", test_dir)).exists());

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_notes_60_62_64() {
    let test_dir = "/tmp/test_notes";
    fs::create_dir_all(test_dir).unwrap();

    let tokens = pass1_parser::process_pass1("cde", &format!("{}/pass1.json", test_dir)).unwrap();
    let ast = pass2_ast::process_pass2(&tokens, &format!("{}/pass2.json", test_dir)).unwrap();
    let events = pass3_events::process_pass3(&ast, &format!("{}/pass3.json", test_dir)).unwrap();

    // Check that we have the right notes
    let note_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == "note_on")
        .collect();

    assert_eq!(note_on_events.len(), 3);
    assert_eq!(note_on_events[0].note, 60);
    assert_eq!(note_on_events[1].note, 62);
    assert_eq!(note_on_events[2].note, 64);

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_output_file_option() {
    let test_dir = "/tmp/test_custom_output";
    fs::create_dir_all(test_dir).unwrap();

    let tokens = pass1_parser::process_pass1("c", &format!("{}/pass1.json", test_dir)).unwrap();
    let ast = pass2_ast::process_pass2(&tokens, &format!("{}/pass2.json", test_dir)).unwrap();
    let events = pass3_events::process_pass3(&ast, &format!("{}/pass3.json", test_dir)).unwrap();

    let custom_output = format!("{}/custom_song.mid", test_dir);
    pass4_midi::process_pass4(&events, &custom_output).unwrap();

    assert!(Path::new(&custom_output).exists());

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}
