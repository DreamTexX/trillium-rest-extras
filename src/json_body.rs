use std::error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use trillium::{Conn, KnownHeaderName, Status};

use crate::error::{ConnApiErrorExt, JSON_SERIALIZE_ERROR};

#[async_trait::async_trait]
pub trait JsonBodyConnExt {
    fn json<V: Serialize>(self, value: V) -> Self;
    #[allow(dead_code)]
    fn with_json<V: Serialize>(self, value: V) -> Self;
    async fn request_body_json<T: DeserializeOwned>(
        &mut self,
    ) -> Result<T, Box<dyn Error>>;
}

#[async_trait::async_trait]
impl JsonBodyConnExt for Conn {
    fn json<V: Serialize>(self, value: V) -> Self {
        match serde_json::to_string(&value) {
            Ok(value) => {
                self.with_status(Status::Ok)
                    .with_response_header(
                        KnownHeaderName::ContentType,
                        "application/json",
                    )
                    .with_body(value)
                    .halt()
            }
            Err(e) => {
                self.error(
                    JSON_SERIALIZE_ERROR
                        .with_data(json!({ "innerError": e.to_string() })),
                )
            }
        }
    }

    fn with_json<V: Serialize>(self, value: V) -> Self {
        match serde_json::to_string(&value) {
            Ok(value) => {
                self.with_response_header(
                    KnownHeaderName::ContentType,
                    "application/json",
                )
                .with_body(value)
            }
            Err(e) => {
                self.error(
                    JSON_SERIALIZE_ERROR
                        .with_data(json!({ "innerError": e.to_string() })),
                )
            }
        }
    }

    async fn request_body_json<T: DeserializeOwned>(
        &mut self,
    ) -> Result<T, Box<dyn Error>> {
        let body = self.request_body().await;
        let bytes = body.read_bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
