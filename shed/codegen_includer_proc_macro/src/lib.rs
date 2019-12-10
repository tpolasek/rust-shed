/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License found in the LICENSE file in the root
 * directory of this source tree.
 */

#![deny(warnings, clippy::all, intra_doc_link_resolution_failure)]
// See https://github.com/rust-lang/rust/pull/60562
// #![deny(missing_docs)]

//! This crate offers a workaround for [issue](https://github.com/rust-lang/rfcs/issues/752).
//! The gist of it is that `include!` proc macro will include the content of
//! lib.rs file stored inside OUT_DIR, presumably generated by cargo's build
//! script.
//!
//! # Example
//!
//! ```
//! ::codegen_includer_proc_macro::include!();
//! fn main() {
//!     helloWorld(); // This was included from $OUT_DIR/lib.rs
//! }
//! ```

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::path::Path;

/// See crate's documentation
#[proc_macro]
pub fn include(_: TokenStream) -> TokenStream {
    let path_to_include = Path::new(&env::var("OUT_DIR").unwrap())
        .join("lib.rs")
        .to_str()
        .unwrap()
        .to_string();

    let result = quote! {
        #[path = #path_to_include]
        #[allow(unused_attributes)]
        mod codegen_included;
        pub use codegen_included::*;
    };
    result.into()
}
