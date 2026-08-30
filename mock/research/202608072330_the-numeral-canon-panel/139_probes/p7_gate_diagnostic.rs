// p7: AN AD-HOC QUICK SPIKE WITH NO SUBSTANCE, in the workspace's own words.
// It is not a bench, it did not run on the bench harness, and it prices
// nothing. It exists to separate two candidate mechanisms for why
// bitpack-write-contend-shared's stress tests do not finish, so the gate
// finding names a cause rather than a symptom.
//
// The stress tests call build_bytes once per trial, 4500 times across three
// tests. build_bytes allocates a zeroed buffer sized from MAX_N = 4_194_304
// and then fills only n = 4094 elements of it. The question is whether the
// per-trial buffer is the dominant term.
//
// The control that must fail: the same loop with a buffer sized to what the
// test actually uses must be dramatically cheaper. If both come out the same,
// the buffer is not the mechanism and this spike has refuted its own guess.
use std::time::Instant;

const MAX_N: usize = 4_194_304;
const LOGICAL_BITS: usize = 13;
const TOTAL_BYTES: usize = MAX_N * 2 + MAX_N * 2 + (MAX_N * LOGICAL_BITS) / 8 + 16;
const STRESS_N: usize = 4094;
const RIGHTSIZED: usize = STRESS_N * 2 + STRESS_N * 2 + (STRESS_N * LOGICAL_BITS) / 8 + 16;

struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn build(total: usize, n: usize, seed: u64) -> Vec<u8> {
    let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0002_5EED);
    let mut buf = vec![0u8; total];
    for i in 0..n {
        let v = (rng.next() & 0x1FFF) as u16;
        buf[i * 2..i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    buf
}

fn main() {
    let trials = 200usize;
    println!("TOTAL_BYTES as the crate sizes it = {TOTAL_BYTES} ({:.1} MB)", TOTAL_BYTES as f64 / 1e6);
    println!("right-sized for STRESS_N = {STRESS_N} would be {RIGHTSIZED} bytes");
    println!("ratio = {:.0}x", TOTAL_BYTES as f64 / RIGHTSIZED as f64);
    println!();

    let mut sink = 0u64;
    let t = Instant::now();
    for i in 0..trials {
        let b = build(TOTAL_BYTES, STRESS_N, i as u64);
        sink = sink.wrapping_add(b[0] as u64 + b[TOTAL_BYTES - 1] as u64);
    }
    let as_is = t.elapsed();

    let t = Instant::now();
    for i in 0..trials {
        let b = build(RIGHTSIZED, STRESS_N, i as u64);
        sink = sink.wrapping_add(b[0] as u64 + b[RIGHTSIZED - 1] as u64);
    }
    let right = t.elapsed();

    println!("{trials} builds as the crate sizes them : {:?}", as_is);
    println!("{trials} builds right-sized             : {:?}", right);
    println!("per-build as-is      : {:?}", as_is / trials as u32);
    println!("per-build right-sized: {:?}", right / trials as u32);
    println!("sink (kept so nothing is eliminated): {sink}");
    println!();
    let projected = as_is.as_secs_f64() / trials as f64 * 4500.0;
    println!("projected cost of the 4500 builds the three stress tests perform: {:.1} s", projected);
    println!();
    if right.as_secs_f64() * 4.0 < as_is.as_secs_f64() {
        println!("CONTROL PASSED: right-sizing is more than 4x cheaper, so the buffer");
        println!("  IS a dominant term and the gate finding names a real cause.");
    } else {
        println!("CONTROL FAILED: right-sizing changes little, so the buffer is NOT the");
        println!("  mechanism and my guess is refuted. The cost is somewhere else.");
    }
}
