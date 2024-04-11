use std::error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;
use trillium::{Conn, KnownHeaderName, Status};

use crate::conn_try;

#[async_trait::async_trait]
pub trait JsonBodyConnExt {
    fn json<V: Serialize>(self, value: V) -> Self;
    #[allow(dead_code)]
    fn with_json<V: Serialize>(self, value: V) -> Self;
    async fn request_body_json<T: DeserializeOwned>(&mut self) -> Result<T, Box<dyn Error>>;
}

#[async_trait::async_trait]
impl JsonBodyConnExt for Conn {
    fn json<V: Serialize>(self, value: V) -> Self {
        let serialized = conn_try!(
            serde_json::to_string(&value),
            self,
            crate::error::JSON_SERIALIZE_ERROR
        );

        self.with_status(Status::Ok)
            .with_response_header(KnownHeaderName::ContentType, "application/json")
            .with_body(serialized)
            .halt()
    }

    fn with_json<V: Serialize>(self, value: V) -> Self {
        let serialized = conn_try!(
            serde_json::to_string(&value),
            self,
            crate::error::JSON_SERIALIZE_ERROR
        );

        self.with_response_header(KnownHeaderName::ContentType, "application/json")
            .with_body(serialized)
    }

    async fn request_body_json<T: DeserializeOwned>(&mut self) -> Result<T, Box<dyn Error>> {
        let body = self.request_body().await;
        let bytes = body.read_bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
