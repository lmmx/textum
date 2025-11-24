//! Error reporting with miette diagnostics.
//!
//! This module provides utilities for displaying facet-args parsing errors
//! with nice graphical diagnostics using miette's report handler.

use miette::{GraphicalReportHandler, GraphicalTheme, ReportHandler};
use std::fmt;

/// Initializes the global miette report handler for pretty error output.
///
/// Call this once at the start of your CLI to enable graphical diagnostics
/// for all miette errors throughout the application.
///
/// # Errors
///
/// Returns an error if the miette hook cannot be installed (rare).
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// textum::cli::report::install_handler()?;
/// // Now all diagnostics will use pretty formatting
/// # Ok(())
/// # }
/// ```
pub fn install_handler() -> Result<(), Box<dyn std::error::Error>> {
    miette::set_hook(Box::new(|_| {
        Box::new(GraphicalReportHandler::new_themed(
            GraphicalTheme::unicode_nocolor(),
        ))
    }))?;
    Ok(())
}

/// A wrapper that formats a miette diagnostic with the graphical report handler.
///
/// This wrapper provides on-demand graphical formatting of diagnostics without
/// requiring a global handler installation. Useful for one-off error displays
/// or testing.
///
/// # Examples
///
/// ```
/// # use textum::cli::report::DiagnosticDisplay;
/// # fn get_error() -> impl miette::Diagnostic { /* ... */ }
/// let err = get_error();
/// eprintln!("{}", DiagnosticDisplay(&err));
/// ```
pub struct DiagnosticDisplay<'a>(pub &'a dyn miette::Diagnostic);

impl fmt::Display for DiagnosticDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reporter = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
        reporter.debug(self.0, f)?;
        Ok(())
    }
}
