# Demo Features

This demo provides a comprehensive MML to SMF conversion experience with audio playback and visualization.

## Features

### 1. MML to SMF Conversion
- Convert Music Macro Language (MML) to Standard MIDI File (SMF) format
- Real-time conversion as you type (with 500ms debounce)
- Quick example buttons for common MML patterns

### 2. SMF to YM2151 JSON Conversion
- Automatically converts generated SMF to YM2151 log format JSON
- Shows register-level event data similar to hardware FM synthesis chips
- JSON output displays timestamps, register addresses, and data values

### 3. Waveform Visualization
- **Full Waveform Preview**: Shows the complete audio waveform rendered from the MIDI data
- **Real-time Waveform**: Updates at 60fps during playback showing current audio signal
- **FFT Spectrum Analyzer**: Real-time frequency spectrum display with color-coded bars

### 4. Audio Playback
- Play button to start audio playback
- Stop button to halt playback
- Synthesized audio using simple sine wave generation
- Powered by Tone.js for Web Audio API integration

### 5. WAV Export
- Export button to download audio as WAV file
- 16-bit PCM format at 44.1kHz sample rate
- Stereo output

## Technologies Used

- **Rust WASM**: Core MML to SMF conversion
- **web-tree-sitter**: MML parsing
- **Tone.js**: Audio playback and analysis
- **Web Audio API**: Audio synthesis and real-time processing
- **Canvas API**: Waveform and spectrum visualization

## Usage

1. Enter MML code in the textarea (or click an example button)
2. Wait for automatic conversion (or conversion happens immediately for examples)
3. View the generated JSON, waveform preview, and MIDI download link
4. Click "Play" to hear the audio with real-time visualization
5. Click "Export WAV" to download the audio file

## Implementation Notes

### Simplified YM2151 Conversion
The SMF to YM2151 JSON conversion is simplified for demo purposes. A production implementation would require:
- Full MIDI parsing using a proper MIDI library
- Complete YM2151 register mapping
- Accurate timing calculation
- Support for all YM2151 features (operators, envelope, LFO, etc.)

### Audio Synthesis
The current implementation uses simple sine wave synthesis for each MIDI note. For more accurate playback:
- Consider using a Web MIDI API connection to a synthesizer
- Implement more complex synthesis (ADSR envelopes, harmonics, etc.)
- Or integrate actual YM2151 emulation (like web-ym2151 WASM)

### Future Enhancements
- Integration with actual web-ym2151 WASM for authentic FM synthesis
- More sophisticated MIDI parsing
- Additional visualization options (spectrogram, oscilloscope patterns)
- Playback controls (pause, seek, loop)
- Volume control and effects
