//! Runner for probe 2's whole-matrix sweeps. Counts are asserted, so a vacuous
//! sweep cannot pass.

#[path = "probe_2_quantise_as_a_crossing.rs"]
mod q;

fn main() {
    let (cells, refusals, disagreements) = q::sweep();
    println!("Precise sweep: cells={cells} refusals={refusals} disagreements={disagreements}");
    assert_eq!(cells, 16_000, "the sweep must cover every (x, q) cell");
    assert_eq!(
        refusals, 5_679,
        "per-(x,q) refusals must match probe 1's 5,679 cells (5,679,000 pairs)"
    );
    assert_eq!(
        disagreements, 0,
        "the typed operation must refuse on exactly probe 1's predicate"
    );

    let (wcells, clamped, wrong) = q::sweep_warm();
    println!("Warm sweep: cells={wcells} clamped={clamped} wrong={wrong}");
    assert_eq!(wcells, 16_000);
    assert_eq!(
        clamped, 5_679,
        "Warm must clamp on exactly the cells Precise refuses on"
    );
    assert_eq!(wrong, 0, "Warm must be total and land on the far point");

    let (hcells, hwrapped, hwrong) = q::sweep_hot();
    println!("Hot sweep: cells={hcells} wrapped={hwrapped} wrong={hwrong}");
    assert_eq!(hcells, 16_000);
    assert_eq!(
        hwrapped, 5_679,
        "Hot must reduce modulo on exactly the cells Precise refuses on"
    );
    assert_eq!(
        hwrong, 0,
        "Hot must be total and closed on the target's mantissas"
    );

    // The two tiers differ in carrier size, which is the cost of the refusing tier
    // and the reason the delivery question is worth asking at all.
    println!(
        "size_of Total = {}, size_of Fallible = {}",
        core::mem::size_of::<q::Total>(),
        core::mem::size_of::<q::Fallible>()
    );

    println!("OK: one body, four presets, refusal set identical to the range predicate");
}
