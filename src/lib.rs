#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/radio-rs/refs/heads/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/radio-rs/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod common;

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

pub use common::{Orientation, Size, Type};
