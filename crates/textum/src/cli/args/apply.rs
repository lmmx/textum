use facet::Facet;

#[derive(Facet)]
pub struct ApplyArgs {
    /// Path to JSON file (reads from stdin if not provided)
    #[facet(positional, default)]
    pub patch_file: Option<String>,

    /// Preview changes
    #[facet(named, short = 'n')]
    pub dry_run: bool,

    /// Show diff
    #[facet(named, short = 'd')]
    pub diff: bool,

    /// Verbose output
    #[facet(named, short = 'v')]
    pub verbose: bool,
}
