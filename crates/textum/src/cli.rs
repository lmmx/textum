//! textum: A syntactic patching library with char-level granularity.
//!
//! Command-line interface for applying patches from JSON.
//!
//! `textum` provides a robust way to apply patches to source files using rope data structures
//! for efficient editing and tree-sitter for syntactic awareness. Unlike traditional line-based
//! patch formats, textum operates at character granularity and can compose multiple patches
//! with automatic offset tracking.
//!
//! Reads a JSON array of patches from a file or stdin and applies them to their target files.
//! Modified files are written back to disk unless `--dry-run` is specified.
#![allow(clippy::multiple_crate_versions)]

/// Command-line interface for applying patches from JSON.
#[cfg(feature = "cli")]
pub mod inner {
    use facet::Facet;
    use std::fs;
    use std::io::{self, Read};
    use textum::{Patch, PatchSet};

    mod diff {
        use std::io::{self, Write};

        /// Print a diff-style view of changes for a single file.
        ///
        /// Uses ANSI color codes to highlight additions (green) and deletions (red).
        /// Shows ctx lines around each change for readability.
        pub fn print_diff(file: &str, original: &str, modified: &str) {
            const CONTEXT_LINES: usize = 3;
            const COLOR_RED: &str = "\x1b[31m";
            const COLOR_GREEN: &str = "\x1b[32m";
            const COLOR_CYAN: &str = "\x1b[36m";
            const COLOR_RESET: &str = "\x1b[0m";

            let stdout = io::stdout();
            let mut handle = stdout.lock();

            // Print file header
            writeln!(handle, "{COLOR_CYAN}--- {file}{COLOR_RESET}").ok();
            writeln!(handle, "{COLOR_CYAN}+++ {file}{COLOR_RESET}").ok();

            let orig_lines: Vec<&str> = original.lines().collect();
            let mod_lines: Vec<&str> = modified.lines().collect();

            // Find changed lines
            let mut changes = Vec::new();
            for (i, (orig, modi)) in orig_lines.iter().zip(mod_lines.iter()).enumerate() {
                if orig != modi {
                    changes.push(i);
                }
            }

            // Handle length differences
            if orig_lines.len() != mod_lines.len() {
                for i in
                    orig_lines.len().min(mod_lines.len())..orig_lines.len().max(mod_lines.len())
                {
                    changes.push(i);
                }
            }

            if changes.is_empty() {
                return;
            }

            // Group consecutive changes into hunks
            let mut hunks = Vec::new();
            let mut current_hunk_start = changes[0];
            let mut current_hunk_end = changes[0];

            for &line_idx in &changes[1..] {
                if line_idx <= current_hunk_end + CONTEXT_LINES * 2 {
                    current_hunk_end = line_idx;
                } else {
                    hunks.push((current_hunk_start, current_hunk_end));
                    current_hunk_start = line_idx;
                    current_hunk_end = line_idx;
                }
            }
            hunks.push((current_hunk_start, current_hunk_end));

            // Print each hunk
            for (hunk_start, hunk_end) in hunks {
                let ctx_start = hunk_start.saturating_sub(CONTEXT_LINES);
                let ctx_end =
                    (hunk_end + CONTEXT_LINES + 1).min(orig_lines.len().max(mod_lines.len()));

                let orig_count = ctx_end
                    .saturating_sub(ctx_start)
                    .min(orig_lines.len().saturating_sub(ctx_start));
                let mod_count = ctx_end
                    .saturating_sub(ctx_start)
                    .min(mod_lines.len().saturating_sub(ctx_start));

                writeln!(
                    handle,
                    "{COLOR_CYAN}@@ -{},{} +{},{} @@{COLOR_RESET}",
                    ctx_start + 1,
                    orig_count,
                    ctx_start + 1,
                    mod_count
                )
                .ok();

                // Print ctx and changes
                for i in ctx_start..ctx_end {
                    if i < hunk_start || i > hunk_end {
                        // Context line
                        if i < orig_lines.len() {
                            writeln!(handle, " {}", orig_lines[i]).ok();
                        }
                    } else {
                        // Changed line
                        if i < orig_lines.len() {
                            writeln!(handle, "{COLOR_RED}-{}{COLOR_RESET}", orig_lines[i]).ok();
                        }
                        if i < mod_lines.len() {
                            writeln!(handle, "{COLOR_GREEN}+{}{COLOR_RESET}", mod_lines[i]).ok();
                        }
                    }
                }

                writeln!(handle).ok();
            }
        }
    }

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
        println!("  [PATCH_FILE]  Path to JSON file containing patches (reads from stdin if not provided)");
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
}

/// Hint replacement CLI for when the cli module is used without building the cli feature.
#[cfg(not(feature = "cli"))]
pub mod inner {
    /// Provide a hint to the user that they did not build this crate with the cli feature.
    #[cfg(not(feature = "cli"))]
    pub fn main() {
        eprintln!("Please build with the cli feature to run the CLI");
        std::process::exit(1);
    }
}

pub use inner::main;
