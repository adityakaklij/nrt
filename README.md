# `nrt` - A Rust-Based Clone of the `cat` Command

`nrt` is a lightweight and efficient command-line utility built in Rust, designed to replicate and extend the functionality of the classic Unix `cat` command.
Whether you need to quickly view file contents, inspect file metadata, or perform basic file operations, `nrt` provides a versatile toolset.

## Features

`nrt` enhances the standard `cat` command with the following capabilities:

- **File Size**: Display the size of the file in bytes.
- **Creation Time**: Show the creation time of the file in UTC.
- **Last Modified Time**: Display the last modified time of the file in UTC.
- **File Permissions**: List the file's mode (permissions) in numeric form.
- **Line Count**: Count the total number of lines in the file.
- **Non-Empty Lines**: Print all non-empty lines, each with its corresponding line number.
- **Rename Files**: Rename the file to a new specified filename.
- **Copy File Content**: Copy the entire content of the file.
- **Basic Concatenation**: View the content of one or multiple files, similar to the standard `cat` command.

## Installation

Get the latest version of nrt:

```sh
# Using curl
curl -L -o nrt https://github.com/adityakaklij/nrt/releases/download/v0.1.0/nrt && chmod +x nrt && sudo mv nrt /usr/local/bin/

# Using apt

# Using brew

```


## Usage

`nrt` is easy to use with a variety of options to suit your needs. 
The general syntax is:

```sh
nrt <filename> [OPTION]
```

### Options

| Option               | Description                                               |
|----------------------|-----------------------------------------------------------|
| `--size`             | Show the size of the file in bytes.                        |
| `--mt`               | Display the last modified time of the file in UTC.         |
| `--ct`               | Display the creation time of the file in UTC.              |
| `--mode`             | Show the file's mode (permissions) in numeric form.        |
| `--lc`               | Count and display the number of lines in the file.         |
| `--n`                | Print all non-empty lines with their line numbers.         |
| `--cp`               | Copy the entire content of the file.                       |
| `--rn <new_filename>`| Rename the file to a new specified filename.               |
| `--v`             | Latest version.                                               |
| `--help`             | Display a help message with usage instructions.            |



<br>
<hr>

## Contributing
Contributions are welcome! If you find a bug, have a feature request, or want to contribute in any way, feel free to open an issue or submit a pull request.

<hr>

## License
nrt is licensed under the GPL-3.0