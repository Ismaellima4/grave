//! # Grave
//!
//! A lightweight web framework built on top of [Axum](https://docs.rs/axum)
//! that provides a declarative `app!` macro for defining routes and configuration.

// Macro definitions live in src/macros/
#[macro_use]
mod macros;

// Re-export axum and tokio so macro-generated code can reference them via `$crate::`.
pub use axum;
pub use tokio;

// Re-export commonly used extractors and response types.
pub use axum::Json;
pub use axum::Router;
pub use axum::extract::Path;
pub use axum::extract::State;

/// Re-export the full extract module for convenience.
pub mod extract {
    pub use axum::extract::*;
}
