use std::{fs::{self, File}, io::Write, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse};
use rand::rngs::OsRng;
use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, RsaPrivateKey, RsaPublicKey};
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::{db::UserExt, error::HttpError, models::User, AppState};



pub async fn generate_key(
    app_state: Arc<AppState>,
    user: User,
) -> Result<impl IntoResponse, HttpError> {

    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, 2048)
    .map_err(|e| {
        HttpError::server_error(e.to_string())
    })?;

    let public_key = RsaPublicKey::from(&private_key);

    let private_key_pem = private_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let public_key_prm = public_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let public_key_b64 = STANDARD.encode(public_key_prm.as_bytes());

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    app_state.db_client
    .save_user_key(user_id.clone(), public_key_b64.clone())
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let private_keys_dir = "assets/private_keys";
    fs::create_dir_all(&private_keys_dir)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let pem_file_path = format!("{}/{}.pem",private_keys_dir,user_id.clone());
    let mut file = File::create(&pem_file_path)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    file.write_all(private_key_pem.as_bytes())
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok((StatusCode::OK, "true"))

}