use facet::Facet;
use std::fs;
use std::io::{self, Read};
use textum::{Patch, PatchSet};

mod diff;

use diff::print_diff;

#[derive(Facet)]
#[allow(clippy::struct_excessive_bools)]
struct Args {
    /// Path to JSON file containing patches (reads from stdin if not provided)
    #[facet(positional, default)]
    patch_file: Option<String>,

    /// Preview changes without writing to disk
    #[facet(named, short = 'n')]
    dry_run: bool,

    /// Show diff of changes (implies --dry-run)
    #[facet(named, short = 'd')]
    diff: bool,

    /// Show verbose output
    #[facet(named, short = 'v')]
    verbose: bool,

    /// Show this help message
    #[facet(named, short = 'h')]
    help: bool,
}

fn print_usage() {
    println!("Usage: textum [OPTIONS] [PATCH_FILE]");
    println!();
    println!("Apply syntactic patches to source files with char-level granularity.");
    println!();
    println!("Arguments:");
    println!(
        "  [PATCH_FILE]  Path to JSON file containing patches (reads from stdin if not provided)"
    );
    println!();
    println!("Options:");
    println!("  -n, --dry-run  Preview changes without writing to disk");
    println!("  -d, --diff     Show diff of changes (implies --dry-run)");
    println!("  -v, --verbose  Show verbose output");
    println!("  -h, --help     Show this help message");
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
    let args: Args = facet_args::from_std_args()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("{e}")))?;

    if args.help {
        print_usage();
        std::process::exit(0);
    }

    // Read input from file or stdin
    let input = if let Some(path) = args.patch_file {
        fs::read_to_string(&path)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

    // Parse patches from JSON using facet
    // Parse patches from JSON using facet
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

    // Apply patches
    let is_dry_run = args.dry_run || args.diff;

    if is_dry_run {
        // Use apply_to_files to inspect without writing
        match set.apply_to_files() {
            Ok(results) => {
                for (file, content) in &results {
                    eprintln!("Would patch: {file}");
                    if args.diff {
                        // Read original content
                        let original = fs::read_to_string(file)?;
                        print_diff(file, &original, content);
                    } else if args.verbose {
                        println!("=== {file} ===\n{content}");
                    }
                }

                if !args.verbose {
                    eprintln!(
                        "Dry run complete ({} file(s)). Use -v to see changes.",
                        results.len()
                    );
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    } else {
        // Use write_to_files for direct persistence
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
