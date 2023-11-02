#![allow(dead_code)]
#![doc = include_str!("../readme.md")]

#[cfg(feature = "icon_ico")]
pub mod icon;
mod link;
#[cfg(feature = "manifest")]
mod manifest;
#[cfg(feature = "manifest")]
pub use manifest::manifest;
#[cfg(feature = "versioninfo")]
pub mod versioninfo;

#[cfg(test)]
mod test;
