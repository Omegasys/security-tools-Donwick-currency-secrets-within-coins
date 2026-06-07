#[cfg(test)]
mod tests {
    use axiom_crypto::hashing::hash_bytes;

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = hash_bytes(data);
        let hash2 = hash_bytes(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_uniqueness() {
        let hash1 = hash_bytes(b"abc");
        let hash2 = hash_bytes(b"def");
        assert_ne!(hash1, hash2);
    }
}
