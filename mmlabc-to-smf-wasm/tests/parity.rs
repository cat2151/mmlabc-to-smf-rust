use mmlabc_to_smf::{pass2_ast, pass3_events, pass4_midi};
use mmlabc_to_smf_wasm::{parse_tree_to_tokens, ParseTreeNode, Position};
use mmlabc_to_smf::{pass1_parser, tree_sitter_mml};
use tree_sitter::{Node, Parser};

const PARITY_CASES: &[&str] = &[
    "'<>",
    "'<c>'",
    "'c<e>g'",
    "'c<eg'",
    "'c<g'",
    "'c8eg'",
    "'c8eg''d4fa'",
    "'c8eg'd4",
    "'ce'",
    "'ce''df'",
    "'ce8g'",
    "'ceg'",
    "'ceg'de",
    "'ceg2.'",
    "'cg';e",
    "@0'ceg'",
    "@0c",
    "@0c;@128cr8d",
    "@0c;@128d",
    "@0c;@128d;@1e",
    "@0c;@1e",
    "@0c@0d",
    "@0c@127d",
    "@0c@1d@127e",
    "@0c4;@128d8;@1e2",
    "@0cde;@128cde;@1cde",
    "@0l8c",
    "@0o4c",
    "@0t120c;@128t60d",
    "@10c+",
    "@127c",
    "@128c",
    "@128c;@1d;@128e",
    "@128o4l8v10cde",
    "@1c@3d@2e",
    "@1cde",
    "@2cde",
    "<'ceg'",
    "<>",
    "<c+>d-",
    "c",
    "c-",
    "c-d-e-f-g-a-b-",
    "c;e",
    "c;e;g",
    "c@1d",
    "c@5d",
    "c+",
    "c+4.d-8.",
    "c+8d-4",
    "C+D-E+",
    "c+d+e+f+g+a+b+",
    "c+de-f",
    "c<<c",
    "c<c",
    "c<c;e>e",
    "c<c<<c>>c>c",
    "c<c>c",
    "c<cde",
    "c<r>c",
    "c>>c",
    "c>c",
    "c1",
    "c1....",
    "c16",
    "c1d2e3",
    "c2.",
    "c4.",
    "c4.;e8.",
    "c4..",
    "c4...",
    "c4.d8e4.f8",
    "c4d8e2",
    "c64",
    "c8",
    "c8;e4",
    "cd;ef;ga",
    "cd'eg'",
    "CDE",
    "cdefgab",
    "cdefgab<cdefgab>cdefgab",
    "crc",
    "crc;ere",
    "crdre",
    "crrrc",
    "ct120d",
    "ct60d",
    "cv8d",
    "kt-12c",
    "kt-1c",
    "kt-2cde",
    "kt0c",
    "kt12c",
    "kt1c",
    "kt1c;kt-1e",
    "kt1c+",
    "kt1c<c",
    "kt1c>c",
    "kt1ckt2dkt-1e",
    "kt1cr",
    "kt1d-",
    "kt1l8c",
    "kt1v8c",
    "kt2'ceg'",
    "KT2c",
    "kt2cde",
    "l16c",
    "l1c",
    "l2c",
    "l4..c",
    "l4.c",
    "l4.cd",
    "l4.cl8d",
    "l4c",
    "l4c4.",
    "l4c8",
    "l4c8d",
    "l8'ceg'",
    "l8c",
    "l8c;l4e",
    "l8c+",
    "l8cd;l4ef",
    "l8cde",
    "l8cl4cl1c",
    "l8o4c",
    "o3co2co1c",
    "o4c",
    "o4c;o6e",
    "o4c<c",
    "o4cde",
    "o4co5co6c",
    "o4kt1c",
    "o4kt2c<kt-1d>kt0e",
    "o5c",
    "o5c8o4d4",
    "o5cdefgab",
    "o6c",
    "o6c>c",
    "o7co8c",
    "o8kt10c",
    "r",
    "r4.c8",
    "r8c4r2",
    "t120'ceg'",
    "t120c",
    "t120c+",
    "t120l8c",
    "t120o4c",
    "t150c",
    "t1ct255d",
    "t60c",
    "t60c;t120e",
    "t60ct120dt150e",
    "v15c",
    "v15c+",
    "v15l8c",
    "v15o4c",
    "v1c",
    "v1cv15d",
    "v1cv8dv15e",
    "v8'ceg'",
    "v8@0c",
    "v8c",
    "v8c;v15e",
    "v8cde",
    "v8cr",
    "v8t120c",
];

fn parse_tree_from_mml(mml: &str) -> ParseTreeNode {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_mml::language())
        .expect("Failed to set tree-sitter language");
    let tree = parser.parse(mml, None).expect("Failed to parse MML");
    convert_node(tree.root_node(), mml.as_bytes())
}

fn convert_node(node: Node<'_>, source: &[u8]) -> ParseTreeNode {
    let start = node.start_position();
    let end = node.end_position();
    let children = (0..node.child_count())
        .filter_map(|idx| node.child(idx))
        .map(|child| convert_node(child, source))
        .collect::<Vec<_>>();

    ParseTreeNode {
        node_type: node.kind().to_string(),
        start_position: Some(Position {
            row: start.row,
            column: start.column,
        }),
        end_position: Some(Position {
            row: end.row,
            column: end.column,
        }),
        children: (!children.is_empty()).then_some(children),
        text: node.utf8_text(source).ok().map(ToOwned::to_owned),
    }
}

#[test]
fn web_parse_path_matches_native_for_native_mml_cases() {
    for mml in PARITY_CASES {
        let native_tokens = pass1_parser::parse_mml(mml);
        let parse_tree = parse_tree_from_mml(mml);
        let mut chord_id = 0;
        let web_tokens = parse_tree_to_tokens(&parse_tree, None, &mut chord_id);

        assert_eq!(web_tokens, native_tokens, "token mismatch for {mml}");

        let native_ast = pass2_ast::tokens_to_ast(&native_tokens);
        let web_ast = pass2_ast::tokens_to_ast(&web_tokens);
        assert_eq!(web_ast, native_ast, "AST mismatch for {mml}");

        let native_events = pass3_events::ast_to_events(&native_ast, true);
        let web_events = pass3_events::ast_to_events(&web_ast, true);
        assert_eq!(web_events, native_events, "event mismatch for {mml}");

        let native_midi = pass4_midi::events_to_midi(&native_events)
            .unwrap_or_else(|_| panic!("native MIDI conversion failed for {mml}"));
        let web_midi = pass4_midi::events_to_midi(&web_events)
            .unwrap_or_else(|_| panic!("web MIDI conversion failed for {mml}"));
        assert_eq!(web_midi, native_midi, "MIDI mismatch for {mml}");
    }
}
