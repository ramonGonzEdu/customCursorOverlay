// Struct which contains a string and a lazy hash which is calculated when the string is hashed for the first time

// This is a wrapper around a string which calculates a hash of the string when it is first hashed

use std::hash::{Hash, Hasher};

pub struct HashString {
    string: String,
    hash: u64,
}

impl HashString {
    pub fn new(string: String) -> Self {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        string.hash(&mut hash);
        let hash = hash.finish();
        Self { string, hash }
    }
}

impl std::fmt::Debug for HashString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashString")
            .field("string", &self.string)
            .field("hash", &self.hash)
            .finish()
    }
}

impl std::fmt::Display for HashString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl std::hash::Hash for HashString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}
