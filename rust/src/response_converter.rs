
use anyhow::Context;
use async_trait::async_trait;
use axum::body::Bytes;
use axum::body::Body;
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;

use crate::app_error::AppError;

pub trait BytesToConverter {
    fn to_string(&self)-> String;
}

impl BytesToConverter for Bytes {
    fn to_string(&self)-> String {
        String::from_utf8(self[..].to_vec()).unwrap()
    }
}

#[async_trait] 
pub trait ResponseBody {
    async fn get_body_as_string(self)-> Result<String, AppError>;
    async fn get_json<T>(self)-> Result<T, AppError> where T:DeserializeOwned;
}

#[async_trait]
impl ResponseBody for axum::response::Response<Body> {
    async fn get_body_as_string(self)-> Result<String, AppError>{
        Ok(self.into_body()
            .collect()
            .await?
            .to_bytes()
            .to_string())
    }
    async fn get_json<T>(self)-> Result<T, AppError> where T:DeserializeOwned{
        let body = self.get_body_as_string().await?;
        Ok(serde_json::from_str(&body).unwrap())
    }
}

#[async_trait] 
impl ResponseBody for reqwest::Response {
    async fn get_body_as_string(self)-> Result<String, AppError>{
        Ok(self.bytes().await.context("failed to get response body")?.to_string())
    }
    async fn get_json<T>(self)-> Result<T, AppError> where T: DeserializeOwned {
        let body = self.get_body_as_string().await?;
        Ok(serde_json::from_str(&body).context("failed to convert to struct")?)
    }
}