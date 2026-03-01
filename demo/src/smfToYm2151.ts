// Converts an SMF (Standard MIDI File) to simplified YM2151 log format JSON
import { MidiReader } from './midiReader.js';

interface YM2151Event {
    time: number;
    address: number;
    data: number;
}

export interface YM2151Json {
    events: YM2151Event[];
}

export function smfToYM2151Json(smfData: Uint8Array): YM2151Json {
    const events: YM2151Event[] = [];

    try {
        if (!smfData || smfData.length < 14) {
            return { events };
        }

        const reader = new MidiReader(smfData);

        if (reader.readChar4() !== 'MThd') {
            throw new Error('Not an SMF file (missing MThd)');
        }

        const headerLength = reader.readUint32();
        reader.readUint16(); // _formatType (read to advance buffer position)
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
                    if (runningStatus === null) {
                        throw new Error('Running status without previous status');
                    }
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
                    const isNoteOff = statusHighNibble === 0x80 || !isNoteOn;

                    if (isNoteOn) {
                        events.push({ time: currentTimeSeconds, address: 0x08, data: 0x78 });
                        events.push({ time: currentTimeSeconds + 0.001, address: 0x28, data: 0x00 });
                        events.push({
                            time: currentTimeSeconds + 0.002,
                            address: 0x30,
                            data: (Math.floor(note / 12) << 4) | (note % 12),
                        });
                    } else if (isNoteOff) {
                        events.push({ time: currentTimeSeconds, address: 0x08, data: 0x00 });
                    }
                }
            }

            reader.offset = trackEnd;
        }
    } catch (e) {
        console.warn('Error parsing MIDI:', e);
    }

    return { events };
}
