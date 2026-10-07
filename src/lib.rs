//! Core types and algorithms shared by Graphite consumers.

#[cfg(test)]
mod tests {
    #[test]
    fn rust_version_is_pinned_to_1_85() {
        assert_eq!(env!("CARGO_PKG_RUST_VERSION"), "1.85");
    }
}
