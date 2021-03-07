//!
//! euolaVM executable ball resolving module.
//!
//! This provides two formats:
//!  - Assembly(*.s)
//!  - Binary(*.euo)
//!
//! Binary is faster than Assembly, but not human-friendly. Binaries can be generated
//! from Assembly.
//!

pub mod asm;
pub mod bin;

use crate::vmem::Var;
use anyhow::anyhow;

/// Load from file.
pub fn resolve(path: &str) -> Result<(), anyhow::Error> {
    if path.ends_with(".s") {
        asm::resolve(path)
    } else if path.ends_with(".euo") {
        bin::resolve(path)
    } else {
        Err(anyhow!("unknown file format: only `*.s` and `*.euo` are allowed!"))
    }
}

/// Generate `Var` from two strings.
pub fn ins(t: &str, c: &str) -> Result<Var, anyhow::Error> {
    use crate::vmem::{BytesRef, CreateNull, ObjectRef, StringRef, VectorRef};

    Ok(match t {
        "8" => Var::I8(c.parse()?),
        "9" => Var::U8(c.parse()?),
        "16" => Var::I16(c.parse()?),
        "17" => Var::U16(c.parse()?),
        "32" => Var::I32(c.parse()?),
        "33" => Var::U32(c.parse()?),
        "64" => Var::I64(c.parse()?),
        "65" => Var::U64(c.parse()?),
        "c" => Var::U32(c.parse::<char>()? as u32),
        "U" => {
            if let Some(sd) = c.strip_prefix('f') {
                Var::UString(StringRef::from(sd))
            } else if c == "E" || c == "n" {
                Var::UString(StringRef::empty())
            } else if c == "N" {
                Var::UString(StringRef::null())
            } else {
                return Err(anyhow!("invalid creation of ustring: {}", c));
            }
        }
        "b" => {
            if c == "n" || c == "E" {
                Var::Bytes(BytesRef::empty())
            } else if c == "N" {
                Var::Bytes(BytesRef::null())
            } else {
                return Err(anyhow!("invalid creation of bytes: {}", c));
            }
        }
        "v" => {
            if c == "n" || c == "E" {
                Var::Vector(VectorRef::empty())
            } else if c == "N" {
                Var::Vector(VectorRef::null())
            } else {
                return Err(anyhow!("invalid creation of vector: {}", c));
            }
        }
        _ => {
            if c == "n" {
                Var::Object(ObjectRef::new(t))
            } else if c == "N" {
                Var::Object(ObjectRef::null())
            } else {
                return Err(anyhow!("invalid creation of {}@object: {}", t, c));
            }
        }
    })
}
