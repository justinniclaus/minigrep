# minigrep

## Overview

**minigrep** is a simple command-line tool written in Rust for searching a user-defined substring in a given file. It’s inspired by the “minigrep” example in *The Rust Programming Language* book. This project demonstrates basic Rust concepts like reading command-line arguments, file I/O, error handling, and unit testing.

## Features

- **Substring Search**: Returns all lines containing the query substring.
- **Case Sensitivity**: Searches are case-sensitive by default, but can be toggled off using an environment variable (`IGNORE_CASE`).
- **Minimal Dependencies**: Built entirely in Rust with no extra dependencies, other than what’s in the standard library.

## Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/<YOUR_USERNAME>/minigrep.git
   cd minigrep
   ```

2. **Build and Run:**

    ```bash
    cargo build
    cargo run <QUERY> <FILE_PATH>
    ```

## Usage

**Basic Usage:**


**To perform a case-insensitive search, set IGNORE_CASE:**

IGNORE_CASE=1 cargo run "nobody" poem.txt
