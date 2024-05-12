use anyhow::Context;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{app_error::AppError, response_converter::ResponseBody};

#[derive(Serialize, Deserialize, Debug)]
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
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    name: String,
    job: String
}

pub fn routes()-> Router{
    Router::new()
        .route("/users", post(create_user))
}

async fn create_user(Json(user_request): Json<UserRequest>) -> Result<Json<UserResponse>, AppError> {
    let client = reqwest::Client::new();
    let response = client.post("https://reqres.in/api/users")
        .json(&user_request)
        .send()
        .await.context("failed to call post on https://reqres.in/api/users")?;
    Ok(Json(response.get_json().await?))
}