//! WASM bindings for mmlabc-to-smf
//!
//! This crate provides WebAssembly bindings to enable browser-based MML to SMF conversion.
//! It implements Option A: receiving parse tree JSON from web-tree-sitter and converting
//! to SMF binary, achieving SSOT (Single Source of Truth) by keeping all token extraction
//! logic in Rust.

mod token_extractor;

pub use token_extractor::{parse_tree_to_tokens, ParseTreeNode, Position};

use mmlabc_to_smf::{attachment_json, mml_preprocessor, pass2_ast, pass3_events, pass4_midi};
use wasm_bindgen::prelude::*;

/// Convert MML parse tree JSON to SMF binary (WASM entry point)
///
/// This is the main WASM function that takes a parse tree JSON from
/// web-tree-sitter and returns Standard MIDI File binary data.
///
/// The function now supports multi-channel MML with semicolons through the
/// tree-sitter grammar. The parse tree can contain either:
/// - Direct items (for single-channel MML like "cde")
/// - A channel_groups node (for multi-channel MML like "c;e;g")
///
/// # Arguments
/// * `parse_tree_json` - JSON string representing the parse tree from web-tree-sitter
/// * `mml_source` - Original MML source text (reserved for diagnostics or future features)
///
/// # Returns
/// SMF binary data as Uint8Array
#[wasm_bindgen]
pub fn parse_tree_json_to_smf(parse_tree_json: &str, _mml_source: &str) -> Result<Vec<u8>, JsValue> {
    // Parse the parse tree JSON
    let parse_tree: ParseTreeNode = serde_json::from_str(parse_tree_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    // Extract tokens from parse tree (no channel group for single MML string)
    let mut chord_id = 0;
    let tokens = parse_tree_to_tokens(&parse_tree, None, &mut chord_id);

    // Pass 2: Convert tokens to AST
    let ast = pass2_ast::tokens_to_ast(&tokens);

    // Pass 3: Generate MIDI events
    let events = pass3_events::ast_to_events(&ast, true);

    // Pass 4: Create MIDI file
    let smf_data = pass4_midi::events_to_midi(&events)
        .map_err(|e| JsValue::from_str(&format!("Failed to create MIDI: {}", e)))?;

    Ok(smf_data)
}

/// Convert MML parse tree JSON to attachment JSON string (WASM entry point)
///
/// Generates an "attached JSON" (添付JSON) that describes per-ProgramChange settings
/// (Tone, Portamento, LFO, etc.) separately from the SMF file.
/// The format is compatible with smf-to-ym2151log-rust's attachment input.
///
/// # Arguments
/// * `parse_tree_json` - JSON string representing the parse tree from web-tree-sitter
/// * `mml_source` - Original MML source text (reserved for diagnostics or future features)
///
/// # Returns
/// Attachment JSON string
#[wasm_bindgen]
pub fn parse_tree_json_to_attachment_json(parse_tree_json: &str, _mml_source: &str) -> Result<String, JsValue> {
    // Parse the parse tree JSON
    let parse_tree: ParseTreeNode = serde_json::from_str(parse_tree_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    // Extract tokens from parse tree
    let mut chord_id = 0;
    let tokens = parse_tree_to_tokens(&parse_tree, None, &mut chord_id);

    // Pass 2: Convert tokens to AST
    let ast = pass2_ast::tokens_to_ast(&tokens);

    // Pass 3: Generate MIDI events
    let events = pass3_events::ast_to_events(&ast, true);

    // Generate attachment JSON
    let json = attachment_json::generate_attachment_json(&events)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate attachment JSON: {}", e)))?;

    Ok(json)
}

/// Extract a leading JSON block from MML text (WASM entry point)
///
/// Implements the JSON-in-MML provisional spec: if the MML string starts with a
/// JSON object (`{…}`) or array (`[…]`), that block is returned as the embedded
/// JSON (intended as attachment JSON / 添付JSON), and the remainder is the actual
/// MML to be parsed.
///
/// Returns a JSON-encoded object with two fields:
/// - `"embeddedJson"`: `null` or the extracted JSON string
/// - `"remainingMml"`: the MML text after stripping the JSON prefix
///
/// # Arguments
/// * `mml` - Full MML input string, possibly starting with a JSON block
///
/// # Returns
/// JSON string: `{"embeddedJson": <null|string>, "remainingMml": <string>}`
#[wasm_bindgen]
pub fn preprocess_mml(mml: &str) -> Result<String, JsValue> {
    let result = mml_preprocessor::extract_embedded_json(mml);
    let output = serde_json::json!({
        "embeddedJson": result.embedded_json,
        "remainingMml": result.remaining_mml,
    });
    serde_json::to_string(&output)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tree_to_tokens_simple_notes() {
        let parse_tree = ParseTreeNode {
            node_type: "source_file".to_string(),
            start_position: None,
            end_position: None,
            text: None,
            children: Some(vec![
                ParseTreeNode {
                    node_type: "note_with_modifier".to_string(),
                    start_position: None,
                    end_position: None,
                    text: None,
                    children: Some(vec![ParseTreeNode {
                        node_type: "note".to_string(),
                        start_position: None,
                        end_position: None,
                        text: Some("c".to_string()),
                        children: None,
                    }]),
                },
                ParseTreeNode {
                    node_type: "note_with_modifier".to_string(),
                    start_position: None,
                    end_position: None,
                    text: None,
                    children: Some(vec![ParseTreeNode {
                        node_type: "note".to_string(),
                        start_position: None,
                        end_position: None,
                        text: Some("d".to_string()),
                        children: None,
                    }]),
                },
            ]),
        };

        let mut chord_id = 0;
        let tokens = parse_tree_to_tokens(&parse_tree, None, &mut chord_id);

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, "note");
        assert_eq!(tokens[0].value, "c");
        assert_eq!(tokens[1].token_type, "note");
        assert_eq!(tokens[1].value, "d");
    }

    #[test]
    fn test_parse_tree_json_deserialization_and_token_extraction() {
        // Test the JSON parsing and token extraction without wasm_bindgen
        let parse_tree_json = r#"{
            "type": "source_file",
            "children": [
                {
                    "type": "note_with_modifier",
                    "children": [
                        {"type": "note", "text": "c"}
                    ]
                },
                {
                    "type": "note_with_modifier",
                    "children": [
                        {"type": "note", "text": "d"}
                    ]
                },
                {
                    "type": "note_with_modifier",
                    "children": [
                        {"type": "note", "text": "e"}
                    ]
                }
            ]
        }"#;

        // Parse the JSON
        let parse_tree: ParseTreeNode = serde_json::from_str(parse_tree_json).unwrap();
        
        // Extract tokens
        let mut chord_id = 0;
        let tokens = parse_tree_to_tokens(&parse_tree, None, &mut chord_id);
        
        // Verify tokens
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, "note");
        assert_eq!(tokens[0].value, "c");
        assert_eq!(tokens[1].token_type, "note");
        assert_eq!(tokens[1].value, "d");
        assert_eq!(tokens[2].token_type, "note");
        assert_eq!(tokens[2].value, "e");
    }

    #[test]
    fn test_parse_tree_json_with_commands() {
        // Test parsing JSON with tempo, program, and velocity commands
        let parse_tree_json = r#"{
            "type": "source_file",
            "children": [
                {"type": "tempo_set", "text": "t120"},
                {"type": "program_change", "text": "@1"},
                {"type": "velocity_set", "text": "v100"},
                {
                    "type": "note_with_modifier",
                    "children": [
                        {"type": "note", "text": "c"}
                    ]
                }
            ]
        }"#;

        let parse_tree: ParseTreeNode = serde_json::from_str(parse_tree_json).unwrap();
        let mut chord_id = 0;
        let tokens = parse_tree_to_tokens(&parse_tree, None, &mut chord_id);
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, "tempo_set");
        assert_eq!(tokens[0].value, "t120");
        assert_eq!(tokens[1].token_type, "program_change");
        assert_eq!(tokens[1].value, "@1");
        assert_eq!(tokens[2].token_type, "velocity_set");
        assert_eq!(tokens[2].value, "v100");
        assert_eq!(tokens[3].token_type, "note");
        assert_eq!(tokens[3].value, "c");
    }

    #[test]
    fn test_invalid_json_parsing() {
        let invalid_json = "not valid json";
        
        let result: Result<ParseTreeNode, _> = serde_json::from_str(invalid_json);
        
        // Should fail to parse
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tree_with_channel_groups() {
        // Test parsing JSON with channel_groups (semicolons in MML)
        let parse_tree_json = r#"{
            "type": "source_file",
            "children": [
                {
                    "type": "channel_groups",
                    "children": [
                        {
                            "type": "channel_group",
                            "children": [
                                {
                                    "type": "note_with_modifier",
                                    "children": [
                                        {"type": "note", "text": "c"}
                                    ]
                                }
                            ]
                        },
                        {
                            "type": "channel_group",
                            "children": [
                                {
                                    "type": "note_with_modifier",
                                    "children": [
                                        {"type": "note", "text": "e"}
                                    ]
                                }
                            ]
                        },
                        {
                            "type": "channel_group",
                            "children": [
                                {
                                    "type": "note_with_modifier",
                                    "children": [
                                        {"type": "note", "text": "g"}
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let parse_tree: ParseTreeNode = serde_json::from_str(parse_tree_json).unwrap();
        let mut chord_id = 0;
        let tokens = parse_tree_to_tokens(&parse_tree, None, &mut chord_id);
        
        // Verify we got 3 tokens, one for each channel
        assert_eq!(tokens.len(), 3);
        
        // Verify they have the correct channel_group assignments
        assert_eq!(tokens[0].token_type, "note");
        assert_eq!(tokens[0].value, "c");
        assert_eq!(tokens[0].channel_group, Some(0));
        
        assert_eq!(tokens[1].token_type, "note");
        assert_eq!(tokens[1].value, "e");
        assert_eq!(tokens[1].channel_group, Some(1));
        
        assert_eq!(tokens[2].token_type, "note");
        assert_eq!(tokens[2].value, "g");
        assert_eq!(tokens[2].channel_group, Some(2));
    }
}
