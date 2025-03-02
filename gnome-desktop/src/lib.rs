// Take a look at the license at the top of the repository in the LICENSE file.

//! # GNOME-DESKTOP bindings
//!
//! This library contains safe Rust bindings for gnome-desktop.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

// no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
mod auto;
pub use auto::functions::*;
pub use auto::traits::*;
pub use auto::*;
