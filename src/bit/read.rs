use super::accessor::{BitAccessor, BitAdapter};
use crate::error::*;
macro_rules! read_any {
    ($reader:expr, $len:expr, $ty:ty) => {{
        let mut bits = [false; $len];
        let size = $reader.read(&mut bits);
        if size != $len {
            return Err(Error::InvalidSize);
        }
        Ok(bits
            .into_iter()
            .enumerate()
            .fold(0, |out, (i, b)| out | (b as $ty) << i))
    }};
}
pub trait BitRead {
    #[must_use]
    fn read(&mut self, buf: &mut [bool]) -> usize;
    fn read_bit(&mut self) -> Result<bool> {
        let mut buf = [false; 1];
        let size = self.read(&mut buf);
        if size != 1 {
            return Err(Error::InvalidSize);
        }
        Ok(buf[0])
    }
    fn read_4bits(&mut self) -> Result<u8> {
        read_any!(self, 4, u8)
    }
    fn read_u8(&mut self) -> Result<u8> {
        read_any!(self, 8, u8)
    }
}

macro_rules! gen_bit_read_ext {
    ($([$n:expr, $ty:ty, $fn_le:ident, $fn_be:ident]), +) => {
        $(
            fn $fn_le(&mut self) -> Result<$ty> {
                read_any!(self, $n, $ty)
            }
            fn $fn_be(&mut self) -> Result<$ty> {
                self.$fn_le().map(|v| {
                    let mut bytes = v.to_le_bytes();
                    bytes.reverse();
                    <$ty>::from_be_bytes(bytes)
                })
            }
        ) +
    };
    (@float $([$n:expr, $ty:ty, $fn_le:ident, $fn_be:ident]), +) => {
        $(
            fn $fn_le(&mut self) -> Result<$ty> {
                self.$fn_be().map(|v| {
                    let mut bytes = v.to_be_bytes();
                    bytes.reverse();
                    <$ty>::from_le_bytes(bytes)
                })
            }
            fn $fn_be(&mut self) -> Result<$ty> {
                let mut bytes = [0; $n / 8];
                for b in &mut bytes {
                    *b = self.read_u8()?;
                }
                Ok(<$ty>::from_be_bytes(bytes))
            }
        ) +
    };

}

pub trait BitReadExt: BitRead {
    gen_bit_read_ext!(
        [16, u16, read_u16_le, read_u16_be],
        [24, u32, read_u24_le, read_u24_be],
        [32, u32, read_u32_le, read_u32_be],
        [64, u64, read_u64_le, read_u64_be],
        [128, u128, read_u128_le, read_u128_be],
        [16, i16, read_i16_le, read_i16_be],
        [32, i32, read_i32_le, read_i32_be],
        [64, i64, read_i64_le, read_i64_be],
        [128, i128, read_i128_le, read_i128_be]
    );
    gen_bit_read_ext!(@float [32, f32, read_f32_le, read_f32_be], [64, f64, read_f64_le, read_f64_be]);
}

impl<R: BitRead> BitReadExt for R {}

#[derive(Debug, Clone)]
pub struct BitBuf<T> {
    buf: T,
    index: usize,
    offset: usize,
}
impl<T: BitAdapter<Accessor = N>, N: BitAccessor> Iterator for BitBuf<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [false];
        if self.read(&mut buf) != 1 {
            None
        } else {
            Some(buf[0])
        }
    }
}

impl<T> BitBuf<T> {
    pub fn new(buf: T) -> Self {
        Self {
            buf,
            index: 0,
            offset: 0,
        }
    }
    /// # Safety
    pub unsafe fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    /// # Safety
    pub unsafe fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
}
impl<T: BitAdapter<Accessor = N>, N: BitAccessor> BitBuf<T> {
    pub fn remain(&self) -> usize {
        self.len() - self.position()
    }
    pub fn position(&self) -> usize {
        self.index * N::BIT_SIZE + self.offset
    }
    fn len(&self) -> usize {
        self.buf.accessor().len() * N::BIT_SIZE
    }
    fn is_empty(&self) -> bool {
        self.remain() == 0
    }
    pub fn accessor(&self) -> &[N] {
        self.buf.accessor()
    }
}

impl<N: BitAccessor, T: BitAdapter<Accessor = N>> BitRead for BitBuf<T> {
    fn read(&mut self, buf: &mut [bool]) -> usize {
        if self.is_empty() || buf.is_empty() {
            return 0;
        }
        let size = copy_bits_from_accessor(buf, &self.accessor()[self.index..], self.offset);
        let pos = BitBuf::position(self) + size;
        self.index = pos / N::BIT_SIZE;
        self.offset = pos % N::BIT_SIZE;
        size
    }
}
fn copy_bits_from_accessor<N: BitAccessor>(bits: &mut [bool], accessor: &[N], pos: usize) -> usize {
    let mut index = 0;
    let mut pos = pos;
    for byte in accessor {
        for idx in 0..N::BIT_SIZE {
            if bits.len() <= index {
                return bits.len();
            }
            if pos > 0 {
                pos -= 1;
            } else {
                bits[index] = byte.bit(idx);
                index += 1;
            }
        }
    }
    index
}
