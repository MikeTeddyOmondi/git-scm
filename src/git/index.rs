#![allow(unused)]

use serde::{Deserialize, Serialize};

use crate::git::blob::Blob;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    entries: HashMap<PathBuf, Blob>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: PathBuf, blob: Blob) {
        self.entries.insert(path, blob);
    }

    pub fn remove(&mut self, path: &PathBuf) {
        self.entries.remove(path);
    }

    pub fn get_entries(&self) -> &HashMap<PathBuf, Blob> {
        &self.entries
    }

    pub fn save_to_disk(&self, path: &Path) -> Result<()> {
        let index_path = path.join(".git-scm").join("index");
        let serialized = bincode::serialize(self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Serialization error: {}", e))
        })?;
        fs::write(index_path, serialized)?;
        Ok(())
    }

    pub fn load_from_disk(path: &Path) -> Result<Self> {
        let index_path = path.join(".git-scm").join("index");
        if index_path.exists() {
            let data = fs::read(&index_path)?;
            let index: Index = bincode::deserialize(&data).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("Deserialization error: {}", e))
            })?;
            Ok(index)
        } else {
            Ok(Index::new())
        }
    }
}
