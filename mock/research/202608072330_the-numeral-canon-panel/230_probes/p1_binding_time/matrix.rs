// The 2x2: evaluation site against build profile, on one expression at a time.
//
// Built once per (operation, site) so a const refusal takes down only its own
// compilation. The runtime half of each pair is the identical expression with
// the identical types, so the only thing that moves between the two halves is
// where it is evaluated.
//
// OP is set by the caller: add, sub, shl.
fn main() {
    let a: u8 = std::hint::black_box(200);
    let b: u8 = std::hint::black_box(100);
    let one: u8 = std::hint::black_box(1);
    let nine: u32 = std::hint::black_box(9);
    let zero: u8 = std::hint::black_box(0);
    match std::env::args().nth(1).unwrap_or_default().as_str() {
        "add" => println!("{}", a + b),
        "sub" => println!("{}", zero - one),
        "shl" => println!("{}", one << nine),
        other => panic!("no such op: {other}"),
    }
}
