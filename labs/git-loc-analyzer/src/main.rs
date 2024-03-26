use chrono::{DateTime, NaiveDate, Utc, TimeZone};
use git2::{Commit, Error as GitError, Repository};
use std::collections::HashMap;
use std::{env, path::Path};

#[derive(Debug)]
enum AnalyzeError {
    Git(GitError),
    Io(std::io::Error),
    ParseDate(chrono::ParseError),
    NotInRepo,
}

impl From<GitError> for AnalyzeError {
    fn from(err: GitError) -> Self {
        AnalyzeError::Git(err)
    }
}

impl From<std::io::Error> for AnalyzeError {
    fn from(err: std::io::Error) -> Self {
        AnalyzeError::Io(err)
    }
}

impl From<chrono::ParseError> for AnalyzeError {
    fn from(err: chrono::ParseError) -> Self {
        AnalyzeError::ParseDate(err)
    }
}

fn is_commit_in_range(
    commit: &Commit,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> bool {
    if let Some(start_date) = start_date {
        if commit.time().seconds() < start_date.timestamp() {
            return false;
        }
    }

    if let Some(end_date) = end_date {
        if commit.time().seconds() > end_date.timestamp() {
            return false;
        }
    }

    true
}

fn analyze_repo(
    repo_path: &Path,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<HashMap<String, usize>, AnalyzeError> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut user_loc: HashMap<String, usize> = HashMap::new();

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;

        if !is_commit_in_range(&commit, start_date, end_date) {
            continue;
        }

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
    let args: Vec<String> = env::args().collect();
    let mut start_date: Option<DateTime<Utc>> = None;
    let mut end_date: Option<DateTime<Utc>> = None;

    for i in 0..args.len() {
        match args[i].as_str() {
            "--start-date" => {
                let date_str = &args[i + 1];
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?.and_hms_opt(0, 0, 0).unwrap();
                start_date = Some(Utc.from_utc_datetime(&date));
            }
            "--end-date" => {
                let date_str = &args[i + 1];
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?.and_hms_opt(23, 59, 59).unwrap();
                end_date = Some(Utc.from_utc_datetime(&date));
            }
            _ => {}
        }
    }

    let current_dir = std::env::current_dir()?;
    let repo = match Repository::discover(&current_dir) {
        Ok(repo) => repo,
        Err(_) => return Err(AnalyzeError::NotInRepo),
    };

    match analyze_repo(repo.path(), start_date, end_date) {
        Ok(user_loc) => {
            for (user, loc) in user_loc {
                println!("{}: {} lines", user, loc);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
    Ok(())
}
