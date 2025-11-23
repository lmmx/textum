//! Command-line argument definitions and help text.
//!
//! This module defines the argument structure for all textum commands,
//! including replace, delete, and apply operations. Each command has its
//! own argument type with appropriate flags and options.

use facet::Facet;

pub mod apply;
pub mod delete;
pub mod replace;

pub use apply::ApplyArgs;
pub use delete::DeleteArgs;
pub use replace::ReplaceArgs;

/// Top-level command enumeration for the textum CLI.
///
/// Each variant corresponds to a major operation mode:
/// - [`Replace`](Command::Replace): Find and replace text
/// - [`Delete`](Command::Delete): Remove text
/// - [`Apply`](Command::Apply): Apply patches from JSON
#[derive(Facet)]
#[repr(u8)]
#[allow(dead_code)] // Variants constructed by Facet macro
pub enum Command {
    /// Replace text in files
    Replace(ReplaceArgs),
    /// Delete text from files
    Delete(DeleteArgs),
    /// Apply patches from JSON
    Apply(ApplyArgs),
}

/// Top-level CLI arguments structure.
///
/// Captures the command to execute and any global flags like `--help`.
#[derive(Facet)]
pub struct Args {
    /// The command to execute (replace, delete, or apply)
    #[facet(positional)]
    pub command: Command,

    /// Show help message
    #[facet(named, short = 'h')]
    pub help: bool,
}

/// Print comprehensive usage information to stdout.
///
/// Displays the command structure, available options, and usage examples
/// for all textum commands. Called when the user provides `--help` or
/// when argument parsing fails.
///
/// # Examples
///
/// ```no_run
/// textum::cli::args::print_usage();
/// ```
pub fn print_usage() {
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
    println!("  # Between markers");
    println!("  textum replace '<!-- start -->' 'content' --until '<!-- end -->' README.md");
    println!();
    println!("  # Delete lines");
    println!("  textum delete --lines 5:10 file.txt");
    println!();
    println!("  # JSON mode");
    println!("  textum apply patches.json");
}
