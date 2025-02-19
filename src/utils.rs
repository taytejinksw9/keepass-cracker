pub fn generate_key(password: &str) -> Vec<u8> {
    let mut key = vec![0u8; 32];
    key.copy_from_slice(&password.as_bytes()[..32]);
    key
}