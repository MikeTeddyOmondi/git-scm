#![allow(unused)]

// use std::collections::HashMap;
// use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Branch {
    name: String,
    head_commit_id: Option<String>,
    created_at: u64,
    commits: Vec<String>,
}

impl Branch {
    pub fn new(name: String) -> Self {
        Branch {
            name,
            head_commit_id: None,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            commits: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn head_commit_id(&self) -> Option<&str> {
        self.head_commit_id.as_deref()
    }

    pub fn add_commit(&mut self, commit_id: String) {
        self.head_commit_id = Some(commit_id.clone());
        self.commits.push(commit_id);
    }

    pub fn commits(&self) -> &Vec<String> {
        &self.commits
    }
}