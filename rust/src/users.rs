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

pub fn routes(app_state: AppState)-> Router{
    Router::new()
        .route("/users", post(create_user))
        .with_state(app_state)
}



async fn create_user(
    State(AppState{user_base_url}): State<AppState>,
    Json(user_request): Json<UserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let client = reqwest::Client::new();
    let response = client.post(format!("{}/users", user_base_url))
        .json(&user_request)
        .send()
        .await.context("failed to call post on https://reqres.in/api/users")?;
    Ok(Json(response.get_json().await?))
}