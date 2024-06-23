use axum::extract::Multipart;

mod upload;

pub async fn upload(mut uploaded_file: Multipart) {
    if let Some(file) = uploaded_file.next_field().await.unwrap() {
        println!("{:?}", file)
    }
}
