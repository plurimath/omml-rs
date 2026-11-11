//! Complete generated OMML model graph and native XML codecs.
use lutaml_rs::{contracts, xml as xml_rt};

pub mod bindings;

#[path = "generated/models.rs"]
pub mod models;

#[path = "generated/xml_codecs.rs"]
pub mod xml_codecs;

#[cfg(feature = "model-serde")]
mod dispatch;

#[cfg(feature = "model-serde")]
mod ffi;
