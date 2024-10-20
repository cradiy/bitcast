mod read;
pub use read::ByteReadExt;
use std::io::Result;
pub trait ByteCast: Sized {
    fn byte_cast<R: ByteReadExt>(buf: &mut R) -> Result<Self>;
}