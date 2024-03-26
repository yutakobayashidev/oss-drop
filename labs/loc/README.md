# Git LOC Analyzer

Git LOC Analyzer is a Rust-based CLI tool that analyzes the lines added and deleted in each commit of a Git repository and reports the total number of changed lines for each user.

The purpose of this project is to provide insights into the contributions of individual developers within a Git repository. By analyzing the lines of code (LOC) added and deleted, the tool aims to identify incentives such as recognizing the hiring of good engineers through the reduction of LOC contributed by a company's founders, and to allocate appropriate rewards for open source contributions.

## Features

- Analyze the lines added and deleted in a Git repository
- Aggregate the number of changed lines per user for each commit

## Usage

1. Clone or download this repository.
2. Ensure that the Rust environment is set up.
3. Build the program by running the following command in the command line:

```bash
cargo run --release -- --start-date 2023-01-01 --end-date 2023-12-31
```

## License

This project is licensed under the MIT License.


