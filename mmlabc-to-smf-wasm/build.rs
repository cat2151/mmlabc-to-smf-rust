// Build script for WASM crate to compile tree-sitter parser

use std::path::PathBuf;

fn main() {
    // Only compile tree-sitter parser when building for WASM
    let target = std::env::var("TARGET").unwrap();
    
    if target.starts_with("wasm32") {
        println!("cargo:warning=Attempting to compile tree-sitter parser for WASM target");
        
        let src_dir: PathBuf = ["../tree-sitter-mml", "src"].iter().collect();
        let parser_c = src_dir.join("parser.c");
        
        // Try to compile parser.c for WASM
        let result = cc::Build::new()
            .file(&parser_c)
            .include(&src_dir)
            .warnings(false)
            .try_compile("tree-sitter-mml");
            
        match result {
            Ok(_) => {
                println!("cargo:warning=Successfully compiled parser.c for WASM");
            }
            Err(e) => {
                println!("cargo:warning=Failed to compile parser.c for WASM: {}", e);
                println!("cargo:warning=This is expected - tree-sitter C parser cannot be compiled to wasm32-unknown-unknown");
                // Don't panic, just continue without it
            }
        }
    }
}
