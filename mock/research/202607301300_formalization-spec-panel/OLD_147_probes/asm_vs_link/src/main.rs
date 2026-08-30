use asm_vs_link::{accumulate, saturating_pass};

fn main() {
    let a: Vec<u32> = (0..4096u32).collect();
    let mut b: Vec<u32> = vec![1; 4096];
    accumulate(&a, &mut b);

    let c: Vec<u16> = (0..4096u32).map(|v| v as u16).collect();
    let mut d: Vec<u16> = vec![7; 4096];
    saturating_pass(&c, &mut d);

    println!("{} {}", b[4095], d[4095]);
}
