
 const runtime = chrome.runtime || browser.runtime;

// This was helpful here -> https://github.com/theberrigan/rust-wasm-chrome-ext/blob/master/extension/js/content.js
const wasm_mod_url = runtime.getURL("brisk_auth_extension.js");

const loadupWasmMod = async () => {
  const { default: init } = await import(wasm_mod_url);
  return init().catch(() => null);
};

(async () => {
  const mod = await loadupWasmMod();

  if (mod) {
    const { run } = mod;
    run();
  }
})();
