# FileBundle

![Rust CI](https://github.com/OSAugInt/file-bundle/workflows/Rust%20CI/badge.svg)


FileBundle is a command-line utility written in Rust that allows you to bundle multiple files into a single output file. It provides flexible options for file selection, output customization, and supports complex glob patterns for inclusion and exclusion of files.

## Features

- Recursively search directories for files
- Use glob patterns to include or exclude files
- Customize output file name and extension
- Specify custom separators between files in the bundle
- Efficient file traversal using the `ignore` crate
- Flexible pattern matching with the `glob` crate

## Installation

To install FileBundle, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo, you can install FileBundle by cloning this repository and building it:

```bash
git clone https://github.com/OSAugInt/file-bundle.git
cd file-bundle
cargo build --release
```

The compiled binary will be available in `target/release/fbundle`.

## Usage

```
fbundle [OPTIONS] -f <FILE_SEPERATOR> -g <GLOB_PATTERN>...
```

### Options

- `-n, --bundle-name <NAME>`: Set the name of the output bundle file (default: 'file_bundle')
- `-s, --src-dir <DIR>`: Specify the source directory to search for files (default: current directory)
- `-o, --out-dir <DIR>`: Set the output directory for the bundle file (default: current directory)
- `-e, --dst-ext <EXT>`: Set the file extension for the output bundle file (default: '.txt')
- `-f, --file-sep <SEP>`: Specify a custom separator string to use between files in the bundle
- `-g, --src-globs <PATTERNS>`: Provide one or more glob patterns to match source files. Use '!' prefix for exclusion patterns. Can be specified multiple times for multiple patterns.

## Examples

1. Bundle all .txt files in the current directory:
   ```
   fbundle -g '*.txt'
   ```

2. Bundle .rs files, excluding test files, from a specific directory:
   ```
   fbundle -s ./src -g '**/*.rs' -g '!**/*_test.rs'
   ```

3. Create a bundle with a custom name and separator:
   ```
   fbundle -n 'my_bundle' -f '---FILE---' -g '**/*.md'
   ```

4. Bundle files with multiple include and exclude patterns:
   ```
   fbundle -g '**/*.{js,ts}' -g '!**/node_modules/**' -g '!**/dist/**'
   ```

## How It Works

FileBundle recursively searches the specified source directory for files matching the given glob patterns. It then concatenates the contents of these files into a single output file. Each file in the bundle is preceded by the specified separator string and the file's path relative to the source directory.

The tool uses the `ignore` crate for efficient file traversal and the `glob` crate for pattern matching. Glob patterns are case-insensitive by default.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.