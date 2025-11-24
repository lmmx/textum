//! Core CLI entry point and command routing.
//!
//! This module orchestrates the command-line interface by parsing arguments,
//! displaying help when requested, and routing commands to their respective handlers.

pub mod args;
pub mod diff;
pub mod handlers;
pub mod report;
pub mod utils;

use args::Command;
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
    // Install miette handler for pretty diagnostics
    report::install_handler().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to install error handler: {e}"),
        )
    })?;

    // Get raw args
    let all_args: Vec<String> = std::env::args().collect();

    // Check for help flag
    if all_args.iter().any(|arg| arg == "-h" || arg == "--help") {
        args::print_usage();
        return Ok(());
    }

    // Need at least program name + command
    if all_args.len() < 2 {
        eprintln!("Error: No command specified");
        args::print_usage();
        std::process::exit(1);
    }

    let command_name = &all_args[1];

    // Prepare args for facet parsing: JUST the subcommand args, NO program name
    let subcommand_args: Vec<&str> = all_args[2..].iter().map(|s| s.as_str()).collect();

    let command = match command_name.as_str() {
        "replace" => {
            let replace_args: args::ReplaceArgs = match facet_args::from_slice(&subcommand_args) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("{}", report::DiagnosticDisplay(&e));
                    std::process::exit(1);
                }
            };
            Command::Replace(replace_args)
        }
        "delete" => {
            let delete_args: args::DeleteArgs = match facet_args::from_slice(&subcommand_args) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("{}", report::DiagnosticDisplay(&e));
                    std::process::exit(1);
                }
            };
            Command::Delete(delete_args)
        }
        "apply" => {
            let apply_args: args::ApplyArgs = match facet_args::from_slice(&subcommand_args) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("{}", report::DiagnosticDisplay(&e));
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
