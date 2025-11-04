//! Pass 4: Convert MIDI events to Standard MIDI File
//! Outputs SMF file.

use crate::types::MidiEvent;
use anyhow::Result;
use midly::{Format, Header, MetaMessage, MidiMessage, Timing, TrackEvent, TrackEventKind};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

// Default tempo: 500000 microseconds per beat = 120 BPM
const DEFAULT_TEMPO_USEC_PER_BEAT: u32 = 500000;

/// Convert MIDI events to SMF data
///
/// # Arguments
/// * `events` - List of MIDI event structures from Pass 3
///
/// # Returns
/// SMF data as bytes
pub fn events_to_midi(events: &[MidiEvent]) -> Result<Vec<u8>> {
    // Check if we have multiple channels
    let has_multiple_channels = events.iter().any(|e| e.channel > 0);

    // Create header: Use Format 1 if multiple channels, Format 0 otherwise
    let format = if has_multiple_channels {
        Format::Parallel
    } else {
        Format::SingleTrack
    };
    let header = Header::new(format, Timing::Metrical(480.into()));

    let mut tracks = Vec::new();

    if has_multiple_channels {
        // Format 1: Separate tracks for each channel
        // First, create a tempo track
        let tempo_track = vec![
            TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(DEFAULT_TEMPO_USEC_PER_BEAT.into())),
            },
            TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            },
        ];
        tracks.push(tempo_track);

        // Get unique channels and create a track for each
        let channels: Vec<u8> = {
            let mut unique_channels: Vec<u8> = events
                .iter()
                .map(|e| e.channel)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            unique_channels.sort_unstable();
            unique_channels
        };

        for channel in channels {
            let mut track = Vec::new();

            // Filter events for this channel and sort by time
            let mut channel_events: Vec<_> = events
                .iter()
                .filter(|e| e.channel == channel)
                .cloned()
                .collect();
            channel_events.sort_by_key(|e| e.time);

            let mut prev_time = 0;
            for event in channel_events {
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
                        channel: channel.into(),
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

            tracks.push(track);
        }
    } else {
        // Format 0: Single track
        let mut track = Vec::new();

        // Add tempo
        track.push(TrackEvent {
            delta: 0.into(),
            kind: TrackEventKind::Meta(MetaMessage::Tempo(DEFAULT_TEMPO_USEC_PER_BEAT.into())),
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
                    channel: event.channel.into(),
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

        tracks.push(track);
    }

    // Create SMF structure
    let smf = midly::Smf { header, tracks };

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
