mod app_error;
mod response_converter;
mod users;
use axum::Router;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Clone)]
struct AppState {
    pub user_base_url: String
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = Router::new().merge(routes("https://reqres.in/api".to_string()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());


    axum::serve(listener, app).await.unwrap();
}

fn routes(user_base_url: String) -> Router {
    let app_state = AppState{user_base_url: user_base_url};
    Router::new().merge(users::routes(app_state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{response_converter::ResponseBody,
        users::UserResponse,
    };
    use axum::{body::Body, http::Request};
    use httpmock::{Method::POST, MockServer};
    use hyper::{header, Method, StatusCode};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn local_post_endpoint_calls_reqres_post_endpoint() {
        let routes = routes("https://reqres.in/api".to_string());

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

    #[tokio::test]
    async fn mockserver_create_user() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then|{
            when.method(POST)
                .path("/users")
                .json_body(json!({"name": "the name", "job": "the role"}));
            then.status(200)
                .json_body(json!({"name": "the name", "job": "the role", "id": "the id"}));
        });
        let routes = routes(mock_server.base_url());

        let response = routes
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({"name": "the name", "job": "the role"})).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

        mock.assert();
        let user_response: UserResponse = response.get_json().await.unwrap();
        assert_eq!(UserResponse::new("the name".to_string(), "the role".to_string(), "the id".to_string()), user_response);
    }
}
