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
        use super::PatchSet;
        use std::io::{self, Write};

        /// Print a diff-style view of changes for a single file.
        ///
        /// Uses ANSI color codes to highlight additions (green) and deletions (red).
        /// Shows context lines around each change for readability.
        pub fn print_diff(file: &str, original: &str, patch_set: &PatchSet) {
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

            let original_rope = textum::Rope::from_str(original);

            // Collect patches for this file and resolve their ranges
            let mut changes: Vec<(usize, usize, String)> = Vec::new();
            for patch in &patch_set.patches {
                if patch.file.as_deref() == Some(file) {
                    if let Ok(resolution) = patch.snippet.resolve(&original_rope) {
                        changes.push((resolution.start, resolution.end, patch.replacement.clone()));
                    }
                }
            }

            if changes.is_empty() {
                return;
            }

            changes.sort_by_key(|(start, _, _)| *start);

            // For each change, show context around it
            for (start_char, end_char, replacement) in changes {
                // Convert char positions to line numbers for context display
                let start_line = original_rope.char_to_line(start_char);
                let end_line = if end_char > 0 && end_char <= original_rope.len_chars() {
                    original_rope.char_to_line(end_char.saturating_sub(1))
                } else {
                    start_line
                };

                // Calculate context window
                let context_start_line = start_line.saturating_sub(CONTEXT_LINES);
                let context_end_line =
                    (end_line + CONTEXT_LINES + 1).min(original_rope.len_lines());

                // Print hunk header
                let orig_line_count = end_line.saturating_sub(start_line) + 1;
                let replacement_line_count = if replacement.is_empty() {
                    0
                } else {
                    replacement.lines().count().max(1)
                };

                writeln!(
                    handle,
                    "{COLOR_CYAN}@@ -{},{} +{},{} @@{COLOR_RESET}",
                    start_line + 1,
                    orig_line_count,
                    start_line + 1,
                    replacement_line_count
                )
                .ok();

                // Print context before change
                for line_idx in context_start_line..start_line {
                    let line = original_rope.line(line_idx);
                    write!(handle, " {line}").ok();
                    if !line.to_string().ends_with('\n') {
                        writeln!(handle).ok();
                    }
                }

                // Print removed lines (red)
                for line_idx in start_line..=end_line {
                    if line_idx < original_rope.len_lines() {
                        let line = original_rope.line(line_idx);
                        write!(handle, "{COLOR_RED}-{line}{COLOR_RESET}").ok();
                        if !line.to_string().ends_with('\n') {
                            writeln!(handle).ok();
                        }
                    }
                }

                // Print added lines (green)
                if !replacement.is_empty() {
                    for line in replacement.lines() {
                        writeln!(handle, "{COLOR_GREEN}+{line}{COLOR_RESET}").ok();
                    }
                    // Handle trailing content without newline
                    if !replacement.ends_with('\n') && replacement.lines().last().is_none() {
                        writeln!(handle).ok();
                    }
                }

                // Print context after change
                let context_after_start = end_line + 1;
                for line_idx in context_after_start..context_end_line {
                    if line_idx < original_rope.len_lines() {
                        let line = original_rope.line(line_idx);
                        write!(handle, " {line}").ok();
                        if !line.to_string().ends_with('\n') {
                            writeln!(handle).ok();
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
                            print_diff(file, &original, &set);
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
