//! Map positions in MML text to the sounding units they belong to.
//!
//! An editor that wants to preview "the note under the cursor" needs to know
//! which piece of text is one musical event. Splitting the text itself would
//! mean a second interpretation of the MML grammar, so this module reads the
//! same parse tree the converter uses: every unit is one node of the CST, and
//! its [`SoundingUnit::note_indices`] point straight into `ast.notes` of
//! [`crate::pass2_ast`].
//!
//! ```text
//! sounding_units("o5 'ceg' r4 c")
//!   -> [ Command "o5", Chord "'ceg'", Rest "r4", Note "c" ]
//! ```
//!
//! A chord is a single unit covering both quotes, so a cursor sitting on the
//! closing `'` is still inside the chord. Whitespace belongs to no unit.

use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::parse_tree_tokens::SpannedToken;
use crate::types::Token;
use crate::{mml_preprocessor, pass1_parser, pass2_ast};

/// What kind of musical event a unit is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitKind {
    /// A single note, including its modifier, length and dots (`d+4.`).
    Note,
    /// A chord (`'ceg'`), quotes included.
    Chord,
    /// A rest (`r4`).
    Rest,
    /// Anything that changes later interpretation without sounding by itself:
    /// `o5`, `l8`, `<`, `>`, `v12`, `t120`, `@1`, `kt-2`.
    Command,
}

impl UnitKind {
    /// Whether the unit produces sound on its own.
    pub fn is_sounding(self) -> bool {
        matches!(self, UnitKind::Note | UnitKind::Chord)
    }
}

/// One musical event, and where it is written.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundingUnit {
    /// Byte range of the unit in the text that was passed in.
    pub byte_range: Range<usize>,
    pub kind: UnitKind,
    /// Indices into `ast.notes` of [`pass2_ast::tokens_to_ast`] for the same
    /// MML text. A chord has one index per constituent note; a `Command` unit
    /// that reaches the AST (`@1`, `t120`) has one; the rest have none.
    pub note_indices: Vec<usize>,
    /// The unit is written but not finished, such as `'ceg` with no closing
    /// quote. It still parses, so it still has notes.
    pub incomplete: bool,
}

/// Split MML text into sounding units.
///
/// A leading attachment JSON block is stripped as in
/// [`crate::mml_to_smf_bytes`], but the returned byte ranges stay relative to
/// `mml` — the text the caller holds — not to the stripped MML.
pub fn sounding_units(mml: &str) -> Vec<SoundingUnit> {
    let preprocessed = mml_preprocessor::extract_embedded_json(mml);
    let offset = preprocessed.remaining_offset;
    let mut units = raw_sounding_units(&preprocessed.remaining_mml);
    if offset != 0 {
        for unit in &mut units {
            unit.byte_range.start += offset;
            unit.byte_range.end += offset;
        }
    }
    units
}

/// Split MML text with no attachment JSON prefix into sounding units.
pub fn raw_sounding_units(mml: &str) -> Vec<SoundingUnit> {
    let spanned = pass1_parser::parse_mml_spanned(mml);
    let tokens: Vec<Token> = spanned.iter().map(|s| s.token.clone()).collect();
    let (_, note_sources) = pass2_ast::tokens_to_ast_with_sources(&tokens);
    build_units(&spanned, &note_sources)
}

/// The unit a text cursor sitting *before* byte `byte` touches.
///
/// A cursor is between two bytes, so the unit it just left wins: a unit ending
/// exactly at `byte` is preferred over the one covering `byte`. Walking through
/// `cde` with the arrow keys therefore reports `c`, `d`, `e` as the cursor
/// passes each note, and a cursor at the very end of `'ceg'` still reports the
/// chord. With nothing to its left the cursor takes the unit on its right, so a
/// cursor at the very start of `cde` reports `c`.
///
/// Returns `None` when the cursor touches no unit at all, such as inside
/// whitespace. Use [`unit_covering`] instead for plain containment.
pub fn unit_at(units: &[SoundingUnit], byte: usize) -> Option<usize> {
    units
        .iter()
        .position(|unit| unit.byte_range.end == byte)
        .or_else(|| unit_covering(units, byte))
}

/// The unit whose text contains byte `byte`, if any.
pub fn unit_covering(units: &[SoundingUnit], byte: usize) -> Option<usize> {
    units
        .iter()
        .position(|unit| unit.byte_range.start <= byte && byte < unit.byte_range.end)
}

/// Every unit overlapping `range`, for playing back a selection.
///
/// A unit counts as selected when any of its bytes is inside `range`, so a
/// partly selected chord is included whole. An empty range selects nothing.
pub fn units_in(units: &[SoundingUnit], range: Range<usize>) -> Vec<usize> {
    if range.start >= range.end {
        return Vec::new();
    }
    units
        .iter()
        .enumerate()
        .filter(|(_, unit)| unit.byte_range.start < range.end && range.start < unit.byte_range.end)
        .map(|(index, _)| index)
        .collect()
}

/// Group tokens that share a byte range into units, and hand each unit the AST
/// entries its tokens produced.
///
/// Every token of a chord carries the chord's range, so equal adjacent ranges
/// are exactly the chord grouping the grammar already made.
fn build_units(spanned: &[SpannedToken], note_sources: &[usize]) -> Vec<SoundingUnit> {
    let mut units = Vec::new();
    let mut ast_cursor = 0;
    let mut token_start = 0;

    while token_start < spanned.len() {
        let range = &spanned[token_start].byte_range;
        let mut token_end = token_start + 1;
        while token_end < spanned.len() && &spanned[token_end].byte_range == range {
            token_end += 1;
        }

        // AST entries come out in token order, so one forward walk assigns them.
        let mut note_indices = Vec::new();
        while ast_cursor < note_sources.len() && note_sources[ast_cursor] < token_end {
            if note_sources[ast_cursor] >= token_start {
                note_indices.push(ast_cursor);
            }
            ast_cursor += 1;
        }

        if let Some(byte_range) = range.clone() {
            let group = &spanned[token_start..token_end];
            units.push(SoundingUnit {
                byte_range,
                kind: unit_kind(group),
                note_indices,
                incomplete: group.iter().any(|spanned| spanned.incomplete),
            });
        }

        token_start = token_end;
    }

    units
}

fn unit_kind(group: &[SpannedToken]) -> UnitKind {
    let notes = || group.iter().filter(|s| s.token.token_type == "note");

    if notes().any(|spanned| spanned.token.chord_id.is_some()) {
        UnitKind::Chord
    } else if notes().next().is_some() {
        UnitKind::Note
    } else if group.iter().any(|s| s.token.token_type == "rest") {
        UnitKind::Rest
    } else {
        UnitKind::Command
    }
}
