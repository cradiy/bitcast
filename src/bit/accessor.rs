/// Accessing bit-level data.
pub trait BitAccessor {
    const BIT_SIZE: usize;
    fn bit(&self, idx: usize) -> bool;
}


pub trait BitAdapter {
    type Accessor;
    fn accessor(&self) -> &[Self::Accessor];
}


mod _impl {
    use super::{BitAccessor, BitAdapter};
    macro_rules! impl_accessor {
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
        (@float $($ty:ty), +) => {
            $(
                impl BitAccessor for $ty {
                    const BIT_SIZE: usize = std::mem::size_of::<$ty>() * 8;
                    fn bit(&self, idx: usize) -> bool {
                        self.to_bits().bit(idx)
                    }
                }
            ) +
        };
    }
    impl BitAccessor for bool {
        const BIT_SIZE: usize = 1;
        
        fn bit(&self, idx: usize) -> bool {
            idx == 0 && *self
        }
    }
    macro_rules! impl_as_accessor {
        (@impl $([$ty:ty, $accessor:ty]), +) => {
            $(
                impl BitAdapter for $ty {
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
                impl<const N: usize> BitAdapter for [$ty; N] {
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
                impl BitAdapter for $ty {
                    type Accessor = u8;
                    fn accessor(&self) -> &[Self::Accessor] {
                        self.as_bytes()
                    }
                }
            ) +
        };
    }
    
    impl_as_accessor!(u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128, f32, f64, bool);
    impl_as_accessor!(@str str, &str, &&str);
    #[cfg(feature = "std")]
    impl_as_accessor!(@str &&::std::string::String, &::std::string::String, ::std::string::String);
    impl_accessor!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    impl_accessor!(@float f32, f64);
}
