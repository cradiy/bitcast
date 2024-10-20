use std::io::Read;

use std::io::Result;

macro_rules! read_any {
    ($r:expr, $ty:ty, $n:expr) => {
        {
            let mut buf = [0; $n];
            $r.read_exact(&mut buf)?;
            Ok(<$ty>::from_be_bytes(buf))
        }
    };
}

macro_rules! gen_byte_read_ext {
    ($([$n:expr, $ty:ty, $fn_le:ident, $fn_be:ident]), +) => {
        $(
            fn $fn_be(&mut self) -> Result<$ty> {
                read_any!(self, $ty, $n)
            }
            fn $fn_le(&mut self) -> Result<$ty> {
                read_any!(self, $ty, $n).map(|v| {
                    let mut bytes = v.to_be_bytes();
                    bytes.reverse();
                    <$ty>::from_le_bytes(bytes)
                })
            }
        ) +
    };
}

pub trait ByteReadExt: Read {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_be_bytes(buf))
    }
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(i8::from_be_bytes(buf))
    }
    fn read_u24_be(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf[1..])?;
        Ok(u32::from_be_bytes(buf))
    }
    fn read_u24_le(&mut self) -> Result<u32> {
        self.read_u24_be().map(|v| {
            let mut bytes = v.to_be_bytes();
            bytes.reverse();
            u32::from_le_bytes(bytes)
        })
    }
    gen_byte_read_ext!(
        [2, u16, read_u16_le, read_u16_be],
        [4, u32, read_u32_le, read_u32_be],
        [8, u64, read_u64_le, read_u64_be],
        [16, u128, read_u128_le, read_u128_be],
        [2, i16, read_i16_le, read_i16_be],
        [4, i32, read_i32_le, read_i32_be],
        [8, i64, read_i64_le, read_i64_be],
        [16, i128, read_i128_le, read_i128_be],
        [4, f32, read_f32_le, read_f32_be],
        [8, f64, read_f64_le, read_f64_be]
    );
}

impl<R: Read> ByteReadExt for R {
    
}