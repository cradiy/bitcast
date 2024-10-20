mod accessor;
mod read;
pub use accessor::*;
pub use read::*;
use crate::error::*;
pub trait BitCast: Sized {
    fn bit_cast<R: BitRead>(reader: &mut R) -> Result<Self>;    
}

impl BitCast for u8 {
    fn bit_cast<R: BitRead>(reader: &mut R) -> Result<Self> {
        reader.read_u8()
    }
}

impl BitCast for bool {
    fn bit_cast<R: BitRead>(reader: &mut R) -> Result<Self> {
        
        reader.read_bit()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConstStr<const BYTE_SIZE: usize> {
    data: [u8; BYTE_SIZE],
}
impl<const N: usize> Default for ConstStr<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const BIT_SIZE: usize> ConstStr<BIT_SIZE>  {
    pub fn new() -> Self {
        Self {
            data: [0; BIT_SIZE],
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }
    pub fn as_str(&self) -> core::result::Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.data)
    }
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    pub unsafe fn as_str_unchecked(&self) -> &str {
        core::str::from_utf8_unchecked(&self.data)
    }
}
impl<const N: usize> AsRef<str> for ConstStr<N> {
    fn as_ref(&self) -> &str {
        unsafe { self.as_str_unchecked() }
    }
}

impl<const N: usize> BitCast for ConstStr<N>  {
    fn bit_cast<R: BitRead>(reader: &mut R) -> Result<Self> {
        let mut data = [0; N];        
        for byte in data.iter_mut() {
            *byte = reader.read_u8()?;
        }
        Ok(ConstStr {
            data
        })
    }
}


impl<const N: usize> BitCast for [bool; N] {
    fn bit_cast<R: BitRead>(reader: &mut R) -> Result<Self> {
        let mut buf = [false; N];        
        let size = reader.read(&mut buf);
        if size != N {
            return Err(Error::InvalidSize);
        }
        Ok(buf)
    }
}
impl<const N: usize> BitCast for [u8; N] {
    fn bit_cast<R: BitRead>(reader: &mut R) -> Result<Self> {
        let mut data = [0; N];        
        for byte in data.iter_mut() {
            *byte = reader.read_u8()?;
        }
        Ok(data)
    }
}