use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{Rng, rngs::OsRng};
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use std::fs;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn generate_rsa_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut OsRng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn encrypt_aes(key: &[u8], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let iv: [u8;16] = rand::random();
    let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext);
    (ciphertext, iv.to_vec())
}

pub fn decrypt_aes(key: &[u8], ciphertext: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(ciphertext).expect("Decryption failed")
}

pub fn encrypt_rsa(public_key: &RsaPublicKey, data: &[u8]) -> Vec<u8> {
    public_key.encrypt(&mut OsRng, PaddingScheme::new_pkcs1v15_encrypt(), data).expect("RSA encryption failed")
}

pub fn decrypt_rsa(private_key: &RsaPrivateKey, encrypted_data: &[u8]) -> Vec<u8> {
    private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), encrypted_data).expect("RSA decryption failed")
}
