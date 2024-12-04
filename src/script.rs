mod git;

use std::path::Path;
use git::repository::Repository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a new repository
    let mut repo = Repository::init("./test_repo")?;

    // Add some files
    repo.add(Path::new("./test_repo/file1.txt"))?;
    repo.add(Path::new("./test_repo/file2.txt"))?;

    // Commit changes
    let commit_id = repo.commit(
        "initial commit".to_string(), 
        "Doe <doe@gmail.com>".to_string()
    )?;

    println!("Committed changes with ID: {}", commit_id);

    // Show commit log
    repo.log();

    Ok(())
}