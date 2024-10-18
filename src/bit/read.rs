

macro_rules! read_any {
    ($reader:expr, $len:expr, $ty:ty) => {
        {
            let mut bits = [false; $len];
            let size = $reader.read(&mut bits);
            if size != $len {
                return None;
            }
            Some(
                bits.into_iter()
                    .rev()
                    .enumerate()
                    .map(|(i, b)| (b as $ty) << i)
                    .sum(),
            )
        }
    };
}
pub trait BitRead {
    #[must_use]
    fn read(&mut self, buf: &mut [bool]) -> usize;
    fn read_4bits(&mut self) -> Option<u8> {
        read_any!(self, 4, u8)
    }
    fn read_u8(&mut self) -> Option<u8> {
        read_any!(self, 8, u8)
    }
}


pub trait BitAccessor {
    const BIT_SIZE: usize;
    fn bit(&self, idx: usize) -> bool;
}

macro_rules! impl_bix {
    ($($ty:ty), +) => {
        $(
            impl BitAccessor for $ty {
                const BIT_SIZE: usize = std::mem::size_of::<$ty>() * 8;
                fn bit(&self, idx: usize) -> bool {
                    (*self & (1 << idx)) != 0
                }
            }
        ) +
    };
}
impl_bix!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl BitAccessor for bool {
    const BIT_SIZE: usize = 1;

    fn bit(&self, idx: usize) -> bool {
        idx == 0 && *self
    }
}
pub trait BitStreamAdapter {
    type Accessor;
    fn accessor(&self) -> &[Self::Accessor];
}
macro_rules! impl_as_accessor {
    (@impl $([$ty:ty, $accessor:ty]), +) => {
        $(
            impl BitStreamAdapter for $ty {
                type Accessor = $accessor;

                fn accessor(&self) -> &[Self::Accessor] {
                    self.as_ref()
                }
            }
        ) +
    };
    ([$($ty:ty), +], $accessor:ty) => {
        impl_as_accessor!(@impl $([$ty, $accessor]), +);
    };
    ($($ty:ty), +) => {
        $(
            impl<const N: usize> BitStreamAdapter for [$ty; N] {
                type Accessor = $ty;

                fn accessor(&self) -> &[Self::Accessor] {
                    &self[..]
                }
            }
            #[cfg(not(feature = "std"))]
            impl_as_accessor!([&[$ty], [$ty]], $ty);
            #[cfg(feature = "std")]
            impl_as_accessor!([&[$ty], [$ty], ::std::vec::Vec<$ty>], $ty);
        ) +
    };
    (@str $($ty:ty), +) => {
        $(
            impl BitStreamAdapter for $ty {
                type Accessor = u8;
                fn accessor(&self) -> &[Self::Accessor] {
                    self.as_bytes()
                }
            }
        ) +
    }
}

impl_as_accessor!(u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128);
impl_as_accessor!(@str str, &str, &&str);
#[cfg(feature = "std")]
impl_as_accessor!(@str &&::std::string::String, &::std::string::String, ::std::string::String);

#[derive(Debug, Clone)]
pub struct BitBuf<T> {
    buf: T,
    index: usize,
    pos: usize,
}

impl<T> BitBuf<T> {
    pub fn new(buf: T) -> Self {
        Self {
            buf,
            index: 0,
            pos: 0,
        }
    }
}
impl<T: BitStreamAdapter<Accessor = N>, N: BitAccessor> BitBuf<T> {
    fn remain(&self) -> usize {
        self.len() - self.position()
    }
    fn position(&self) -> usize {
        self.index * N::BIT_SIZE + self.pos
    }
    fn len(&self) -> usize {
        self.buf.accessor().len() * N::BIT_SIZE
    }
    fn is_empty(&self) -> bool {
        self.remain() == 0
    }
    fn slice(&self) -> &[N] {
        self.buf.accessor()
    }
}

impl<N: BitAccessor, T: BitStreamAdapter<Accessor = N>> BitRead for BitBuf<T> {
    fn read(&mut self, buf: &mut [bool]) -> usize {
        if self.is_empty() || buf.is_empty() {
            return 0;
        }
        let size = copy_bits_from_numbers(buf, &self.slice()[self.index..], self.pos);
        let pos = self.position() + size;
        self.index = pos / N::BIT_SIZE;
        self.pos = pos % N::BIT_SIZE;
        size
    }
}
fn copy_bits_from_numbers<N: BitAccessor>(bits: &mut [bool], numbers: &[N], pos: usize) -> usize {
    let mut index = 0;
    let mut pos = pos;
    for byte in numbers {
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

#[test]
fn test() {
    // let data: [u8; 3] = [0b1010_0010, 0b0101_0101, 0b1111_0011];
    let data = "Hello";
    // let data = [1, 2, 3];
    let mut buf = BitBuf::new(data);
    assert_eq!(buf.read_u8(), Some(b'H'));
    assert_eq!(buf.read_u8(), Some(b'e'));
    assert_eq!(buf.read_u8(), Some(b'l'));
    assert_eq!(buf.read_u8(), Some(b'l'));
    assert_eq!(buf.read_u8(), Some(b'o'));
}
