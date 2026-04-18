//! Token extraction from parse tree nodes

use mmlabc_to_smf::parse_tree_tokens::{parse_generic_tree_to_tokens, GenericParseTreeNode};
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
    let normalized_tree = to_generic_parse_tree(parse_tree);
    parse_generic_tree_to_tokens(&normalized_tree, channel_group, chord_id)
}

fn to_generic_parse_tree(node: &ParseTreeNode) -> GenericParseTreeNode {
    GenericParseTreeNode {
        node_type: node.node_type.clone(),
        text: node.text.clone(),
        children: node
            .children
            .as_ref()
            .map(|children| children.iter().map(to_generic_parse_tree).collect())
            .unwrap_or_default(),
    }
}
