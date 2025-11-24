//! Tests for individual CLI commands (replace, delete, apply).

use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;

mod apply;
mod delete;
mod replace;

#[test]
fn help_flag_displays_usage() {
    cargo_bin_cmd!("textum")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: textum"));
}
