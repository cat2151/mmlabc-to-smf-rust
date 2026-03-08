// Tests for parseMidiNotes using Node.js built-in test runner.
// Run with: node --experimental-strip-types --test tests/parseMidiNotes.test.ts
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { parseMidiNotes } from '../src/parseMidiNotes.ts';

// Minimal SMF helper: format 0, 1 track, given ticks-per-quarter.
// events: array of {deltaTicks, status, data1, data2?}
function buildSmf(ticksPerQuarter: number, events: { deltaTicks: number; status: number; data1: number; data2?: number }[]): Uint8Array {
    // Build track data
    const track: number[] = [];
    for (const ev of events) {
        // VLQ-encode deltaTicks
        let dt = ev.deltaTicks;
        const vlqBytes: number[] = [];
        vlqBytes.unshift(dt & 0x7f);
        dt >>= 7;
        while (dt > 0) {
            vlqBytes.unshift((dt & 0x7f) | 0x80);
            dt >>= 7;
        }
        track.push(...vlqBytes, ev.status, ev.data1);
        if (ev.data2 !== undefined) track.push(ev.data2);
    }
    // End of track meta event
    track.push(0x00, 0xff, 0x2f, 0x00);

    const header = [
        // MThd
        0x4d, 0x54, 0x68, 0x64,
        0, 0, 0, 6,   // header length
        0, 0,          // format 0
        0, 1,          // 1 track
        (ticksPerQuarter >> 8) & 0xff, ticksPerQuarter & 0xff,
    ];
    const trackHeader = [
        0x4d, 0x54, 0x72, 0x6b, // MTrk
        (track.length >> 24) & 0xff,
        (track.length >> 16) & 0xff,
        (track.length >> 8) & 0xff,
        track.length & 0xff,
    ];
    return new Uint8Array([...header, ...trackHeader, ...track]);
}

test('returns empty array for data that is too short', () => {
    assert.deepEqual(parseMidiNotes(new Uint8Array(5)), []);
});

test('returns empty array for data with wrong header', () => {
    const bad = new Uint8Array(14).fill(0);
    assert.deepEqual(parseMidiNotes(bad), []);
});

test('parses a single note-on / note-off pair', () => {
    // 480 ticks/quarter, default tempo 500000 µs/quarter → 1 quarter = 0.5 s
    // Note On C4 at tick 0, Note Off at tick 480 → duration = 0.5 s
    const smf = buildSmf(480, [
        { deltaTicks: 0,   status: 0x90, data1: 60, data2: 64 }, // Note On C4
        { deltaTicks: 480, status: 0x80, data1: 60, data2: 0  }, // Note Off C4
    ]);
    const notes = parseMidiNotes(smf);
    assert.equal(notes.length, 1);
    assert.equal(notes[0].note, 60);
    assert.equal(notes[0].start, 0);
    assert.ok(Math.abs(notes[0].duration - 0.5) < 0.001, `expected duration ≈ 0.5, got ${notes[0].duration}`);
    assert.ok(Math.abs(notes[0].velocity - 64 / 127) < 0.001);
});

test('note-on with velocity 0 is treated as note-off', () => {
    const smf = buildSmf(480, [
        { deltaTicks: 0,   status: 0x90, data1: 60, data2: 64 }, // Note On
        { deltaTicks: 480, status: 0x90, data1: 60, data2: 0  }, // Note On vel=0 → Note Off
    ]);
    const notes = parseMidiNotes(smf);
    assert.equal(notes.length, 1);
    assert.ok(Math.abs(notes[0].duration - 0.5) < 0.001);
});

test('parses two sequential notes', () => {
    const smf = buildSmf(480, [
        { deltaTicks: 0,   status: 0x90, data1: 60, data2: 80 },
        { deltaTicks: 480, status: 0x80, data1: 60, data2: 0  },
        { deltaTicks: 0,   status: 0x90, data1: 62, data2: 80 },
        { deltaTicks: 480, status: 0x80, data1: 62, data2: 0  },
    ]);
    const notes = parseMidiNotes(smf);
    assert.equal(notes.length, 2);
    assert.equal(notes[0].note, 60);
    assert.equal(notes[1].note, 62);
    assert.ok(Math.abs(notes[1].start - 0.5) < 0.001);
});

test('note without note-off gets default duration of 0.25', () => {
    const smf = buildSmf(480, [
        { deltaTicks: 0, status: 0x90, data1: 60, data2: 64 }, // Note On, no Off
    ]);
    const notes = parseMidiNotes(smf);
    assert.equal(notes.length, 1);
    assert.equal(notes[0].duration, 0.25);
});
