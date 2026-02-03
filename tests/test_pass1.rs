//! Unit tests for Pass 1: MML Parser

use mmlabc_to_smf::pass1_parser::*;

#[test]
fn test_parse_single_note() {
    let tokens = parse_mml("c");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, "note");
    assert_eq!(tokens[0].value, "c");
}

#[test]
fn test_parse_multiple_notes() {
    let tokens = parse_mml("cde");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[1].value, "d");
    assert_eq!(tokens[2].value, "e");
}

#[test]
fn test_case_insensitive() {
    let tokens_upper = parse_mml("CDE");
    let tokens_lower = parse_mml("cde");
    assert_eq!(tokens_upper.len(), tokens_lower.len());
    for (upper, lower) in tokens_upper.iter().zip(tokens_lower.iter()) {
        assert_eq!(upper.value, lower.value);
    }
}

#[test]
fn test_all_notes() {
    let tokens = parse_mml("cdefgab");
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].value, "c");
    assert_eq!(tokens[1].value, "d");
    assert_eq!(tokens[2].value, "e");
    assert_eq!(tokens[3].value, "f");
    assert_eq!(tokens[4].value, "g");
    assert_eq!(tokens[5].value, "a");
    assert_eq!(tokens[6].value, "b");
}

#[test]
fn test_ignore_non_notes() {
    let tokens = parse_mml("c1d2e3");
    // Should only parse c, d, e and ignore numbers
    assert_eq!(tokens.len(), 3);
}

#[test]
fn test_empty_string() {
    let tokens = parse_mml("");
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_save_tokens_to_json() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let tokens = parse_mml("cde");
    let filepath = env::temp_dir().join("test_pass1_tokens.json");

    let result = save_tokens_to_json(&tokens, filepath.to_str().unwrap());
    assert!(result.is_ok());
    assert!(Path::new(&filepath).exists());

    // Clean up
    let _ = fs::remove_file(filepath);
}

#[test]
fn test_grammar_tree_structure() {
    use mmlabc_to_smf::tree_sitter_mml;
    use tree_sitter::Parser;
    
    let mut parser = Parser::new();
    let language = tree_sitter_mml::language();
    parser.set_language(&language).expect("Failed to set language");
    
    // Test with semicolons
    let tree = parser.parse("c;e;g", None).expect("Failed to parse");
    let root = tree.root_node();
    println!("Tree for 'c;e;g':");
    println!("{}", root.to_sexp());
    
    // The grammar should have a channel_groups node
    assert!(root.to_sexp().contains("channel_groups"));
    
    // Test without semicolons
    let tree2 = parser.parse("cde", None).expect("Failed to parse");
    let root2 = tree2.root_node();
    println!("\nTree for 'cde':");
    println!("{}", root2.to_sexp());
    
    // Without semicolons, should not have channel_groups
    assert!(!root2.to_sexp().contains("channel_groups"));
}
