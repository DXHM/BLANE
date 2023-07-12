extern crate crypto;

use crypto::rsa::{RsaPrivateKey, RsaPublicKey};
use crypto::symmetriccipher::SymmetricCipherError;
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::aes::{cbc_decryptor, cbc_encryptor, KeySize};
use crypto::blockmodes::NoPadding;

// RSA 加密
fn rsa_encrypt(public_key: &RsaPublicKey, plaintext: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut ciphertext = vec![0; public_key.size()];

    let mut encryptor = cbc_encryptor(
        KeySize::KeySize256,
        &public_key.encryptor(),
        &[0; 32],
        NoPadding,
    );

    let mut read_buffer = ReadBuffer::new(plaintext);
    let mut write_buffer = WriteBuffer::new(&mut ciphertext);

    encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

    match write_buffer.finish() {
        BufferResult::BufferOverflow => Ok(ciphertext),
        _ => Err(SymmetricCipherError::InvalidLength),
    }
}

// RSA 解密
fn rsa_decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut plaintext = vec![0; private_key.size()];

    let mut decryptor = cbc_decryptor(
        KeySize::KeySize256,
        &private_key.decryptor(),
        &[0; 32],
        NoPadding,
    );

    let mut read_buffer = ReadBuffer::new(ciphertext);
    let mut write_buffer = WriteBuffer::new(&mut plaintext);

    decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;

    match write_buffer.finish() {
        BufferResult::BufferOverflow => Ok(plaintext),
        _ => Err(SymmetricCipherError::InvalidLength),
    }
}

fn main() {
    // 生成 RSA 密钥对
    let (private_key, public_key) = RsaPrivateKey::new(512).keypair();

    let message = b"Hello, world!";
    println!("Original message: {:?}", message);

    // 使用公钥加密消息
    let encrypted_message = rsa_encrypt(&public_key, message).unwrap();
    println!("Encrypted message: {:?}", encrypted_message);

    // 使用私钥解密消息
    let decrypted_message = rsa_decrypt(&private_key, &encrypted_message).unwrap();
    println!("Decrypted message: {:?}", decrypted_message);
}
