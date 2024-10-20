use bitcast::{BitBuf, BitReadExt};


fn main() {
    let mut buf = BitBuf::new((-18934i32).to_le_bytes());
    println!("{:?}", buf.read_i32_le());
    let f = 0.34f32;
    println!("{:?}", f.to_be_bytes());

}