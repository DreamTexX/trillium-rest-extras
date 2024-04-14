#[macro_export]
macro_rules! conn_try {
    ($expr:expr, $conn:expr) => {
        $crate::conn_try!($expr, $conn, $crate::error::INTERNAL_SERVER_ERROR)
    };
    ($expr:expr, $conn:expr, $error:expr) => {
        match $expr {
            Ok(value) => value,
            Err(error) => {
                use $crate::error::ConnApiErrorExt;
                #[cfg(debug_assertions)]
                return $conn.error($error
                    .with_data(serde_json::json!({
                        "debug": {
                            "error": error.to_string()
                        }
                    }))
                );
                #[cfg(not(debug_assertions))]
                return $conn.error($error);
            }
        }
    };
}

#[macro_export]
macro_rules! conn_unwrap {
    ($expr:expr, $conn:expr) => {
        $crate::conn_unwrap!($expr, $conn, $crate::error::INTERNAL_SERVER_ERROR)
    };
    ($expr:expr, $conn:expr, $error:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                use $crate::error::ConnApiErrorExt;
                return $conn.error($error);
            }
        }
    };
}

#[macro_export]
macro_rules! extract_payload {
    ($conn:expr, $type:ident) => {{
        use $crate::json_body::JsonBodyConnExt;
        let payload: $type = $crate::conn_try!(
            $conn.request_body_json().await,
            $conn,
            $crate::error::JSON_DESERIALIZE_ERROR
        );
        //$crate::conn_try!(payload.validate(), $conn, $crate::error::VALIDATION_ERROR);
        payload
    }};
}
