
# find

## Overview
`find` is a command-line utility for searching files and directories in a specified path.

## Usage
```
find [OPTIONS] <path>
```

- `<path>`: Specifies the search path.
- `-f, --files`: Find only files.
- `-d, --directories`: Find only directories.

If no options are provided, both files and directories will be displayed by default.

## Examples
- To search for both files and directories in the current directory:
```sh
find .
```

- To search for only files in a specific directory:
```sh
find /path/to/directory -f
```
- To search for only directories in a specific directory:
```sh
find /path/to/directory -d
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.