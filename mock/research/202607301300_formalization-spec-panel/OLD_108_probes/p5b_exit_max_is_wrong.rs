// p5b: is a max routed through an exit merely unavailable at two lanes, or wrong?
fn lanewise_max(a: [u32; 2], b: [u32; 2]) -> [u32; 2] {
    [
        if a[0] < b[0] { b[0] } else { a[0] },
        if a[1] < b[1] { b[1] } else { a[1] },
    ]
}
fn exit_max_all(a: [u32; 2], b: [u32; 2]) -> [u32; 2] {
    let lt = [a[0] < b[0], a[1] < b[1]];
    if lt[0] & lt[1] {
        b
    } else {
        a
    } // reduce with `all`, then branch
}
fn exit_max_any(a: [u32; 2], b: [u32; 2]) -> [u32; 2] {
    let lt = [a[0] < b[0], a[1] < b[1]];
    if lt[0] | lt[1] {
        b
    } else {
        a
    } // reduce with `any`, then branch
}
fn main() {
    let a = [7u32, 2];
    let b = [3u32, 9];
    println!("lanewise: {:?}", lanewise_max(a, b));
    println!("exit/all: {:?}", exit_max_all(a, b));
    println!("exit/any: {:?}", exit_max_any(a, b));
}
