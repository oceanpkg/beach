//! Sandboxing utilities for Ocean.
//!
//! This project is a work in progress. If you want to help make this a reality,
//! contact [Nikolai Vazquez].
//!
//! [Nikolai Vazquez]: https://twitter.com/NikolaiVazquez

#![deny(missing_docs)]

mod chroot;

#[doc(inline)]
pub use self::chroot::Chroot;
