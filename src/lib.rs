//! The official Rust implementation of the SLOP language.
//!
//! Sans' Lovely Properties (SLOP) is a data storage language designed to be
//! tiny, (both in complexity and amount of characters) while still being
//! human-readable.
//!
//! ## Examples
//!
//! ```
//! use slop_rs::Slop;
//!
//! let slop_str = "\
//!     some-string-kv=some value
//!     some-list-kv{
//!         item 1
//!         item 2
//!         item 3
//!     }";
//!
//! println!("{slop_str}");
//! let slop = slop_str.parse::<Slop>().unwrap();
//!
//! assert_eq!(slop.get("some-string-kv"), Some(&"some value".into()));
//! ```

pub mod error;
pub mod slop;
pub mod value;

#[cfg(test)]
mod tests;

pub use slop::*;
pub use value::*;
