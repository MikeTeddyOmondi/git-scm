#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hasher;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::git::blob::Blob;

#[derive(Debug)]
pub struct Commit {
    pub id: String,
    pub parent: Option<String>,
    pub tree: HashMap<PathBuf, Vec<u8>>,
    pub message: String,
    pub timestamp: u64,
    pub author: String,
}

impl Commit {
    pub fn new(
        parent: Option<String>,
        tree: HashMap<PathBuf, Vec<u8>>,
        message: String,
        author: String
    ) -> Self {
        // Generate a simple hash for the commit
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&message, &mut hasher);
        std::hash::Hash::hash(&SystemTime::now(), &mut hasher);
        let id = format!("{:x}", hasher.finish());

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Commit {
            id,
            parent,
            tree,
            message,
            timestamp,
            author,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_tree(&self) -> &HashMap<PathBuf, Vec<u8>> {
        &self.tree
    }
}
