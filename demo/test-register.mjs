// Registers test-loader.mjs as a Node.js module hook via node:module register().
// Use: node --experimental-strip-types --import ./test-register.mjs --test ...
import { register } from 'node:module';
register('./test-loader.mjs', import.meta.url);
