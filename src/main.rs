extern crate openssl;

use std::env;

mod file;
mod encryption;

fn encrypt_block(content: Vec<u8>, keyfile: &str) -> Vec<u8> {
  let key = encryption::generate_key();
  let iv = encryption::generate_nonce();
  let key = key.as_ref();
  let iv = iv.as_ref();

  let mut ciphertext = encryption::encrypt_block(content.as_ref(), key, iv);
  file::write(keyfile, key);

  let mut result = iv.to_vec();
  result.append(&mut ciphertext);
  return result;
}

fn encrypt_stream(content: Vec<u8>, keyfile: &str) -> Vec<u8> {
  let key = encryption::generate_key();
  let iv = encryption::generate_nonce();

  let mut ciphertext = encryption::encrypt_stream(content.as_ref(), key.as_ref(), iv.as_ref());
  file::write(keyfile, key.as_ref());

  let mut result = iv.to_vec();
  result.append(&mut ciphertext);
  return result;
}

fn decrypt_block(content: Vec<u8>, keyfile: &str) -> Vec<u8> {
  let key = file::read(keyfile);

  let length = content.len();
  let iv = content[0..16].as_ref();
  let data = content[16..length].as_ref();

  return encryption::decrypt_block(data, key.as_ref(), iv);
}

fn decrypt_stream(content: Vec<u8>, keyfile: &str) -> Vec<u8> {
  let key = file::read(keyfile);

  let length = content.len();
  let iv = content[0..16].as_ref();
  let data = content[16..length].as_ref();

  return encryption::decrypt_stream(data, key.as_ref(), iv);
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let query = &args[1];
  let source = &args[2];
  let destination = &args[3];
  let keyfile = &args[4];

  let content = file::read(source);

  let result = match query.as_str() {
    "encrypt-block" => Ok(encrypt_block(content, keyfile)),
    "encrypt-stream" => Ok(encrypt_stream(content, keyfile)),
    "decrypt-block" => Ok(decrypt_block(content, keyfile)),
    "decrypt-stream" => Ok(decrypt_stream(content, keyfile)),
    _ => Err("Invalid argument"),
  };

  let result = result.unwrap();
  file::write(destination, result.as_ref());
}
