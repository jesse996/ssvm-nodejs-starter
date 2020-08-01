use base64::{decode, encode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn base64Encode(s: &str) -> String {
    encode(s)
}

#[wasm_bindgen]
pub fn base64Decode(s: &str) -> String {
    // String::from_utf8(decode(s).unwrap()).unwrap()
    String::from_utf8(decode(s).unwrap()).unwrap()
}

#[test]
fn test() {
    let s = "jesse";
    let a = "amVzc2U=";

    // dbg!(base64Encode(s));
    // dbg!(base64Decode(a));

    let encode = base64Encode(s);
    dbg!(&encode);
    let decode = base64Decode(encode.as_str());
    dbg!(decode);
}
