//! Argument definitions for the replace command.

use facet::Facet;

/// Arguments for the `replace` command.
///
/// The replace command finds text (literal or pattern) and replaces it with new content.
/// Supports various targeting modes including literal matching, regex patterns, line ranges,
/// and between-marker selection.
///
/// # Examples
///
/// Replace literal text:
/// ```bash
/// textum replace "old" "new" file.txt
/// ```
///
/// Replace using regex:
/// ```bash
/// textum replace --pattern "fn \w+\(" "pub fn main(" src/*.rs
/// ```
///
/// Replace between markers:
/// ```bash
/// textum replace "<!-- start -->" "content" --until "<!-- end -->" README.md
/// ```
#[allow(clippy::struct_excessive_bools)]
#[derive(Facet)]
pub struct ReplaceArgs {
    /// Text or pattern to find
    #[facet(positional)]
    pub target: String,

    /// Replacement text
    #[facet(positional)]
    pub replacement: String,

    /// Files to modify
    #[facet(positional)]
    pub files: Vec<String>,

    /// Use regex pattern matching
    #[facet(named, default)]
    #[cfg(feature = "regex")]
    pub pattern: bool,

    /// Line range (e.g., "5:10" for lines 5-10)
    ///
    /// When specified, operates on the given line range instead of searching for the target.
    /// The range is inclusive of the start line and exclusive of the end line.
    #[facet(named, default)]
    pub lines: Option<String>,

    /// Replace until another marker
    ///
    /// When specified, replaces content between the target and this end marker.
    /// Use with `--include-markers` to control boundary inclusion.
    #[facet(named, default)]
    pub until: Option<String>,

    /// Exclude boundaries when using --until (default: exclude)
    ///
    /// When true, includes the target and end markers in the replacement.
    /// When false (default), only replaces content between markers.
    #[facet(named, default)]
    pub include_markers: bool,

    /// Preview changes without writing
    #[facet(named, short = 'n', default)]
    pub dry_run: bool,

    /// Show diff of changes
    ///
    /// Implies `--dry-run`. Displays a unified diff showing what would change.
    #[facet(named, short = 'd', default)]
    pub diff: bool,

    /// Verbose output
    #[facet(named, short = 'v', default)]
    pub verbose: bool,
}
