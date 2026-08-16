//! MML Preprocessor
//!
//! Handles preprocessing of MML text before parsing.
//!
//! # JSON-in-MML (provisional spec)
//!
//! If the MML string begins with a JSON object (`{…}`) or JSON array (`[…]`),
//! that leading block is treated as the **attachment JSON** (添付JSON).
//! The remaining text after the JSON is processed as normal MML.
//!
//! Example:
//! ```text
//! [{"ProgramChange":1,"Tone":{"events":[]}}]@1cde
//! ```
//! → attachment JSON: `[{"ProgramChange":1,"Tone":{"events":[]}}]`
//! → MML: `@1cde`

use anyhow::{anyhow, Result};

/// Result of preprocessing an MML string.
#[derive(Debug, Clone, PartialEq)]
pub struct PreprocessResult {
    /// The attachment JSON extracted from the leading block, or `None` if absent.
    pub embedded_json: Option<String>,
    /// The remaining MML text (after stripping the JSON prefix).
    pub remaining_mml: String,
    /// Byte offset of `remaining_mml` within the original input.
    ///
    /// Callers that need to map positions inside `remaining_mml` back to the
    /// original text (editor cursors, diagnostics) add this offset.
    pub remaining_offset: usize,
}

/// Extract a leading JSON block (object or array) from MML text.
///
/// Scans the start of `mml` for a JSON object (`{…}`) or array (`[…]`) and,
/// if found and valid, returns it as `embedded_json` with the rest as `remaining_mml`.
/// Invalid or absent JSON leaves `embedded_json` as `None` and `remaining_mml`
/// equal to the original `mml`.
pub fn extract_embedded_json(mml: &str) -> PreprocessResult {
    let trimmed = mml.trim_start();
    let trimmed_offset = mml.len() - trimmed.len();

    let unchanged = || PreprocessResult {
        embedded_json: None,
        remaining_mml: mml.to_string(),
        remaining_offset: 0,
    };

    let first = match trimmed.chars().next() {
        Some(c) => c,
        None => return unchanged(),
    };

    if first != '{' && first != '[' {
        return unchanged();
    }

    match find_json_end(trimmed) {
        Ok(end_idx) => {
            let json_str = &trimmed[..=end_idx];
            // Validate it is actually parseable JSON
            if serde_json::from_str::<serde_json::Value>(json_str).is_ok() {
                let after_json = &trimmed[end_idx + 1..];
                let rest = after_json.trim_start();
                PreprocessResult {
                    embedded_json: Some(json_str.to_string()),
                    remaining_offset: trimmed_offset + (trimmed.len() - rest.len()),
                    remaining_mml: rest.to_string(),
                }
            } else {
                unchanged()
            }
        }
        Err(_) => unchanged(),
    }
}

/// Find the byte index of the closing `}` or `]` that matches the opening
/// `{` or `[` at position 0 of `s`.
///
/// Returns an error if the input is empty, doesn't start with a bracket, or
/// the brackets are unbalanced.
fn find_json_end(s: &str) -> Result<usize> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escaped = false;

    for (i, &b) in bytes.iter().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if b == b'\\' && in_string {
            escaped = true;
            continue;
        }
        if b == b'"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match b {
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
    }

    Err(anyhow!("unbalanced JSON brackets"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- extract_embedded_json ---

    #[test]
    fn test_no_json_prefix_returns_mml_unchanged() {
        let result = extract_embedded_json("cde");
        assert_eq!(result.embedded_json, None);
        assert_eq!(result.remaining_mml, "cde");
    }

    #[test]
    fn test_empty_string() {
        let result = extract_embedded_json("");
        assert_eq!(result.embedded_json, None);
        assert_eq!(result.remaining_mml, "");
    }

    #[test]
    fn test_json_array_prefix_extracted() {
        let mml = r#"[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde"#;
        let result = extract_embedded_json(mml);
        assert_eq!(
            result.embedded_json.as_deref(),
            Some(r#"[{"ProgramChange":1,"Tone":{"events":[]}}]"#)
        );
        assert_eq!(result.remaining_mml, "@1cde");
    }

    #[test]
    fn test_json_object_prefix_extracted() {
        let mml = r#"{"key":"val"}t120cde"#;
        let result = extract_embedded_json(mml);
        assert_eq!(result.embedded_json.as_deref(), Some(r#"{"key":"val"}"#));
        assert_eq!(result.remaining_mml, "t120cde");
    }

    #[test]
    fn test_json_only_no_remaining_mml() {
        let mml = r#"[{"ProgramChange":1}]"#;
        let result = extract_embedded_json(mml);
        assert_eq!(
            result.embedded_json.as_deref(),
            Some(r#"[{"ProgramChange":1}]"#)
        );
        assert_eq!(result.remaining_mml, "");
    }

    #[test]
    fn test_whitespace_before_json() {
        let mml = r#"  [{"ProgramChange":0,"Tone":{"events":[]}}]cde"#;
        let result = extract_embedded_json(mml);
        assert!(result.embedded_json.is_some());
        assert_eq!(result.remaining_mml, "cde");
    }

    #[test]
    fn test_whitespace_between_json_and_mml() {
        let mml = r#"[{"ProgramChange":1}]   @1cde"#;
        let result = extract_embedded_json(mml);
        assert!(result.embedded_json.is_some());
        assert_eq!(result.remaining_mml, "@1cde");
    }

    #[test]
    fn test_invalid_json_prefix_leaves_input_unchanged() {
        // Brackets but not valid JSON
        let mml = "[invalid json]cde";
        let result = extract_embedded_json(mml);
        assert_eq!(result.embedded_json, None);
        assert_eq!(result.remaining_mml, mml);
    }

    #[test]
    fn test_unbalanced_brackets_leaves_input_unchanged() {
        let mml = "[{unclosed}cde";
        let result = extract_embedded_json(mml);
        assert_eq!(result.embedded_json, None);
        assert_eq!(result.remaining_mml, mml);
    }

    #[test]
    fn test_json_with_nested_objects() {
        let mml = r#"[{"ProgramChange":1,"Tone":{"events":[{"time":0,"addr":"0x28","data":"0x01"}]}}]@1cde"#;
        let result = extract_embedded_json(mml);
        assert!(result.embedded_json.is_some());
        assert_eq!(result.remaining_mml, "@1cde");
    }

    #[test]
    fn test_json_with_escaped_quotes_in_string() {
        // JSON with escaped quotes inside string values
        let mml = r#"{"key":"val\"ue"}cde"#;
        let result = extract_embedded_json(mml);
        assert!(result.embedded_json.is_some());
        assert_eq!(result.remaining_mml, "cde");
    }

    #[test]
    fn test_multiple_program_changes_in_json() {
        let mml = r#"[{"ProgramChange":1,"Tone":{"events":[]}},{"ProgramChange":2,"Tone":{"events":[]}}]@1cde;@2ege"#;
        let result = extract_embedded_json(mml);
        assert!(result.embedded_json.is_some());
        assert_eq!(result.remaining_mml, "@1cde;@2ege");
        // Verify JSON is valid
        let parsed: serde_json::Value =
            serde_json::from_str(result.embedded_json.as_deref().unwrap()).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 2);
    }

    // --- remaining_offset ---

    #[test]
    fn test_remaining_offset_is_zero_without_json() {
        let result = extract_embedded_json("cde");
        assert_eq!(result.remaining_offset, 0);
    }

    #[test]
    fn test_remaining_offset_points_at_mml_in_original_text() {
        let mml = r#"[{"ProgramChange":1}]@1cde"#;
        let result = extract_embedded_json(mml);
        assert_eq!(&mml[result.remaining_offset..], result.remaining_mml);
    }

    #[test]
    fn test_remaining_offset_skips_surrounding_whitespace() {
        let mml = r#"  [{"ProgramChange":1}]   @1cde"#;
        let result = extract_embedded_json(mml);
        assert_eq!(&mml[result.remaining_offset..], result.remaining_mml);
    }

    #[test]
    fn test_remaining_offset_at_end_when_json_only() {
        let mml = r#"[{"ProgramChange":1}]"#;
        let result = extract_embedded_json(mml);
        assert_eq!(result.remaining_offset, mml.len());
        assert_eq!(result.remaining_mml, "");
    }

    // --- find_json_end ---

    #[test]
    fn test_find_json_end_simple_object() {
        assert_eq!(find_json_end(r#"{"a":1}rest"#).unwrap(), 6);
    }

    #[test]
    fn test_find_json_end_simple_array() {
        assert_eq!(find_json_end(r#"[1,2,3]rest"#).unwrap(), 6);
    }

    #[test]
    fn test_find_json_end_nested() {
        assert_eq!(find_json_end(r#"[[1,[2]],3]rest"#).unwrap(), 10);
    }

    #[test]
    fn test_find_json_end_unbalanced_returns_error() {
        assert!(find_json_end(r#"[unclosed"#).is_err());
    }
}
