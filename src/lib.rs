//! Complete generated OMML model graph and native XML codecs.
use lutaml_rs::{contracts, xml as xml_rt};

pub mod bindings;

// Generated sources are kept byte-exact with their generators; rustfmt skips them.
#[rustfmt::skip]
#[path = "generated/models.rs"]
pub mod models;

#[rustfmt::skip]
#[path = "generated/xml_codecs.rs"]
pub mod xml_codecs;

#[cfg(feature = "model-serde")]
#[rustfmt::skip]
mod dispatch;

#[cfg(feature = "model-serde")]
mod ffi;
