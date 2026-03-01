// Canvas-based waveform and FFT visualization
import { state } from './state.js';

export function drawWaveform(audioData: Float32Array): void {
    const canvas = document.getElementById('waveformCanvas') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d')!;
    const { width, height } = canvas;

    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, width, height);

    ctx.strokeStyle = '#0f0';
    ctx.lineWidth = 1;
    ctx.beginPath();

    const step = Math.floor(audioData.length / width);

    for (let i = 0; i < width; i++) {
        const index = i * step;
        const value = audioData[index] || 0;
        const y = (value * 0.5 + 0.5) * height;

        if (i === 0) {
            ctx.moveTo(i, y);
        } else {
            ctx.lineTo(i, y);
        }
    }

    ctx.stroke();
}

export function visualizeRealtime(waveform: any, fft: any): void {
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
        fftCtx.fillStyle = '#000';
        fftCtx.fillRect(0, 0, fftCanvas.width, fftCanvas.height);

        const barWidth = fftCanvas.width / fftValues.length;

        for (let i = 0; i < fftValues.length; i++) {
            const value = fftValues[i];
            const percent = (value + 140) / 140;
            const barHeight = percent * fftCanvas.height;
            const hue = (i / fftValues.length) * 360;

            fftCtx.fillStyle = `hsl(${hue}, 100%, 50%)`;
            fftCtx.fillRect(i * barWidth, fftCanvas.height - barHeight, barWidth - 1, barHeight);
        }

        state.animationId = requestAnimationFrame(draw);
    }

    draw();
}
