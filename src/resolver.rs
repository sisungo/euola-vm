use crate::vmem::Var;
use anyhow::anyhow;
use parking_lot::Mutex;
use rayon::prelude::*;
use smallvec::SmallVec;
use std::{collections::HashMap, fs::read_to_string};

#[doc(hidden)]
trait ResolverOptionExt<T> {
    fn r(self) -> Result<T, anyhow::Error>;
}
impl<T> ResolverOptionExt<T> for Option<T> {
    #[inline]
    fn r(self) -> Result<T, anyhow::Error> {
        self.ok_or_else(|| anyhow!("syntax error: missing arguments or argument is invalid!"))
    }
}

///
/// Load a file.
///
/// NOTE: This shouldn't be `?`d from a VMInterface function, because the error may be unable
/// to downcast to a `&str`. This may panic on interrupt handling.
///
pub fn resolve(path: &str) -> Result<(), anyhow::Error> {
    let file_content = read_to_string(path)?;
    resolve_parsed(file_content.lines())?;
    Ok(())
}

/// Load parsed lines.
pub fn resolve_parsed<'a>(c: impl Iterator<Item = &'a str>) -> Result<(), anyhow::Error> {
    let mut functions = HashMap::with_hasher(ahash::RandomState::default());
    let mut cfname = None;
    for i in c {
        if let Some(si) = i.strip_prefix("|>") {
            if cfname.is_some() {
                return Err(anyhow!("syntax error: function name is not specified"));
            }
            cfname = Some(si);
            functions.insert(si.to_owned(), Vec::new());
            continue;
        } else if i == "<|" {
            cfname = None;
        } else if i.is_empty() || i.starts_with(';') {
            continue;
        } else {
            functions
                .get_mut(cfname.ok_or_else(|| anyhow!("syntax error: function name is not specified"))?)
                .ok_or_else(|| anyhow!("[BUG]FunctionMap didn't contain required element that should always exist. Please report this bug to euolaVM."))?
                .push(tokens(i));
        }
    }
    let errlog = Mutex::new(Vec::new());
    functions.par_iter().for_each(|(k, v)| {
        if let Err(x) = resolve_fn(k, &v[..]) {
            errlog.lock().push(x);
        }
    });
    if errlog.lock().is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "error resolving splited functions: {:?}",
            *errlog.lock()
        ))
    }
}

/// Load a function.
pub fn resolve_fn(n: &str, c: &[SmallVec<[Box<str>; 4]>]) -> Result<(), anyhow::Error> {
    use crate::{context::putvfp, isa::Instruction};

    let mut result = Vec::with_capacity(c.len());
    for i in c.iter() {
        match i.get(0) {
            Some(x) => match &x[..] {
                "N" => {
                    result.push(Instruction::SetConstant(
                        i.get(1).r()?.parse()?,
                        ins(i.get(2).r()?, "N")?,
                    ));
                }
                "v" => {
                    result.push(Instruction::SetConstant(
                        i.get(1).r()?.parse()?,
                        ins(i.get(2).r()?, i.get(3).r()?)?,
                    ));
                }
                "d" => {
                    result.push(Instruction::DynSetConstant(
                        i.get(1).r()?.parse()?,
                        Box::from(&i.get(2).r()?[..]),
                        Box::from(&i.get(3).r()?[..]),
                    ));
                }
                "?" => {
                    result.push(Instruction::IsNull(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                    ));
                }
                "g" => {
                    result.push(Instruction::GetStatic(
                        Box::from(&i.get(1).r()?[..]),
                        i.get(2).r()?.parse()?,
                    ));
                }
                "s" => {
                    result.push(Instruction::SetStatic(
                        Box::from(&i.get(1).r()?[..]),
                        i.get(2).r()?.parse()?,
                    ));
                }
                "G" => {
                    result.push(Instruction::GetField(
                        i.get(1).r()?.parse()?,
                        Box::from(&i.get(2).r()?[..]),
                        i.get(3).r()?.parse()?,
                    ));
                }
                "S" => {
                    result.push(Instruction::SetField(
                        i.get(1).r()?.parse()?,
                        Box::from(&i.get(2).r()?[..]),
                        i.get(3).r()?.parse()?,
                    ));
                }
                "[" => {
                    result.push(Instruction::OffsetGet(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "]" => {
                    result.push(Instruction::OffsetSet(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "T" => {
                    result.push(Instruction::GetTypeId(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                    ));
                }
                "L" => {
                    result.push(Instruction::GetLength(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                    ));
                }
                "D" => {
                    result.push(Instruction::Duplicate(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                    ));
                }
                "t" => {
                    result.push(Instruction::Transmute(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "+" => {
                    result.push(Instruction::Add(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "-" => {
                    result.push(Instruction::Sub(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "*" => {
                    result.push(Instruction::Mul(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "/" => {
                    result.push(Instruction::Div(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "%" => {
                    result.push(Instruction::Rem(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "&" => {
                    result.push(Instruction::And(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "|" => {
                    result.push(Instruction::Or(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "!" => {
                    result.push(Instruction::Not(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                    ));
                }
                "^" => {
                    result.push(Instruction::Xor(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "l" => {
                    result.push(Instruction::Shl(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "R" => {
                    result.push(Instruction::Shr(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "=" => {
                    result.push(Instruction::Equal(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                ">" => {
                    result.push(Instruction::Mt(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "<" => {
                    result.push(Instruction::Lt(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                        i.get(3).r()?.parse()?,
                    ));
                }
                "J" => {
                    result.push(Instruction::Jmp(i.get(1).r()?.parse()?));
                }
                "j" => {
                    result.push(Instruction::Jnz(
                        i.get(1).r()?.parse()?,
                        i.get(2).r()?.parse()?,
                    ));
                }
                "C" => {
                    result.push(Instruction::Call(Box::from(&i.get(1).r()?[..])));
                }
                "c" => {
                    result.push(Instruction::CallPtr(i.get(1).r()?.parse()?));
                }
                "~" => {
                    result.push(Instruction::Int(Box::from(&i.get(1).r()?[..])));
                }
                "r" => {
                    result.push(Instruction::Ret);
                }
                "n" => {
                    result.push(Instruction::Noop);
                }
                _ => return Err(anyhow!("unexpected keyword `{}`", x)),
            },
            None => continue,
        }
    }
    putvfp(n, result.leak());
    Ok(())
}

/// Cut a string into tokens.
fn tokens(s: &str) -> SmallVec<[Box<str>; 4]> {
    let s = s.trim_start();
    let mut buf = SmallVec::new();
    let mut sbuf = String::new();
    let len = s.par_chars().count();
    let mut abing = false;
    let mut inst = false;
    for (c, i) in s.chars().enumerate() {
        if c == len - 1 || (i == ' ' && !abing) {
            if i != '"' {
                sbuf.push(i);
                buf.push(Box::from(sbuf.trim()));
            } else {
                buf.push(Box::from(&sbuf[..]));
            }
            sbuf.clear();
        } else if (i == '"' || i == '\'') && !inst {
            abing = !abing;
        } else if i == '\\' && !inst {
            inst = true;
            continue;
        } else if inst {
            inst = false;
            sbuf.push(match i {
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                'r' => '\r',
                'n' => '\n',
                't' => '\t',
                '0' => '\0',
                'b' => '\x08',
                'e' => '\x1b',
                'v' => '\x0b',
                'f' => '\x0c',
                'a' => '\x07',
                _ => '\u{fffd}',
            });
        } else {
            sbuf.push(i);
        }
    }
    buf
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
