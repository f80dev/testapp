use elrond_wasm::imports;
use elrond_wasm_debug::api::{RustBigUint};

imports!();

fn convert_nibbles_to_u64(values: &[u8]) -> u64 {
    let mut out = 0;
    for &i in values {
        out = out << 4 | i as u64;
    }
    out
}

fn main() {
    let mut c=[0u8,0,0,0,0,0,0,0];
    RustBigUint::from(0xFFFFu64).copy_to_slice_big_endian(&c);
    let b=u64::from_be_bytes(c);
    println!("{}",b);
}
