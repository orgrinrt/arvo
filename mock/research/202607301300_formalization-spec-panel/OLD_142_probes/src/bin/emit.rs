//! A linked binary, so a profile with LTO can be inspected after the link step
//! rather than before it. `--emit=asm` on a library under LTO shows the
//! pre-link code, which is not what ships and not what a bench times.
fn main() {
    let a = [3u16; 8192];
    let b = [5u32; 8192];
    println!(
        "{} {} {} {}",
        p142_asm::c_min_w16_a256(&a),
        p142_asm::c_fit_w16_a256(&a),
        p142_asm::c_lanes_w16_a256(&a),
        p142_asm::c_head_w16_a256(&b),
    );
}
