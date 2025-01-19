# Todo CLI

A command-line interface application to manage your daily tasks efficiently. This application is built using Rust and is designed to help you stay organized and productive. It provides a simple and intuitive interface for adding, listing, marking, and deleting tasks. It's using [supabase](https://supabase.com/) as a database.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- Add new tasks
- List all tasks
- Mark tasks as complete
- Delete tasks
- Simple and intuitive interface

## Installation

```bash
git clone https://github.com/Rasoul678/todo_cli.git
```

```bash
cd todo_cli
```

## Usage

```bash
cargo run -- add "task title" "task description"
```

```bash
cargo run -- list
```

```bash
cargo run -- complete 1
```

```bash
cargo run -- delete 1
```

```bash
cargo run -- clear
```

```bash
cargo run -- help
```

```bash
cargo run -- version
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
