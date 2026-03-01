// Shared mutable application state
export const state = {
    currentSmfData: null as Uint8Array | null,
    currentAudioBuffer: null as AudioBuffer | null,
    audioContext: null as AudioContext | null,
    audioSource: null as any,
    animationId: null as number | null,
    parser: null as any,
    wasmInitialized: false,
    debounceTimer: null as number | null,
};
