// Node.js module loader: remaps local `.js` import specifiers to `.ts`.
// This allows test files to import TypeScript source modules that use
// `.js` extensions in their imports (as required by esbuild/browser builds).
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

export async function resolve(specifier, context, nextResolve) {
    if (specifier.endsWith('.js') && !specifier.startsWith('node:') && !specifier.includes('node_modules')) {
        const tsSpecifier = specifier.slice(0, -3) + '.ts';
        try {
            const result = await nextResolve(tsSpecifier, context);
            const resolved = fileURLToPath(result.url);
            if (existsSync(resolved)) {
                return result;
            }
        } catch {
            // fall through to original specifier
        }
    }
    return nextResolve(specifier, context);
}
