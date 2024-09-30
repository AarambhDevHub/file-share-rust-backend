use std::sync::Arc;

use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};
use axum_extra::extract::cookie::CookieJar;

use crate::{db::UserExt, error::{ErrorMessage, HttpError}, models::User, utils::token, AppState};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddeware {
    pub user: User,
}

pub async fn auth(
    cookie_jar: CookieJar,
    Extension(app_state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, HttpError> {
    let cookies = cookie_jar
            .get("token")
            .map(|cookie| cookie.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(header::AUTHORIZATION)
                    .and_then(|auth_header| auth_header.to_str().ok())
                    .and_then(|auth_value| {
                        if auth_value.starts_with("Bearer ") {
                            Some(auth_value[7..].to_owned())
                        }else {
                            None
                        }
                    })  
            });
    let token = cookies.ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::TokenNotProvided.to_string())
    })?;

    let token_details = 
        match token::decode_token(token, app_state.env.jwt_secret.as_bytes()) {
            Ok(token_details) => token_details,
            Err(_) => {
                return Err(HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()));
            }
        };
    
    let user_id = uuid::Uuid::parse_str(&token_details.to_string()).unwrap();

    let user = app_state.db_client.get_user(Some(user_id), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = user.ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::UserNoLongerExist.to_string())
    })?;

    req.extensions_mut().insert(JWTAuthMiddeware {
        user: user.clone(),
    });

    Ok(next.run(req).await)
}