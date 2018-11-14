use openssl::symm::Cipher;
use openssl::symm::encrypt;
use openssl::symm::decrypt;
use openssl::rand::rand_bytes;

pub fn encrypt_block(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
  let cipher = Cipher::aes_256_cbc();
  return encrypt(cipher, key, Some(iv), data).unwrap();
}

pub fn encrypt_stream(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
  let cipher = Cipher::chacha20();
  return encrypt(cipher, key, Some(iv), data).unwrap();
}

pub fn decrypt_block(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
  let cipher = Cipher::aes_256_cbc();
  return decrypt(cipher, key, Some(iv), data).unwrap();
}

pub fn decrypt_stream(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
  let cipher = Cipher::chacha20();
  return decrypt(cipher, key, Some(iv), data).unwrap();
}

pub fn generate_key() -> [u8; 32] {
  let mut buf = [0; 32];
  rand_bytes(&mut buf).unwrap();
  return buf;
}

pub fn generate_nonce() -> [u8; 16] {
  let mut buf = [0; 16];
  rand_bytes(&mut buf).unwrap();
  return buf;
}
