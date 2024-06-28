use axum::Router;
use axum::routing::post;

use crate::controllers::upload::upload;

pub fn router() -> Router {
    Router::new().route("/upload", post(upload))
}
