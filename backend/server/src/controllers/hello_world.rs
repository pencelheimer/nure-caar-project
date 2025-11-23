/// Hello World
///
/// Hello World Route
#[utoipa::path(get, path = "/hello", responses((status = OK, body = String)))]
pub async fn hello_world() -> String {
    "Hello, World!".to_string()
}
