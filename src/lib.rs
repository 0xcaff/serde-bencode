#[macro_use]
extern crate serde;
extern crate serde_bytes;

pub mod de;
pub mod error;
pub mod ser;
pub mod value;

pub use de::{from_bytes, from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, to_string, Serializer};
