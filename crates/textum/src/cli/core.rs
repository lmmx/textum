pub mod args;
pub mod diff;
pub mod handlers;
pub mod utils;

use args::Args;
use std::io;

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
        args::Command::Replace(replace_args) => handlers::handle_replace(replace_args),
        args::Command::Delete(delete_args) => handlers::handle_delete(delete_args),
        args::Command::Apply(apply_args) => handlers::handle_apply(apply_args),
    }
}
