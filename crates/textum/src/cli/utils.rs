use textum::{Boundary, BoundaryMode, Snippet, Target};

use crate::args::{DeleteArgs, ReplaceArgs};

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
