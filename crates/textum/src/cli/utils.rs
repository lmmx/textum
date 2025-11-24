//! Utility functions for CLI argument processing.
//!
//! This module provides helper functions for converting user-friendly CLI arguments
//! into the internal [`Snippet`] representation used by the textum library.

use textum::{Boundary, BoundaryMode, Snippet, Target};

use crate::args::{DeleteArgs, ReplaceArgs};

/// Parse a line range string into start and end indices.
///
/// Expects format "START:END" where both values are unsigned integers.
/// The range is inclusive of START and exclusive of END (half-open interval).
///
/// # Arguments
///
/// * `range` - Line range string (e.g., "5:10")
///
/// # Returns
///
/// Returns `Ok((start, end))` if parsing succeeds.
///
/// # Errors
///
/// Returns an error string if:
/// - The format is invalid (not "START:END")
/// - Either value cannot be parsed as an unsigned integer
///
/// # Examples
///
/// ```
/// # use textum::cli::utils::parse_line_range;
/// let (start, end) = parse_line_range("5:10").unwrap();
/// assert_eq!(start, 5);
/// assert_eq!(end, 10);
/// ```
pub fn parse_line_range(range: &str) -> Result<(usize, usize), String> {
    let parts: Vec<&str> = range.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid line range: {range}. Expected format: START:END"
        ));
    }
    let start = parts[0]
        .parse()
        .map_err(|_| format!("Invalid start line: {}", parts[0]))?;
    let end = parts[1]
        .parse()
        .map_err(|_| format!("Invalid end line: {}", parts[1]))?;
    Ok((start, end))
}

/// Convert [`ReplaceArgs`] into a [`Snippet`] for patch construction.
///
/// This function handles the various targeting modes supported by the replace command:
/// - Line ranges via `--lines`
/// - Between markers via `--until`
/// - Simple literal or pattern matching
///
/// # Arguments
///
/// * `args` - Replace command arguments
///
/// # Returns
///
/// Returns a [`Snippet`] that can be used to construct a [`Patch`](textum::Patch).
///
/// # Errors
///
/// Returns an error string if:
/// - The line range format is invalid
/// - A regex pattern is invalid (when `--pattern` is used)
///
/// # Examples
///
/// ```
/// # use textum::cli::utils::create_snippet_from_replace_args;
/// # use textum::cli::args::ReplaceArgs;
/// // This would typically be parsed from CLI args
/// # let args = ReplaceArgs {
/// #     target: "old".to_string(),
/// #     replacement: "new".to_string(),
/// #     files: vec!["file.txt".to_string()],
/// #     lines: None,
/// #     until: None,
/// #     include_markers: false,
/// #     dry_run: false,
/// #     diff: false,
/// #     verbose: false,
/// # };
/// let snippet = create_snippet_from_replace_args(&args).unwrap();
/// ```
pub fn create_snippet_from_replace_args(args: &ReplaceArgs) -> Result<Snippet, String> {
    if let Some(range) = &args.lines {
        let (start, end) = parse_line_range(range)?;
        let start_boundary = Boundary::new(Target::Line(start), BoundaryMode::Include);
        let end_boundary = Boundary::new(Target::Line(end), BoundaryMode::Exclude);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    if let Some(end_marker) = &args.until {
        let mode = if args.include_markers {
            BoundaryMode::Include
        } else {
            BoundaryMode::Exclude
        };

        let start_boundary = Boundary::new(Target::Literal(args.target.clone()), mode.clone());
        let end_boundary = Boundary::new(Target::Literal(end_marker.clone()), mode);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    // Simple literal or pattern replacement
    #[cfg(feature = "regex")]
    let target = if args.pattern {
        Target::pattern(&args.target).map_err(|e| format!("Invalid regex pattern: {e}"))?
    } else {
        Target::Literal(args.target.clone())
    };

    #[cfg(not(feature = "regex"))]
    let target = Target::Literal(args.target.clone());

    Ok(Snippet::At(Boundary::new(target, BoundaryMode::Include)))
}

/// Convert [`DeleteArgs`] into a [`Snippet`] for patch construction.
///
/// This function handles the various targeting modes supported by the delete command.
/// The logic is identical to [`create_snippet_from_replace_args`] since both commands
/// support the same targeting modes.
///
/// # Arguments
///
/// * `args` - Delete command arguments
///
/// # Returns
///
/// Returns a [`Snippet`] that can be used to construct a [`Patch`](textum::Patch).
///
/// # Errors
///
/// Returns an error string if:
/// - The line range format is invalid
/// - A regex pattern is invalid (when `--pattern` is used)
///
/// # Examples
///
/// ```
/// # use textum::cli::utils::create_snippet_from_delete_args;
/// # use textum::cli::args::DeleteArgs;
/// // This would typically be parsed from CLI args
/// # let args = DeleteArgs {
/// #     target: "unwanted".to_string(),
/// #     files: vec!["file.txt".to_string()],
/// #     lines: None,
/// #     until: None,
/// #     include_markers: false,
/// #     dry_run: false,
/// #     diff: false,
/// #     verbose: false,
/// # };
/// let snippet = create_snippet_from_delete_args(&args).unwrap();
/// ```
pub fn create_snippet_from_delete_args(args: &DeleteArgs) -> Result<Snippet, String> {
    if let Some(range) = &args.lines {
        let (start, end) = parse_line_range(range)?;
        let start_boundary = Boundary::new(Target::Line(start), BoundaryMode::Include);
        let end_boundary = Boundary::new(Target::Line(end), BoundaryMode::Exclude);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    if let Some(end_marker) = &args.until {
        let mode = if args.include_markers {
            BoundaryMode::Include
        } else {
            BoundaryMode::Exclude
        };

        let start_boundary = Boundary::new(Target::Literal(args.target.clone()), mode.clone());
        let end_boundary = Boundary::new(Target::Literal(end_marker.clone()), mode);
        return Ok(Snippet::Between {
            start: start_boundary,
            end: end_boundary,
        });
    }

    // Simple literal or pattern deletion
    #[cfg(feature = "regex")]
    let target = if args.pattern {
        Target::pattern(&args.target).map_err(|e| format!("Invalid regex pattern: {e}"))?
    } else {
        Target::Literal(args.target.clone())
    };

    #[cfg(not(feature = "regex"))]
    let target = Target::Literal(args.target.clone());

    Ok(Snippet::At(Boundary::new(target, BoundaryMode::Include)))
}
