// Canvas-based waveform visualization

export function drawWaveform(audioData: Float32Array): void {
    const canvas = document.getElementById('waveformCanvas') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d')!;
    const { width, height } = canvas;

    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, width, height);

    if (audioData.length === 0) return;

    // For each pixel column, find min/max of all samples in that range
    const mins = new Float32Array(width);
    const maxs = new Float32Array(width);
    let maxAmp = 0;

    for (let x = 0; x < width; x++) {
        const start = Math.floor(x * audioData.length / width);
        const end = Math.max(start + 1, Math.floor((x + 1) * audioData.length / width));
        let min = audioData[start] ?? 0;
        let max = min;
        for (let j = start + 1; j < end && j < audioData.length; j++) {
            const v = audioData[j];
            if (v < min) min = v;
            if (v > max) max = v;
        }
        mins[x] = min;
        maxs[x] = max;
        if (Math.abs(min) > maxAmp) maxAmp = Math.abs(min);
        if (Math.abs(max) > maxAmp) maxAmp = Math.abs(max);
    }

    const scale = maxAmp > 0 ? 0.95 / maxAmp : 1;

    ctx.fillStyle = '#0f0';
    for (let x = 0; x < width; x++) {
        // Map amplitude to canvas y: positive amplitude → larger y (lower on canvas)
        const yForMin = (mins[x] * scale * 0.5 + 0.5) * height;
        const yForMax = (maxs[x] * scale * 0.5 + 0.5) * height;
        const top = Math.min(yForMin, yForMax);
        const barH = Math.max(1, Math.abs(yForMax - yForMin));
        ctx.fillRect(x, top, 1, barH);
    }
}
