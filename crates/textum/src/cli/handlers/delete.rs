use std::fs;
use std::io;
use textum::{Patch, PatchSet};

use crate::args::DeleteArgs;
use crate::diff::print_diff;
use crate::utils::create_snippet_from_delete_args;

pub fn handle_delete(args: DeleteArgs) -> io::Result<()> {
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
