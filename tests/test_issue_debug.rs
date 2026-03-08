use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;

#[test]
fn debug_note_length_conversion() {
    println!("\n=== Pass 1: Parsing ===");
    let mml = "c1";
    let tokens = parse_mml(mml);
    println!("Input: {}", mml);
    println!("Parsed Token:");
    println!("  token_type: {}", tokens[0].token_type);
    println!("  value: {}", tokens[0].value);
    println!("  note_length: {:?}", tokens[0].note_length);
    
    println!("\n=== Pass 2: AST Conversion ===");
    let ast = tokens_to_ast(&tokens);
    println!("AST Note:");
    println!("  name: {}", ast.notes[0].name);
    println!("  length: {:?}", ast.notes[0].length);
    println!("\nPROBLEM: Token has note_length=Some(1), but AST has length=Some(4)");
    println!("This means the note-specific length is being IGNORED!");
    
    println!("\n=== Testing c64 ===");
    let tokens64 = parse_mml("c64");
    println!("Parsed Token:");
    println!("  note_length: {:?}", tokens64[0].note_length);
    
    let ast64 = tokens_to_ast(&tokens64);
    println!("AST Note:");
    println!("  length: {:?}", ast64.notes[0].length);
}
