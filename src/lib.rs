use base64::{Engine, prelude::BASE64_STANDARD};
use hkdf::Hkdf;
use libaes::Cipher;
use sha2::Sha256;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode(secret: String, data: String, iv: String) -> String {
    let secret = BASE64_STANDARD.decode(secret.as_bytes()).unwrap();
    let secret: &[u8; 16] = secret.as_slice().try_into().unwrap();
    let data = BASE64_STANDARD.decode(data.as_bytes()).unwrap();
    let iv = BASE64_STANDARD.decode(iv.as_bytes()).unwrap();

    let hk = Hkdf::<Sha256>::new(Some(b"bitwarden-send"), secret.as_slice());
    let mut okm = [0u8; 32];
    let info = b"send";
    hk.expand(info, &mut okm).unwrap();

    let cipher = Cipher::new_256(&okm);
    let x = cipher.cbc_decrypt(iv.as_slice(), data.as_slice());
    // println!("{}", String::from_utf8(&x).unwrap());
    String::from_utf8(x).unwrap()
}
// 2.UksuGeuUOU7lSsPQC0IpYQ==|U2Hc6t5p8PDDxJqJjLtSYA==|IhbqwuDeooUvYf/Mn8JOzOgIH1/7qKzurBRI7wUxVV8=
