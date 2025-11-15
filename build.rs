use std::path::PathBuf;
use std::process::Command;

fn main() {
    let dir: PathBuf = ["tree-sitter-mml", "src"].iter().collect();
    let parser_c = dir.join("parser.c");

    // Auto-generate tree-sitter files if parser.c doesn't exist
    if !parser_c.exists() {
        println!("cargo:warning=Generating tree-sitter parser files...");

        let status = Command::new("npx")
            .args(["tree-sitter", "generate"])
            .current_dir("tree-sitter-mml")
            .status()
            .expect("Failed to execute npx tree-sitter generate. Make sure Node.js and npx are installed.");

        if !status.success() {
            panic!("tree-sitter generate failed");
        }

        println!("cargo:warning=Tree-sitter parser files generated successfully");
    }

    cc::Build::new()
        .include(&dir)
        .file(parser_c)
        .compile("tree-sitter-mml");

    println!("cargo:rerun-if-changed=tree-sitter-mml/src/parser.c");
    println!("cargo:rerun-if-changed=tree-sitter-mml/grammar.js");
}
