/// Provide a hint to the user that they did not build this crate with the cli feature.
#[cfg(not(feature = "cli"))]
pub fn main() {
    eprintln!("Please build with the cli feature to run the CLI");
    std::process::exit(1);
}
