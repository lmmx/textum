use facet::Facet;

pub mod apply;
pub mod delete;
pub mod replace;

pub use apply::ApplyArgs;
pub use delete::DeleteArgs;
pub use replace::ReplaceArgs;

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

#[derive(Facet)]
pub struct Args {
    #[facet(positional)]
    pub command: Command,

    /// Show help
    #[facet(named, short = 'h')]
    pub help: bool,
}

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
