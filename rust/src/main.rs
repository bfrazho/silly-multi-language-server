mod app_error;
mod response_converter;
mod users;
use axum::Router;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().merge(routes());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn routes() -> Router {
    Router::new().merge(users::routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{response_converter::ResponseBody,
        users::UserResponse,
    };
    use axum::{body::Body, http::Request};
    use hyper::{header, Method, StatusCode};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn local_post_endpoint_calls_reqres_post_endpoint() {
        let routes = routes();

        let response = routes
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/users")
                    .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&json!({"name": "someone", "job": "role"})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, response.status());

        let user_response: UserResponse = response.get_json().await.unwrap();
        assert_eq!(user_response.get_name(), "someone");
        assert_eq!(user_response.get_job(), "role");
        assert!(i64::from_str_radix(user_response.get_id(), 10).is_ok());
    }
}
