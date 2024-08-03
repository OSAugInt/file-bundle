use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use clap::Parser;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use serde::Deserialize;
use rayon::prelude::*;

#[derive(Debug, Deserialize, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "
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
    -v, --verbose               Enable verbose output

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
    efficient file traversal and the 'glob' crate for pattern matching."
)]
struct FileBundle {
    #[arg(short = 'n', long, default_value = "file_bundle")]
    bundle_name: String,

    #[arg(short = 's', long, default_value = ".")]
    src_dir: PathBuf,

    #[arg(short = 'o', long, default_value = ".")]
    out_dir: PathBuf,

    #[arg(short = 'e', long, default_value = ".txt")]
    dst_ext: String,

    #[arg(short = 'f', long)]
    file_sep: String,

    #[arg(short = 'g', long)]
    src_globs: Vec<String>,

    #[arg(short = 'v', long, default_value_t = false)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let mut args = FileBundle::parse();
    
    args.file_sep = args.file_sep.replace("\\n", "\n");

    let out_path = args.out_dir.join(format!("{}{}", args.bundle_name, args.dst_ext));
    let writer = Mutex::new(BufWriter::new(File::create(&out_path)?));
    
    if args.verbose {
        println!("Glob patterns: {:?}", args.src_globs);
    }

    let mut override_builder = OverrideBuilder::new(&args.src_dir);
    for pattern in &args.src_globs {
        override_builder.add(pattern).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    }
    let overrides = override_builder.build().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let walker = WalkBuilder::new(&args.src_dir)
        .hidden(false)
        .ignore(false)
        .git_ignore(false)
        .overrides(overrides)
        .build();
    
    let files: Vec<_> = walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .collect();

    if args.verbose {
        println!("Total files to process: {}", files.len());
    }

    files.par_iter().try_for_each(|entry| -> io::Result<()> {
        let path = entry.path();
        
        if args.verbose {
            println!("Processing file: {}", path.display());
        }

        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let file_content = match String::from_utf8(contents.clone()) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Warning: File {} is not valid UTF-8. Skipping content.", path.display());
                String::new()
            }
        };

        let output = format!("{} {}\n{}\n", args.file_sep, path.display(), file_content);
        
        let mut writer = writer.lock().unwrap();
        writer.write_all(output.as_bytes())?;
        Ok(())
    })?;
    
    writer.lock().unwrap().flush()?;
    
    println!("Bundle created at: {}", out_path.display());
    Ok(())
}