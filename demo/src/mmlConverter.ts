// MML to SMF conversion: parses MML text, invokes Rust WASM, updates the UI
import { parse_tree_json_to_smf, parse_tree_json_to_attachment_json, preprocess_mml } from '../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js';
import { state } from './state.js';
import { showStatus } from './ui.js';
import { smfToYM2151Json } from './smfToYm2151.js';
import { renderWaveformAndAudio } from './audioRenderer.js';
import { treeToJSON } from './treeToJSON.js';
export { treeToJSON };

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

        // Preprocess: extract embedded JSON from MML using Rust WASM
        // (provisional spec: JSON at start of MML is attachment JSON / 添付JSON)
        const preprocessResult = JSON.parse(preprocess_mml(mml));
        const embeddedJson: string | null = preprocessResult.embeddedJson;
        const mmlToParse: string = preprocessResult.remainingMml;

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