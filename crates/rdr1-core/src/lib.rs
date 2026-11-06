//! Portable Rust components for the RDR1 iOS research project.
//!
//! This crate does not contain the original game's engine or assets.

/// Returns the crate's current version.
pub mod math;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn version_has_three_components() {
        assert_eq!(VERSION.split('.').count(), 3);
    }
}
