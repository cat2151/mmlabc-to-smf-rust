// MML to SMF conversion: parses MML text, invokes Rust WASM, updates the UI
import { parse_tree_json_to_smf } from '../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js';
import { state } from './state.js';
import { showStatus } from './ui.js';
import { smfToYM2151Json } from './smfToYm2151.js';
import { renderWaveformAndAudio } from './audioRenderer.js';

export function treeToJSON(node: any, source: string): any {
    const result: any = {
        type: node.type,
        text: source.substring(node.startIndex, node.endIndex),
    };

    if (node.childCount > 0) {
        result.children = [];
        for (let i = 0; i < node.childCount; i++) {
            result.children.push(treeToJSON(node.child(i), source));
        }
    }

    return result;
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

        const tree = state.parser.parse(mml);
        const parseTreeJSON = treeToJSON(tree.rootNode, mml);
        const parseTreeStr = JSON.stringify(parseTreeJSON);

        document.getElementById('debugInfo')!.textContent =
            'Parse Tree JSON:\n' + JSON.stringify(parseTreeJSON, null, 2);
        document.getElementById('debug')!.classList.remove('hidden');

        showStatus('Converting to SMF via Rust WASM...', 'info');

        const smfData = parse_tree_json_to_smf(parseTreeStr, mml);
        state.currentSmfData = smfData;

        const ym2151Json = await smfToYM2151Json(smfData);
        (document.getElementById('jsonOutput') as HTMLTextAreaElement).value =
            JSON.stringify(ym2151Json, null, 2);
        document.getElementById('jsonSection')!.classList.remove('hidden');

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