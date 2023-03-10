use axum::{
    extract::Path,
    http::{header, HeaderValue, Response, StatusCode},
    routing::get,
    Router, ServiceExt,
};
use std::net::SocketAddr;
use tokio::fs;
use tower::layer::Layer;
use tower_http::normalize_path::NormalizePathLayer;

const DATA_FOLDER: &str = "data";
const PORT: u16 = 8000;

async fn fetch_json_file(path: String) -> Result<String, String> {
    let file_path = format!("{}/{}.json", DATA_FOLDER, path);

    match fs::read_to_string(file_path).await {
        Ok(json) => Ok(json),
        Err(_) => {
            let file_path = format!("{}/{}/index.json", DATA_FOLDER, path);
            match fs::read_to_string(file_path).await {
                Ok(json) => Ok(json),
                Err(_) => Err("File not found".to_owned()),
            }
        }
    }
}

async fn get_json(Path(path): Path<String>) -> Result<Response<String>, StatusCode> {
    // println!("path: {}", path);
    match fetch_json_file(path).await {
        Ok(json) => {
            let mut res = Response::new(json);
            res.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            Ok(res)
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let app = NormalizePathLayer::trim_trailing_slash()
        .layer(Router::new().route("/*path", get(get_json)));
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));

    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
