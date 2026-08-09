// Runs p5's six relations and p1's two strategy sweeps, so the sign-domain claim and
// the conversion claim each carry a result rather than only a compile.
//
// rustc +nightly-2026-05-28 --edition 2024 p6_run_checks.rs --out-dir out && ./out/p6_run_checks

#[path = "p1_target_keyed.rs"]
mod conv;
#[path = "p5_domain_derives_range.rs"]
mod dom;

use conv::{source_strategy_is_inert, target_strategy_governs};
use dom::{checks, includes, AsymmetricLow, Fixed, NonNegative, Numeral, Symmetric};

fn main() {
    let names = [
        "Symmetric(4) inside AsymmetricLow(4)",
        "AsymmetricLow(4) inside Symmetric(4)",
        "AsymmetricLow(4) inside Symmetric(5)",
        "Symmetric(5) inside AsymmetricLow(4)",
        "NonNegative(4) inside AsymmetricLow(4)",
        "AsymmetricLow(4) inside NonNegative(4)",
    ];
    println!("-- the six relations the sign-domain question turns on, radix two, one grid");
    for (n, v) in names.iter().zip(checks()) {
        println!("   {n:<40} {v}");
    }
    println!("   forced: Sym <= Asym from row 1, Asym <= Sym from row 3, so antisymmetry");
    println!("   would give Sym = Asym, and their ranges differ, so no order on the three");
    println!("   domains carries the inclusion order.");

    println!("-- endpoints, derived rather than declared");
    print_ends::<Fixed<2, 4, 0, NonNegative>>("NonNegative(4)");
    print_ends::<Fixed<2, 4, 0, Symmetric>>("Symmetric(4)");
    print_ends::<Fixed<2, 4, 0, AsymmetricLow>>("AsymmetricLow(4)");
    print_ends::<Fixed<2, 1, 0, Symmetric>>("Symmetric(1)");
    print_ends::<Fixed<3, 2, 0, Symmetric>>("Symmetric(2) at radix three");
    print_ends::<Fixed<3, 2, 0, AsymmetricLow>>("AsymmetricLow(2) at radix three");

    println!("-- one grid clause check, stated so a reader can see it is not vacuous");
    println!(
        "   NonNegative(4) at exponent 0 inside NonNegative(4) at exponent -1: {}",
        includes::<Fixed<2, 4, 0, NonNegative>, Fixed<2, 4, -1, NonNegative>>()
    );

    println!("-- conversion: four source strategies, everything else held fixed");
    for v in [3i128, 9, 20, -4] {
        println!("   v={v:>4}  {:?}", source_strategy_is_inert(v));
    }
    println!("-- conversion: four target strategies, everything else held fixed");
    for v in [3i128, 9, 20, -4] {
        println!("   v={v:>4}  {:?}", target_strategy_governs(v));
    }
}

fn print_ends<N: Numeral>(label: &str) {
    println!(
        "   {label:<28} codes={:<5} lo={:<5} hi={:<5}",
        N::codes(),
        N::lo(),
        N::hi()
    );
}
