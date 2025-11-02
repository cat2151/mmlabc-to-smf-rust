//! Pass 4: Convert MIDI events to Standard MIDI File
//! Outputs SMF file.

use crate::types::MidiEvent;
use anyhow::Result;
use midly::{Format, Header, MetaMessage, MidiMessage, Timing, TrackEvent, TrackEventKind};
use std::fs::File;
use std::io::Write;

/// Convert MIDI events to SMF data
///
/// # Arguments
/// * `events` - List of MIDI event structures from Pass 3
///
/// # Returns
/// SMF data as bytes
pub fn events_to_midi(events: &[MidiEvent]) -> Result<Vec<u8>> {
    // Create header: Format 0 (single track), 480 ticks per beat
    let header = Header::new(Format::SingleTrack, Timing::Metrical(480.into()));

    // Create track
    let mut track = Vec::new();

    // Add tempo (500000 microseconds per beat = 120 BPM)
    track.push(TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Meta(MetaMessage::Tempo(500000.into())),
    });

    // Sort events by time
    let mut sorted_events = events.to_vec();
    sorted_events.sort_by_key(|e| e.time);

    let mut prev_time = 0;
    for event in sorted_events {
        let delta_time = event.time - prev_time;

        let midi_msg = match event.event_type.as_str() {
            "note_on" => MidiMessage::NoteOn {
                key: event.note.into(),
                vel: event.velocity.into(),
            },
            "note_off" => MidiMessage::NoteOff {
                key: event.note.into(),
                vel: event.velocity.into(),
            },
            _ => continue,
        };

        track.push(TrackEvent {
            delta: delta_time.into(),
            kind: TrackEventKind::Midi {
                channel: 0.into(),
                message: midi_msg,
            },
        });

        prev_time = event.time;
    }

    // Add end of track
    track.push(TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    // Create SMF structure
    let smf = midly::Smf {
        header,
        tracks: vec![track],
    };

    // Write to bytes
    let mut buffer = Vec::new();
    smf.write_std(&mut buffer)?;

    Ok(buffer)
}

/// Save MIDI data to SMF file
///
/// # Arguments
/// * `midi_data` - SMF data as bytes
/// * `filepath` - Output SMF file path
pub fn save_midi_file(midi_data: &[u8], filepath: &str) -> Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(midi_data)?;
    Ok(())
}

/// Execute Pass 4: Create SMF from MIDI events
///
/// # Arguments
/// * `events` - List of MIDI events from Pass 3
/// * `output_smf` - Output SMF file path
///
/// # Returns
/// SMF data as bytes
pub fn process_pass4(events: &[MidiEvent], output_smf: &str) -> Result<Vec<u8>> {
    let midi_data = events_to_midi(events)?;
    save_midi_file(&midi_data, output_smf)?;
    Ok(midi_data)
}
