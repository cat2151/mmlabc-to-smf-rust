//! Tests for the public in-memory conversion API.

use mmlabc_to_smf::{
    mml_to_smf_bytes, pass1_parser, pass2_ast, pass3_events, pass4_midi,
    raw_mml_to_smf_bytes_with_options, tokens_to_smf_bytes, SmfConversionOptions,
};

#[test]
fn mml_to_smf_bytes_returns_valid_smf() {
    let bytes = mml_to_smf_bytes("cde").unwrap();

    assert!(bytes.starts_with(b"MThd"));
    assert!(bytes.len() > 14);
}

#[test]
fn mml_to_smf_bytes_strips_embedded_json_prefix() {
    let bytes = mml_to_smf_bytes(r#"[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde"#).unwrap();

    assert!(bytes.starts_with(b"MThd"));
}

#[test]
fn raw_mml_to_smf_bytes_with_options_matches_manual_pipeline() {
    let options = SmfConversionOptions {
        use_drum_channel_for_128: false,
    };

    let bytes = raw_mml_to_smf_bytes_with_options("@0c;@128d", options).unwrap();

    let tokens = pass1_parser::parse_mml("@0c;@128d");
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast, false);
    let expected = pass4_midi::events_to_midi(&events).unwrap();

    assert_eq!(bytes, expected);
}

#[test]
fn tokens_to_smf_bytes_accepts_preparsed_tokens() {
    let tokens = pass1_parser::parse_mml("t120o4c");
    let bytes = tokens_to_smf_bytes(&tokens).unwrap();

    assert!(bytes.starts_with(b"MThd"));
}
