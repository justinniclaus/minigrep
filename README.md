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
![image](https://github.com/user-attachments/assets/7a0e9176-0013-4f00-aa16-fef22a827a95)

---

![image](https://github.com/user-attachments/assets/fd8b3c38-1b61-42bd-9a41-341c79815b28)

**To perform a case-insensitive search, set IGNORE_CASE:**
![image](https://github.com/user-attachments/assets/3b3584f9-cac7-48dc-94c7-09aba842e81c)

---

![image](https://github.com/user-attachments/assets/cd6c33ec-55ee-4284-92f7-01e93d1fb6f5)

