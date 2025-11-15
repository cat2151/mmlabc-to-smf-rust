use std::path::PathBuf;
use std::process::Command;

fn main() {
    let grammar_path = PathBuf::from("tree-sitter-mml/grammar.js");
    let src_dir: PathBuf = ["tree-sitter-mml", "src"].iter().collect();
    let parser_c = src_dir.join("parser.c");

    // Check if we need to regenerate: either files don't exist or grammar.js is newer
    let should_generate = if !parser_c.exists() {
        true
    } else if let (Ok(grammar_meta), Ok(parser_meta)) = (
        std::fs::metadata(&grammar_path),
        std::fs::metadata(&parser_c),
    ) {
        if let (Ok(grammar_time), Ok(parser_time)) =
            (grammar_meta.modified(), parser_meta.modified())
        {
            grammar_time > parser_time
        } else {
            false
        }
    } else {
        false
    };

    if should_generate {
        println!("cargo:warning=Tree-sitter grammar changed, regenerating parser files...");

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
        .include(&src_dir)
        .file(parser_c)
        .compile("tree-sitter-mml");

    // Tell cargo to rerun this build script if grammar.js changes
    println!("cargo:rerun-if-changed=tree-sitter-mml/grammar.js");
}
