//! Unified diff output formatting.
//!
//! This module provides functionality for displaying changes in a human-readable
//! unified diff format with ANSI color highlighting. The output is similar to
//! `git diff` and uses context lines to show changes in their surrounding code.

use std::io::{self, Write};

/// Print a diff-style view of changes for a single file.
///
/// Uses ANSI color codes to highlight additions (green) and deletions (red).
/// Shows ctx lines around each change for readability.
pub fn print_diff(file: &str, original: &str, modified: &str) {
    const CONTEXT_LINES: usize = 3;
    const COLOR_RED: &str = "\x1b[31m";
    const COLOR_GREEN: &str = "\x1b[32m";
    const COLOR_CYAN: &str = "\x1b[36m";
    const COLOR_RESET: &str = "\x1b[0m";

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Print file header
    writeln!(handle, "{COLOR_CYAN}--- {file}{COLOR_RESET}").ok();
    writeln!(handle, "{COLOR_CYAN}+++ {file}{COLOR_RESET}").ok();

    let orig_lines: Vec<&str> = original.lines().collect();
    let mod_lines: Vec<&str> = modified.lines().collect();

    // Find changed lines
    let mut changes = Vec::new();
    for (i, (orig, modi)) in orig_lines.iter().zip(mod_lines.iter()).enumerate() {
        if orig != modi {
            changes.push(i);
        }
    }

    // Handle length differences
    if orig_lines.len() != mod_lines.len() {
        for i in orig_lines.len().min(mod_lines.len())..orig_lines.len().max(mod_lines.len()) {
            changes.push(i);
        }
    }

    if changes.is_empty() {
        return;
    }

    // Group consecutive changes into hunks
    let mut hunks = Vec::new();
    let mut current_hunk_start = changes[0];
    let mut current_hunk_end = changes[0];

    for &line_idx in &changes[1..] {
        if line_idx <= current_hunk_end + CONTEXT_LINES * 2 {
            current_hunk_end = line_idx;
        } else {
            hunks.push((current_hunk_start, current_hunk_end));
            current_hunk_start = line_idx;
            current_hunk_end = line_idx;
        }
    }
    hunks.push((current_hunk_start, current_hunk_end));

    // Print each hunk
    for (hunk_start, hunk_end) in hunks {
        let ctx_start = hunk_start.saturating_sub(CONTEXT_LINES);
        let ctx_end = (hunk_end + CONTEXT_LINES + 1).min(orig_lines.len().max(mod_lines.len()));

        let orig_count = ctx_end
            .saturating_sub(ctx_start)
            .min(orig_lines.len().saturating_sub(ctx_start));
        let mod_count = ctx_end
            .saturating_sub(ctx_start)
            .min(mod_lines.len().saturating_sub(ctx_start));

        writeln!(
            handle,
            "{COLOR_CYAN}@@ -{},{} +{},{} @@{COLOR_RESET}",
            ctx_start + 1,
            orig_count,
            ctx_start + 1,
            mod_count
        )
        .ok();

        // Print ctx and changes
        for i in ctx_start..ctx_end {
            if i < hunk_start || i > hunk_end {
                // Context line
                if i < orig_lines.len() {
                    writeln!(handle, " {}", orig_lines[i]).ok();
                }
            } else {
                // Changed line
                if i < orig_lines.len() {
                    writeln!(handle, "{COLOR_RED}-{}{COLOR_RESET}", orig_lines[i]).ok();
                }
                if i < mod_lines.len() {
                    writeln!(handle, "{COLOR_GREEN}+{}{COLOR_RESET}", mod_lines[i]).ok();
                }
            }
        }

        writeln!(handle).ok();
    }
}
