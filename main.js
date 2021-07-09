import init, { run_app } from './pkg/telkom_domain.js';
async function main() {
   await init('/pkg/telkom_domain_bg.wasm');
   run_app();
}
main()