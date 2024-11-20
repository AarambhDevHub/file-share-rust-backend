use crate::error::HttpError;
use aes::cipher::BlockSizeUser;
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use rand::Rng;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

pub async fn encrypt_file(
    file_data: Vec<u8>,
    user_public_key: &RsaPublicKey,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), HttpError> {
    let mut aes_key = [0u8; 32];
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut aes_key);
    rand::thread_rng().fill(&mut iv);

    let cipher = cbc::Encryptor::<Aes256>::new_from_slices(&aes_key, &iv)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    // Clone the file data and pad it
    let mut buffer = file_data.clone();
    let msg_len = buffer.len();

    // Pad the buffer to the block size
    buffer.resize(msg_len + Aes256::block_size(), 0);

    // Encrypt the data
    let encrypted_data = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, msg_len)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let encrypted_aes_key = user_public_key
        .encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, &aes_key)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok((encrypted_aes_key, encrypted_data.to_vec(), iv.to_vec()))
}
