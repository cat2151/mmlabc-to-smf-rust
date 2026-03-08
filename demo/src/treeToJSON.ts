/// Convert a web-tree-sitter parse tree node to a plain JSON-serializable object.
/// Every node (leaf or non-leaf) receives a `text` property so that MML commands
/// with child nodes (l, t, @, v, o, kt, …) retain their full source text when
/// the JSON is forwarded to the Rust WASM converter.
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
