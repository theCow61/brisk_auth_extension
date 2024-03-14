use std::time::{SystemTime, UNIX_EPOCH};
use data_encoding::BASE32_NOPAD;
use security_framework::item::{ItemAddOptions, ItemAddValue, ItemClass, add_item};
use core_foundation::data::CFData;

use std::env;

use hmac_sha1::hmac_sha1;



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
