// Shared mutable application state
export const state = {
    currentSmfData: null as Uint8Array | null,
    currentAudioBuffer: null as AudioBuffer | null,
    audioContext: null as AudioContext | null,
    audioSource: null as any,
    parser: null as any,
    wasmInitialized: false,
    debounceTimer: null as number | null,
    /** True when the user has typed non-empty MML and is waiting for auto-play. */
    pendingPlay: false,
};
