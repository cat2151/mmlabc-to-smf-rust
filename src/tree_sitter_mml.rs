use tree_sitter::Language;

extern "C" {
    fn tree_sitter_mml() -> Language;
}

/// Get the tree-sitter Language for MML
pub fn language() -> Language {
    unsafe { tree_sitter_mml() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_load_grammar() {
        let lang = language();
        assert!(lang.node_kind_count() > 0);
    }
}
