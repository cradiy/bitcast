pub mod bit;
pub mod byte;

pub trait ByteCast {
    fn cast() -> Self;
}