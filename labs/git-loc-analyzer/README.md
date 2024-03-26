# Git LOC Analyzer

Git LOC Analyzer is a Rust-based CLI tool that analyzes the lines added and deleted in each commit of a Git repository and reports the total number of changed lines for each user.

The purpose of this project is to provide insights into the contributions of individual developers within a Git repository. By analyzing the lines of code (LOC) added and deleted, the tool aims to identify incentives such as recognizing the hiring of good engineers through the reduction of LOC contributed by a company's founders, and to allocate appropriate rewards for open source contributions.

## Features

- Analyze the lines added and deleted in a Git repository
- Aggregate the number of changed lines per user for each commit

## Installation

```bash
cargo install git-loc-analyzer
```

## Usage

After installing the tool, you can run it from the command line. Navigate to the root directory of your Git repository and run:

```bash
git-loc-analyzer --start-date 2023-01-01 --end-date 2023-12-31
```

This will analyze the commits in the repository for the specified date range and output the number of lines added and deleted by each user.

## License

This project is licensed under the MIT License.


