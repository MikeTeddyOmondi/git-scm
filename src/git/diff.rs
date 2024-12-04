#![allow(unused)]

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub enum DiffType {
    Added,
    Modified,
    Deleted,
    Unchanged,
}

#[derive(Debug)]
pub struct DiffResult {
    pub file_path: PathBuf,
    pub diff_type: DiffType,
    pub old_content: Option<Vec<u8>>,
    pub new_content: Option<Vec<u8>>,
}

pub struct Differ;

impl Differ {
    pub fn diff(
        old_tree: &HashMap<PathBuf, Vec<u8>>,
        new_tree: &HashMap<PathBuf, Vec<u8>>
    ) -> Vec<DiffResult> {
        let mut diffs = Vec::new();

        // Check for added and modified files
        for (path, new_content) in new_tree {
            if let Some(old_content) = old_tree.get(path) {
                // File exists in both trees
                if old_content != new_content {
                    diffs.push(DiffResult {
                        file_path: path.clone(),
                        diff_type: DiffType::Modified,
                        old_content: Some(old_content.clone()),
                        new_content: Some(new_content.clone()),
                    });
                }
            } else {
                // New file
                diffs.push(DiffResult {
                    file_path: path.clone(),
                    diff_type: DiffType::Added,
                    old_content: None,
                    new_content: Some(new_content.clone()),
                });
            }
        }

        // Check for deleted files
        for (path, _) in old_tree {
            if !new_tree.contains_key(path) {
                diffs.push(DiffResult {
                    file_path: path.clone(),
                    diff_type: DiffType::Deleted,
                    old_content: old_tree.get(path).cloned(),
                    new_content: None,
                });
            }
        }

        diffs
    }

    pub fn render_diff(diff: &DiffResult) -> String {
        match diff.diff_type {
            DiffType::Added => format!("+ Added: {}", diff.file_path.display()),
            DiffType::Deleted => format!("- Deleted: {}", diff.file_path.display()),
            DiffType::Modified => format!(
                "* Modified: {} (Old size: {}, New size: {})", 
                diff.file_path.display(),
                diff.old_content.as_ref().map(|c| c.len()).unwrap_or(0),
                diff.new_content.as_ref().map(|c| c.len()).unwrap_or(0)
            ),
            DiffType::Unchanged => format!("  Unchanged: {}", diff.file_path.display()),
        }
    }
}