//! Token extraction from parse tree nodes

use mmlabc_to_smf::types::Token;
use serde::{Deserialize, Serialize};

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
    } else if kind == "key_transpose" {
        if let Some(text) = &node.text {
            tokens.push(Token {
                token_type: "key_transpose".to_string(),
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
