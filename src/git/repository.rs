#![allow(unused)]

use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use crate::git::blob::Blob;
use crate::git::branch::Branch;
use crate::git::commit::Commit;
use crate::git::diff::{DiffResult, Differ};
use crate::git::index::Index;
use crate::git::merge::{MergeConflict, Merger};

#[derive(Debug)]
pub struct Repository {
    root_path: PathBuf,
    index: Index,
    commits: HashMap<String, Commit>,
    branches: HashMap<String, Branch>,
    current_branch: String,
}

impl Repository {
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        let root_path = path.as_ref().to_path_buf();

        // Create .vcrs directory
        let git_dir = root_path.join(".git-scm");
        fs::create_dir_all(git_dir.join("objects"))?;
        fs::create_dir_all(git_dir.join("refs"))?;

        // Create main branch
        let mut branches = HashMap::new();
        let main_branch = Branch::new("main".to_string());
        branches.insert("main".to_string(), main_branch);

        Ok(Repository {
            root_path,
            index: Index::new(),
            commits: HashMap::new(),
            branches,
            current_branch: "main".to_string(),
        })
    }

    fn parse_gitignore(&self) -> Result<GlobSet> {
        let gitignore_path = self.root_path.join(".gitignore");
        let mut builder = GlobSetBuilder::new();

        if gitignore_path.exists() {
            let patterns = fs::read_to_string(gitignore_path)?;
            for line in patterns.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    builder.add(Glob::new(trimmed).map_err(|e| {
                        Error::new(std::io::ErrorKind::InvalidInput, e.to_string())
                    })?);
                }
            }
        }

        builder
            .build()
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn add(&mut self, path: &Path) -> Result<()> {
        let ignore_set = self.parse_gitignore()?;

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();

                // Skip ignored files and directories
                if ignore_set.is_match(&entry_path) {
                    continue;
                }

                if entry_path.is_file() {
                    let blob = Blob::from_file(&entry_path)?;
                    self.index.add(entry_path, blob);
                } else if entry_path.is_dir() {
                    // Recursively add directory contents
                    self.add(&entry_path)?;
                }
            }
        } else if path.is_file() {
            // Skip ignored files
            if ignore_set.is_match(path) {
                return Ok(());
            }

            let blob = Blob::from_file(path)?;
            self.index.add(path.to_path_buf(), blob);
        }

        // Persist the index after adding files
        self.index.save_to_disk(&self.root_path)?;

        Ok(())
    }

    pub fn commit(&mut self, message: String, author: String) -> Result<String> {
        // Load the index from disk to ensure it's up to date
        self.index = Index::load_from_disk(&self.root_path)?;

        // Convert index entries to tree (HashMap<PathBuf, Blob>)
        let tree = self
            .index
            .get_entries()
            .iter()
            .map(|(path, blob)| (path.clone(), blob.clone().to_vec()))
            .collect::<HashMap<PathBuf, Vec<u8>>>();

        // Get the last commit's ID for the current branch
        let parent = self
            .branches
            .get(&self.current_branch)
            .and_then(|branch| branch.head_commit_id().map(|id| id.to_string()));

        // Create new commit
        let commit = Commit::new(parent, tree, message, author);
        let commit_id = commit.get_id().to_string();

        // Save commit
        self.commits.insert(commit_id.clone(), commit);

        // Update current branch
        if let Some(branch) = self.branches.get_mut(&self.current_branch) {
            branch.add_commit(commit_id.clone());
        }

        // Save the repository state to disk
        self.index = Index::new(); // Clear the index
        self.index.save_to_disk(&self.root_path)?; // Save the cleared index

        Ok(commit_id)
    }

    pub fn log(&self) {
        for (id, commit) in &self.commits {
            println!("Commit ID: {}", id);
            println!("Message: {}", commit.message);
            println!("Author: {}", commit.author);
            println!("Timestamp: {}", commit.timestamp);
            println!("Parent: {:?}", commit.parent);
            println!("------------------------------");
            println!("Commit Tree:");
            for (path, blob) in &commit.tree {
                println!("  Path: {:?}, Blob: {:?}", path, blob);
            }
            println!("------------------------------");
        }
    }

    pub fn create_branch(&mut self, name: String) -> Result<()> {
        if self.branches.contains_key(&name) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Branch already exists",
            ));
        }

        let mut new_branch = Branch::new(name.clone());

        // If current branch has a head commit, use it as the starting point
        if let Some(branch) = self.branches.get(&self.current_branch) {
            if let Some(head_commit_id) = branch.head_commit_id() {
                new_branch.add_commit(head_commit_id.to_string());
            }
        }

        self.branches.insert(name.clone(), new_branch);
        Ok(())
    }

    pub fn switch_branch(&mut self, name: &str) -> Result<()> {
        if !self.branches.contains_key(name) {
            return Err(Error::new(ErrorKind::NotFound, "Branch not found"));
        }

        self.current_branch = name.to_string();
        Ok(())
    }

    pub fn diff(&self, branch1: &str, branch2: &str) -> Result<Vec<DiffResult>> {
        let branch1_commits = self
            .branches
            .get(branch1)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Branch not found"))?;
        let branch2_commits = self
            .branches
            .get(branch2)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Branch not found"))?;

        // Get the latest commits for both branches
        let branch1_commit_id = branch1_commits
            .head_commit_id()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No commits in branch"))?;
        let branch2_commit_id = branch2_commits
            .head_commit_id()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No commits in branch"))?;

        let branch1_commit = self
            .commits
            .get(branch1_commit_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Commit not found"))?;
        let branch2_commit = self
            .commits
            .get(branch2_commit_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Commit not found"))?;

        // Perform diff
        Ok(Differ::diff(
            &branch1_commit.get_tree(),
            &branch2_commit.get_tree(),
        ))
    }

    pub fn merge(
        &mut self,
        base_branch: &str,
        source_branch: &str,
        target_branch: &str,
    ) -> Result<()> {
        // Get branches
        let base_branch_obj = self
            .branches
            .get(base_branch)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Base branch not found"))?;
        let source_branch_obj = self
            .branches
            .get(source_branch)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Source branch not found"))?;
        let target_branch_obj = self
            .branches
            .get(target_branch)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Target branch not found"))?;

        // Get latest commits
        let base_commit_id = base_branch_obj
            .head_commit_id()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No commits in base branch"))?;
        let source_commit_id = source_branch_obj
            .head_commit_id()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No commits in source branch"))?;
        let target_commit_id = target_branch_obj
            .head_commit_id()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No commits in target branch"))?;

        // Get commit objects
        let base_commit = self
            .commits
            .get(base_commit_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Base commit not found"))?;
        let source_commit = self
            .commits
            .get(source_commit_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Source commit not found"))?;
        let target_commit = self
            .commits
            .get(target_commit_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Target commit not found"))?;

        // Perform three-way merge using the Merger module
        match Merger::merge(
            &base_commit.get_tree(),
            &source_commit.get_tree(),
            &target_commit.get_tree(),
        ) {
            Ok(merged_tree) => {
                // Create a new commit for the merge
                let parent_ids = vec![base_commit_id, source_commit_id, target_commit_id];
                let parent_str = parent_ids.join(",");

                let commit = Commit::new(
                    Some(parent_str),
                    merged_tree,
                    format!(
                        "Merge {} and {} into {}",
                        source_branch, target_branch, base_branch
                    ),
                    "merge-tool".to_string(),
                );
                let commit_id = commit.get_id().to_string();

                // Save the commit and update the base branch
                self.commits.insert(commit_id.clone(), commit);
                if let Some(branch) = self.branches.get_mut(base_branch) {
                    branch.add_commit(commit_id);
                }

                Ok(())
            }
            Err(conflicts) => {
                // Handle merge conflicts
                for conflict in conflicts {
                    println!("Merge conflict: {:?}", conflict);
                }
                Err(Error::new(ErrorKind::Other, "Merge conflicts detected"))
            }
        }
    }

    // method to support retrieving the current branch name
    pub fn current_branch(&self) -> &str {
        &self.current_branch
    }

    // method to support logging branches
    pub fn branches(&self) -> &HashMap<String, Branch> {
        &self.branches
    }
}
