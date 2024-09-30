use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::Rng;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

use crate::error::HttpError;

pub async fn encrypt_file(
    file_data: Vec<u8>,
    user_public_key: &RsaPublicKey
) -> Result<(Vec<u8>,Vec<u8>,Vec<u8>), HttpError> {

    let mut aes_key = [0u8; 32];
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut aes_key);
    rand::thread_rng().fill(&mut iv);

    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(&aes_key, &iv)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut buffer = file_data.clone();
    let encrypted_data = cipher.encrypt_vec(&mut buffer);

    let encrypted_aes_key = user_public_key.encrypt(
        &mut rand::thread_rng(), 
        Pkcs1v15Encrypt, 
        &aes_key
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok((
        encrypted_aes_key,
        encrypted_data,
        iv.to_vec(),
    ))
}