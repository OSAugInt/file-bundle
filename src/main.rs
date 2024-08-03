use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use glob::{glob_with, MatchOptions};
use ignore::WalkBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, Parser)]
#[command(author, version, about, long_about = "
FileBundle - A utility for bundling multiple files into a single output file.

USAGE:
    fbundle [OPTIONS] -f <FILE_SEPERATOR> -g <GLOB_PATTERN>...

OPTIONS:
    -n, --bundle-name <NAME>    Set the name of the output bundle file (default: 'file_bundle')
    -s, --src-dir <DIR>         Specify the source directory to search for files (default: current directory)
    -o, --out-dir <DIR>         Set the output directory for the bundle file (default: current directory)
    -e, --dst-ext <EXT>         Set the file extension for the output bundle file (default: '.txt')
    -f, --file-sep <SEP>        Specify a custom separator string to use between files in the bundle
    -g, --src-globs <PATTERNS>  Provide one or more glob patterns to match source files
                                Use '!' prefix for exclusion patterns
                                Can be specified multiple times for multiple patterns

DESCRIPTION:
    This tool bundles multiple files into a single output file. It recursively searches
    the specified source directory for files matching the given glob patterns, concatenates
    their contents, and writes them to the output file.

    Files are separated in the output by the specified separator string, followed by the
    file's path relative to the source directory.

EXAMPLES:
    1. Bundle all .txt files in the current directory:
       fbundle -g '*.txt'

    2. Bundle .rs files, excluding test files, from a specific directory:
       fbundle -s ./src -g '**/*.rs' -g '!**/*_test.rs'

    3. Create a bundle with a custom name and separator:
       fbundle -n 'my_bundle' -f '---FILE---' -g '**/*.md'

    4. Bundle files with multiple include and exclude patterns:
       fbundle -g '**/*.{js,ts}' -g '!**/node_modules/**' -g '!**/dist/**'

NOTE:
    Glob patterns are case-insensitive by default. The tool uses the 'ignore' crate for
    efficient file traversal and the 'glob' crate for pattern matching.")]
struct FileBundle {
    #[arg(short = 'n', long, default_value = "file_bundle")]
    bundle_name: String,
    
    #[arg(short = 's', long, default_value = ".")]
    src_dir: String,
    
    #[arg(short = 'o', long, default_value = ".")]
    out_dir: String,
    
    #[arg(short = 'e', long, default_value = ".txt")]
    dst_ext: String,
    
    #[arg(short = 'f', long)]
    file_sep: String,
    
    #[arg(short = 'g', long)]
    src_globs: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = FileBundle::parse();
    
    let out_path = Path::new(&args.out_dir).join(format!("{}{}", args.bundle_name, args.dst_ext));
    let mut out_file = File::create(&out_path)?;
    
    let src_dir = Path::new(&args.src_dir);
    let mut walker = WalkBuilder::new(src_dir);
    
    for glob_pattern in &args.src_globs {
        if glob_pattern.starts_with('!') {
            walker.add_ignore(glob_pattern.trim_start_matches('!'));
        } else {
            walker.add_custom_ignore_filename(&glob_pattern);
        }
    }

    for result in walker.build() {
        match result {
            Ok(entry) => {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let path = entry.path();
                    if should_include_file(path, &args.src_globs, src_dir) {
                        write!(out_file, "{} {}\n", args.file_sep, path.display())?;
                        let contents = fs::read_to_string(path)?;
                        write!(out_file, "{}\n", contents)?;
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    
    println!("Bundle created at: {}", out_path.display());
    Ok(())
}

fn should_include_file(file_path: &Path, patterns: &[String], base_dir: &Path) -> bool {
    let relative_path = file_path.strip_prefix(base_dir).unwrap_or(file_path);
    for pattern in patterns {
        let is_exclude = pattern.starts_with('!');
        let pattern = pattern.trim_start_matches('!');
        let full_pattern = base_dir.join(pattern).to_string_lossy().into_owned();
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        match glob_with(&full_pattern, options) {
            Ok(mut paths) => {
                let matched = paths.any(|p| p.as_ref().map_or(false, |p| p == file_path));
                if is_exclude && matched {
                    return false;
                } else if !is_exclude && matched {
                    return true;
                }
            }
            Err(e) => {
                eprintln!("Error in glob pattern '{}': {}", pattern, e);
                continue;
            }
        }
    }
    false
}