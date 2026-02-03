//! Pass 1: Parse MML string and create token list using tree-sitter
//! Outputs debug JSON.

use crate::tree_sitter_mml;
use crate::types::Token;
use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use tree_sitter::{Parser, TreeCursor};

/// Parse MML string into tokens using tree-sitter
///
/// # Arguments
/// * `mml_string` - MML format string (e.g., "cde" or "c;e;g")
///
/// # Returns
/// List of token structures with type, value, and channel_group
pub fn parse_mml(mml_string: &str) -> Vec<Token> {
    let mut parser = Parser::new();
    let language = tree_sitter_mml::language();
    parser
        .set_language(&language)
        .expect("Failed to set tree-sitter language");

    let tree = parser
        .parse(mml_string, None)
        .expect("Failed to parse MML string");
    let root_node = tree.root_node();

    let mut cursor = root_node.walk();
    let mut tokens = Vec::new();
    let mut global_chord_id = 0;

    // Check if the root has channel_groups
    if cursor.goto_first_child() {
        let first_child_kind = cursor.node().kind();
        
        if first_child_kind == "channel_groups" {
            // Process channel groups
            if cursor.goto_first_child() {
                let mut channel_idx = 0;
                loop {
                    let child_node = cursor.node();
                    if child_node.kind() == "channel_group" {
                        // Process this channel group
                        let mut child_cursor = child_node.walk();
                        extract_tokens(
                            &mut child_cursor,
                            mml_string,
                            &mut tokens,
                            Some(channel_idx),
                            &mut global_chord_id,
                        );
                        channel_idx += 1;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        } else {
            // No channel groups, process items directly with no channel_group
            cursor.goto_parent();
            extract_tokens(&mut cursor, mml_string, &mut tokens, None, &mut global_chord_id);
        }
    }

    tokens
}

fn extract_tokens(
    cursor: &mut TreeCursor,
    source: &str,
    tokens: &mut Vec<Token>,
    channel_group: Option<usize>,
    chord_id: &mut usize,
) {
    let node = cursor.node();
            let kind = node.kind();

            if kind == "chord" {
                // Found a chord - increment chord_id for this chord group
                let current_chord_id = *chord_id;
                *chord_id += 1;

                // Extract all notes within the chord
                if cursor.goto_first_child() {
                    loop {
                        let child_node = cursor.node();
                        if child_node.kind() == "note_with_modifier" {
                            // Extract note, modifier, length, and dots from note_with_modifier
                            let (note_value, modifier, note_length, dots) =
                                extract_note_and_modifier(cursor, source);
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
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
                }
            } else if kind == "note_with_modifier" {
                // Extract note, modifier, length, and dots from note_with_modifier
                let (note_value, modifier, note_length, dots) =
                    extract_note_and_modifier(cursor, source);
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
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
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
                // Extract rest length and dots
                let mut rest_length = None;
                let mut rest_dots = None;

                if cursor.goto_first_child() {
                    loop {
                        let child_node = cursor.node();
                        let child_kind = child_node.kind();

                        if child_kind == "note_length" {
                            if let Ok(text) = child_node.utf8_text(source.as_bytes()) {
                                if let Ok(length) = text.parse::<u32>() {
                                    rest_length = Some(length);
                                }
                            }
                        } else if child_kind == "dots" {
                            if let Ok(text) = child_node.utf8_text(source.as_bytes()) {
                                rest_dots = Some(text.len() as u32);
                            }
                        }

                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
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
                // Extract length_set length and dots
                let mut length_value = None;
                let mut length_dots = None;

                if cursor.goto_first_child() {
                    loop {
                        let child_node = cursor.node();
                        let child_kind = child_node.kind();

                        if child_kind == "dots" {
                            if let Ok(text) = child_node.utf8_text(source.as_bytes()) {
                                length_dots = Some(text.len() as u32);
                            }
                        }

                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
                }

                // Parse the entire text to get the l value
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
                    if let Some(length_str) = text.strip_prefix('l') {
                        // Strip dots if present to get the numeric part
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
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
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
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
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
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
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
                if cursor.goto_first_child() {
                    loop {
                        extract_tokens(cursor, source, tokens, channel_group, chord_id);
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
                }
            }
        }

fn extract_note_and_modifier(
    cursor: &mut TreeCursor,
    source: &str,
) -> (String, Option<String>, Option<u32>, Option<u32>) {
    let mut note_value = String::new();
    let mut modifier = None;
    let mut note_length = None;
    let mut dots = None;

    if cursor.goto_first_child() {
        loop {
            let child_node = cursor.node();
            let child_kind = child_node.kind();

            if child_kind == "note" {
                if let Ok(text) = child_node.utf8_text(source.as_bytes()) {
                    note_value = text.to_ascii_lowercase();
                }
            } else if child_kind == "modifier" {
                if let Ok(text) = child_node.utf8_text(source.as_bytes()) {
                    modifier = Some(text.to_string());
                }
            } else if child_kind == "note_length" {
                if let Ok(text) = child_node.utf8_text(source.as_bytes()) {
                    if let Ok(length) = text.parse::<u32>() {
                        note_length = Some(length);
                    }
                }
            } else if child_kind == "dots" {
                if let Ok(text) = child_node.utf8_text(source.as_bytes()) {
                    dots = Some(text.len() as u32);
                }
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }

    (note_value, modifier, note_length, dots)
}

#[derive(Serialize)]
struct TokenOutput {
    pass: u8,
    description: String,
    tokens: Vec<Token>,
}

/// Save tokens to JSON file for debugging
///
/// # Arguments
/// * `tokens` - List of token structures
/// * `filepath` - Output JSON file path
pub fn save_tokens_to_json(tokens: &[Token], filepath: &str) -> Result<()> {
    let output = TokenOutput {
        pass: 1,
        description: "Parsed tokens".to_string(),
        tokens: tokens.to_vec(),
    };

    let json = serde_json::to_string_pretty(&output)?;
    let mut file = File::create(filepath)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Execute Pass 1: Parse MML and save tokens
///
/// # Arguments
/// * `mml_string` - MML format string
/// * `output_json` - Output JSON file path
///
/// # Returns
/// List of tokens
pub fn process_pass1(mml_string: &str, output_json: &str) -> Result<Vec<Token>> {
    let tokens = parse_mml(mml_string);
    save_tokens_to_json(&tokens, output_json)?;
    Ok(tokens)
}
