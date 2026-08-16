# mmlabc-to-smf-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.com/cat2151/mmlabc-to-smf-rust"><img src="https://img.shields.io/badge/📖-DeepWiki-blue.svg" alt="DeepWiki"></a>
</p>

This is a Rust library and CLI for converting Music Macro Language (MML) to Standard MIDI File (SMF).

## Overview

- Rust Library: `mmlabc_to_smf`
- CLI Binary: `mmlabc-to-smf`
- Includes `mmlabc-to-smf-wasm/` for browsers, and `demo/` and `demo-library/` for verification.
- Primary use case is as an MML compilation engine for projects like clap-mml-render-tui and web-ym2151.

### Supplementary Notes
- MML to SMF conversion consists of 4 passes.
- The library API does not output intermediate files and can generate SMF byte sequences in memory.
- When executing the CLI, intermediate results of each pass are output as JSON for debugging.

## Current Implementation Status

The README has been updated to reflect the current implementation. At least the following features have been implemented and tested in the current codebase:

- Basic Notes: `c d e f g a b`
- Sharps / Flats: `+`, `-`
- Rests: `r`
- Note Length Specification: `c4`, `d8`, `l8`, `l4.`
- Dotted Notes: `c4.`, `c4..`, `c1....`
- Octave Operations: `<` (octave up), `>` (octave down), `o4`, `o5` (set octave)
- Chords: `'ceg'`
- Multiple Channels: `;`
- Tempo: `t120`
- Velocity: `v1` to `v15`
- Program Change: `@0` to `@127`
- Key Transposition: `kt1`, `kt-2`
- Attachment JSON Output: `--attachment-output`
- Embedded attachment JSON at the beginning of MML: `[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde`
- Special marker for drum channel assignment for channels containing `@128` (can be disabled in settings)

You can check the number and list of tests with `cargo test -- --list`.

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

By default, it attempts to play the generated MIDI with `cat-play-mml`. This can be changed to a different player in the configuration file.

### Library

To obtain only the SMF byte sequence without creating intermediate JSON files, use the top-level API.

```toml
mmlabc-to-smf = { git = "https://github.com/cat2151/mmlabc-to-smf-rust.git", package = "mmlabc-to-smf", default-features = false, features = ["parser"] }
```

```rust
use mmlabc_to_smf::{mml_to_smf_bytes, raw_mml_to_smf_bytes_with_options, SmfConversionOptions};

let smf_bytes = mml_to_smf_bytes("cde")?;

let options = SmfConversionOptions {
    use_drum_channel_for_128: false,
};
let smf_bytes = raw_mml_to_smf_bytes_with_options("@0c;@128d", options)?;
```

`mml_to_smf_bytes` removes embedded attachment JSON from the beginning of the MML before conversion. For MML where JSON has already been removed, you can use `raw_mml_to_smf_bytes` / `raw_mml_to_smf_bytes_with_options`.

Public modules:

- `attachment_json`
- `config`
- `mml_preprocessor`
- `pass2_ast`
- `pass3_events`
- `pass4_midi`
- `types`
- `pass1_parser`, `tree_sitter_mml` (when `parser` feature is enabled. The `cli` feature includes `parser`)

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
| Key Transpose | `ktN`, `kt-N` | `kt2c`, `kt-1d` |

Notes:

- The default note length is `l8` (eighth note).
- `v1` to `v15` are internally converted to MIDI velocity (`0` to `127`).
- `@128`, when used within a channel separated by `;`, is assigned to MIDI channel 9 (0-based) by default.

## Attachment JSON

By using `--attachment-output`, you can output attachment JSON for each Program Change.

```bash
cargo run -- "@1cde" --no-play \
  --attachment-output attachment.json \
  -o output.mid
```

Additionally, if you write a JSON object or array at the beginning of the MML string, that part will be extracted and used as attachment JSON.

```text
[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde
```

## Configuration File

You can change the behavior by placing `mmlabc-to-smf-rust.toml` in the execution directory.

```toml
external_smf_player = "cat-play-mml"
use_drum_channel_for_128 = true
```

- `external_smf_player`: The command used for automatic playback.
- `use_drum_channel_for_128`: Whether to assign channels containing `@128` to the drum channel.

For details, please refer to [`mmlabc-to-smf-rust.toml.example`](mmlabc-to-smf-rust.toml.example).

## Debug Output

The following files are output during CLI execution:

- `pass1_tokens.json`
- `pass2_ast.json`
- `pass3_events.json`
- `output.mid` (or the file specified by `--output`)
- Attachment JSON when `--attachment-output` is specified.

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

### If modifying the tree-sitter grammar

When updating `tree-sitter-mml/grammar.js`, the generated artifacts must also be updated accordingly.

```bash
cargo build
# Or
cd tree-sitter-mml
npm install
npx tree-sitter generate
```

For a regular Rust build, Node.js is not required as committed artifacts are available.

### Demos

- `demo/`: Browser demo
- `demo-library/`: Library usage example
- `mmlabc-to-smf-wasm/`: WASM crate for the web

## References

- mmlabc command system: [cat2151/mml2abc](https://github.com/cat2151/mml2abc)
- Original Python implementation: [cat2151/mmlabc-to-smf](https://github.com/cat2151/mmlabc-to-smf)

## License

MIT License. See [LICENSE](LICENSE) for details.