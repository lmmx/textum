//! Argument definitions for the apply command.

use facet::Facet;

/// Arguments for the `apply` command.
///
/// The apply command reads patches from a JSON file (or stdin) and applies
/// them to their target files. This is the programmatic interface for textum,
/// suitable for complex patch operations and tool integration.
///
/// # JSON Format
///
/// The JSON should be an array of patch objects:
/// ```json
/// [
///   {
///     "file": "src/main.rs",
///     "snippet": {
///       "At": {
///         "target": {"Literal": "old"},
///         "mode": "Include"
///       }
///     },
///     "replacement": "new"
///   }
/// ]
/// ```
///
/// # Examples
///
/// Apply from file:
/// ```bash
/// textum apply patches.json
/// ```
///
/// Apply from stdin:
/// ```bash
/// echo '[...]' | textum apply
/// ```
///
/// Preview changes:
/// ```bash
/// textum apply patches.json --dry-run --diff
/// ```
#[derive(Facet)]
pub struct ApplyArgs {
    /// Path to JSON file (reads from stdin if not provided)
    #[facet(positional, default)]
    pub patch_file: Option<String>,

    /// Preview changes
    #[facet(named, short = 'n')]
    pub dry_run: bool,

    /// Show diff
    ///
    /// Implies `--dry-run`. Displays a unified diff showing what would change.
    #[facet(named, short = 'd')]
    pub diff: bool,

    /// Verbose output
    #[facet(named, short = 'v')]
    pub verbose: bool,
}
