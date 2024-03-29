use std::time::{SystemTime, UNIX_EPOCH};
use data_encoding::BASE32_NOPAD;
use std::env;
use hmac_sha1::hmac_sha1;

use wasm_bindgen::prelude::*;
use web_sys::{Element, Document, HtmlElement, HtmlInputElement, Storage, js_sys::eval};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn popup() {
    alert("test");
}

#[wasm_bindgen(module = "/stores.js")]
extern "C" {
    pub fn getLocalStorage(key: &str) -> String;
}


// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn run() {
    alert("hi");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // let val = document.create_element("h1").unwrap();
    // 
    // val.set_inner_html("YOO");
    // body.append_child(&val).unwrap();
    
    // let it = document.get_element_by_id("input38").unwrap();
    // it.set_text_content(Some("hi"));
    // body.append_child(&it).unwrap();
    //

    let button = document.create_element("button").unwrap();
    button.set_id("insert-button");
    button.set_inner_html("Click Here!");
    body.append_child(&button).unwrap();

    
    let a = Closure::<dyn FnMut()>::new(move || {
        let auth_code = document.get_element_by_id("input8").unwrap().dyn_into::<HtmlInputElement>().unwrap(); // probably subject to change

        let storage = window.local_storage().unwrap().unwrap();
        match storage.get_item("key").unwrap() {
            Some(value) => {
                auth_code.set_value(&value);
            },
            None => {
                auth_code.set_value("0");
            }
        }
        
        auth_code.set_value(&getLocalStorage("key"));


    });

    button.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();

    // while document.get_element_by_id("input8") == None {
    //
    // }
    //
    // let auth_code = document.get_element_by_id("input8").unwrap().dyn_into::<HtmlInputElement>().unwrap(); // probably subject to change
    // auth_code.set_value("333");

    // email.set_inner_html("Test");

    // let div = document.create_element("div").unwrap();
    // div.set_id("test");
    // div.append_child(&val).unwrap();
    // body.append_child(&div).unwrap();
}


fn main() {


    let key_b32 = env::var("BRISK_AUTH_KEY").unwrap();

    let key = BASE32_NOPAD.decode(key_b32.as_bytes()).unwrap();
    //
    // let item_class = ItemClass::key();
    // let cf_data = CFData::from_buffer(&key);
    // let item_add_value = ItemAddValue::Data { class: item_class, data: cf_data };
    // let item_add_options = ItemAddOptions::new(item_add_value);
    // add_item(item_add_options.to_dictionary()).unwrap();



    let time_step = 30; // 30 seconds
    let current_unix_time_from_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

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
    println!("{}", code);
    


}
