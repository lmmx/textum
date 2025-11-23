use facet::Facet;

#[derive(Facet)]
pub struct DeleteArgs {
    /// Text or pattern to delete
    #[facet(positional)]
    pub target: String,

    /// Files to modify
    #[facet(positional)]
    pub files: Vec<String>,

    /// Use regex pattern matching
    #[facet(named)]
    #[cfg(feature = "regex")]
    pub pattern: bool,

    /// Line range (e.g., "5:10")
    #[facet(named)]
    pub lines: Option<String>,

    /// Delete until another marker
    #[facet(named)]
    pub until: Option<String>,

    /// Include boundaries when using --until
    #[facet(named)]
    pub include_markers: bool,

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
