// Tests for audioBufferToWav using Node.js built-in test runner.
// Run with: node --experimental-strip-types --test tests/audioBufferToWav.test.ts
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { audioBufferToWav } from '../src/wavExport.ts';

// Minimal duck-typed mock of the browser AudioBuffer API
function mockAudioBuffer(channels: Float32Array[], sampleRate: number): any {
    return {
        numberOfChannels: channels.length,
        length: channels[0].length,
        sampleRate,
        getChannelData: (ch: number) => channels[ch],
    };
}

test('output starts with RIFF/WAVE header', () => {
    const buf = mockAudioBuffer([new Float32Array(4)], 44100);
    const wav = audioBufferToWav(buf);
    const view = new DataView(wav);
    const riff = String.fromCharCode(view.getUint8(0), view.getUint8(1), view.getUint8(2), view.getUint8(3));
    const wave = String.fromCharCode(view.getUint8(8), view.getUint8(9), view.getUint8(10), view.getUint8(11));
    assert.equal(riff, 'RIFF');
    assert.equal(wave, 'WAVE');
});

test('output size is 44 header bytes + 16-bit PCM data', () => {
    // 1 channel, 10 samples → 10 * 2 bytes = 20 bytes data
    const buf = mockAudioBuffer([new Float32Array(10)], 44100);
    const wav = audioBufferToWav(buf);
    assert.equal(wav.byteLength, 44 + 10 * 2);
});

test('stereo output has correct byte size', () => {
    // 2 channels, 10 samples → 10 * 2 channels * 2 bytes = 40 bytes data
    const buf = mockAudioBuffer([new Float32Array(10), new Float32Array(10)], 44100);
    const wav = audioBufferToWav(buf);
    assert.equal(wav.byteLength, 44 + 10 * 2 * 2);
});

test('sample rate is written into WAV header', () => {
    const buf = mockAudioBuffer([new Float32Array(4)], 55930);
    const wav = audioBufferToWav(buf);
    const view = new DataView(wav);
    assert.equal(view.getUint32(24, true), 55930);
});

test('positive full-scale sample encodes near 0x7FFF', () => {
    const samples = new Float32Array([1.0]);
    const buf = mockAudioBuffer([samples], 44100);
    const wav = audioBufferToWav(buf);
    const view = new DataView(wav);
    const pcm = view.getInt16(44, true);
    assert.equal(pcm, 0x7fff);
});

test('negative full-scale sample encodes near -0x8000', () => {
    const samples = new Float32Array([-1.0]);
    const buf = mockAudioBuffer([samples], 44100);
    const wav = audioBufferToWav(buf);
    const view = new DataView(wav);
    const pcm = view.getInt16(44, true);
    assert.equal(pcm, -0x8000);
});

test('silence encodes to zero PCM values', () => {
    const samples = new Float32Array(4); // all zeros
    const buf = mockAudioBuffer([samples], 44100);
    const wav = audioBufferToWav(buf);
    const view = new DataView(wav);
    for (let i = 0; i < 4; i++) {
        assert.equal(view.getInt16(44 + i * 2, true), 0);
    }
});
