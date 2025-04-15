use base64::{Engine, prelude::BASE64_STANDARD};
use hkdf::Hkdf;
use libaes::Cipher;
use sha2::Sha256;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decrypt(secret: String, data: String, iv: String) -> Result<String, String> {
    let secret = BASE64_STANDARD
        .decode(secret.as_bytes())
        .map_err(|err| err.to_string())?;
    let secret: &[u8; 16] = secret.as_slice().try_into().unwrap();
    let data = BASE64_STANDARD
        .decode(data.as_bytes())
        .map_err(|err| err.to_string())?;
    let iv = BASE64_STANDARD
        .decode(iv.as_bytes())
        .map_err(|err| err.to_string())?;

    let hk = Hkdf::<Sha256>::new(Some(b"bitwarden-send"), secret.as_slice());
    let mut okm = [0u8; 32];
    let info = b"send";
    hk.expand(info, &mut okm).map_err(|err| err.to_string())?;

    let cipher = Cipher::new_256(&okm);
    let x = cipher.cbc_decrypt(iv.as_slice(), data.as_slice());
    // println!("{}", String::from_utf8(&x).unwrap());
    String::from_utf8(x).map_err(|err| err.to_string())
}
