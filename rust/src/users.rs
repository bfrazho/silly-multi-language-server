use anyhow::Context;
use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{app_error::AppError, response_converter::ResponseBody, AppState};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct UserResponse {
    name: String,
    job: String,
    id: String,
}


#[cfg(test)]
pub mod test {
    use super::UserResponse;

    impl UserResponse{
        pub fn get_name(&self)-> &str {
            &self.name
        }
        pub fn get_job(&self)-> &str {
            &self.job
        }
        pub fn get_id(&self)-> &str {
            &self.id
        }
        pub fn new(name: String, job: String, id: String)-> Self{
            Self{name, job, id}
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    name: String,
    job: String
}

pub fn routes(app_state: &AppState)-> Router{
    Router::new()
        .route("/users", post(create_user))
        .with_state(app_state.to_owned())
}

async fn create_user(
    State(AppState{user_base_url, client, ..}): State<AppState>,
    Json(user_request): Json<UserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let response = client.post(format!("{}/users", user_base_url))
        .json(&user_request)
        .send()
        .await.context("failed to call post on https://reqres.in/api/users")?;
    Ok(Json(response.get_json().await?))
}

#[cfg(test)]
mod tests {
    use crate::{routes, AppState};
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
        let routes = routes(AppState::default().set_user_base_url("https://reqres.in/api"));

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
        let input_json = json!({"name": "the name", "job": "the role"});
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then|{
            when.method(POST)
                .path("/users")
                .json_body(input_json.clone());
            then.status(200)
                .json_body(json!({"name": "the name", "job": "the role", "id": "the id"}));
        });
        let routes = routes(AppState::default().set_user_base_url(&mock_server.base_url()));

        let response = routes
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&input_json).unwrap(),
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
