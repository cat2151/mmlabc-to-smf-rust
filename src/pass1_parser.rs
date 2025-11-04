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
/// List of token structures with type, value, and chord_group
pub fn parse_mml(mml_string: &str) -> Vec<Token> {
    // Split by semicolons to identify chord groups
    let chord_groups: Vec<&str> = mml_string.split(';').collect();
    let mut tokens = Vec::new();

    // Only assign chord_group if there are multiple groups (i.e., semicolons present)
    let has_chord = chord_groups.len() > 1;

    for (group_idx, group) in chord_groups.iter().enumerate() {
        let chord_group = if has_chord { Some(group_idx) } else { None };
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
            chord_group: Option<usize>,
        ) {
            let node = cursor.node();
            let kind = node.kind();

            if kind == "note" {
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
                    tokens.push(Token {
                        token_type: "note".to_string(),
                        value: text.to_ascii_lowercase(),
                        chord_group,
                    });
                }
            }

            if cursor.goto_first_child() {
                loop {
                    extract_tokens(cursor, source, tokens, chord_group);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        extract_tokens(&mut cursor, group, &mut tokens, chord_group);
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
