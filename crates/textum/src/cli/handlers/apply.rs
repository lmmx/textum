use std::fs;
use std::io::{self, Read};
use textum::{Patch, PatchSet};

use crate::args::ApplyArgs;
use crate::diff::print_diff;

pub fn handle_apply(args: ApplyArgs) -> io::Result<()> {
    let input = if let Some(path) = args.patch_file {
        fs::read_to_string(&path)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

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
