//! Sandboxing utilities for [Ocean].
//!
//! [Ocean]: https://github.com/oceanpkg/ocean

#![deny(missing_docs)]

mod chroot;

#[doc(inline)]
pub use self::chroot::Chroot;
