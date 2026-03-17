// Audio playback via Tone.js
import * as Tone from './tone/index.js';
import { state } from './state.js';
import { showStatus } from './ui.js';

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

        player.start();

        document.getElementById('playButton')!.setAttribute('disabled', 'true');
        document.getElementById('stopButton')!.removeAttribute('disabled');

        player.onstop = () => {
            document.getElementById('playButton')!.removeAttribute('disabled');
            document.getElementById('stopButton')!.setAttribute('disabled', 'true');
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

    document.getElementById('playButton')!.removeAttribute('disabled');
    document.getElementById('stopButton')!.setAttribute('disabled', 'true');
}
