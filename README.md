# mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.com/cat2151/mmlabc-to-smf-rust"><img src="https://img.shields.io/badge/📖-DeepWiki-blue.svg" alt="DeepWiki"></a>
</p>

A Rust-based library and CLI for converting Music Macro Language (MML) to Standard MIDI File (SMF).

## Overview

- Rust Library: `mmlabc_to_smf`
- CLI Binary: `mmlabc-to-smf`
- Converts MML to SMF in a 4-pass structure
- Outputs intermediate results of each pass as JSON
- Includes `mmlabc-to-smf-wasm/` for browsers, and `demo/` and `demo-library/` for verification

## Current Implementation Status

The README content has been updated to reflect the implementation. The current codebase implements and tests at least the following:

- Basic notes: `c d e f g a b`
- Sharps / Flats: `+`, `-`
- Rests: `r`
- Note length specification: `c4`, `d8`, `l8`, `l4.`
- Dotted notes: `c4.`, `c4..`, `c1....`
- Octave operations: `<`, `>`, `o4`, `o5`
- Chords: `'ceg'`
- Multiple channels: `;`
- Tempo: `t120`
- Velocity: `v1` to `v15`
- Program Change: `@0` to `@127`
- Key Transposition: `kt1`, `kt-2`
- Attachment JSON output: `--attachment-output`
- Embedded attachment JSON at the beginning of MML: `[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde`
- Special marker for drum channel assignment for channels containing `@128` (can be disabled in settings)

The number of tests and a list can be checked with `cargo test -- --list`.

## Usage

### CLI

```bash
cargo run -- "cdefgab" --no-play
cargo run -- "t120 l4 c d e f" --no-play
cargo run -- "o4 'ceg' r8 >c" --no-play
cargo run -- "@0c;@128d;@1e" --no-play -o output.mid
```

Main options:

- `-o, --output <PATH>`: Output SMF file (default: `output.mid`)
- `--attachment-output <PATH>`: Output attachment JSON
- `--no-play`: Disable automatic playback after generation

By default, the generated MIDI will attempt to play using `cat-play-mml`. This player can be changed in the configuration file.

### Library

Public modules:

- `attachment_json`
- `config`
- `mml_preprocessor`
- `pass2_ast`
- `pass3_events`
- `pass4_midi`
- `types`
- `pass1_parser`, `tree_sitter_mml` (when `cli` feature is enabled)

## Supported MML Notation

| Type | Notation | Example |
| --- | --- | --- |
| Notes | `cdefgab` | `cde` |
| Modifiers | `+`, `-` | `c+ d-` |
| Rests | `r` | `cr8d` |
| Note Length | Number / `l` | `c4`, `l8cde` |
| Dots | `.` | `c4.`, `l4..c` |
| Octave | `<` = octave up, `>` = octave down, `oN` = set octave | `o4c<d>e` |
| Chords | `'...'` | `'ceg'` |
| Channel Split | `;` | `c;e;g` |
| Tempo | `tN` | `t120c` |
| Velocity | `vN` | `v8cde` |
| Program | `@N` | `@1c` |
| Key Transposition | `ktN`, `kt-N` | `kt2c`, `kt-1d` |

Notes:

- The default note length is `l8` (eighth note).
- `v1` to `v15` are internally converted to MIDI velocity (`0` to `127`).
- `@128` is by default assigned to MIDI channel 9 (0-based) when used in a channel separated by `;`.

## Attachment JSON

Using `--attachment-output`, you can output attachment JSON for each Program Change.

```bash
cargo run -- "@1cde" --no-play \
  --attachment-output attachment.json \
  -o output.mid
```

Also, if you write a JSON object or JSON array at the beginning of the MML string, that part will be extracted and used as attachment JSON.

```text
[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde
```

## Configuration File

You can modify the behavior by placing `mmlabc-to-smf-rust.toml` in the execution directory.

```toml
external_smf_player = "cat-play-mml"
use_drum_channel_for_128 = true
```

- `external_smf_player`: Command used for automatic playback
- `use_drum_channel_for_128`: Whether to assign channels containing `@128` to the drum channel

Refer to [`mmlabc-to-smf-rust.toml.example`](mmlabc-to-smf-rust.toml.example) for details.

## Debug Output

When the CLI is executed, the following files are output:

- `pass1_tokens.json`
- `pass2_ast.json`
- `pass3_events.json`
- `output.mid` (or the file specified by `--output`)
- Attachment JSON if `--attachment-output` is specified

## Development

### Build / Test / Lint

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check
```

Format if necessary:

```bash
cargo fmt
```

### Changing tree-sitter Grammar

When `tree-sitter-mml/grammar.js` is updated, the generated artifacts must also be updated accordingly.

```bash
cargo build
# or
cd tree-sitter-mml
npm install
npx tree-sitter generate
```

Node.js is not required for a regular Rust build if the committed generated artifacts are already present.

### Demo

- `demo/`: Browser demo
- `demo-library/`: Library usage example
- `mmlabc-to-smf-wasm/`: Web-oriented WASM crate

## References

- mmlabc command system: [cat2151/mml2abc](https://github.com/cat2151/mml2abc)
- Original Python implementation: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)

## License

MIT License. See [LICENSE](LICENSE) for details.