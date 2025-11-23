//! Core CLI entry point and command routing.
//!
//! This module orchestrates the command-line interface by parsing arguments,
//! displaying help when requested, and routing commands to their respective handlers.

pub mod args;
pub mod diff;
pub mod handlers;
pub mod utils;

use args::Args;
use std::io;

/// Entry point for the textum command-line interface.
///
/// Parses command-line arguments using the facet framework and routes
/// to the appropriate handler based on the command type (replace, delete, or apply).
///
/// # Errors
///
/// Returns an [`io::Error`] if:
/// - Command-line argument parsing fails
/// - The selected handler encounters an I/O error
///
/// # Examples
///
/// This function is typically called from the binary entry point:
///
/// ```no_run
/// # fn main() -> std::io::Result<()> {
/// textum::cli::main()
/// # }
/// ```
pub fn main() -> io::Result<()> {
    let args: Args = match facet_args::from_std_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {e}");
            args::print_usage();
            std::process::exit(1);
        }
    };

    if args.help {
        args::print_usage();
        return Ok(());
    }

    match args.command {
        args::Command::Replace(ref replace_args) => handlers::handle_replace(replace_args),
        args::Command::Delete(ref delete_args) => handlers::handle_delete(delete_args),
        args::Command::Apply(ref apply_args) => handlers::handle_apply(apply_args),
    }
}
