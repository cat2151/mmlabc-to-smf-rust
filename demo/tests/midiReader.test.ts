// Tests for MidiReader using Node.js built-in test runner.
// Run with: node --experimental-strip-types --test tests/midiReader.test.ts
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { MidiReader } from '../src/midiReader.ts';

test('readUint8 reads single byte and advances offset', () => {
    const reader = new MidiReader(new Uint8Array([0x42, 0x00]));
    assert.equal(reader.readUint8(), 0x42);
    assert.equal(reader.offset, 1);
});

test('readUint16 reads big-endian 16-bit value', () => {
    const reader = new MidiReader(new Uint8Array([0x01, 0xf4])); // 500
    assert.equal(reader.readUint16(), 500);
    assert.equal(reader.offset, 2);
});

test('readUint32 reads big-endian 32-bit value', () => {
    const reader = new MidiReader(new Uint8Array([0x00, 0x00, 0x01, 0xe0])); // 480
    assert.equal(reader.readUint32(), 480);
    assert.equal(reader.offset, 4);
});

test('readChar4 reads 4-byte ASCII string', () => {
    const reader = new MidiReader(new Uint8Array([0x4d, 0x54, 0x68, 0x64])); // "MThd"
    assert.equal(reader.readChar4(), 'MThd');
    assert.equal(reader.offset, 4);
});

test('readVlq reads single-byte VLQ value', () => {
    const reader = new MidiReader(new Uint8Array([0x40])); // 64
    assert.equal(reader.readVlq(), 64);
});

test('readVlq reads multi-byte VLQ value', () => {
    // 0x81 0x00 = 128 in VLQ encoding
    const reader = new MidiReader(new Uint8Array([0x81, 0x00]));
    assert.equal(reader.readVlq(), 128);
});

test('skip advances offset by n bytes', () => {
    const reader = new MidiReader(new Uint8Array([0x01, 0x02, 0x03, 0x04]));
    reader.skip(3);
    assert.equal(reader.offset, 3);
    assert.equal(reader.readUint8(), 0x04);
});

test('length property returns data length', () => {
    const reader = new MidiReader(new Uint8Array([0x01, 0x02, 0x03]));
    assert.equal(reader.length, 3);
});

test('readUint8 at end of buffer throws', () => {
    const reader = new MidiReader(new Uint8Array([0x01]));
    reader.readUint8();
    assert.throws(() => reader.readUint8(), /Unexpected EOF/);
});

test('readChar4 with insufficient data throws', () => {
    const reader = new MidiReader(new Uint8Array([0x4d, 0x54])); // only 2 bytes
    assert.throws(() => reader.readChar4(), /Unexpected EOF/);
});
