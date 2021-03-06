use crate::vmem::Var;
use std::fmt::{self, Debug, Formatter};

/// An instruction.
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Set constant to SIL. This should not set a reference unless you are sure that it should be
    /// never freed. Always use this to set a value.
    SetConstant(usize, Var),
    /// Set constant to SIL, but not convert time.
    DynSetConstant(usize, Box<str>, Box<str>),
    /// Judge if A is null, write 1 to B, or write 0 to B.
    IsNull(usize, usize),
    /// Get a static variable, copy to SIL. A is const, and B is the address to store value got.
    GetStatic(Box<str>, usize),
    /// Set a static variable to specified value on SIL. A is const, and B is the address to get
    /// the value to store.
    SetStatic(Box<str>, usize),
    /// Get a field of an object, copy to SIL. A is an object, and B is const, C is the address to
    /// store value got.
    GetField(usize, Box<str>, usize),
    /// Set a field of an object, copy to SIL. A is an object, and B is const, C is the address to
    /// get value to store.
    SetField(usize, Box<str>, usize),
    /// Get an element in a vector with specified index. A is an vector, and B is u64, C is the
    /// address to store value got.
    OffsetGet(usize, usize, usize),
    /// Set an element in a vector with specified index. A is an vector, and B is u64, C is the
    /// address to get the value to store.
    OffsetSet(usize, usize, usize),
    /// Get Type ID of an object. A can be anything, and B is the address to store the value got. B
    /// will be `UString` type.
    GetTypeId(usize, usize),
    /// Get length of a vector or bytes. A can be vector or bytes, and B is the address to store
    /// the value got. B will be `u64` type. It is undefined behavior to get the length of an
    /// object, string or primitive type, but it may be defined in the future.
    GetLength(usize, usize),
    /// Copy data on SIL. A is src, and B is dest. This copies reference of reference types.
    Duplicate(usize, usize),
    /// Converts an integer to another type. A is src, B is const, and C is the address to store
    /// the value got. B can be integer type descriptors, such as 8 for i8, 9 for u8, 16 for i16,
    /// 17 for u16, etc.
    Transmute(usize, usize, usize),
    /// Integer add operation. Both A and B are integers, and C is the address to store the value
    /// got.
    Add(usize, usize, usize),
    /// Integer sub operation. Both A and B are integers, and C is the address to store the value
    /// got.
    Sub(usize, usize, usize),
    /// Integer mul operation. Both A and B are integers, and C is the address to store the value
    /// got.
    Mul(usize, usize, usize),
    /// Integer div operation. Both A and B are integers, and C is the address to store the value
    /// got.
    Div(usize, usize, usize),
    /// Integer rem operation. Both A and B are integers, and C is the address to store the value
    /// got.
    Rem(usize, usize, usize),
    /// Bitwise and operation. A and B should have the same length, primitive type, and C is the
    /// address to store the value got.
    And(usize, usize, usize),
    /// Bitwise or operation. A and B should have the same length, primitive type, and C is the
    /// address to store the value got.
    Or(usize, usize, usize),
    /// Bitwise not operation. A and B should be one of the primitive types, and C is the address
    /// to store the value got.
    Not(usize, usize),
    /// Bitwise xor operation. A and B should have the same length, primitive type, and C is the
    /// address to store the value got.
    Xor(usize, usize, usize),
    /// Bitwise left.
    Shl(usize, usize, usize),
    /// Bitwise right.
    Shr(usize, usize, usize),
    /// Equal comparation. A and B can be anything, and C is the address to store the value got.
    /// However, A should have the same type as B. It will compare data of value types, and it will
    /// compare data of primitive types.
    Equal(usize, usize, usize),
    /// Integer more-than comparation. Both A and B should be integers, and C is the address to
    /// store the value got.
    Mt(usize, usize, usize),
    /// Integer less-than comparation. Both A and B should be integers, and C is the address to
    /// store the value got.
    Lt(usize, usize, usize),
    /// Jump to specified address. A is const.
    Jmp(usize),
    /// Jump to specified address if A is not zero. B is const.
    Jnz(usize, usize),
    /// Call specified function directly, not through a pointer.
    Call(Box<str>),
    /// Call a function through a `function pointer`(UString).
    CallPtr(usize),
    /// Interrupt.
    Int(Box<str>),
    /// Make this function return.
    Ret,
    /// No-op.
    Noop,
}

/// The virtual function pointer.
pub type VirtFuncPtr = &'static [Instruction];
/// The native function pointer with VM-friendly ABI.
pub type NativeFuncPtr = fn(&mut [Var]) -> Result<(), anyhow::Error>;

/// A function pointer both virtual and native.
#[derive(Clone)]
pub enum FuncPtr {
    /// A VM function pointer.
    Virtual(VirtFuncPtr),
    /// A native function pointer.
    Native(NativeFuncPtr),
}
impl Debug for FuncPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Virtual(x) => write!(f, "{:?}", x),
            Self::Native(x) => write!(f, "(native:{})", x as *const _ as usize),
        }
    }
}

/// An interrupt handler type. This provides least methods, because this should be handled
/// manually.
#[derive(Debug, Clone)]
pub enum InterruptHandler {
    /// Ignore the interrupt.
    Ignore,
    /// Abort the program. If the argument is `Some`, print a message.
    Abort(Option<String>),
    /// Catched by VM function pointer.
    Handler(VirtFuncPtr),
}
