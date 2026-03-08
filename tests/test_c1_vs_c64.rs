use mmlabc_to_smf::pass1_parser::*;
use mmlabc_to_smf::pass2_ast::*;
use mmlabc_to_smf::pass3_events::*;

#[test]
fn test_c1_vs_c64_durations() {
    println!("\n=== Testing c1 vs c64 ===\n");
    
    // Test c1 (whole note)
    println!("Testing c1:");
    let tokens_c1 = parse_mml("c1");
    println!("  Token note_length: {:?}", tokens_c1[0].note_length);
    
    let ast_c1 = tokens_to_ast(&tokens_c1);
    println!("  AST length: {:?}", ast_c1.notes[0].length);
    
    let events_c1 = ast_to_events(&ast_c1, false);
    let c1_duration = if events_c1.len() > 1 {
        events_c1[1].time - events_c1[0].time
    } else {
        0
    };
    println!("  Duration: {} ticks", c1_duration);
    println!("  note_on: time={}, note_off: time={}", events_c1[0].time, events_c1[1].time);
    
    // Test c64 (64th note)
    println!("\nTesting c64:");
    let tokens_c64 = parse_mml("c64");
    println!("  Token note_length: {:?}", tokens_c64[0].note_length);
    
    let ast_c64 = tokens_to_ast(&tokens_c64);
    println!("  AST length: {:?}", ast_c64.notes[0].length);
    
    let events_c64 = ast_to_events(&ast_c64, false);
    let c64_duration = if events_c64.len() > 1 {
        events_c64[1].time - events_c64[0].time
    } else {
        0
    };
    println!("  Duration: {} ticks", c64_duration);
    println!("  note_on: time={}, note_off: time={}", events_c64[0].time, events_c64[1].time);
    
    // Compare durations
    println!("\n=== Duration Comparison ===");
    println!("c1 duration: {} ticks (expected 1920)", c1_duration);
    println!("c64 duration: {} ticks (expected 30)", c64_duration);
    println!("Durations equal: {}", c1_duration == c64_duration);
    
    // Test calculations manually
    println!("\n=== Manual Duration Calculation ===");
    println!("c1: 1920 / 1 = {}", 1920 / 1);
    println!("c64: 1920 / 64 = {}", 1920 / 64);
    
    assert_eq!(c1_duration, 1920, "c1 (whole note) should be 1920 ticks");
    assert_eq!(c64_duration, 30, "c64 (64th note) should be 30 ticks");
    assert_ne!(c1_duration, c64_duration, "c1 and c64 should have different durations");
}
