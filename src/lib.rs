use base64::{Engine, prelude::BASE64_STANDARD};
use hkdf::Hkdf;
use libaes::Cipher;
use sha2::Sha256;
use wasm_bindgen::prelude::*;

fn decode_secret(data: &str) -> Result<[u8; 16], String> {
    let mut secret_b64: Vec<u8> = data
        .chars()
        .map(|symbol| match symbol {
            '-' => '+',
            '_' => '/',
            x => x,
        })
        .map(|x| x.try_into().unwrap())
        .collect();
    secret_b64.push('='.try_into().unwrap());
    secret_b64.push('='.try_into().unwrap());

    let secret = BASE64_STANDARD
        .decode(secret_b64)
        .map_err(|err| err.to_string())?;
    let secret: [u8; 16] = secret.as_slice().try_into().unwrap();
    Ok(secret)
}

fn decode_data(data: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut iv: Vec<u8> = Vec::new();
    let mut payload: Vec<u8> = Vec::new();

    enum State {
        Iv,
        Payload,
    }
    let mut current_value = State::Iv;

    // Skip 2, cause first symbol is type, second is separator
    for symbol in data.chars().skip(2) {
        if symbol == '|' {
            match current_value {
                State::Iv => {
                    current_value = State::Payload;
                    continue;
                }
                State::Payload => {
                    break;
                }
            }
        };
        match current_value {
            State::Payload => payload.push(symbol.try_into().unwrap()),
            State::Iv => iv.push(symbol.try_into().unwrap()),
        }
    }
    Ok((
        BASE64_STANDARD.decode(iv).map_err(|err| err.to_string())?,
        BASE64_STANDARD
            .decode(payload)
            .map_err(|err| err.to_string())?,
    ))
}

fn decrypt(secret: &[u8; 16], iv: &[u8], data: &[u8]) -> Result<String, String> {
    let hk = Hkdf::<Sha256>::new(Some(b"bitwarden-send"), secret.as_slice());
    let mut okm = [0u8; 32];
    let info = b"send";
    hk.expand(info, &mut okm).map_err(|err| err.to_string())?;

    let cipher = Cipher::new_256(&okm);
    let x = cipher.cbc_decrypt(iv, data);
    String::from_utf8(x).map_err(|err| err.to_string())
}

#[wasm_bindgen]
pub fn decrypt_send(secret: &str, title: &str, body: &str) -> Result<Send, String> {
    let secret = decode_secret(secret)?;
    let title = decode_data(title)?;
    let title = decrypt(&secret, &title.0, &title.1)?;
    let body = decode_data(body)?;
    let body = decrypt(&secret, &body.0, &body.1)?;
    Ok(Send {
        title,
        secret: body,
    })
}

#[wasm_bindgen(getter_with_clone)]
pub struct Send {
    pub title: String,
    pub secret: String,
}
