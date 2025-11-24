//! Argument definitions for the delete command.

use facet::Facet;

/// Arguments for the `delete` command.
///
/// The delete command removes text from files. It supports the same targeting
/// modes as replace but removes matched content instead of replacing it.
///
/// # Examples
///
/// Delete literal text:
/// ```bash
/// textum delete "unwanted" file.txt
/// ```
///
/// Delete line range:
/// ```bash
/// textum delete --lines 5:10 file.txt
/// ```
///
/// Delete content between markers:
/// ```bash
/// textum delete "<!-- start -->" --until "<!-- end -->" file.md
/// ```
#[allow(clippy::struct_excessive_bools)]
#[derive(Facet)]
pub struct DeleteArgs {
    /// Text or pattern to delete
    #[facet(positional)]
    pub target: String,

    /// Files to modify
    #[facet(positional)]
    pub files: Vec<String>,

    /// Use regex pattern matching
    #[facet(named, default)]
    #[cfg(feature = "regex")]
    pub pattern: bool,

    /// Line range (e.g., "5:10")
    ///
    /// When specified, deletes the given line range instead of searching for the target.
    /// The range is inclusive of the start line and exclusive of the end line.
    #[facet(named, default)]
    pub lines: Option<String>,

    /// Delete until another marker
    ///
    /// When specified, deletes content between the target and this end marker.
    /// Use with `--include-markers` to control boundary inclusion.
    #[facet(named, default)]
    pub until: Option<String>,

    /// Include boundaries when using --until
    ///
    /// When true, includes the target and end markers in the deletion.
    /// When false (default), only deletes content between markers.
    #[facet(named, default)]
    pub include_markers: bool,

    /// Preview changes
    #[facet(named, short = 'n', default)]
    pub dry_run: bool,

    /// Show diff
    ///
    /// Implies `--dry-run`. Displays a unified diff showing what would be deleted.
    #[facet(named, short = 'd', default)]
    pub diff: bool,

    /// Verbose output
    #[facet(named, short = 'v', default)]
    pub verbose: bool,
}
