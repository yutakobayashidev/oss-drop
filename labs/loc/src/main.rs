use git2::{Repository, Error as GitError};
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
enum AnalyzeError {
    Git(GitError),
    Io(std::io::Error),
}

impl From<GitError> for AnalyzeError {
    fn from(err: GitError) -> AnalyzeError {
        AnalyzeError::Git(err)
    }
}

impl From<std::io::Error> for AnalyzeError {
    fn from(err: std::io::Error) -> AnalyzeError {
        AnalyzeError::Io(err)
    }
}

fn analyze_repo(repo_path: &Path) -> Result<HashMap<String, usize>, AnalyzeError> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut user_loc: HashMap<String, usize> = HashMap::new();

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let author_email = commit.author().email().unwrap_or("unknown").to_string();

        let parent_commit = commit.parents().next();
        let tree = commit.tree()?;
        let parent_tree = match parent_commit {
            Some(parent) => Some(parent.tree()?),
            None => None,
        };

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
        let stats = diff.stats()?;

        *user_loc.entry(author_email).or_insert(0) += stats.insertions() + stats.deletions();
    }

    Ok(user_loc)
}

fn main() -> Result<(), AnalyzeError> {
    let current_dir = std::env::current_dir()?;
    match analyze_repo(&current_dir) {
        Ok(user_loc) => {
            for (user, loc) in user_loc {
                println!("{}: {} lines", user, loc);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
    Ok(())
}
