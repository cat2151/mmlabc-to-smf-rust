//! Pass 1: Parse MML string and create token list
//! Outputs debug JSON.

use crate::types::Token;
use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

/// Parse MML string into tokens
///
/// # Arguments
/// * `mml_string` - MML format string (e.g., "cde")
///
/// # Returns
/// List of token structures with type and value
pub fn parse_mml(mml_string: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for ch in mml_string.chars() {
        let lower_ch = ch.to_ascii_lowercase();
        if matches!(lower_ch, 'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b') {
            tokens.push(Token {
                token_type: "note".to_string(),
                value: lower_ch.to_string(),
            });
        }
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
