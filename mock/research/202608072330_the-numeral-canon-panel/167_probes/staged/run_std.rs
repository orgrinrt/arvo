// Build 2: the identical machinery under std, so the checks can print.
include!("expr.rs");

const fn m8() -> i64 {
    mask(8)
}

fn main() {
    let a = Lit::<16>(0xBEEF);
    let b = Lit::<16>(0x1234);
    let c = Lit::<16>(0x00FF);
    let e = Mul(Add(a, b), c);

    println!(
        "NC20  forward width computed from the description alone, no sink: {}",
        <Mul<Add<Lit<16>, Lit<16>>, Lit<16>> as Expr>::FWD_W
    );

    let wn = e.work_w::<8>();
    let ww = e.work_w::<63>();
    println!(
        "NC19  work width at demand 8 = {wn}, at demand 63 = {ww}, moved = {}",
        wn != ww
    );

    let narrow = observe::<8, _>(e);
    let wide = observe::<63, _>(e) & m8();
    println!(
        "NC18a congruence-only chain: narrow {narrow} vs wide-then-mask {wide}, agree = {}",
        narrow == wide
    );

    let f = Add(Shr::<_, 4>(Mul(a, b)), c);
    let fnw = observe::<8, _>(f);
    let fw = observe::<63, _>(f) & m8();
    println!(
        "NC18b with a blocking Shr node: narrow {fnw} vs wide-then-mask {fw}, agree = {}",
        fnw == fw
    );

    let wrong = ((((a.eval::<8>().wrapping_mul(b.eval::<8>())) & m8()) >> 4)
        .wrapping_add(c.eval::<8>()))
        & m8();
    println!("NC18c a lowering that WRONGLY passes the demand through the shift gives {wrong}, correct is {fnw}, differ = {}", wrong != fnw);

    println!(
        "PASSES_DEMAND: Add {}, Mul {}, Shr {}",
        <Add<Lit<16>, Lit<16>> as Expr>::PASSES_DEMAND,
        <Mul<Lit<16>, Lit<16>> as Expr>::PASSES_DEMAND,
        <Shr<Lit<16>, 4> as Expr>::PASSES_DEMAND
    );
    println!(
        "size_of description = {} bytes, no vtable, no allocation",
        std::mem::size_of_val(&e)
    );
}
