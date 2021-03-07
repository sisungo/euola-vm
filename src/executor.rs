use crate::{
    context::{getfp, getstatic, int, putstatic, ExecUnit, Thread},
    isa::{FuncPtr, Instruction},
    libraw::iohmgr::{CeIdGen, FakeHasher},
    resolver::ins,
    vmem::Var,
};
use anyhow::anyhow;
use std::collections::HashMap;

macro_rules! impl_vmb {
    ($a: expr, $b: tt, $c: expr) => {
        match $a {
            Var::I8(x) => match $c {
                Var::I8(y) => Ok(Var::I8(*x $b *y)),
                Var::U8(y) => Ok(Var::I8(*x $b (*y as i8))),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U8(x) => match $c {
				Var::I8(y) => Ok(Var::U8(*x $b (*y as u8))),
                Var::U8(y) => Ok(Var::U8(*x $b *y)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::I16(x) => match $c {
				Var::I16(y) => Ok(Var::I16(*x $b *y)),
                Var::U16(y) => Ok(Var::I16(*x $b (*y as i16))),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U16(x) => match $c {
				Var::I16(y) => Ok(Var::U16(*x $b (*y as u16))),
                Var::U16(y) => Ok(Var::U16(*x $b *y)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::I32(x) => match $c {
				Var::I32(y) => Ok(Var::I32(*x $b *y)),
                Var::U32(y) => Ok(Var::I32(*x $b (*y as i32))),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U32(x) => match $c {
				Var::I32(y) => Ok(Var::U32(*x $b (*y as u32))),
                Var::U32(y) => Ok(Var::U32(*x $b *y)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::I64(x) => match $c {
				Var::I64(y) => Ok(Var::I64(*x $b *y)),
                Var::U64(y) => Ok(Var::I64(*x $b (*y as i64))),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U64(x) => match $c {
				Var::I64(y) => Ok(Var::U64(*x $b (*y as u64))),
                Var::U64(y) => Ok(Var::U64(*x $b *y)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            _ => Err(anyhow!("raw::fatal::not_an_integer")),
        }
    }
}
macro_rules! impl_vbe {
    ($a: expr, $b: tt, $c: expr) => {
        match $a {
            Var::I8(x) => match $c {
                Var::I8(y) => Ok(*x $b *y),
                Var::U8(y) => Ok(*x $b (*y as i8)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U8(x) => match $c {
				Var::I8(y) => Ok(*x $b (*y as u8)),
                Var::U8(y) => Ok(*x $b *y),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::I16(x) => match $c {
				Var::I16(y) => Ok(*x $b *y),
                Var::U16(y) => Ok(*x $b (*y as i16)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U16(x) => match $c {
				Var::I16(y) => Ok(*x $b (*y as u16)),
                Var::U16(y) => Ok(*x $b *y),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::I32(x) => match $c {
				Var::I32(y) => Ok(*x $b *y),
                Var::U32(y) => Ok(*x $b (*y as i32)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U32(x) => match $c {
				Var::I32(y) => Ok(*x $b (*y as u32)),
                Var::U32(y) => Ok(*x $b *y),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::I64(x) => match $c {
				Var::I64(y) => Ok(*x $b *y),
                Var::U64(y) => Ok(*x $b (*y as i64)),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            Var::U64(x) => match $c {
				Var::I64(y) => Ok(*x $b (*y as u64)),
                Var::U64(y) => Ok(*x $b *y),
                _ => Err(anyhow!("raw::fatal::math_type_error")),
            },
            _ => Err(anyhow!("raw::fatal::not_an_integer")),
        }
    }
}

macro_rules! core_inner {
    ($a: expr, $b: expr) => {
        match $a {
            Instruction::SetConstant(a, b) => $b.sset(a, b)?,
            Instruction::DynSetConstant(a, b, c) => $b.sset(
                a,
                match ins(&*b, &*c) {
                    Ok(x) => x,
                    Err(_) => return Err(anyhow!("raw::fatal::dynset_error")),
                },
            )?,
            Instruction::IsNull(a, b) => $b.sset(b, Var::U8($b.sget(a)?.is_null()? as u8))?,
            Instruction::GetStatic(a, b) => $b.sset(
                b,
                getstatic(&*a).ok_or_else(|| anyhow!("raw::fatal::static_not_found"))?,
            )?,
            Instruction::SetStatic(a, b) => putstatic(&*a, $b.sget(b)?.to_owned()),
            Instruction::GetField(a, b, c) => $b.sset(
                c,
                $b.sget(a)?
                    .as_objref()
                    .ok_or_else(|| anyhow!("raw::fatal::not_an_object"))?
                    .get(&*b)?
                    .to_owned(),
            )?,
            Instruction::SetField(a, b, c) => $b
                .sget(a)?
                .as_objref()
                .ok_or_else(|| anyhow!("raw::fatal::not_an_object"))?
                .set(&*b, $b.sget(c)?.to_owned())?,
            Instruction::OffsetGet(a, b, c) => $b.sset(
                c,
                $b.sget(a)?.offset_get(
                    $b.sget(b)?
                        .as_usize()
                        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?,
                )?,
            )?,
            Instruction::OffsetSet(a, b, c) => $b.sget(a)?.offset_set(
                $b.sget(b)?
                    .as_usize()
                    .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?,
                $b.sget(c)?.to_owned(),
            )?,
            Instruction::GetTypeId(a, b) => $b.sset(b, Var::UString($b.sget(a)?.typeid()?.into()))?,
            Instruction::GetLength(a, b) => $b.sset(b, Var::U64($b.sget(a)?.rcl()? as u64))?,
            Instruction::Duplicate(a, b) => $b.sset(b, $b.sget(a)?.to_owned())?,
            Instruction::Transmute(a, b, c) => $b.sset(c, itc($b.sget(a)?, b)?)?,
            Instruction::Add(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, +, $b.sget(b)?)?)?,
            Instruction::Sub(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, -, $b.sget(b)?)?)?,
            Instruction::Mul(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, *, $b.sget(b)?)?)?,
            Instruction::Div(a, b, c) => {
                if !$b
                    .sget(b)?
                    .is_not_zero()
                    .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?
                {
                    return Err(anyhow!("raw::fatal::divide_zero"));
                }
                $b.sset(c, impl_vmb!($b.sget(a)?, /, $b.sget(b)?)?)?
            }
            Instruction::Rem(a, b, c) => {
                if !$b
                    .sget(b)?
                    .is_not_zero()
                    .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?
                {
                    return Err(anyhow!("raw::fatal::divide_zero"));
                }
                $b.sset(c, impl_vmb!($b.sget(a)?, %, $b.sget(b)?)?)?
            }
            Instruction::And(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, &, $b.sget(b)?)?)?,
            Instruction::Or(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, |, $b.sget(b)?)?)?,
            Instruction::Not(a, b) => $b.sset(
                b,
                bnot($b.sget(a)?).ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
            )?,
            Instruction::Xor(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, ^, $b.sget(b)?)?)?,
            Instruction::Shl(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, <<, $b.sget(b)?)?)?,
            Instruction::Shr(a, b, c) => $b.sset(c, impl_vmb!($b.sget(a)?, >>, $b.sget(b)?)?)?,
            Instruction::Equal(a, b, c) => {
                $b.sset(c, Var::U8(impl_vbe!($b.sget(a)?, ==, $b.sget(b)?)? as u8))?
            }
            Instruction::Mt(a, b, c) => {
                $b.sset(c, Var::U8(impl_vbe!($b.sget(a)?, >, $b.sget(b)?)? as u8))?
            }
            Instruction::Lt(a, b, c) => {
                $b.sset(c, Var::U8(impl_vbe!($b.sget(a)?, <, $b.sget(b)?)? as u8))?
            }
            Instruction::Jmp(a) => $b.jmp(a),
            Instruction::Jnz(a, b) => {
                if $b
                    .sget(a)?
                    .is_not_zero()
                    .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?
                {
                    $b.jmp(b)
                }
            }
            Instruction::Call(a) => {
                $b.call(getfp(&*a).ok_or_else(|| anyhow!("raw::fatal::no_such_func"))?)?
            }
            Instruction::CallPtr(a) => $b.call(
                getfp(
                    &*$b
                        .sget(a)?
                        .as_sr()
                        .ok_or_else(|| anyhow!("raw::fatal::segfault"))?
                        .borrow()?,
                )
                .ok_or_else(|| anyhow!("raw::fatal::no_such_func"))?,
            )?,
            Instruction::Int(a) => return Err(anyhow!("{}", &*a)),
            Instruction::Ret => {
                if !$b.ret() {
                    return Ok(());
                }
            }
            Instruction::Noop => std::hint::spin_loop(),
        };
    }
}

/// Core executing engine.
pub fn core(ctx: &mut Thread) -> Result<(), anyhow::Error> {
    loop {
        let cur = match ctx.next() {
            Some(x) => x,
            None => {
                let stat = ctx.ret();
                if !stat {
                    return Err(anyhow!("raw::fatal::early_eof"));
                } else {
                    return Err(anyhow!("raw::fatal::func_not_returned"));
                }
            }
        };
        core_inner!(cur, ctx)
    }
}

/// Start a thread.
pub fn start(mut ctx: Thread) {
    loop {
        let val = core(&mut ctx);
        let tmp;
        match val {
            Ok(()) => break,
            Err(x) => {
                if int(match x.downcast::<&str>() {
                    Ok(x) => x,
                    Err(x) => {
                        tmp = x.downcast::<String>().unwrap();
                        &tmp
                    }
                }) {
                    eprintln!("{:?}", ctx);
                    eprintln!("\nAborting...");
                    std::process::exit(-1);
                }
            }
        }
    }
}

/// Start a thread with NO OWNERSHIP.
pub fn start_noo(ctx: &mut Thread) {
    loop {
        let val = core(ctx);
        let tmp;
        match val {
            Ok(()) => break,
            Err(x) => {
                if int(match x.downcast::<&str>() {
                    Ok(x) => x,
                    Err(x) => {
                        tmp = x.downcast::<String>().unwrap();
                        &tmp
                    }
                }) {
                    eprintln!("{:?}", ctx);
                    eprintln!("\nAborting...");
                    std::process::exit(-1);
                }
            }
        }
    }
}

/// Start a thread with coroutines.
pub fn start_coro(ctx: Thread) {
    let mut idgen = CeIdGen::new();
    let mut table: HashMap<_, _, FakeHasher> = HashMap::default();
    idgen.next();
    table.insert(idgen.next(), ctx);
    loop {
        let mut cache = Vec::with_capacity(table.len());
        table.keys().for_each(|x| cache.push(*x));
        'world: for i in cache {
            'coro: while let Some(x) = table.get_mut(&i) {
                match core(x) {
                    Ok(()) => {
                        table.remove(&i);
                        idgen.free(i);
                        break 'coro;
                    }
                    Err(x) => match x.downcast::<String>() {
                        Ok(y) => match &y[..] {
                            "raw::coro::yield" => break 'coro,
                            "raw::coro::getcid" => {
                                table.get_mut(&i).unwrap().sset(100, Var::U64(i)).unwrap();
                                break 'coro;
                            }
                            "raw::coro::exit" => {
                                table.remove(&i);
                                idgen.free(i);
                                break 'world;
                            }
                            "raw::coro::kill" => {
                                coro_kill(&mut table, i, &mut idgen);
                                break 'world;
                            }
                            "raw::coro::spawn" => {
                                coro_spawn(&mut table, i, &mut idgen);
                                break 'coro;
                            }
                            "raw::coro::is_alive" => {
                                coro_isalive(&mut table, i, &mut idgen);
                                break 'coro;
                            }
                            "raw::coro::dump" => {
                                let dumped = format!("{{ Table {:?}, IdGen {:?} }}", table, idgen);
                                table
                                    .get_mut(&i)
                                    .unwrap()
                                    .sset(100, Var::UString(dumped.into()))
                                    .unwrap();
                                break 'coro;
                            }
                            _ => {
                                if int(&y[..]) {
                                    eprintln!("{:?}", table);
                                    table.remove(&i);
                                    idgen.free(i);
                                    break 'world;
                                }
                            }
                        },
                        Err(x) => {
                            if int(x.downcast::<&str>().unwrap()) {
                                eprintln!("{:?}", table);
                                table.remove(&i);
                                idgen.free(i);
                                break 'world;
                            }
                        }
                    },
                };
            }
        }
    }
}

/// Corotine-living judgement.
#[inline]
fn coro_isalive(table: &mut HashMap<u64, Thread, FakeHasher>, i: u64, idgen: &mut CeIdGen) {
    let id = match table.get(&i).unwrap().sget(100).unwrap().as_u64_strict() {
        Some(z) => z,
        None => {
            table.remove(&i);
            idgen.free(i);
            return;
        }
    };
    let stat = table.get(&id).is_some();
    table
        .get_mut(&i)
        .unwrap()
        .sset(100, Var::U8(stat as u8))
        .unwrap();
}

/// Kill a corotine.
#[inline]
fn coro_kill(table: &mut HashMap<u64, Thread, FakeHasher>, i: u64, idgen: &mut CeIdGen) {
    let id = match table.get(&i).unwrap().sget(100).unwrap().as_u64_strict() {
        Some(z) => z,
        None => {
            table.remove(&i);
            idgen.free(i);
            return;
        }
    };
    if table.get(&id).is_some() {
        table.remove(&id);
        idgen.free(id);
    }
}

/// Start a corotine.
#[inline]
fn coro_spawn(table: &mut HashMap<u64, Thread, FakeHasher>, i: u64, idgen: &mut CeIdGen) {
    let a = match table.get_mut(&i) {
        Some(z) => z,
        None => return,
    };
    let fp = match a.sget(100).unwrap().as_sr() {
        Some(z) => z,
        None => return,
    };
    let fp = match fp.borrow() {
        Ok(z) => z,
        Err(_) => return,
    };
    let args = match a.sget(101).unwrap() {
        Var::Vector(x) => match x.borrow() {
            Ok(z) => z,
            Err(_) => return,
        },
        _ => return,
    };
    let mut new_coro = Thread::new(
        match match getfp(&fp) {
            Some(x) => x,
            None => return,
        } {
            FuncPtr::Virtual(x) => x,
            _ => return,
        },
    );
    if args.len() > 50 {
        return;
    }
    for i in 0..args.len() {
        new_coro
            .sset(100 + i, unsafe { args.get_unchecked(i).to_owned() })
            .unwrap();
    }
    drop(args);
    let id = idgen.next();
    a.sset(100, Var::U64(id)).unwrap();
    table.insert(id, new_coro);
}

macro_rules! impl_itc {
    ($a:expr, $b: ty) => {
        match $a {
            Var::I8(x) => Ok(x as $b),
            Var::U8(x) => Ok(x as $b),
            Var::I16(x) => Ok(x as $b),
            Var::U16(x) => Ok(x as $b),
            Var::I32(x) => Ok(x as $b),
            Var::U32(x) => Ok(x as $b),
            Var::I64(x) => Ok(x as $b),
            Var::U64(x) => Ok(x as $b),
            _ => Err(anyhow!("raw::fatal::transmute_np")),
        }
    };
}
/// Integer type convertion.
fn itc(a: &Var, b: usize) -> Result<Var, anyhow::Error> {
    let a = a.to_owned();
    match b {
        8 => Ok(Var::I8(impl_itc!(a, i8)?)),
        9 => Ok(Var::U8(impl_itc!(a, u8)?)),
        16 => Ok(Var::I16(impl_itc!(a, i16)?)),
        17 => Ok(Var::U16(impl_itc!(a, u16)?)),
        32 => Ok(Var::I32(impl_itc!(a, i32)?)),
        33 => Ok(Var::U32(impl_itc!(a, u32)?)),
        64 => Ok(Var::I64(impl_itc!(a, i64)?)),
        65 => Ok(Var::U64(impl_itc!(a, u64)?)),
        _ => Err(anyhow!("raw::fatal::transmute_te")),
    }
}

/// Bitwise not.
fn bnot(a: &Var) -> Option<Var> {
    match a {
        Var::I8(x) => Some(Var::I8(!*x)),
        Var::U8(x) => Some(Var::U8(!*x)),
        Var::I16(x) => Some(Var::I16(!*x)),
        Var::U16(x) => Some(Var::U16(!*x)),
        Var::I32(x) => Some(Var::I32(!*x)),
        Var::U32(x) => Some(Var::U32(!*x)),
        Var::I64(x) => Some(Var::I64(!*x)),
        Var::U64(x) => Some(Var::U64(!*x)),
        _ => None,
    }
}
