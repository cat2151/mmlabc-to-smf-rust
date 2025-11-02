//! Pass 2: Convert tokens to AST (Abstract Syntax Tree)
//! Outputs debug JSON.

use crate::types::{Ast, AstNote, Token};
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Convert tokens to AST structure
///
/// # Arguments
/// * `tokens` - List of token structures
///
/// # Returns
/// AST structure with note events
pub fn tokens_to_ast(tokens: &[Token]) -> Ast {
    let note_to_midi: HashMap<&str, u8> = [
        ("c", 60), // Middle C (C4)
        ("d", 62),
        ("e", 64),
        ("f", 65),
        ("g", 67),
        ("a", 69),
        ("b", 71),
    ]
    .iter()
    .cloned()
    .collect();

    let mut notes = Vec::new();

    for token in tokens {
        if token.token_type == "note" {
            let midi_note = note_to_midi
                .get(token.value.as_str())
                .copied()
                .unwrap_or(60);
            notes.push(AstNote {
                note_type: "note".to_string(),
                pitch: midi_note,
                name: token.value.clone(),
            });
        }
    }

    Ast {
        ast_type: "sequence".to_string(),
        notes,
    }
}

#[derive(Serialize)]
struct AstOutput {
    pass: u8,
    description: String,
    ast: Ast,
}

/// Save AST to JSON file for debugging
///
/// # Arguments
/// * `ast` - AST structure
/// * `filepath` - Output JSON file path
pub fn save_ast_to_json(ast: &Ast, filepath: &str) -> Result<()> {
    let output = AstOutput {
        pass: 2,
        description: "Abstract Syntax Tree".to_string(),
        ast: ast.clone(),
    };

    let json = serde_json::to_string_pretty(&output)?;
    let mut file = File::create(filepath)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Execute Pass 2: Create AST from tokens
///
/// # Arguments
/// * `tokens` - List of tokens from Pass 1
/// * `output_json` - Output JSON file path
///
/// # Returns
/// AST structure
pub fn process_pass2(tokens: &[Token], output_json: &str) -> Result<Ast> {
    let ast = tokens_to_ast(tokens);
    save_ast_to_json(&ast, output_json)?;
    Ok(ast)
}
