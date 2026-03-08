use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;

#[test]
fn test_c1_vs_c64_durations() {
    // Test c1 (whole note)
    let tokens_c1 = parse_mml("c1");
    assert_eq!(tokens_c1.len(), 1, "c1 should produce exactly 1 token");
    let ast_c1 = tokens_to_ast(&tokens_c1);
    assert_eq!(ast_c1.notes.len(), 1, "c1 AST should have exactly 1 note");
    let events_c1 = ast_to_events(&ast_c1, false);
    assert!(
        events_c1.len() > 1,
        "c1 should generate at least 2 events (note_on + note_off)"
    );
    let c1_duration = events_c1[1].time - events_c1[0].time;

    // Test c64 (64th note)
    let tokens_c64 = parse_mml("c64");
    assert_eq!(tokens_c64.len(), 1, "c64 should produce exactly 1 token");
    let ast_c64 = tokens_to_ast(&tokens_c64);
    assert_eq!(ast_c64.notes.len(), 1, "c64 AST should have exactly 1 note");
    let events_c64 = ast_to_events(&ast_c64, false);
    assert!(
        events_c64.len() > 1,
        "c64 should generate at least 2 events (note_on + note_off)"
    );
    let c64_duration = events_c64[1].time - events_c64[0].time;

    assert_eq!(c1_duration, 1920, "c1 (whole note) should be 1920 ticks");
    assert_eq!(c64_duration, 30, "c64 (64th note) should be 30 ticks");
    assert_ne!(
        c1_duration, c64_duration,
        "c1 and c64 should have different durations"
    );
}
