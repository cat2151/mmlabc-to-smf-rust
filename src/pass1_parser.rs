//! Pass 1: Parse MML string and create token list using tree-sitter
//! Outputs debug JSON.

use crate::parse_tree_tokens::{
    parse_generic_tree_to_spanned_tokens, GenericParseTreeNode, SpannedToken,
};
use crate::tree_sitter_mml;
use crate::types::Token;
use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use tree_sitter::Parser;

/// Parse MML string into tokens using tree-sitter
///
/// # Arguments
/// * `mml_string` - MML format string (e.g., "cde" or "c;e;g")
///
/// # Returns
/// List of token structures with type, value, and channel_group
pub fn parse_mml(mml_string: &str) -> Vec<Token> {
    parse_mml_spanned(mml_string)
        .into_iter()
        .map(|spanned| spanned.token)
        .collect()
}

/// Parse MML string into tokens that keep the byte range they came from.
///
/// Same tokens as [`parse_mml`]; the byte ranges are relative to `mml_string`,
/// which must already have any embedded attachment JSON stripped.
pub fn parse_mml_spanned(mml_string: &str) -> Vec<SpannedToken> {
    let mut parser = Parser::new();
    let language = tree_sitter_mml::language();
    parser
        .set_language(&language)
        .expect("Failed to set tree-sitter language");

    let tree = parser
        .parse(mml_string, None)
        .expect("Failed to parse MML string");
    let mut global_chord_id = 0;
    let normalized_tree = tree_sitter_node_to_generic(tree.root_node(), mml_string.as_bytes());
    parse_generic_tree_to_spanned_tokens(&normalized_tree, None, &mut global_chord_id)
}

fn tree_sitter_node_to_generic(node: tree_sitter::Node<'_>, source: &[u8]) -> GenericParseTreeNode {
    let mut cursor = node.walk();
    let children = node
        .children(&mut cursor)
        .map(|child| tree_sitter_node_to_generic(child, source))
        .collect();

    GenericParseTreeNode {
        node_type: node.kind().to_string(),
        text: node.utf8_text(source).ok().map(ToOwned::to_owned),
        byte_range: Some(node.byte_range()),
        is_missing: node.is_missing(),
        children,
    }
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
