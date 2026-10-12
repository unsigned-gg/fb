//! fb (Fast Build & Framebuffer Engine)
//!
//! Terminal canvas, pixel and glyph rendering primitives, and fast build graph execution for dagr.
//!
//! # Architecture
//! Anchored in the systems principles of **Carl Lerche & Manish Goregaokar**.


pub use fb_domain::*;

/// Initializes the `fb` systems engine.
pub fn init() -> anyhow::Result<()> {
    Ok(())
}
