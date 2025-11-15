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
    // Split by semicolons to identify channel groups
    let channel_groups: Vec<&str> = mml_string.split(';').collect();
    let mut tokens = Vec::new();

    // Only assign channel_group if there are multiple groups (i.e., semicolons present)
    let has_multiple_channels = channel_groups.len() > 1;

    // Track chord ID across all channel groups
    let mut global_chord_id = 0;

    for (group_idx, group) in channel_groups.iter().enumerate() {
        let channel_group = if has_multiple_channels {
            Some(group_idx)
        } else {
            None
        };
        let mut parser = Parser::new();
        let language = tree_sitter_mml::language();
        parser
            .set_language(&language)
            .expect("Failed to set tree-sitter language");

        let tree = parser
            .parse(group, None)
            .expect("Failed to parse MML string");
        let root_node = tree.root_node();

        let mut cursor = root_node.walk();

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
                            // Extract note and modifier from note_with_modifier
                            let (note_value, modifier) = extract_note_and_modifier(cursor, source);
                            tokens.push(Token {
                                token_type: "note".to_string(),
                                value: note_value,
                                channel_group,
                                chord_id: Some(current_chord_id),
                                modifier,
                            });
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
                }
            } else if kind == "note_with_modifier" {
                // Extract note and modifier from note_with_modifier
                let (note_value, modifier) = extract_note_and_modifier(cursor, source);
                tokens.push(Token {
                    token_type: "note".to_string(),
                    value: note_value,
                    channel_group,
                    chord_id: None,
                    modifier,
                });
            } else if kind == "octave_up" {
                tokens.push(Token {
                    token_type: "octave_up".to_string(),
                    value: "<".to_string(),
                    channel_group,
                    chord_id: None,
                    modifier: None,
                });
            } else if kind == "octave_down" {
                tokens.push(Token {
                    token_type: "octave_down".to_string(),
                    value: ">".to_string(),
                    channel_group,
                    chord_id: None,
                    modifier: None,
                });
            } else if kind == "octave_set" {
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
                    tokens.push(Token {
                        token_type: "octave_set".to_string(),
                        value: text.to_string(),
                        channel_group,
                        chord_id: None,
                        modifier: None,
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
        ) -> (String, Option<String>) {
            let mut note_value = String::new();
            let mut modifier = None;

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
                    }

                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }

            (note_value, modifier)
        }

        extract_tokens(
            &mut cursor,
            group,
            &mut tokens,
            channel_group,
            &mut global_chord_id,
        );
    }

    tokens
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
