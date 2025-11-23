//! Command handlers for CLI operations.
//!
//! This module contains the implementation of each textum command.
//! Each handler is responsible for:
//! - Converting CLI arguments to patches
//! - Executing the patch operation
//! - Handling dry-run and diff modes
//! - Reporting results to the user

mod apply;
mod delete;
mod replace;

pub use apply::handle_apply;
pub use delete::handle_delete;
pub use replace::handle_replace;
