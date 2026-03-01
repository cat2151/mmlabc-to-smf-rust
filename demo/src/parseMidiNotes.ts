// Parses SMF binary data to extract note events with timing information
import { MidiReader } from './midiReader.js';

export interface NoteInfo {
    note: number;
    start: number;
    duration: number;
    velocity: number;
}

export function parseMidiNotes(smfData: Uint8Array): NoteInfo[] {
    const notes: NoteInfo[] = [];

    try {
        if (!smfData || smfData.length < 14) {
            return notes;
        }

        const reader = new MidiReader(smfData);

        if (reader.readChar4() !== 'MThd') {
            return notes;
        }

        const headerLength = reader.readUint32();
        reader.readUint16(); // formatType (unused)
        const numTracks = reader.readUint16();
        const division = reader.readUint16();

        if (headerLength > 6) {
            reader.skip(headerLength - 6);
        }

        let tempoMicrosPerQuarter = 500000;

        function deltaTicksToSeconds(deltaTicks: number): number {
            if (division === 0) return 0;
            return (deltaTicks * tempoMicrosPerQuarter) / (division * 1e6);
        }

        const activeNotes = new Map<number, { start: number; velocity: number }>();

        for (let trackIndex = 0; trackIndex < numTracks; trackIndex++) {
            if (reader.offset + 8 > reader.length) break;

            const id = reader.readChar4();
            const trackLength = reader.readUint32();
            const trackEnd = reader.offset + trackLength;

            if (id !== 'MTrk') {
                reader.offset = trackEnd;
                continue;
            }

            let currentTimeSeconds = 0.0;
            let runningStatus: number | null = null;

            while (reader.offset < trackEnd) {
                const deltaTicks = reader.readVlq();
                currentTimeSeconds += deltaTicksToSeconds(deltaTicks);

                if (reader.offset >= trackEnd) break;

                let statusByte = reader.readUint8();

                if (statusByte < 0x80) {
                    if (runningStatus === null) break;
                    reader.offset--;
                    statusByte = runningStatus;
                } else {
                    runningStatus = statusByte;
                }

                // Meta event
                if (statusByte === 0xff) {
                    if (reader.offset >= trackEnd) break;
                    const metaType = reader.readUint8();
                    const length = reader.readVlq();
                    if (reader.offset + length > trackEnd) {
                        reader.offset = trackEnd;
                        break;
                    }
                    if (metaType === 0x51 && length === 3) {
                        tempoMicrosPerQuarter =
                            (smfData[reader.offset] << 16) |
                            (smfData[reader.offset + 1] << 8) |
                            smfData[reader.offset + 2];
                    }
                    reader.skip(length);
                    continue;
                }

                // SysEx events
                if (statusByte === 0xf0 || statusByte === 0xf7) {
                    const length = reader.readVlq();
                    reader.skip(length);
                    continue;
                }

                const statusHighNibble = statusByte & 0xf0;

                let dataBytesNeeded = 0;
                if (statusHighNibble === 0xc0 || statusHighNibble === 0xd0) {
                    dataBytesNeeded = 1;
                } else if (statusHighNibble >= 0x80 && statusHighNibble <= 0xe0) {
                    dataBytesNeeded = 2;
                } else {
                    break;
                }

                if (reader.offset + dataBytesNeeded > trackEnd) break;

                const data1 = reader.readUint8();
                const data2 = dataBytesNeeded === 2 ? reader.readUint8() : 0;

                if (statusHighNibble === 0x90 || statusHighNibble === 0x80) {
                    const note = data1;
                    const velocity = data2;
                    const isNoteOn = statusHighNibble === 0x90 && velocity > 0;

                    if (isNoteOn) {
                        activeNotes.set(note, { start: currentTimeSeconds, velocity: velocity / 127.0 });
                    } else {
                        if (activeNotes.has(note)) {
                            const noteStart = activeNotes.get(note)!;
                            notes.push({
                                note,
                                start: noteStart.start,
                                duration: currentTimeSeconds - noteStart.start,
                                velocity: noteStart.velocity,
                            });
                            activeNotes.delete(note);
                        }
                    }
                }
            }

            reader.offset = trackEnd;
        }

        // Handle notes without a note-off (give them a default duration)
        for (const [note, noteStart] of activeNotes.entries()) {
            notes.push({ note, start: noteStart.start, duration: 0.25, velocity: noteStart.velocity });
        }
    } catch (e) {
        console.warn('Error parsing MIDI notes:', e);
    }

    return notes;
}
