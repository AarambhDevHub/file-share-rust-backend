use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use validator::Validate;

use crate::{db::UserExt, dtos::{RequestQueryDto, UserReceiveFileDto, UserReceiveFileListResponseDto, UserSendFileDto, UserSendFileListResponseDto}, error::HttpError, middleware::JWTAuthMiddeware, AppState};

pub fn get_file_list_handler() -> Router {
    Router::new()
       .route("/send", get(get_user_shared_files))
       .route("/receive", get(get_receive_shared_files))
}


pub async fn get_user_shared_files(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let (shared_files, total_count) = app_state.db_client
        .get_sent_files(user_id.clone(), page as u32, limit)
       .await
       .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filter_send_files = UserSendFileDto::filter_send_user_files(&shared_files);

    let response = UserSendFileListResponseDto {
        status: "success".to_string(),
        files: filter_send_files,
        results: total_count,
    };

    Ok(Json(response))
}

pub async fn get_receive_shared_files(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let (receive_files, total_count) = app_state.db_client
        .get_receive_files(user_id.clone(), page as u32, limit)
       .await
       .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filter_receive_files = UserReceiveFileDto::filter_receive_user_files(&receive_files);

    let response = UserReceiveFileListResponseDto {
        status: "success".to_string(),
        files: filter_receive_files,
        results: total_count,
    };

    Ok(Json(response))
}