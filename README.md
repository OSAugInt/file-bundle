# FileBundle

![Rust CI](https://github.com/OSAugInt/file-bundle/workflows/Rust%20CI/badge.svg)

FileBundle is a command-line utility written in Rust that allows you to bundle multiple files into a single output file. It provides flexible options for file selection, output customization, and supports complex patterns for inclusion and exclusion of files.

## Features

- Recursively search directories for files
- Use patterns to include or exclude files
- Customize output file name and extension
- Specify custom separators between files in the bundle
- Efficient file traversal and pattern matching using the `ignore` crate
- Parallel processing for improved performance on multi-core systems

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
- `-g, --src-globs <PATTERNS>`: Provide one or more patterns to match source files. Use '!' prefix for exclusion patterns. Can be specified multiple times for multiple patterns.
- `-v, --verbose`: Enable verbose output

## Examples

1. Bundle all .txt files in the current directory:
   ```
   fbundle -f '---' -g '*.txt'
   ```

2. Bundle .rs files, excluding test files, from a specific directory:
   ```
   fbundle -s ./src -f '====' -g '**/*.rs' -g '!**/*_test.rs'
   ```

3. Create a bundle with a custom name and separator:
   ```
   fbundle -n 'my_bundle' -f '---FILE---' -g '**/*.md'
   ```

4. Bundle files with multiple include and exclude patterns:
   ```
   fbundle -f '----' -g '**/*.{js,ts}' -g '!**/node_modules/**' -g '!**/dist/**'
   ```

## How It Works

FileBundle recursively searches the specified source directory for files matching the given patterns. It then concatenates the contents of these files into a single output file. Each file in the bundle is preceded by the specified separator string and the file's path relative to the source directory.

The tool uses the `ignore` crate for efficient file traversal and pattern matching. This allows for complex include/exclude rules similar to `.gitignore` files. The `rayon` crate is used for parallel processing, allowing for efficient bundling on multi-core systems.

## Performance Considerations

- FileBundle uses parallel processing for improved performance on multi-core systems.
- Large files are read entirely into memory, which may impact performance for very large files or on systems with limited RAM.
- The program collects all matching files before processing, which could be memory-intensive for directories with a vast number of files.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.