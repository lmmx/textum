//! Core CLI entry point and command routing.
//!
//! This module orchestrates the command-line interface by parsing arguments,
//! displaying help when requested, and routing commands to their respective handlers.

pub mod args;
pub mod diff;
pub mod handlers;
pub mod utils;

use args::{Args, Command};
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
    // Check for help flag before facet parsing
    let std_args: Vec<String> = std::env::args().collect();
    if std_args.iter().any(|arg| arg == "-h" || arg == "--help") {
        args::print_usage();
        return Ok(());
    }

    let args: Args = match facet_args::from_std_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {e}");
            args::print_usage();
            std::process::exit(1);
        }
    };

    // Parse command manually
    if args.args.is_empty() {
        eprintln!("Error: No command specified");
        args::print_usage();
        std::process::exit(1);
    }

    let command_name = &args.args[0];

    // Build argument slice for re-parsing WITHOUT the subcommand name
    // Just: ["textum", ...remaining args after subcommand]
    let mut command_args: Vec<String> = vec!["textum".to_string()];
    command_args.extend(args.args[1..].iter().cloned());

    // Convert to Vec<&str> for from_slice
    let command_args_strs: Vec<&str> = command_args.iter().map(|s| s.as_str()).collect();

    let command = match command_name.as_str() {
        "replace" => {
            let replace_args: args::ReplaceArgs = match facet_args::from_slice(&command_args_strs) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            };
            Command::Replace(replace_args)
        }
        "delete" => {
            let delete_args: args::DeleteArgs = match facet_args::from_slice(&command_args_strs) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            };
            Command::Delete(delete_args)
        }
        "apply" => {
            let apply_args: args::ApplyArgs = match facet_args::from_slice(&command_args_strs) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            };
            Command::Apply(apply_args)
        }
        _ => {
            eprintln!("Error: Unknown command '{command_name}'");
            args::print_usage();
            std::process::exit(1);
        }
    };

    match command {
        Command::Replace(ref replace_args) => handlers::handle_replace(replace_args),
        Command::Delete(ref delete_args) => handlers::handle_delete(delete_args),
        Command::Apply(ref apply_args) => handlers::handle_apply(apply_args),
    }
}
