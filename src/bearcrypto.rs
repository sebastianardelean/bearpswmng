use std::io::Write;

use orion::hazardous::{
    aead::xchacha20poly1305::{seal, open, Nonce, SecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
    stream::xchacha20::XCHACHA_NONCESIZE,
};

use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;
use orion::kdf::{derive_key, Password, Salt};
use rand::rand_core::{TryRngCore, OsRng};



const CHUNK_SIZE: usize = 128; 

fn get_random(dest: &mut [u8]) {
    OsRng.try_fill_bytes(dest).unwrap();

}

fn nonce() -> Vec<u8> {
    let mut randoms: [u8; 24] = [0; 24];
    get_random(&mut randoms);
    return randoms.to_vec();
}

fn auth_tag() -> Vec<u8> {
    let mut randoms: [u8; 32] = [0; 32];
    get_random(&mut randoms);
    return randoms.to_vec();
}


fn simple_split_encrypted(cipher_text: &[u8]) -> (Vec<u8>, Vec<u8>) {
    return (
        cipher_text[..CHACHA_KEYSIZE].to_vec(),
        cipher_text[CHACHA_KEYSIZE..].to_vec(),
        );
}


fn create_key(password: String, nonce: Vec<u8>) -> SecretKey {
    let password = Password::from_slice(password.as_bytes()).unwrap();
    let salt = Salt::from_slice(nonce.as_slice()).unwrap();
    let kdf_key = derive_key(&password, &salt, 15, 1024, CHACHA_KEYSIZE as u32).unwrap();
    let key = SecretKey::from_slice(kdf_key.unprotected_as_bytes()).unwrap();
    return key;
}




fn encrypt_core(
    dist: &mut Vec<u8>,      // output buffer instead of File
    contents: Vec<u8>,
    key: &SecretKey,
    nonce: Nonce,
) -> Result<(), orion::errors::UnknownCryptoError> {
    let ad = auth_tag();

    // Compute the total output length
    let output_len = contents
        .len()
        .checked_add(POLY1305_OUTSIZE + ad.len())
        .expect("Plaintext is too long");

    let mut output = vec![0u8; output_len];

    // Copy associated data at the beginning
    output[..CHACHA_KEYSIZE].copy_from_slice(ad.as_ref());

    // Encrypt using seal
    seal(
        key,
        &nonce,
        contents.as_slice(),
        Some(ad.as_ref()),
        &mut output[CHACHA_KEYSIZE..],
    )?;

    // Append encrypted chunk to the output buffer
    dist.extend_from_slice(&output);

    Ok(())
}

fn decrypt_core (
    dist: &mut Vec<u8>,       // output buffer
    contents: Vec<u8>,    // encrypted chunk
    key: &SecretKey,
    nonce: Nonce,
) -> Result<(), orion::errors::UnknownCryptoError> {
    
    let split = simple_split_encrypted(contents.as_slice());

    let mut output = vec![0u8; split.1.len() - POLY1305_OUTSIZE];

    open(&key, &nonce, split.1.as_slice(), Some(split.0.as_slice()), &mut output).unwrap();
    dist.extend_from_slice(&output);
    Ok(())
}


pub fn encrypt (
    plaintext: Vec<u8>,
    password: String
) -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {

    let mut ciphertext: Vec<u8> = Vec::new();
    let nonce = nonce();
    ciphertext.write_all(nonce.as_slice()).unwrap();

    let key = create_key(password, nonce.clone());
    let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();

    for src_chunk in plaintext.chunks(CHUNK_SIZE) {
        encrypt_core(&mut ciphertext,src_chunk.to_vec(), &key, nonce)?;
        
    }
    Ok(ciphertext)
}

pub fn decrypt(
    ciphertext: Vec<u8>,
    password: String,
) -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {
    let nonce_bytes = &ciphertext[..XCHACHA_NONCESIZE];
    let ciphertext_body = &ciphertext[XCHACHA_NONCESIZE..];
    let nonce = Nonce::from_slice(nonce_bytes).unwrap();
    let key = create_key(password, nonce_bytes.to_vec());

    let mut plaintext = Vec::new();

    for chunk in ciphertext_body.chunks(CHUNK_SIZE + CHACHA_KEYSIZE + POLY1305_OUTSIZE) {
        decrypt_core(&mut plaintext, chunk.to_vec(), &key, nonce)?;
    }

    Ok(plaintext)
}
