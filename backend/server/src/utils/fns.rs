pub fn masked_api_key(api_key: &str) -> String {
    format!("...{}", api_key[api_key.len() - 13..].to_string())
}
