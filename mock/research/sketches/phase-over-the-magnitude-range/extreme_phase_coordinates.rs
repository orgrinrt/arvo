fn old(pn: i64, pd: i64) -> bool {
    if pd == 0 { return false; }
    pn % pd == 0 && { let s = -(pn / pd); s >= -128 && s <= 127 }
}
fn new(pn: i64, pd: i64) -> Option<i64> {
    if pd == 0 { return None; }
    if pn == 0 { return Some(0); }
    let num = -(pn as i128);
    let den = pd as i128;
    if den == 0 || num % den != 0 { return None; }
    let slot = num / den;
    if slot < i64::MIN as i128 || slot > i64::MAX as i128 { return None; }
    Some(slot as i64)
}
fn main() {
    for (pn, pd, label) in [
        (i64::MIN, -1i64, "MIN % -1"),
        (i64::MIN, 1i64, "-(MIN / 1)"),
        (i64::MAX, -1i64, "MAX / -1"),
        (i64::MIN, i64::MIN, "MIN / MIN"),
    ] {
        println!("--- {label}: pn={pn} pd={pd}");
        println!("    new = {:?}", new(pn, pd));
        let r = std::panic::catch_unwind(move || old(pn, pd));
        match r { Ok(v) => println!("    old = Ok({v})"), Err(_) => println!("    old = PANICKED") }
    }
}
