use base64::{decode, encode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn base64Encode(s: &str) -> String {
    encode(s)
}

#[wasm_bindgen]
pub fn base64Decode(s: &str) -> String {
    String::from_utf8(decode(s).unwrap()).unwrap()
}

#[test]
fn test() {
    let s = "jesse";
    let a = "amVzc2U=";

    assert_eq!(a,base64Encode(s));
    assert_eq!(s,base64Decode(a));
}
