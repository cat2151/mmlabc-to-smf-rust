// Synthesizes audio from MIDI data and manages the AudioContext
import { parseMidiNotes } from './parseMidiNotes.js';
import { drawWaveform } from './visualization.js';
import { state } from './state.js';

interface AudioData {
    left: Float32Array;
    right: Float32Array;
}

export async function midiToAudio(smfData: Uint8Array): Promise<AudioData> {
    const sampleRate = 44100;

    try {
        const notes = parseMidiNotes(smfData);

        if (notes.length === 0) {
            const numSamples = Math.floor(sampleRate * 1.0);
            return { left: new Float32Array(numSamples), right: new Float32Array(numSamples) };
        }

        const maxEndTime = Math.max(...notes.map(n => n.start + n.duration));
        const duration = maxEndTime + 0.5;
        const numSamples = Math.floor(sampleRate * duration);

        const left = new Float32Array(numSamples);
        const right = new Float32Array(numSamples);

        for (const noteInfo of notes) {
            const freq = 440 * Math.pow(2, (noteInfo.note - 69) / 12);
            const startSample = Math.floor(noteInfo.start * sampleRate);
            const durationSamples = Math.floor(noteInfo.duration * sampleRate);

            for (let i = 0; i < durationSamples; i++) {
                const sampleIndex = startSample + i;
                if (sampleIndex < numSamples) {
                    const t = i / sampleRate;
                    const envelope = Math.exp(-3 * t);
                    const value = Math.sin(2 * Math.PI * freq * t) * envelope * noteInfo.velocity * 0.3;
                    left[sampleIndex] += value;
                    right[sampleIndex] += value;
                }
            }
        }

        return { left, right };
    } catch (error) {
        console.error('Error in midiToAudio:', error);
        const numSamples = Math.floor(sampleRate * 1.0);
        return { left: new Float32Array(numSamples), right: new Float32Array(numSamples) };
    }
}

export async function renderWaveformAndAudio(smfData: Uint8Array): Promise<void> {
    try {
        if (!state.audioContext) {
            state.audioContext = new AudioContext();
        }

        const audioData = await midiToAudio(smfData);

        state.currentAudioBuffer = state.audioContext.createBuffer(
            2,
            audioData.left.length,
            44100
        );

        state.currentAudioBuffer.getChannelData(0).set(audioData.left);
        state.currentAudioBuffer.getChannelData(1).set(audioData.right);

        drawWaveform(audioData.left);
    } catch (error) {
        console.error('Error rendering waveform:', error);
    }
}
