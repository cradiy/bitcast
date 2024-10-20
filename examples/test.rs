use bitcast::{BitAccessor, BitBuf};
fn main() {
    let b = 2.33f32.to_bits();
    let buf = BitBuf::new([b]);
    let b: u32 = buf.enumerate().fold(0, |out, (idx, bit)| {
        out | (if bit { 1 } else {0} << idx)
    });
    println!("{}", f32::from_be_bytes(b.to_be_bytes()))
}
