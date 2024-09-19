<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/codicate/gitty">
    <img src="https://book.git-scm.com/images/logos/downloads/Git-Icon-1788C.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Gitty</h3>

  <p align="center">
    Git built in Rust
  </p>
  <hr/>
</div>

### About

This project is a simplified implementation of Git built from scratch in Rust. My primary motivation for building this project was to deepen my understanding of both the Rust programming language and version control systems. Through this project, I explored various Rust features such as file handling, error management, and data structures, while also gaining hands-on experience with fundamental Git operations.

### Experience

At the heart of Git is a simple yet elegant system. Commits are stored as tree objects, which capture the state of the repository at a specific point in time. Each commit, branch, and tag is tracked using hashes, allowing Git to efficiently manage changes and ensure data integrity. Branches and tags are just references (or refs) pointing to specific commits, while file changes are automatically tracked by hashing each file, eliminating the need for complex comparison logic. This made git surprisingly simple to implement in Rust.

### Usage

To get started, clone this repository and build the project using Cargo.

```bash
git clone <this-repo>
cd <this-repo>
cargo build
```

Run the CLI with:

```bash
cargo run -- <command>
```

### Commands

#### Help

```bash
cargo run -- help
```

Display help information for the CLI.

#### Init

```bash
cargo run -- init
```

Initialize a new repository.

#### Commit

```bash
cargo run -- commit -m "message"
```

Create a new commit with a message.

#### Log

```bash
cargo run -- log
```

Show the commit history.

#### Status

```bash
cargo run -- status
```

Display the current state of the working directory.

#### Branch

```bash
cargo run -- branch
```

List current active branches

#### Checkout (Create and Switch to New Branch)

```bash
cargo run -- checkout -b <name>
```

Create a new branch and switch to it, or switch to an existing branch without `-b`.

#### Tag

```bash
cargo run -- tag <commit-hash> <name>
```

Tag a specific commit with a name so that we can use it for `checkout` or `reset`.

#### Reset

```bash
cargo run -- reset <commit-hash>
```

Reset the working directory to a specific commit.

#### Show

```bash
cargo run -- show <commit-hash>
```

Show details of a specific commit, including file diffs.
