pub use casey::lower;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use trillium::{Conn, KnownHeaderName, Status};

fn status_serialize<S>(status: &Status, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16((*status).into())
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub r#type: &'static str,
    pub title: &'static str,
    #[serde(serialize_with = "status_serialize")]
    pub status: Status,
    pub detail: &'static str,
    #[serde(flatten)]
    pub data: Option<Value>,
}

#[allow(unused)]
impl ApiError {
    pub fn with_detail(self, detail: &'static str) -> Self {
        let mut this = self.clone();
        this.detail = detail;
        this
    }

    pub fn with_title(self, title: &'static str) -> Self {
        let mut this = self.clone();
        this.title = title;
        this
    }

    pub fn with_data<D>(self, data: D) -> Self
    where
        D: Serialize,
    {
        let mut this = self.clone();
        this.data = match json!(data) {
            Value::Null => None,
            Value::Bool(value) => Some(json!({ "data": value })),
            Value::Number(value) => Some(json!({ "data": value })),
            Value::String(value) => Some(json!({ "data": value })),
            Value::Array(value) => Some(json!({ "data": value })),
            Value::Object(value) => Some(json!(value)),
        };
        this
    }
}

#[macro_export]
macro_rules! prefab_errors {
    ($(($i:ident, $t:expr, $d:expr, $s:ident),)+) => {
        $crate::prefab_errors!("https://docs.isua.land/problems/", $(($i, $t, $d, $s),)+);
    };
    ($base_url:expr, $(($i:ident, $t:expr, $d:expr, $s:ident),)+) => {
        $(
            #[allow(unused)]
            pub const $i: $crate::error::ApiError = $crate::error::ApiError {
                r#type: concat!($base_url, $crate::error::lower!(stringify!($i))),
                title: $t,
                status: trillium::Status::$s,
                detail: $d,
                data: None,
            };
        )*
    };
}

prefab_errors!(
    (DUMMY, "This is a dummy error.", "This error is solely for demonstrating the framework's ability to use problem+json errors.", ImATeapot),
    (NOT_FOUND, "Resource not found.", "The requested resource is not available. It might have been deleted or never existed.", NotFound),
    (INTERNAL_SERVER_ERROR, "Internal Server Error", "There was an error while processing your request that could not be recovered.", InternalServerError),
    (JSON_SERIALIZE_ERROR, "Internal Server Error", "Response payload could not be serialized as JSON.", InternalServerError),
    (JSON_DESERIALIZE_ERROR, "Bad Request", "The provided payload could not be parsed from a json string.", BadRequest),
    (VALIDATION_ERROR, "Invalid Payload", "The provided payload did not match the expected format", BadRequest),
);

pub trait ConnApiErrorExt {
    /// Halt the request and respond with a problem+json error.
    fn error(self, error: ApiError) -> Self;
}

impl ConnApiErrorExt for Conn {
    fn error(self, error: ApiError) -> Self {
        self.with_status(error.status)
            .with_response_header(
                KnownHeaderName::ContentType,
                "application/problem+json",
            )
            .with_body(serde_json::to_string(&error).unwrap())
            .halt()
    }
}
