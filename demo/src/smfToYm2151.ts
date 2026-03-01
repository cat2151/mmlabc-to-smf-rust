// Converts an SMF (Standard MIDI File) to YM2151 log format JSON using the smf-to-ym2151log-rust WASM library
import init, { smf_to_ym2151_json } from '../smf-to-ym2151log-wasm/smf_to_ym2151log.js';

// Hex string format (e.g. "0x08", "0x28") as returned by smf-to-ym2151log-rust WASM library
export interface YM2151Event {
    time: number;
    addr: string;
    data: string;
}

export interface YM2151Json {
    events: YM2151Event[];
    event_count?: number;
}

let wasmInitialized = false;

async function ensureInitialized(): Promise<void> {
    if (!wasmInitialized) {
        await init();
        wasmInitialized = true;
    }
}

export async function smfToYM2151Json(smfData: Uint8Array): Promise<YM2151Json> {
    await ensureInitialized();
    const jsonStr = smf_to_ym2151_json(smfData);
    return JSON.parse(jsonStr) as YM2151Json;
}
