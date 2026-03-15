// Canvas-based waveform and FFT visualization
import { state } from './state.js';

const FFT_LABEL_HEIGHT = 14; // pixels reserved at bottom of FFT canvas for frequency labels
const FFT_EDGE_PADDING = 4; // pixels to skip at canvas edges when drawing frequency labels

export function drawWaveform(audioData: Float32Array): void {
    const canvas = document.getElementById('waveformCanvas') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d')!;
    const { width, height } = canvas;

    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, width, height);

    // Normalize to 95% of canvas height
    let maxAmp = 0;
    for (let i = 0; i < audioData.length; i++) {
        const abs = Math.abs(audioData[i]);
        if (abs > maxAmp) maxAmp = abs;
    }
    const scale = maxAmp > 0 ? 0.95 / maxAmp : 1;

    ctx.strokeStyle = '#0f0';
    ctx.lineWidth = 1;
    ctx.beginPath();

    const step = Math.floor(audioData.length / width);

    for (let i = 0; i < width; i++) {
        const index = i * step;
        const value = (audioData[index] || 0) * scale;
        const y = (value * 0.5 + 0.5) * height;

        if (i === 0) {
            ctx.moveTo(i, y);
        } else {
            ctx.lineTo(i, y);
        }
    }

    ctx.stroke();
}

export function visualizeRealtime(waveform: any, fft: any, sampleRate: number): void {
    const waveCanvas = document.getElementById('realtimeWaveform') as HTMLCanvasElement;
    const fftCanvas = document.getElementById('realtimeFFT') as HTMLCanvasElement;
    const waveCtx = waveCanvas.getContext('2d')!;
    const fftCtx = fftCanvas.getContext('2d')!;

    function draw() {
        const waveformValues: Float32Array = waveform.getValue();
        waveCtx.fillStyle = '#000';
        waveCtx.fillRect(0, 0, waveCanvas.width, waveCanvas.height);

        waveCtx.strokeStyle = '#0f0';
        waveCtx.lineWidth = 2;
        waveCtx.beginPath();

        for (let i = 0; i < waveformValues.length; i++) {
            const x = i * (waveCanvas.width / waveformValues.length);
            const y = (1 - (waveformValues[i] + 1) / 2) * waveCanvas.height;

            if (i === 0) {
                waveCtx.moveTo(x, y);
            } else {
                waveCtx.lineTo(x, y);
            }
        }
        waveCtx.stroke();

        const fftValues: Float32Array = fft.getValue();
        const barAreaHeight = fftCanvas.height - FFT_LABEL_HEIGHT;
        fftCtx.fillStyle = '#000';
        fftCtx.fillRect(0, 0, fftCanvas.width, fftCanvas.height);

        const barWidth = fftCanvas.width / fftValues.length;

        // Monotone bars
        fftCtx.fillStyle = '#6688aa';
        for (let i = 0; i < fftValues.length; i++) {
            const value = fftValues[i];
            const percent = (value + 140) / 140;
            const barHeight = percent * barAreaHeight;
            fftCtx.fillRect(i * barWidth, barAreaHeight - barHeight, barWidth - 1, barHeight);
        }

        // Frequency labels every 1kHz, ticks every 500Hz
        const nyquist = sampleRate / 2;
        fftCtx.font = '8px Arial';
        fftCtx.textAlign = 'center';
        for (let freq = 500; freq < nyquist; freq += 500) {
            const x = (freq / nyquist) * fftCanvas.width;
            if (x < FFT_EDGE_PADDING || x > fftCanvas.width - FFT_EDGE_PADDING) continue;
            fftCtx.fillStyle = '#556677';
            fftCtx.fillRect(x, barAreaHeight, 1, freq % 1000 === 0 ? 4 : 2);
            if (freq % 1000 === 0) {
                fftCtx.fillStyle = '#889aaa';
                fftCtx.fillText(`${freq / 1000}k`, x, fftCanvas.height - 1);
            }
        }

        state.animationId = requestAnimationFrame(draw);
    }

    draw();
}
