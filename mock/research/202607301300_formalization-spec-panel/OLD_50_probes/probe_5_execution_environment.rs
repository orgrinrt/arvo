//! Probe 5: what the executing machine does with a float, as opposed to what the format says.
//!
//! rustc --edition 2021 -O probe_5_execution_environment.rs -o /tmp/p5 && /tmp/p5
//!
//! Target: aarch64-apple-darwin. Every number and instruction below is that target's.
//!
//! The design can declare a rounding direction and an underflow policy in a numeral's type.
//! This probe measures whether the machine honours them, by writing FPCR directly and
//! recomputing the identical Rust expression. If the answer changes, the type did not carry
//! the guarantee, and the design owes a contract to the layer that can.
//!
//! FPCR bits used, per the Arm A-profile architecture reference:
//!   [23:22] RMode: 00 nearest-even, 01 toward +inf, 10 toward -inf, 11 toward zero
//!   [24]    FZ:    flush-to-zero for single and double precision

use std::hint::black_box;

#[cfg(target_arch = "aarch64")]
fn read_fpcr() -> u64 {
    let v: u64;
    unsafe { core::arch::asm!("mrs {}, fpcr", out(reg) v, options(nomem, nostack)) };
    v
}

#[cfg(target_arch = "aarch64")]
fn write_fpcr(v: u64) {
    unsafe { core::arch::asm!("msr fpcr, {}", in(reg) v, options(nomem, nostack)) };
}

fn main() {
    #[cfg(not(target_arch = "aarch64"))]
    {
        println!("this probe measures aarch64 FPCR; skipped on this target");
        return;
    }

    #[cfg(target_arch = "aarch64")]
    {
        let base = read_fpcr();
        println!(
            "FPCR on entry: {base:#018x}  (RMode={}, FZ={})",
            (base >> 22) & 3,
            (base >> 24) & 1
        );

        // ---- 1. the rounding direction is not in the instruction ----
        let a = black_box(1.0f32);
        let b = black_box(3.0f32);
        let rn = black_box(a / b);
        println!(
            "\n1.0/3.0 under the entry mode: {:#010x} ({rn:e})",
            rn.to_bits()
        );

        for (name, mode) in [
            ("toward +inf", 1u64),
            ("toward -inf", 2),
            ("toward zero", 3),
        ] {
            write_fpcr((base & !(3 << 22)) | (mode << 22));
            let r = black_box(black_box(a) / black_box(b));
            write_fpcr(base);
            println!(
                "1.0/3.0 with RMode={mode} ({name}): {:#010x}  differs from nearest: {}",
                r.to_bits(),
                r.to_bits() != rn.to_bits()
            );
        }

        // ---- 2. gradual underflow is not in the format either ----
        let tiny = black_box(f32::from_bits(0x0080_0000)); // smallest normal
        let half = black_box(0.5f32);
        let sub = black_box(tiny * half);
        println!(
            "\nMIN_POSITIVE * 0.5 under the entry mode: {:#010x} (subnormal: {})",
            sub.to_bits(),
            sub != 0.0 && sub.to_bits() & 0x7f80_0000 == 0
        );
        write_fpcr(base | (1 << 24));
        let flushed = black_box(black_box(tiny) * black_box(half));
        write_fpcr(base);
        println!(
            "MIN_POSITIVE * 0.5 with FZ=1:            {:#010x} (flushed to zero: {})",
            flushed.to_bits(),
            flushed == 0.0
        );

        // ---- 3. the compiler's own constant folding is IEEE regardless ----
        // This one is computed at compile time by rustc's APFloat, never by the FPU.
        const FOLDED: f32 = 1.0f32 / 3.0f32;
        const FOLDED_SUB: f32 = f32::from_bits(0x0080_0000) * 0.5;
        write_fpcr((base & !(3 << 22)) | (3 << 22) | (1 << 24));
        let runtime_div = black_box(black_box(1.0f32) / black_box(3.0f32));
        let runtime_sub = black_box(black_box(f32::from_bits(0x0080_0000)) * black_box(0.5f32));
        write_fpcr(base);
        println!(
            "\nconst-folded 1.0/3.0     {:#010x}   runtime under RZ+FZ {:#010x}   agree: {}",
            FOLDED.to_bits(),
            runtime_div.to_bits(),
            FOLDED.to_bits() == runtime_div.to_bits()
        );
        println!(
            "const-folded subnormal   {:#010x}   runtime under RZ+FZ {:#010x}   agree: {}",
            FOLDED_SUB.to_bits(),
            runtime_sub.to_bits(),
            FOLDED_SUB.to_bits() == runtime_sub.to_bits()
        );

        // ---- 4. what a fused multiply-add is ----
        let x = black_box(1.0f32 + f32::EPSILON);
        let y = black_box(1.0f32 - f32::EPSILON / 2.0);
        let fused = x.mul_add(y, black_box(-1.0f32));
        let unfused = black_box(x * y) + black_box(-1.0f32);
        println!(
            "\nmul_add vs separate multiply then add: {:e} against {:e}, identical: {}",
            fused,
            unfused,
            fused.to_bits() == unfused.to_bits()
        );
    }
}
