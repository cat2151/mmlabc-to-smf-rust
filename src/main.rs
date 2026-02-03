//! MML to SMF Converter - Main CLI
//!
//! Converts Music Macro Language format string to Standard MIDI File
//! using a 4-pass architecture.

use anyhow::Result;
use clap::Parser;
use mmlabc_to_smf::{config::Config, pass1_parser, pass2_ast, pass3_events, pass4_midi};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MML format string (e.g., "cde")
    mml_string: String,

    /// Output MIDI file path
    #[arg(short, long, default_value = "output.mid")]
    output: String,

    /// Disable auto-playing the generated MIDI file
    /// Default player is cat-play-mml, configurable via mmlabc-to-smf-rust.toml
    #[arg(long, default_value_t = false)]
    no_play: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration
    let config = Config::load().unwrap_or_default();

    println!("Converting MML: {}", args.mml_string);

    // Pass 1: Parse MML string to tokens
    println!("Pass 1: Parsing MML...");
    let tokens = pass1_parser::process_pass1(&args.mml_string, "pass1_tokens.json")?;
    println!("  Generated {} tokens → pass1_tokens.json", tokens.len());

    // Pass 2: Convert tokens to AST
    println!("Pass 2: Creating AST...");
    let ast = pass2_ast::process_pass2(&tokens, "pass2_ast.json")?;
    println!(
        "  Generated AST with {} notes → pass2_ast.json",
        ast.notes.len()
    );

    // Pass 3: Convert AST to MIDI events
    println!("Pass 3: Creating MIDI events...");
    let events = pass3_events::process_pass3(
        &ast,
        "pass3_events.json",
        config.use_drum_channel_for_128,
    )?;
    println!("  Generated {} events → pass3_events.json", events.len());

    // Pass 4: Convert events to SMF
    println!("Pass 4: Creating MIDI file...");
    pass4_midi::process_pass4(&events, &args.output)?;
    println!("  Generated MIDI file → {}", args.output);

    println!("\nConversion complete!");
    println!("Output files:");
    println!("  - pass1_tokens.json (debug)");
    println!("  - pass2_ast.json (debug)");
    println!("  - pass3_events.json (debug)");
    println!("  - {} (final output)", args.output);

    // Auto-play the generated MIDI file (unless --no-play is specified)
    if !args.no_play {
        let player_command = config.get_player_command();
        println!("\nAttempting to play MIDI file with {}...", player_command);
        match Command::new(player_command).arg(&args.output).spawn() {
            Ok(mut child) => {
                println!("Started {} with {}", player_command, args.output);
                // Wait for the player to finish
                match child.wait() {
                    Ok(status) => {
                        if status.success() {
                            println!("Playback completed successfully");
                        } else {
                            println!("Playback exited with status: {}", status);
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to wait for {}: {}", player_command, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to start {}: {}", player_command, e);
                eprintln!(
                    "The MIDI file was generated successfully, but {} could not be executed.",
                    player_command
                );
                eprintln!("To play the file, install {} or configure a different player in mmlabc-to-smf-rust.toml.", player_command);
                eprintln!("To disable auto-play, use the --no-play flag.");
            }
        }
    }

    Ok(())
}
