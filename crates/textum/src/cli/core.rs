use facet::Facet;
use std::fs;
use std::io::{self, Read};
use textum::{Boundary, BoundaryMode, Patch, PatchSet, Snippet, Target};

mod diff;

use diff::print_diff;

#[derive(Facet)]
#[repr(u8)]
enum Command {
    /// Replace text in files
    Replace(ReplaceArgs),
    /// Delete text from files
    Delete(DeleteArgs),
    /// Apply patches from JSON
    Apply(ApplyArgs),
}

#[derive(Facet)]
struct ReplaceArgs {
    /// Text or pattern to find
    #[facet(positional)]
    target: String,

    /// Replacement text
    #[facet(positional)]
    replacement: String,

    /// Files to modify
    #[facet(positional)]
    files: Vec<String>,

    /// Use regex pattern matching
    #[facet(named)]
    #[cfg(feature = "regex")]
    pattern: bool,

    /// Line range (e.g., "5:10" for lines 5-10)
    #[facet(named)]
    lines: Option<String>,

    /// Replace until another marker
    #[facet(named)]
    until: Option<String>,

    /// Exclude boundaries when using --until (default: exclude)
    #[facet(named)]
    include_markers: bool,

    /// Preview changes without writing
    #[facet(named, short = 'n')]
    dry_run: bool,

    /// Show diff of changes
    #[facet(named, short = 'd')]
    diff: bool,

    /// Verbose output
    #[facet(named, short = 'v')]
    verbose: bool,
}

#[derive(Facet)]
struct DeleteArgs {
    /// Text or pattern to delete
    #[facet(positional)]
    target: String,

    /// Files to modify
    #[facet(positional)]
    files: Vec<String>,

    /// Use regex pattern matching
    #[facet(named)]
    #[cfg(feature = "regex")]
    pattern: bool,

    /// Line range (e.g., "5:10")
    #[facet(named)]
    lines: Option<String>,

    /// Delete until another markers
    #[facet(named)]
    until: Option<String>,

    /// Include boundaries when using --until
    #[facet(named)]
    include_markers: bool,

    /// Preview changes
    #[facet(named, short = 'n')]
    dry_run: bool,

    /// Show diff
    #[facet(named, short = 'd')]
    diff: bool,

    /// Verbose output
    #[facet(named, short = 'v')]
    verbose: bool,
}

#[derive(Facet)]
struct ApplyArgs {
    /// Path to JSON file (reads from stdin if not provided)
    #[facet(positional, default)]
    patch_file: Option<String>,

    /// Preview changes
    #[facet(named, short = 'n')]
    dry_run: bool,

    /// Show diff
    #[facet(named, short = 'd')]
    diff: bool,

    /// Verbose output
    #[facet(named, short = 'v')]
    verbose: bool,
}

#[derive(Facet)]
struct Args {
    #[facet(positional)]
    command: Command,

    /// Show help
    #[facet(named, short = 'h')]
    help: bool,
}

fn parse_line_range(range: &str) -> Result<(usize, usize), String> {
    let parts: Vec<&str> = range.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid line range: {range}. Expected format: START:END"
        ));
    }
    let start = parts[0]
        .parse()
        .map_err(|_| format!("Invalid start line: {}", parts[0]))?;
    let end = parts[1]
        .parse()
        .map_err(|_| format!("Invalid end line: {}", parts[1]))?;
    Ok((start, end))
}

fn create_snippet_from_replace_args(args: &ReplaceArgs) -> Result<Snippet, String> {
    if let Some(range) = &args.lines {
        let (start, end) = parse_line_range(range)?;
        let start_boundary = Boundary::new(Target::Line(start), BoundaryMode::Include);
        let end_boundary = Boundary::new(Target::Line(end), BoundaryMode::Exclude);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    if let Some(end_marker) = &args.until {
        let mode = if args.include_markers {
            BoundaryMode::Include
        } else {
            BoundaryMode::Exclude
        };

        let start_boundary = Boundary::new(Target::Literal(args.target.clone()), mode.clone());
        let end_boundary = Boundary::new(Target::Literal(end_marker.clone()), mode);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    // Simple literal or pattern replacement
    #[cfg(feature = "regex")]
    let target = if args.pattern {
        Target::pattern(&args.target).map_err(|e| format!("Invalid regex pattern: {e}"))?
    } else {
        Target::Literal(args.target.clone())
    };

    #[cfg(not(feature = "regex"))]
    let target = Target::Literal(args.target.clone());

    Ok(Snippet::At(Boundary::new(target, BoundaryMode::Include)))
}

fn create_snippet_from_delete_args(args: &DeleteArgs) -> Result<Snippet, String> {
    if let Some(range) = &args.lines {
        let (start, end) = parse_line_range(range)?;
        let start_boundary = Boundary::new(Target::Line(start), BoundaryMode::Include);
        let end_boundary = Boundary::new(Target::Line(end), BoundaryMode::Exclude);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    if let Some(end_marker) = &args.until {
        let mode = if args.include_markers {
            BoundaryMode::Include
        } else {
            BoundaryMode::Exclude
        };

        let start_boundary = Boundary::new(Target::Literal(args.target.clone()), mode.clone());
        let end_boundary = Boundary::new(Target::Literal(end_marker.clone()), mode);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    // Simple literal or pattern deletion
    #[cfg(feature = "regex")]
    let target = if args.pattern {
        Target::pattern(&args.target).map_err(|e| format!("Invalid regex pattern: {e}"))?
    } else {
        Target::Literal(args.target.clone())
    };

    #[cfg(not(feature = "regex"))]
    let target = Target::Literal(args.target.clone());

    Ok(Snippet::At(Boundary::new(target, BoundaryMode::Include)))
}

fn handle_replace(args: ReplaceArgs) -> io::Result<()> {
    let snippet = create_snippet_from_replace_args(&args)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let mut set = PatchSet::new();
    for file in &args.files {
        set.add(Patch {
            file: Some(file.clone()),
            snippet: snippet.clone(),
            replacement: args.replacement.clone(),
            #[cfg(feature = "symbol_path")]
            symbol_path: None,
        });
    }

    let is_dry_run = args.dry_run || args.diff;

    if is_dry_run {
        match set.apply_to_files() {
            Ok(results) => {
                for (file, content) in &results {
                    eprintln!("Would patch: {file}");
                    if args.diff {
                        let original = fs::read_to_string(file)?;
                        print_diff(file, &original, content);
                    } else if args.verbose {
                        println!("=== {file} ===\n{content}");
                    }
                }
                if !args.verbose {
                    eprintln!("Dry run complete ({} file(s))", results.len());
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    } else {
        match set.write_to_files() {
            Ok(()) => {
                if args.verbose {
                    eprintln!("Successfully patched {} file(s)", args.files.len());
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn handle_delete(args: DeleteArgs) -> io::Result<()> {
    let snippet = create_snippet_from_delete_args(&args)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let mut set = PatchSet::new();
    for file in &args.files {
        set.add(Patch {
            file: Some(file.clone()),
            snippet: snippet.clone(),
            replacement: String::new(), // Empty replacement = deletion
            #[cfg(feature = "symbol_path")]
            symbol_path: None,
        });
    }

    let is_dry_run = args.dry_run || args.diff;

    if is_dry_run {
        match set.apply_to_files() {
            Ok(results) => {
                for (file, content) in &results {
                    eprintln!("Would patch: {file}");
                    if args.diff {
                        let original = fs::read_to_string(file)?;
                        print_diff(file, &original, content);
                    } else if args.verbose {
                        println!("=== {file} ===\n{content}");
                    }
                }
                if !args.verbose {
                    eprintln!("Dry run complete ({} file(s))", results.len());
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    } else {
        match set.write_to_files() {
            Ok(()) => {
                if args.verbose {
                    eprintln!("Successfully patched {} file(s)", args.files.len());
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn handle_apply(args: ApplyArgs) -> io::Result<()> {
    // Your existing JSON parsing logic
    let input = if let Some(path) = args.patch_file {
        fs::read_to_string(&path)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

    let patches: Vec<Patch> = match facet_json::from_str(&input) {
        Ok(patches) => patches,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    if args.verbose {
        eprintln!("Loaded {} patch(es)", patches.len());
    }

    let mut set = PatchSet::new();
    for patch in patches {
        set.add(patch);
    }

    let is_dry_run = args.dry_run || args.diff;

    if is_dry_run {
        match set.apply_to_files() {
            Ok(results) => {
                for (file, content) in &results {
                    eprintln!("Would patch: {file}");
                    if args.diff {
                        let original = fs::read_to_string(file)?;
                        print_diff(file, &original, content);
                    } else if args.verbose {
                        println!("=== {file} ===\n{content}");
                    }
                }
                if !args.verbose {
                    eprintln!("Dry run complete ({} file(s))", results.len());
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    } else {
        match set.write_to_files() {
            Ok(()) => {
                if args.verbose {
                    eprintln!("Successfully patched {} file(s)", set.len());
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: textum <COMMAND> [OPTIONS]");
    println!();
    println!("A syntactic patching tool with char-level granularity.");
    println!();
    println!("Commands:");
    println!("  replace <TARGET> <REPLACEMENT> <FILES>...  Replace text in files");
    println!("  delete <TARGET> <FILES>...                 Delete text from files");
    println!("  apply [PATCH_FILE]                         Apply JSON patches");
    println!();
    println!("Replace/Delete Options:");
    println!("  --pattern              Use regex pattern matching");
    println!("  --lines START:END      Operate on line range");
    println!("  --until END_MARKER     Operate between TARGET and END_MARKER");
    println!("  --include-markers      Include boundary markers (default: exclude)");
    println!("  -n, --dry-run          Preview changes");
    println!("  -d, --diff             Show diff");
    println!("  -v, --verbose          Verbose output");
    println!();
    println!("Examples:");
    println!("  # Simple replacement");
    println!("  textum replace 'old' 'new' file.txt");
    println!();
    println!("  # Between markers (like your README case)");
    println!("  textum replace '<!-- start -->' 'content' --until '<!-- end -->' README.md");
    println!();
    println!("  # Delete lines");
    println!("  textum delete --lines 5:10 file.txt");
    println!();
    println!("  # JSON mode");
    println!("  textum apply patches.json");
}

#[cfg(feature = "cli")]
/// Entry point for the `textum` command-line interface.
///
/// Reads JSON patches from a file or stdin and applies them to their target files.
///
/// # Errors
///
/// Returns an [`io::Error`] if:
/// - command-line argument parsing fails,
/// - the input file cannot be read,
/// - patch JSON is malformed,
/// - or writing the modified files fails.
///
/// The process will also exit with a non-zero status if patch application fails.
pub fn main() -> io::Result<()> {
    let args: Args = match facet_args::from_std_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {e}");
            print_usage();
            std::process::exit(1);
        }
    };

    if args.help {
        print_usage();
        return Ok(());
    }

    match args.command {
        Command::Replace(replace_args) => handle_replace(replace_args),
        Command::Delete(delete_args) => handle_delete(delete_args),
        Command::Apply(apply_args) => handle_apply(apply_args),
    }
}
