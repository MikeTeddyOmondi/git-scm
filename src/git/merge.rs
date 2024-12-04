#![allow(unused)]

use std::collections::HashMap;
use std::path::PathBuf;

use super::blob::Blob;

#[derive(Debug)]
pub enum MergeConflict {
    ContentConflict {
        file_path: PathBuf,
        base_content: Vec<u8>,
        branch1_content: Vec<u8>,
        branch2_content: Vec<u8>,
    },
    FileConflict {
        file_path: PathBuf,
        conflict_type: FileConflictType,
    },
}

#[derive(Debug)]
pub enum FileConflictType {
    DeletedInOneBranch,
    AddedInBothBranches,
}

pub struct Merger;

impl Merger {
    pub fn merge(
        base_tree: &HashMap<PathBuf, Vec<u8>>,
        branch1_tree: &HashMap<PathBuf, Vec<u8>>,
        branch2_tree: &HashMap<PathBuf, Vec<u8>>,
    ) -> Result<HashMap<PathBuf, Vec<u8>>, Vec<MergeConflict>> {
        let mut merged_tree = base_tree.clone();
        let mut conflicts = Vec::new();

        // Check all files in both branches
        let all_paths: std::collections::HashSet<_> =
            branch1_tree.keys().chain(branch2_tree.keys()).collect();

        for path in all_paths {
            match (
                base_tree.get(path),
                branch1_tree.get(path),
                branch2_tree.get(path),
            ) {
                // File deleted in base, added in one branch
                (None, Some(b1_content), None) => {
                    merged_tree.insert(path.clone(), b1_content.clone());
                }
                (None, None, Some(b2_content)) => {
                    merged_tree.insert(path.clone(), b2_content.clone());
                }

                // File exists in base and both branches
                (Some(base_content), Some(b1_content), Some(b2_content)) => {
                    if b1_content == b2_content {
                        // Same content in both branches
                        merged_tree.insert(path.clone(), b1_content.clone());
                    } else if b1_content == base_content {
                        // No change in first branch, use second branch content
                        merged_tree.insert(path.clone(), b2_content.clone());
                    } else if b2_content == base_content {
                        // No change in second branch, use first branch content
                        merged_tree.insert(path.clone(), b1_content.clone());
                    } else {
                        // Conflicting modifications
                        conflicts.push(MergeConflict::ContentConflict {
                            file_path: path.clone(),
                            base_content: base_content.clone(),
                            branch1_content: b1_content.clone(),
                            branch2_content: b2_content.clone(),
                        });
                    }
                }

                // Conflict scenarios
                (Some(_), Some(_), None) | (Some(_), None, Some(_)) => {
                    conflicts.push(MergeConflict::FileConflict {
                        file_path: path.clone(),
                        conflict_type: FileConflictType::DeletedInOneBranch,
                    });
                }
                (None, Some(_), Some(_)) => {
                    conflicts.push(MergeConflict::FileConflict {
                        file_path: path.clone(),
                        conflict_type: FileConflictType::AddedInBothBranches,
                    });
                }
                (None, None, None) => {
                    println!("Triple None case")
                }
                (Some(_), None, None) => {
                    println!("Impossible case")
                }, // Impossible case
            }
        }

        if conflicts.is_empty() {
            Ok(merged_tree)
        } else {
            Err(conflicts)
        }
    }
}

// impl Merger {
//     pub fn merge(
//         base_tree: &HashMap<PathBuf, Blob>,
//         branch1_tree: &HashMap<PathBuf, Blob>,
//         branch2_tree: &HashMap<PathBuf, Blob>,
//     ) -> Result<HashMap<PathBuf, Blob>, Vec<MergeConflict>> {
//         let mut merged_tree = base_tree.clone();
//         let mut conflicts = Vec::new();

//         // Collect all unique paths from the trees
//         let all_paths: std::collections::HashSet<_> =
//             branch1_tree.keys().chain(branch2_tree.keys()).collect();

//         for path in all_paths {
//             match (
//                 base_tree.get(path),
//                 branch1_tree.get(path),
//                 branch2_tree.get(path),
//             ) {
//                 // File added in one branch
//                 (None, Some(blob1), None) => {
//                     merged_tree.insert(path.clone(), blob1.clone());
//                 }
//                 (None, None, Some(blob2)) => {
//                     merged_tree.insert(path.clone(), blob2.clone());
//                 }

//                 // File exists in base and both branches
//                 (Some(base_blob), Some(blob1), Some(blob2)) => {
//                     if blob1 == blob2 {
//                         // No conflict: same content in both branches
//                         merged_tree.insert(path.clone(), blob1.clone());
//                     } else if blob1 == base_blob {
//                         // No change in branch 1, use branch 2's content
//                         merged_tree.insert(path.clone(), blob2.clone());
//                     } else if blob2 == base_blob {
//                         // No change in branch 2, use branch 1's content
//                         merged_tree.insert(path.clone(), blob1.clone());
//                     } else {
//                         // Conflicting modifications
//                         conflicts.push(MergeConflict::ContentConflict {
//                             file_path: path.clone(),
//                             base_content: base_blob.get_content().to_vec(),
//                             branch1_content: blob1.get_content().to_vec(),
//                             branch2_content: blob2.get_content().to_vec(),
//                         });
//                     }
//                 }

//                 // File deleted in one branch but modified in the other
//                 (Some(base_blob), Some(blob1), None) | (Some(base_blob), None, Some(blob2)) => {
//                     conflicts.push(MergeConflict::FileConflict {
//                         file_path: path.clone(),
//                         conflict_type: FileConflictType::DeletedInOneBranch,
//                     });
//                 }

//                 // File added in both branches with different content
//                 (None, Some(blob1), Some(blob2)) => {
//                     if blob1 != blob2 {
//                         conflicts.push(MergeConflict::FileConflict {
//                             file_path: path.clone(),
//                             conflict_type: FileConflictType::AddedInBothBranches,
//                         });
//                     } else {
//                         merged_tree.insert(path.clone(), blob1.clone());
//                     }
//                 }

//                 // Unreachable cases
//                 (None, None, None) => unreachable!("Unexpected triple None case"),
//                 (Some(_), None, None) => unreachable!("Base tree has a file but branches do not"),
//             }
//         }

//         if conflicts.is_empty() {
//             Ok(merged_tree)
//         } else {
//             Err(conflicts)
//         }
//     }
// }