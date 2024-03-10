# Find Tool

A simple command-line tool to find files and directories in a specified path, written in Rust.

## Features

- Find files and directories in a specified path.
- Optionally find only files or directories.
- Colorized output for better readability.

## Installation

Make sure you have Rust and Cargo installed on your system. Then, clone this repository and navigate to the project directory.

```sh
    git clone <repository_url>
    cd find_tool
```

Build the project using Cargo:

```sh
    cargo build --release
```

The compiled binary will be available in the `target/release` directory.
You can add this to yout path manually or if you want to use `cargo install` to make your tool available globally on your system, you can do so by following these steps:

1. Navigate to your project directory in the terminal.

2. Build your project using Cargo:

```sh
    cargo build --release
```

3. Once the build completes, use `cargo install` to install your tool globally:

```sh
    cargo install --path .
```

This command will install your tool globally on your system. Now, users can run your tool from anywhere in the terminal by simply typing its name, followed by any command line arguments.


## Usage

Run the program with the following command:

```sh
    ./target/release/find_tool <search_path> [-f] [-d]
```

Replace `<search_path>` with the directory you want to search in. Optionally, include the flags `-f` to find only files and `-d` to find only directories.

### Examples

- To find both files and directories in a specific path:

```sh
    ./target/release/find_tool /path/to/search
```

- To find only files in a specific path:

```sh
    ./target/release/find_tool /path/to/search -f
```

- To find only directories in a specific path:

```sh
    ./target/release/find_tool /path/to/search -d
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
