//! Shared token extraction from a normalized parse tree.

use std::ops::Range;

use crate::types::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParseTreeNode {
    pub node_type: String,
    pub text: Option<String>,
    /// Byte range of this node in the source text, when the producer knows it.
    ///
    /// `pass1_parser` fills this in from tree-sitter. Producers that only have a
    /// serialized parse tree without byte offsets leave it `None`.
    pub byte_range: Option<Range<usize>>,
    /// tree-sitter inserted this node to recover from a syntax error, so the
    /// source text does not actually contain it (e.g. the closing `'` of an
    /// unfinished chord).
    pub is_missing: bool,
    pub children: Vec<GenericParseTreeNode>,
}

/// A token together with the source text it came from.
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    /// Byte range of the *sounding unit* this token belongs to. Every token of a
    /// chord carries the whole chord's range, both quotes included.
    pub byte_range: Option<Range<usize>>,
    /// The unit is syntactically unfinished (e.g. `'ceg` with no closing quote).
    pub incomplete: bool,
}

pub fn parse_generic_tree_to_tokens(
    parse_tree: &GenericParseTreeNode,
    channel_group: Option<usize>,
    chord_id: &mut usize,
) -> Vec<Token> {
    parse_generic_tree_to_spanned_tokens(parse_tree, channel_group, chord_id)
        .into_iter()
        .map(|spanned| spanned.token)
        .collect()
}

/// Same as [`parse_generic_tree_to_tokens`], but each token keeps the byte range
/// of the sounding unit it came from.
pub fn parse_generic_tree_to_spanned_tokens(
    parse_tree: &GenericParseTreeNode,
    channel_group: Option<usize>,
    chord_id: &mut usize,
) -> Vec<SpannedToken> {
    let mut tokens = Vec::new();
    extract_tokens_from_node(parse_tree, &mut tokens, channel_group, chord_id);
    tokens
}

/// Whether the subtree contains a node tree-sitter had to invent to recover
/// from a syntax error.
fn has_missing(node: &GenericParseTreeNode) -> bool {
    node.is_missing || node.children.iter().any(has_missing)
}

fn push_spanned(tokens: &mut Vec<SpannedToken>, node: &GenericParseTreeNode, token: Token) {
    tokens.push(SpannedToken {
        token,
        byte_range: node.byte_range.clone(),
        incomplete: has_missing(node),
    });
}

/// A token that only carries the node's own text, such as `o5` or `@1`.
fn text_token(token_type: &str, value: String, channel_group: Option<usize>) -> Token {
    Token {
        token_type: token_type.to_string(),
        value,
        channel_group,
        chord_id: None,
        modifier: None,
        note_length: None,
        dots: None,
    }
}

fn extract_tokens_from_node(
    node: &GenericParseTreeNode,
    tokens: &mut Vec<SpannedToken>,
    channel_group: Option<usize>,
    chord_id: &mut usize,
) {
    let kind = node.node_type.as_str();

    if kind == "channel_groups" {
        let mut channel_idx = 0;
        for child in &node.children {
            if child.node_type == "channel_group" {
                extract_tokens_from_node(child, tokens, Some(channel_idx), chord_id);
                channel_idx += 1;
            }
        }
    } else if kind == "channel_group" {
        for child in &node.children {
            extract_tokens_from_node(child, tokens, channel_group, chord_id);
        }
    } else if kind == "chord" {
        let mut chord_tokens = Vec::new();
        let mut has_note = false;

        for child in &node.children {
            if child.node_type == "note_with_modifier" {
                let (note_value, modifier, note_length, dots) = extract_note_and_modifier(child);
                if note_value.is_empty() {
                    continue;
                }
                has_note = true;
                chord_tokens.push(Token {
                    token_type: "note".to_string(),
                    value: note_value,
                    channel_group,
                    chord_id: None,
                    modifier,
                    note_length,
                    dots,
                });
            } else if let Some((token_type, value)) = octave_shift_token(&child.node_type) {
                chord_tokens.push(text_token(token_type, value.to_string(), channel_group));
            }
        }

        if has_note {
            let current_chord_id = *chord_id;
            *chord_id += 1;
            for token in &mut chord_tokens {
                token.chord_id = Some(current_chord_id);
            }
        }

        // The chord is one sounding unit, so every one of its tokens points at
        // the chord node's range rather than at the single note inside it.
        for token in chord_tokens {
            push_spanned(tokens, node, token);
        }
    } else if kind == "note_with_modifier" {
        let (note_value, modifier, note_length, dots) = extract_note_and_modifier(node);
        if !note_value.is_empty() {
            push_spanned(
                tokens,
                node,
                Token {
                    token_type: "note".to_string(),
                    value: note_value,
                    channel_group,
                    chord_id: None,
                    modifier,
                    note_length,
                    dots,
                },
            );
        }
    } else if let Some((token_type, value)) = octave_shift_token(kind) {
        push_spanned(
            tokens,
            node,
            text_token(token_type, value.to_string(), channel_group),
        );
    } else if kind == "rest" {
        let mut rest_length = None;
        let mut rest_dots = None;

        for child in &node.children {
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

        push_spanned(
            tokens,
            node,
            Token {
                token_type: "rest".to_string(),
                value: "r".to_string(),
                channel_group,
                chord_id: None,
                modifier: None,
                note_length: rest_length,
                dots: rest_dots,
            },
        );
    } else if kind == "length_set" {
        let mut length_value = None;
        let mut length_dots = None;

        for child in &node.children {
            if child.node_type == "dots" {
                if let Some(text) = &child.text {
                    length_dots = Some(text.len() as u32);
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

            push_spanned(
                tokens,
                node,
                Token {
                    token_type: "length_set".to_string(),
                    value: text.clone(),
                    channel_group,
                    chord_id: None,
                    modifier: None,
                    note_length: length_value,
                    dots: length_dots,
                },
            );
        }
    } else if matches!(
        kind,
        "octave_set" | "program_change" | "tempo_set" | "velocity_set" | "key_transpose"
    ) {
        if let Some(text) = &node.text {
            push_spanned(tokens, node, text_token(kind, text.clone(), channel_group));
        }
    } else {
        for child in &node.children {
            extract_tokens_from_node(child, tokens, channel_group, chord_id);
        }
    }
}

/// `<` / `>` are the only tokens whose value is fixed by their node type.
/// Returns the token type and its literal value.
fn octave_shift_token(kind: &str) -> Option<(&'static str, &'static str)> {
    match kind {
        "octave_up" => Some(("octave_up", "<")),
        "octave_down" => Some(("octave_down", ">")),
        _ => None,
    }
}

fn extract_note_and_modifier(
    node: &GenericParseTreeNode,
) -> (String, Option<String>, Option<u32>, Option<u32>) {
    let mut note_value = String::new();
    let mut modifier = None;
    let mut note_length = None;
    let mut dots = None;

    for child in &node.children {
        if child.node_type == "note" {
            if let Some(text) = &child.text {
                note_value = text.to_ascii_lowercase();
            }
        } else if child.node_type == "modifier" {
            if let Some(text) = &child.text {
                modifier = Some(text.clone());
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

    (note_value, modifier, note_length, dots)
}
