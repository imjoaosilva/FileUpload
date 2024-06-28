use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::Multipart,
    http::{Response, StatusCode},
    response::IntoResponse,
};

pub async fn upload(
    mut uploaded_file: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(field) = uploaded_file.next_field().await.unwrap() {
        let file_name = format!(
            "{}{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            field.file_name().unwrap()
        );
        let bytes = field.bytes().await.unwrap();
        fs::write(format!("../uploads/{}", file_name), bytes).unwrap();

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(file_name)
            .unwrap())
    } else {
        Err((StatusCode::BAD_REQUEST, "No file uploaded".to_string()))
    }
}
