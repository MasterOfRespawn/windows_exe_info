#![allow(dead_code)]

//! A [`Cargo` build script](http://doc.crates.io/build-script.html) library to
//! handle inclusion of Windows icons, version information and application
//! manifests without the use of external `.rc` files.
//!
//! Inspired by and using [embed_resource](https://docs.rs/embed-resource).
//! Use embed_resource when the `windres` command is not on PATH.
//!
//! Generic image format conversion requires
//! [imagemagick](https://imagemagick.org). Imagemagick needs to be in PATH for
//! the conversion functions to work. If imagemagick is not found, the build
//! script will fail.
//!
//! The only icon format available without imagemagick is `.ico`.
//!
//! This crate only works on windows as resource scripts are a windows thing.
//! Linking for web will fail (I have not yet checked linux or macos).
//! Use [build_cfg](https://docs.rs/build_cfg) for checking the target being windows.
//! Build_cfg is required for cross architecture compilation.
//!
//! ## Features
//! - build_cfg: use [build_cfg](https://docs.rs/build_cfg) to use the correct target architecture
//! - embed_resource: use [embed_resource](https://docs.rs/embed-resource) crate for selecting `.rc` compiler
//! - icon_ico: basic icon linking
//! - icon_png: png format support using imagemagick
//! - icon_xxx: generic format support using imagemagick
//! - icon_placeholder: add a placeholder todo icon
//! - icon_autodetect: autodetect icon format and use specific conversion function (currently only redirecting to the generic function)
//! - manifest: allow embedding application manifest files in the executable
//! - versioninfo: allow adding windows version information to the executable

pub mod icon;
mod link;
#[cfg(feature = "manifest")]
mod manifest;
pub use manifest::manifest;
#[cfg(feature = "versioninfo")]
pub mod versioninfo;

#[cfg(test)]
mod test;
