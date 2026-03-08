// Tests for treeToJSON using Node.js built-in test runner.
// Run with: node --experimental-strip-types --test tests/treeToJSON.test.ts
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { treeToJSON } from '../src/treeToJSON.ts';

// Helper: build a mock web-tree-sitter node from a plain descriptor.
// Each descriptor has { type, start, end, children? }.
function mockNode(descriptor: { type: string; start: number; end: number; children?: any[] }): any {
    const children = descriptor.children ?? [];
    return {
        type: descriptor.type,
        startIndex: descriptor.start,
        endIndex: descriptor.end,
        childCount: children.length,
        child: (i: number) => children[i],
    };
}

// "cde" source — leaf note nodes
test('leaf node includes text', () => {
    const source = 'cde';
    const node = mockNode({ type: 'note', start: 0, end: 1 });
    const json = treeToJSON(node, source);

    assert.equal(json.type, 'note');
    assert.equal(json.text, 'c');
    assert.equal(json.children, undefined);
});

// l4 — length_set has a child note_length but still needs its own text ("l4")
// This is the regression case: before the fix, non-leaf text was omitted.
test('non-leaf node (length_set) includes text — regression for l/t/@/v/o/kt bug', () => {
    const source = 'l4c';
    const noteLength = mockNode({ type: 'note_length', start: 1, end: 2 });
    const lengthSet = mockNode({ type: 'length_set', start: 0, end: 2, children: [noteLength] });
    const json = treeToJSON(lengthSet, source);

    assert.equal(json.type, 'length_set');
    assert.equal(json.text, 'l4', 'length_set must carry its full source text for Rust WASM to parse the value');
    assert.equal(json.children?.length, 1);
    assert.equal(json.children[0].type, 'note_length');
    assert.equal(json.children[0].text, '4');
});

// t120 — tempo_set with numeric child
test('non-leaf node (tempo_set) includes text — regression for l/t/@/v/o/kt bug', () => {
    const source = 't120';
    const numNode = mockNode({ type: 'number', start: 1, end: 4 });
    const tempoSet = mockNode({ type: 'tempo_set', start: 0, end: 4, children: [numNode] });
    const json = treeToJSON(tempoSet, source);

    assert.equal(json.text, 't120');
    assert.equal(json.children?.length, 1);
    assert.equal(json.children[0].text, '120');
});

// Nested tree: source_file > length_set > note_length
test('treeToJSON recursively serialises entire tree with text at every level', () => {
    const source = 'l8c';
    const noteLength = mockNode({ type: 'note_length', start: 1, end: 2 });
    const lengthSet = mockNode({ type: 'length_set', start: 0, end: 2, children: [noteLength] });
    const note = mockNode({ type: 'note_with_modifier', start: 2, end: 3 });
    const root = mockNode({ type: 'source_file', start: 0, end: 3, children: [lengthSet, note] });
    const json = treeToJSON(root, source);

    assert.equal(json.text, 'l8c');
    assert.equal(json.children.length, 2);
    assert.equal(json.children[0].text, 'l8');
    assert.equal(json.children[0].children[0].text, '8');
    assert.equal(json.children[1].text, 'c');
});
