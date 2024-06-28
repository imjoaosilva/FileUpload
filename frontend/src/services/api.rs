use reqwest::Response;
use web_sys::FileList;

pub async fn upload_file(f: FileList) -> Response {
    let file = gloo::file::File::from(f.get(0).unwrap());
    let data = gloo::file::futures::read_as_bytes(&file)
        .await
        .expect("Failed to read file");

    let form = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(data).file_name(file.name()),
    );

    let res = reqwest::Client::new()
        .post("http://localhost:3000/api/v1/upload")
        .multipart(form)
        .send()
        .await
        .unwrap();

    res
}
