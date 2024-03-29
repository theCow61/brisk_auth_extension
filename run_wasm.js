
const runtime = chrome.runtime || browser.runtime;

async function run() {
  await wasm_bindgen(runtime.getURL("brisk_auth_extension_bg.wasm"));
}

run();
