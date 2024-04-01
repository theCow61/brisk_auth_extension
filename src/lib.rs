use data_encoding::BASE32_NOPAD;
use hmac_sha1::hmac_sha1;

use wasm_bindgen::prelude::*;
use web_sys::{Element, Document, HtmlElement, HtmlInputElement, Storage, js_sys::{eval, Reflect}};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn popup() {
    alert("test");
}

/// getLocalStorage in stores.js should run this upon getting the key
#[wasm_bindgen]
pub fn got_key(key: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let auth_code = document.get_element_by_id("input8").unwrap().dyn_into::<HtmlInputElement>().unwrap(); // probably subject to change
    auth_code.set_value(&format!("{}", totp_code(key)));
}

#[wasm_bindgen(module = "/stores.js")]
extern "C" {
    pub fn getLocalStorage(key: &str);
}

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let button = document.create_element("button").unwrap();
    button.set_id("insert-button");
    button.set_inner_html("Click Here!");
    body.append_child(&button).unwrap();

    
    let a = Closure::<dyn FnMut()>::new(move || {
        // TODO: find the input field instead of using "input8"
        getLocalStorage("key");
    });

    button.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

fn totp_code(key: &str) -> u32 {


    let key = BASE32_NOPAD.decode(key.as_bytes()).unwrap();
    //
    // let item_class = ItemClass::key();
    // let cf_data = CFData::from_buffer(&key);
    // let item_add_value = ItemAddValue::Data { class: item_class, data: cf_data };
    // let item_add_options = ItemAddOptions::new(item_add_value);
    // add_item(item_add_options.to_dictionary()).unwrap();
    let time_step = 30; // 30 seconds
    // let current_unix_time_from_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let current_unix_time_from_epoch: u64 = web_sys::js_sys::Date::now() as u64 / 1000;


    let time_input = current_unix_time_from_epoch / time_step;

    // let h_result = HMAC::mac(&time_input.to_be_bytes(), &key);
    let h_result: [u8; 20] = hmac_sha1(&key, &time_input.to_be_bytes());
    // let h_result: [u8; 20] = HMAC::mac(&key, &time_input.to_le_bytes());
    let offset = (h_result[19] & 0x0f) as usize; // last 4 bits of last byte of our h_result
    
    // 0000 0000  0000 0000  0000 0000  0000 0000
    // 1'st byte  2'nd byte  3'rd byte  4'th byte
    // Big Endian
    let piece = ((h_result[offset] as u32) & 0x7f) << 24 | (h_result[offset + 1] as u32) << 16 | (h_result[offset + 2] as u32) << 8 | (h_result[offset + 3] as u32);
    let code = piece % 1000000;
    code
}

