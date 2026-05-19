use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn encode_base64(input: &str) -> String {
    general_purpose::STANDARD.encode(input.as_bytes())
}

pub fn decode_base64(input: &str) -> Option<String> {
    let bytes = general_purpose::STANDARD.decode(input).ok()?;
    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto() {
        let input = "hello";
        let hash = hash_sha256(input);
        assert_eq!(hash, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        
        let encoded = encode_base64(input);
        assert_eq!(encoded, "aGVsbG8=");
        
        let decoded = decode_base64(&encoded);
        assert_eq!(decoded, Some(input.to_string()));
    }
}
