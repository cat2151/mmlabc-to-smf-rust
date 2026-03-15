// Application entry point: initializes parsers, WASM, and registers global handlers
import { Parser, Language } from './web-tree-sitter.js';
import init from '../mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js';
import { state } from './state.js';
import { showStatus, loadExample } from './ui.js';
import { convertMML } from './mmlConverter.js';
import { playAudio, stopAudio } from './audioPlayback.js';
import { exportWav } from './wavExport.js';

async function initialize(): Promise<void> {
    try {
        await Parser.init();
        state.parser = new Parser();

        const wasmUrl = new URL('../tree-sitter-mml/tree-sitter-mml.wasm', import.meta.url).href;
        const lang = await Language.load(wasmUrl);
        state.parser.setLanguage(lang);

        await init();
        state.wasmInitialized = true;

        showStatus('Ready! Parser and WASM initialized.', 'success');

        const mmlInput = document.getElementById('mmlInput') as HTMLTextAreaElement;
        mmlInput.addEventListener('input', () => {
            if (state.debounceTimer !== null) {
                clearTimeout(state.debounceTimer);
            }
            state.debounceTimer = setTimeout(async () => {
                await convertMML();
                if (mmlInput.value.trim() && state.currentAudioBuffer) {
                    await playAudio();
                }
            }, 1000) as unknown as number;
        });

        const exampleSelect = document.getElementById('exampleSelect') as HTMLSelectElement | null;
        if (exampleSelect) {
            exampleSelect.addEventListener('change', () => {
                if (exampleSelect.value) {
                    loadExample(exampleSelect.value);
                    exampleSelect.selectedIndex = 0;
                }
            });
        }

        await convertMML();
    } catch (error: any) {
        showStatus('Error initializing: ' + error.message, 'error');
        console.error('Initialization error:', error);
    }
}

window.addEventListener('beforeunload', () => {
    if (state.debounceTimer !== null) {
        clearTimeout(state.debounceTimer);
        state.debounceTimer = null;
    }
});

// Expose functions for HTML onclick attributes
(window as any).convertMML = convertMML;
(window as any).loadExample = loadExample;
(window as any).playAudio = playAudio;
(window as any).stopAudio = stopAudio;
(window as any).exportWav = exportWav;

initialize();
