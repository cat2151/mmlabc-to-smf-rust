// Synthesizes audio from YM2151 JSON events using the web-ym2151 WASM library (Emscripten)
import { drawWaveform } from './visualization.js';
import { state } from './state.js';
import type { YM2151Event } from './smfToYm2151.js';

// OPM (YM2151) hardware sample rate
const OPM_CLOCK = 3579545;
const CLOCK_STEP = 64;
const OPM_SAMPLE_RATE = OPM_CLOCK / CLOCK_STEP; // ≈ 55930 Hz

function waitForWebYm2151(): Promise<void> {
    return new Promise((resolve, reject) => {
        const maxWaitMs = 10000;
        const startTime = Date.now();
        const check = () => {
            const mod = (window as any).Module;
            if (mod && typeof mod._generate_sound === 'function') {
                resolve();
            } else if (Date.now() - startTime > maxWaitMs) {
                reject(new Error('web-ym2151 WASM not ready (timeout)'));
            } else {
                setTimeout(check, 50);
            }
        };
        check();
    });
}

function calculateDuration(events: YM2151Event[]): number {
    if (events.length === 0) return 1.0;
    let maxTime = events[0].time;
    for (let i = 1; i < events.length; i++) {
        const t = events[i].time;
        if (t > maxTime) {
            maxTime = t;
        }
    }
    return maxTime + 0.5;
}

export async function renderWaveformAndAudio(events: YM2151Event[]): Promise<void> {
    try {
        await waitForWebYm2151();

        const mod = (window as any).Module;

        if (!events || events.length === 0) {
            return;
        }

        const durationSec = calculateDuration(events);
        const numFrames = Math.floor(OPM_SAMPLE_RATE * durationSec);

        const STRUCT_SIZE = 8; // Must match C struct: float time (4) + uint8 addr (1) + uint8 data (1) + padding (2)
        // See web-ym2151/demo-library/index.html for the authoritative struct layout definition
        const dataPtr = mod._malloc(events.length * STRUCT_SIZE);
        if (!dataPtr) {
            console.error('web-ym2151: failed to allocate memory for YM2151 event buffer');
            return;
        }
        const view = new DataView(mod.HEAPU8.buffer);

        let actualFrames: number;
        try {
            for (let i = 0; i < events.length; i++) {
                const baseAddr = dataPtr + i * STRUCT_SIZE;
                view.setFloat32(baseAddr, events[i].time, true);
                const addrByte = parseInt(events[i].addr, 16);
                const dataByte = parseInt(events[i].data, 16);
                mod.HEAPU8[baseAddr + 4] = isNaN(addrByte) ? 0 : addrByte;
                mod.HEAPU8[baseAddr + 5] = isNaN(dataByte) ? 0 : dataByte;
            }

            actualFrames = mod._generate_sound(dataPtr, events.length, numFrames);
        } finally {
            mod._free(dataPtr);
        }

        if (actualFrames <= 0) {
            console.error('web-ym2151: no frames generated');
            return;
        }

        const left = new Float32Array(actualFrames);
        const right = new Float32Array(actualFrames);

        for (let i = 0; i < actualFrames; i++) {
            left[i] = mod._get_sample(i * 2);
            right[i] = mod._get_sample(i * 2 + 1);
        }

        mod._free_buffer();

        if (!state.audioContext) {
            const AudioContextClass =
                (window as any).AudioContext || (window as any).webkitAudioContext;
            state.audioContext = new AudioContextClass();
        }

        state.currentAudioBuffer = state.audioContext.createBuffer(2, actualFrames, OPM_SAMPLE_RATE);
        state.currentAudioBuffer.getChannelData(0).set(left);
        state.currentAudioBuffer.getChannelData(1).set(right);

        drawWaveform(left);
    } catch (error) {
        console.error('Error rendering audio with web-ym2151:', error);
    }
}
