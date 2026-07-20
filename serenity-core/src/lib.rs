#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::unwrap_used))]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serenity_utils;

#[macro_use]
mod internal;

#[cfg(feature = "builder")]
pub mod builder;
#[cfg(feature = "cache")]
pub mod cache;
pub mod error;
pub mod model;
#[cfg(feature = "utils")]
pub mod utils;

#[cfg(feature = "http")]
pub use serenity_http as http;
