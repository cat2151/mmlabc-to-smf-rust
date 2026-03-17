// Audio playback via Tone.js with real-time visualization
import * as Tone from './tone/index.js';
import { state } from './state.js';
import { showStatus } from './ui.js';
import { visualizeRealtime } from './visualization.js';

export async function playAudio(): Promise<void> {
    if (!state.currentAudioBuffer) {
        showStatus('No audio to play. Please convert MML first.', 'error');
        return;
    }

    try {
        stopAudio();

        await Tone.start();

        const player = new Tone.Player(state.currentAudioBuffer).toDestination();
        player.loop = false;

        const waveform = new Tone.Waveform(1024);
        const fft = new Tone.FFT(4096);

        player.connect(waveform);
        player.connect(fft);

        player.start();

        document.getElementById('playButton')!.setAttribute('disabled', 'true');
        document.getElementById('stopButton')!.removeAttribute('disabled');

        visualizeRealtime(waveform, fft, Tone.getContext().sampleRate);

        player.onstop = () => {
            document.getElementById('playButton')!.removeAttribute('disabled');
            document.getElementById('stopButton')!.setAttribute('disabled', 'true');
            if (state.animationId !== null) {
                cancelAnimationFrame(state.animationId);
                state.animationId = null;
            }
        };

        state.audioSource = player;
    } catch (error: any) {
        console.error('Error playing audio:', error);
        showStatus('Error playing audio: ' + error.message, 'error');
    }
}

export function stopAudio(): void {
    if (state.audioSource) {
        try {
            state.audioSource.stop();
        } catch (_e) {
            // Ignore errors when stopping
        }
        state.audioSource = null;
    }

    if (state.animationId !== null) {
        cancelAnimationFrame(state.animationId);
        state.animationId = null;
    }

    document.getElementById('playButton')!.removeAttribute('disabled');
    document.getElementById('stopButton')!.setAttribute('disabled', 'true');
}
