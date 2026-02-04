//! WASM bindings for mmlabc-to-smf
//!
//! This crate provides WebAssembly bindings to enable browser-based MML to SMF conversion.
//! It implements Option A: receiving parse tree JSON from web-tree-sitter and converting
//! to SMF binary, achieving SSOT (Single Source of Truth) by keeping all token extraction
//! logic in Rust.

use mmlabc_to_smf::{pass2_ast, pass3_events, pass4_midi, types::Token};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Position in source text (row, column)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

/// Parse tree node from web-tree-sitter
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseTreeNode {
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ParseTreeNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Extract tokens from parse tree JSON
///
/// This function implements the core token extraction logic from parse tree,
/// maintaining SSOT by keeping this logic in Rust (not duplicated in JavaScript).
pub fn parse_tree_to_tokens(
    parse_tree: &ParseTreeNode,
    channel_group: Option<usize>,
    chord_id: &mut usize,
) -> Vec<Token> {
    let mut tokens = Vec::new();
    extract_tokens_from_node(parse_tree, &mut tokens, channel_group, chord_id);
    tokens
}

fn extract_tokens_from_node(
    node: &ParseTreeNode,
    tokens: &mut Vec<Token>,
    channel_group: Option<usize>,
    chord_id: &mut usize,
) {
    let kind = &node.node_type;

    if kind == "channel_groups" {
        // Process channel groups - extract tokens from each channel_group
        if let Some(children) = &node.children {
            let mut channel_idx = 0;
            for child in children {
                if child.node_type == "channel_group" {
                    extract_tokens_from_node(child, tokens, Some(channel_idx), chord_id);
                    channel_idx += 1;
                }
            }
        }
    } else if kind == "channel_group" {
        // Process items within a channel group
        if let Some(children) = &node.children {
            for child in children {
                extract_tokens_from_node(child, tokens, channel_group, chord_id);
            }
        }
    } else if kind == "chord" {
        // Found a chord - increment chord_id for this chord group
        let current_chord_id = *chord_id;
        *chord_id += 1;

        // Extract all notes within the chord
        if let Some(children) = &node.children {
            for child in children {
                if child.node_type == "note_with_modifier" {
                    let (note_value, modifier, note_length, dots) =
                        extract_note_and_modifier(child);
                    tokens.push(Token {
                        token_type: "note".to_string(),
                        value: note_value,
                        channel_group,
                        chord_id: Some(current_chord_id),
                        modifier,
                        note_length,
                        dots,
                    });
                }
            }
        }
    } else if kind == "note_with_modifier" {
        let (note_value, modifier, note_length, dots) = extract_note_and_modifier(node);
        tokens.push(Token {
            token_type: "note".to_string(),
            value: note_value,
            channel_group,
            chord_id: None,
            modifier,
            note_length,
            dots,
        });
    } else if kind == "octave_up" {
        tokens.push(Token {
            token_type: "octave_up".to_string(),
            value: "<".to_string(),
            channel_group,
            chord_id: None,
            modifier: None,
            note_length: None,
            dots: None,
        });
    } else if kind == "octave_down" {
        tokens.push(Token {
            token_type: "octave_down".to_string(),
            value: ">".to_string(),
            channel_group,
            chord_id: None,
            modifier: None,
            note_length: None,
            dots: None,
        });
    } else if kind == "octave_set" {
        if let Some(text) = &node.text {
            tokens.push(Token {
                token_type: "octave_set".to_string(),
                value: text.to_string(),
                channel_group,
                chord_id: None,
                modifier: None,
                note_length: None,
                dots: None,
            });
        }
    } else if kind == "rest" {
        let mut rest_length = None;
        let mut rest_dots = None;

        if let Some(children) = &node.children {
            for child in children {
                if child.node_type == "note_length" {
                    if let Some(text) = &child.text {
                        if let Ok(length) = text.parse::<u32>() {
                            rest_length = Some(length);
                        }
                    }
                } else if child.node_type == "dots" {
                    if let Some(text) = &child.text {
                        rest_dots = Some(text.len() as u32);
                    }
                }
            }
        }

        tokens.push(Token {
            token_type: "rest".to_string(),
            value: "r".to_string(),
            channel_group,
            chord_id: None,
            modifier: None,
            note_length: rest_length,
            dots: rest_dots,
        });
    } else if kind == "length_set" {
        let mut length_value = None;
        let mut length_dots = None;

        if let Some(children) = &node.children {
            for child in children {
                if child.node_type == "dots" {
                    if let Some(text) = &child.text {
                        length_dots = Some(text.len() as u32);
                    }
                }
            }
        }

        if let Some(text) = &node.text {
            if let Some(length_str) = text.strip_prefix('l') {
                let numeric_part = length_str.trim_end_matches('.');
                if let Ok(length) = numeric_part.parse::<u32>() {
                    length_value = Some(length);
                }
            }

            tokens.push(Token {
                token_type: "length_set".to_string(),
                value: text.to_string(),
                channel_group,
                chord_id: None,
                modifier: None,
                note_length: length_value,
                dots: length_dots,
            });
        }
    } else if kind == "program_change" {
        if let Some(text) = &node.text {
            tokens.push(Token {
                token_type: "program_change".to_string(),
                value: text.to_string(),
                channel_group,
                chord_id: None,
                modifier: None,
                note_length: None,
                dots: None,
            });
        }
    } else if kind == "tempo_set" {
        if let Some(text) = &node.text {
            tokens.push(Token {
                token_type: "tempo_set".to_string(),
                value: text.to_string(),
                channel_group,
                chord_id: None,
                modifier: None,
                note_length: None,
                dots: None,
            });
        }
    } else if kind == "velocity_set" {
        if let Some(text) = &node.text {
            tokens.push(Token {
                token_type: "velocity_set".to_string(),
                value: text.to_string(),
                channel_group,
                chord_id: None,
                modifier: None,
                note_length: None,
                dots: None,
            });
        }
    } else {
        // For other node types, recurse into children
        if let Some(children) = &node.children {
            for child in children {
                extract_tokens_from_node(child, tokens, channel_group, chord_id);
            }
        }
    }
}

fn extract_note_and_modifier(
    node: &ParseTreeNode,
) -> (String, Option<String>, Option<u32>, Option<u32>) {
    let mut note_value = String::new();
    let mut modifier = None;
    let mut note_length = None;
    let mut dots = None;

    if let Some(children) = &node.children {
        for child in children {
            if child.node_type == "note" {
                if let Some(text) = &child.text {
                    note_value = text.to_ascii_lowercase();
                }
            } else if child.node_type == "modifier" {
                if let Some(text) = &child.text {
                    modifier = Some(text.to_string());
                }
            } else if child.node_type == "note_length" {
                if let Some(text) = &child.text {
                    if let Ok(length) = text.parse::<u32>() {
                        note_length = Some(length);
                    }
                }
            } else if child.node_type == "dots" {
                if let Some(text) = &child.text {
                    dots = Some(text.len() as u32);
                }
            }
        }
    }

    (note_value, modifier, note_length, dots)
}

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
