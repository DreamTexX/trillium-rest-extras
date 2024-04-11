# trillium-rest-extras

A simple collection of extensions for [trillium](https://trillium.rs) that aims to provide a more fluid experience when using it to build Rest APIs.

## Json Body

### `conn.json(Value) -> Conn`

This method will serialise the provided value, set the HTTP status code to OK (200), set the content type to `application/json' and halt. If a serialisation error occurs, it will fail and return an internal server error.

### `conn.with_json(Value) -> Conn`

This method will serialise the provided value or return an internal server error when serialisation fails.

### `conn.request_body_json() -> Result<Value, Box<dyn Error>>`

This method will deserialise the request payload from a json string.

## Macros

### `conn_try!(Result, Conn, ?Error)`

This macro evaluates the specified result and returns the Ok value or returns the function with the specified error or an internal server error.

### `conn_unwrap!(Expression, Conn, ?Error)`

This macro unwraps the given expression returning the value or returns the function with the specified error or an internal server error.

## Typed Parameters

### `conn.typed_param(Name) -> Option<Value>`

Extracts an url parameter as the given type. Returns None if the type conversion fails or the parameter does not exist.

## Error

