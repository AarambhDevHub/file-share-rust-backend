use crate::error::HttpError;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use aes::Aes256;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

pub async fn decrypt_file(
    encrypted_aes_key: Vec<u8>,
    encrypted_file_data: Vec<u8>,
    iv: Vec<u8>,
    user_private_key: &RsaPrivateKey,
) -> Result<Vec<u8>, HttpError> {
    let aes_key = user_private_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_aes_key)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let cipher = cbc::Decryptor::<Aes256>::new_from_slices(&aes_key, &iv)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut buffer = encrypted_file_data.clone();

    let decrypted_data = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(decrypted_data.to_vec())
}
