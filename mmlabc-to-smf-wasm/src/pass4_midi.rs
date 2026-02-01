//! Pass 4: Convert MIDI events to Standard MIDI File

use crate::types::MidiEvent;
use anyhow::Result;
use midly::{Format, Header, MetaMessage, MidiMessage, Timing, TrackEvent, TrackEventKind};
use std::collections::HashSet;

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
        // First, create a tempo track with all tempo events
        let mut tempo_track = Vec::new();

        // Add default tempo at time 0
        tempo_track.push(TrackEvent {
            delta: 0.into(),
            kind: TrackEventKind::Meta(MetaMessage::Tempo(DEFAULT_TEMPO_USEC_PER_BEAT.into())),
        });

        // Collect all tempo_set events and add to tempo track
        let mut tempo_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == "tempo_set")
            .cloned()
            .collect();
        tempo_events.sort_by_key(|e| e.time);

        let mut prev_time = 0;
        for event in tempo_events {
            let delta_time = event.time - prev_time;
            tempo_track.push(TrackEvent {
                delta: delta_time.into(),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(event.tempo.unwrap().into())),
            });
            prev_time = event.time;
        }

        // End of tempo track
        tempo_track.push(TrackEvent {
            delta: 0.into(),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        tracks.push(tempo_track);

        // Get unique channels and create a track for each
        let channels: Vec<u8> = {
            let mut unique_channels: Vec<u8> = events
                .iter()
                .filter(|e| e.event_type != "tempo_set") // Exclude tempo events
                .map(|e| e.channel)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            unique_channels.sort_unstable();
            unique_channels
        };

        for channel in channels {
            let mut track = Vec::new();

            // Filter events for this channel and sort by time (exclude tempo events)
            let mut channel_events: Vec<_> = events
                .iter()
                .filter(|e| e.channel == channel && e.event_type != "tempo_set")
                .cloned()
                .collect();
            channel_events.sort_by_key(|e| e.time);

            let mut prev_time = 0;
            for event in channel_events {
                let delta_time = event.time - prev_time;

                let midi_msg = match event.event_type.as_str() {
                    "note_on" => MidiMessage::NoteOn {
                        key: event.note.unwrap().into(),
                        vel: event.velocity.unwrap().into(),
                    },
                    "note_off" => MidiMessage::NoteOff {
                        key: event.note.unwrap().into(),
                        vel: event.velocity.unwrap().into(),
                    },
                    "program_change" => MidiMessage::ProgramChange {
                        program: event.program.unwrap().into(),
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

            // Handle tempo_set events (Meta messages)
            if event.event_type == "tempo_set" {
                track.push(TrackEvent {
                    delta: delta_time.into(),
                    kind: TrackEventKind::Meta(MetaMessage::Tempo(event.tempo.unwrap().into())),
                });
                prev_time = event.time;
                continue;
            }

            let midi_msg = match event.event_type.as_str() {
                "note_on" => MidiMessage::NoteOn {
                    key: event.note.unwrap().into(),
                    vel: event.velocity.unwrap().into(),
                },
                "note_off" => MidiMessage::NoteOff {
                    key: event.note.unwrap().into(),
                    vel: event.velocity.unwrap().into(),
                },
                "program_change" => MidiMessage::ProgramChange {
                    program: event.program.unwrap().into(),
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

