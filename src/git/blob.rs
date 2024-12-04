#![allow(unused)]

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Blob {
    content: Vec<u8>,
    hash: String,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());

        Blob { content, hash }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let content = fs::read(path)?;
        Ok(Blob::new(content))
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    /// Converts the Blob's content into a Vec<u8>.
    pub fn to_vec(&self) -> Vec<u8> {
        self.content.clone()
    }
}
