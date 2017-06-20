use {Error, ErrorKind};
use std;

/// A value which can act as an array index.
pub trait Index : Copy + Into<u64> {
    fn from_u64(v: u64) -> Result<Self, Error>;
}

macro_rules! impl_index {
    ($ty:ident) => {
        impl Index for $ty {
            fn from_u64(i: u64) -> Result<$ty, Error> {
                if  i < $ty::max_value() as u64 {
                    Ok(i as _)
                } else {
                    let bits_available = std::mem::size_of::<$ty>() as u8 * 8;
                    return Err(ErrorKind::IndexTooSmall(i, bits_available).into())
                }
            }
        }
    }
}

impl_index!(u64);
impl_index!(u32);
impl_index!(u16);
impl_index!(u8);

