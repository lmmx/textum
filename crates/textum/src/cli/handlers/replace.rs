//! Handler for the replace command.

use std::fs;
use std::io;
use textum::{Patch, PatchSet};

use crate::args::ReplaceArgs;
use crate::diff::print_diff;
use crate::utils::create_snippet_from_replace_args;

/// Execute the replace command with the given arguments.
///
/// This handler:
/// 1. Converts CLI arguments to a snippet
/// 2. Creates patches for each target file
/// 3. Applies patches (or shows preview if dry-run)
/// 4. Writes results to disk (unless dry-run)
///
/// # Arguments
///
/// * `args` - Replace command arguments parsed from CLI
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns an [`io::Error`] if:
/// - Snippet creation fails (invalid arguments)
/// - File I/O fails
/// - Patch application fails
///
/// The process will exit with status 1 if patch application fails.
///
/// # Examples
///
/// This function is called by the CLI router and not typically invoked directly.
pub fn handle_replace(args: &ReplaceArgs) -> io::Result<()> {
    let snippet = create_snippet_from_replace_args(args)
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
