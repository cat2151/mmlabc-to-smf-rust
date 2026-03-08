//! Unit tests for attachment JSON generation

use mmlabc_to_smf::attachment_json::*;
use mmlabc_to_smf::pass1_parser::parse_mml;
use mmlabc_to_smf::pass2_ast::tokens_to_ast;
use mmlabc_to_smf::pass3_events::ast_to_events;

#[test]
fn test_attachment_json_no_program_change() {
    let tokens = parse_mml("cde");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let entries = generate_attachment_entries(&events);

    // With no @N command, should default to program 0
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].program_change, 0);
    assert!(entries[0].tone.events.is_empty());
}

#[test]
fn test_attachment_json_single_program_change() {
    let tokens = parse_mml("@1cde");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let entries = generate_attachment_entries(&events);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].program_change, 1);
    assert!(entries[0].tone.events.is_empty());
}

#[test]
fn test_attachment_json_multiple_program_changes() {
    let tokens = parse_mml("@1c@3d@2e");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let entries = generate_attachment_entries(&events);

    // Should have 3 unique entries, sorted by program number
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].program_change, 1);
    assert_eq!(entries[1].program_change, 2);
    assert_eq!(entries[2].program_change, 3);
}

#[test]
fn test_attachment_json_repeated_program_change_deduplicated() {
    let tokens = parse_mml("@0c@0d");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let entries = generate_attachment_entries(&events);

    // @0 appears twice but should be deduplicated to one entry
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].program_change, 0);
}

#[test]
fn test_generate_attachment_json_valid_json() {
    let tokens = parse_mml("@2cde");
    let ast = tokens_to_ast(&tokens);
    let events = ast_to_events(&ast, true);

    let json = generate_attachment_json(&events).unwrap();

    // Must be valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Must be an array
    assert!(parsed.is_array());
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);

    // Must have ProgramChange and Tone fields
    assert_eq!(arr[0]["ProgramChange"], 2);
    assert!(arr[0]["Tone"].is_object());
    assert!(arr[0]["Tone"]["events"].is_array());
}
