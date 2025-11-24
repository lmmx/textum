//! Stub implementation for when CLI feature is disabled.

/// Provide a hint to the user that they did not build this crate with the cli feature.
///
/// This function is compiled when the `cli` feature is not enabled. It prints
/// an error message and exits with status 1 to inform the user they need to
/// rebuild with the CLI feature.
///
/// # Examples
///
/// If you attempt to run the binary without the `cli` feature:
/// ```text
/// $ cargo build
/// $ ./target/debug/textum
/// Please build with the cli feature to run the CLI
/// ```
///
/// To fix, rebuild with the feature:
/// ```text
/// $ cargo build --features cli
/// ```
#[cfg(not(feature = "cli"))]
pub fn main() {
    eprintln!("Please build with the cli feature to run the CLI");
    std::process::exit(1);
}
