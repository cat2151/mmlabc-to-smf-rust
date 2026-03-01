// WAV file export from an AudioBuffer
import { state } from './state.js';
import { showStatus } from './ui.js';

function writeString(view: DataView, offset: number, str: string): void {
    for (let i = 0; i < str.length; i++) {
        view.setUint8(offset + i, str.charCodeAt(i));
    }
}

export function audioBufferToWav(buffer: AudioBuffer): ArrayBuffer {
    const numChannels = buffer.numberOfChannels;
    const sampleRate = buffer.sampleRate;
    const bitDepth = 16;
    const bytesPerSample = bitDepth / 8;
    const blockAlign = numChannels * bytesPerSample;
    const dataLength = buffer.length * numChannels * bytesPerSample;

    const output = new ArrayBuffer(44 + dataLength);
    const view = new DataView(output);

    writeString(view, 0, 'RIFF');
    view.setUint32(4, 36 + dataLength, true);
    writeString(view, 8, 'WAVE');

    writeString(view, 12, 'fmt ');
    view.setUint32(16, 16, true);
    view.setUint16(20, 1, true); // PCM format
    view.setUint16(22, numChannels, true);
    view.setUint32(24, sampleRate, true);
    view.setUint32(28, sampleRate * blockAlign, true);
    view.setUint16(32, blockAlign, true);
    view.setUint16(34, bitDepth, true);

    writeString(view, 36, 'data');
    view.setUint32(40, dataLength, true);

    let offset = 44;
    for (let i = 0; i < buffer.length; i++) {
        for (let channel = 0; channel < numChannels; channel++) {
            const sample = buffer.getChannelData(channel)[i];
            const clampedSample = Math.max(-1, Math.min(1, sample));
            const pcmValue = Math.floor(clampedSample < 0 ? clampedSample * 0x8000 : clampedSample * 0x7fff);
            view.setInt16(offset, pcmValue, true);
            offset += 2;
        }
    }

    return output;
}

export function exportWav(): void {
    if (!state.currentAudioBuffer) {
        showStatus('No audio to export. Please convert MML first.', 'error');
        return;
    }

    try {
        const wavData = audioBufferToWav(state.currentAudioBuffer);
        const blob = new Blob([wavData], { type: 'audio/wav' });
        const url = URL.createObjectURL(blob);

        const a = document.createElement('a');
        a.href = url;
        a.download = 'output.wav';
        a.click();

        setTimeout(() => URL.revokeObjectURL(url), 100);
        showStatus('WAV file exported successfully!', 'success');
    } catch (error: any) {
        console.error('Error exporting WAV:', error);
        showStatus('Error exporting WAV: ' + error.message, 'error');
    }
}
