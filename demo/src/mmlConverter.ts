// MML to SMF conversion: parses MML text, invokes Rust WASM, updates the UI
import { parse_tree_json_to_smf, parse_tree_json_to_attachment_json } from '../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js';
import { state } from './state.js';
import { showStatus } from './ui.js';
import { smfToYM2151Json } from './smfToYm2151.js';
import { renderWaveformAndAudio } from './audioRenderer.js';
import { treeToJSON } from './treeToJSON.js';
export { treeToJSON };

/**
 * Extract embedded JSON from the start of MML text.
 *
 * Provisional spec: writing JSON directly at the beginning of MML is recognized
 * as the attachment JSON (添付JSON). The remaining text after the JSON is the
 * actual MML to parse.
 *
 * Example: `[{"ProgramChange":1,"Tone":{"events":[]}}]@1cde`
 *   → json = '[{"ProgramChange":1,"Tone":{"events":[]}}]', remainingMml = '@1cde'
 */
export function extractEmbeddedJson(mml: string): { json: string | null; remainingMml: string } {
    const trimmed = mml.trimStart();
    if (!trimmed.startsWith('{') && !trimmed.startsWith('[')) {
        return { json: null, remainingMml: mml };
    }

    let depth = 0;
    let inString = false;
    let escaped = false;
    let endIdx = -1;

    for (let i = 0; i < trimmed.length; i++) {
        const c = trimmed[i];
        if (escaped) { escaped = false; continue; }
        if (c === '\\' && inString) { escaped = true; continue; }
        if (c === '"') { inString = !inString; continue; }
        if (inString) continue;
        if (c === '{' || c === '[') depth++;
        else if (c === '}' || c === ']') {
            depth--;
            if (depth === 0) { endIdx = i; break; }
        }
    }

    if (endIdx === -1) {
        return { json: null, remainingMml: mml };
    }

    const jsonStr = trimmed.substring(0, endIdx + 1);
    try {
        JSON.parse(jsonStr);
        return { json: jsonStr, remainingMml: trimmed.substring(endIdx + 1).trimStart() };
    } catch {
        return { json: null, remainingMml: mml };
    }
}

export async function convertMML(): Promise<void> {
    if (!state.wasmInitialized || !state.parser) {
        showStatus('Not initialized yet. Please wait...', 'error');
        return;
    }

    const mml = (document.getElementById('mmlInput') as HTMLTextAreaElement).value.trim();

    if (!mml) {
        const statusDiv = document.getElementById('status')!;
        statusDiv.textContent = '';
        statusDiv.className = 'hidden';

        const debugSection = document.getElementById('debug');
        if (debugSection) debugSection.classList.add('hidden');

        const debugInfo = document.getElementById('debugInfo');
        if (debugInfo) debugInfo.textContent = '';
        return;
    }

    try {
        showStatus('Parsing MML...', 'info');

        // Extract embedded JSON from MML (provisional spec: JSON at start of MML is attachment JSON)
        const { json: embeddedJson, remainingMml } = extractEmbeddedJson(mml);
        const mmlToParse = remainingMml;

        const tree = state.parser.parse(mmlToParse);
        const parseTreeJSON = treeToJSON(tree.rootNode, mmlToParse);
        const parseTreeStr = JSON.stringify(parseTreeJSON);

        document.getElementById('debugInfo')!.textContent =
            'Parse Tree JSON:\n' + JSON.stringify(parseTreeJSON, null, 2);
        document.getElementById('debug')!.classList.remove('hidden');

        showStatus('Converting to SMF via Rust WASM...', 'info');

        const smfData = parse_tree_json_to_smf(parseTreeStr, mmlToParse);
        state.currentSmfData = smfData;

        const ym2151Json = await smfToYM2151Json(smfData);
        (document.getElementById('jsonOutput') as HTMLTextAreaElement).value =
            JSON.stringify(ym2151Json, null, 2);
        document.getElementById('jsonSection')!.classList.remove('hidden');

        // Display attachment JSON: use embedded JSON from MML if present, otherwise generate from events
        const attachmentOutput = document.getElementById('attachmentJsonOutput') as HTMLTextAreaElement | null;
        if (attachmentOutput) {
            if (embeddedJson !== null) {
                try {
                    attachmentOutput.value = JSON.stringify(JSON.parse(embeddedJson), null, 2);
                } catch {
                    console.warn('Embedded JSON could not be pretty-printed; displaying raw:', embeddedJson);
                    attachmentOutput.value = embeddedJson;
                }
            } else {
                try {
                    const attachmentJson = parse_tree_json_to_attachment_json(parseTreeStr, mmlToParse);
                    attachmentOutput.value = attachmentJson;
                } catch (attachErr) {
                    console.warn('Attachment JSON generation failed:', attachErr);
                }
            }
        }

        await renderWaveformAndAudio(ym2151Json.events);

        const blob = new Blob([smfData], { type: 'audio/midi' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'output.mid';
        a.textContent = 'Download MIDI File';

        showStatus('✅ Conversion successful! SMF size: ' + smfData.length + ' bytes', 'success');
        const statusDiv = document.getElementById('status')!;
        statusDiv.appendChild(document.createElement('br'));
        statusDiv.appendChild(a);

        document.getElementById('playbackControls')!.style.display = 'block';
        document.getElementById('visualizerSection')!.style.display = 'block';
    } catch (error: any) {
        showStatus('Error: ' + error.message, 'error');
        console.error('Conversion error:', error);
    }
}