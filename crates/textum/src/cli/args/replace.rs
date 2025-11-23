use facet::Facet;

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
    #[facet(named)]
    #[cfg(feature = "regex")]
    pub pattern: bool,

    /// Line range (e.g., "5:10" for lines 5-10)
    #[facet(named)]
    pub lines: Option<String>,

    /// Replace until another marker
    #[facet(named)]
    pub until: Option<String>,

    /// Exclude boundaries when using --until (default: exclude)
    #[facet(named)]
    pub include_markers: bool,

    /// Preview changes without writing
    #[facet(named, short = 'n')]
    pub dry_run: bool,

    /// Show diff of changes
    #[facet(named, short = 'd')]
    pub diff: bool,

    /// Verbose output
    #[facet(named, short = 'v')]
    pub verbose: bool,
}
