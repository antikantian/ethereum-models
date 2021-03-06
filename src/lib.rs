extern crate bigdecimal;
extern crate ethereum_types;
extern crate fixed_hash;
extern crate num;
extern crate rustc_serialize;
extern crate twox_hash;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod contracts;
pub mod error;
pub mod objects;
pub mod types;

pub use error::{Error, ErrorKind};

use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;

fn opt_u64_from_str<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    u64::from_str_radix(fixed_hash::clean_0x(&s), 16).map(Option::Some).map_err(de::Error::custom)
}

fn u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    u64::from_str_radix(fixed_hash::clean_0x(&s), 16).map_err(de::Error::custom)
}