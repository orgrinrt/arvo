#![no_std]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// 256-bit unsigned addition via the carrying_add idiom over 4 limbs of u64,
// the same shape a WideBits<32>-backed Adjustment/Bias composition would need.
#[no_mangle]
pub fn add256(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut out = [0u64; 4];
    let mut carry = false;
    let mut i = 0;
    while i < 4 {
        let (s, c) = a[i].carrying_add(b[i], carry);
        out[i] = s;
        carry = c;
        i += 1;
    }
    out
}
