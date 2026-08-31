// p4: on the pinned nightly, what can a const generic width surface actually
// carry without an unstable feature reaching the consumer?
//
// WHY THIS RUNS. `question::the_width_surface_crossing` is the one place seat
// 221 and seat 222 differ on a call rather than on a fact. 221 took option 6,
// the literal in the type with the structural nat demoted to a hidden
// projection; 222 took option 5, the alias carrying the const and the algebra
// keyed on nats. Both files name the same thing as what separates them, the
// price of crossing back, and both say they did not measure it. Both also flag
// the row as their least certain.
//
// Before that price is worth measuring, option 6 has to be available at all,
// and two retirement rows say the constructions it needs were refused:
// `retirement::dl_width_arithmetic_as_a_const_generic` and
// `retirement::dl_const_generic_width_comparison_in_a_where_clause`. Neither
// says which toolchain refused them. This probe asks the pinned one.
//
// THE BAR IS NOT "DOES IT COMPILE". It is
// `obligation::the_unstable_machinery_does_not_reach_a_consumer`: "A consumer
// naming arvo's types needs no unstable compiler features, no nightly-only
// attributes and no feature gates of its own, whatever arvo uses internally to
// build them." So every arm below is compiled TWICE, once with the feature
// gate and once without, and only the ungated column decides anything.
//
// THE ARMS.
//   A1  a const generic parameter used as a value in a body                 (baseline)
//   A2  arithmetic on const generic widths in a TYPE position, W + F
//   A3  a comparison of const generic widths in a where clause, W >= F
//   A4  arithmetic in an associated const, then used as a value
//   A5  an associated const of a trait used as a const generic argument
//   A6  a consumer-facing type alias fixing the const, which is option 5's
//       shape, so the two arrangements are compared on one instrument
//   A7  a const assertion in an associated const, which is the option 7 route:
//       declare the output width and check it is wide enough
//
// WHAT MUST FAIL, declared before the run.
//   C1  A1 must compile ungated. If the baseline needs a feature the toolchain
//       is not the one the pin names and nothing below means anything.
//   C2  At least one arm must FAIL ungated. If everything compiles without a
//       gate then the retirement rows are describing a toolchain nobody is on
//       and this probe has not found the boundary, it has found no boundary.
//   C3  At least one arm that fails ungated must SUCCEED gated. Without that,
//       a failure is a fact about the arm's code rather than about the feature
//       gate, and the ungated column would not be measuring what it claims.
//   C4  A deliberately broken arm must fail in BOTH columns, so a success is
//       not the compiler accepting anything put in front of it.
//   C5  A7's assertion must REFUSE a violating instantiation. Without this,
//       "A7 compiles" says only that the assertion never fired, which is what
//       every vacuous check says about itself. The first run of this probe had
//       A7 compiling and no arm proving the assertion does anything, and that
//       run is kept as
//       `p4_v1_a7_compiled_with_no_arm_proving_the_assertion_fires.out`.
//
// C5 THEN VOIDED THE RUN AND THE VOID IS THE FINDING. Under `--emit metadata`
// the violating instantiation compiled, because metadata-only emission does
// not monomorphise and a post-monomorphisation assertion never runs. That run
// is kept as `p4_v2_c5_voided_under_metadata_only_emission.out`. The repair is
// to compile every arm under BOTH emissions, because the difference is not an
// artifact of the probe: `cargo check` emits metadata and `cargo build`
// codegens, so a check that only fires under codegen is a check a consumer's
// own `cargo check` does not run, and that is a property of option 7 rather
// than of this instrument.
//
// SCOPE. One toolchain, the pinned nightly. Edition 2021. Seven arms plus one
// broken control, each a whole crate compiled on its own.

use std::io::Write;
use std::process::Command;

struct Arm {
    name: &'static str,
    what: &'static str,
    feature: &'static str,
    src: &'static str,
}

const ARMS: &[Arm] = &[
    Arm {
        name: "A1",
        what: "const generic as a value in a body (baseline)",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize>;
impl<const W: usize> N<W> {
    pub const fn width(&self) -> usize { W }
}
pub fn use_it(n: &N<13>) -> usize { n.width() }
"#,
    },
    Arm {
        name: "A2",
        what: "arithmetic on const widths in a TYPE position: N<{W + F}>",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize>;
pub fn widen<const W: usize, const F: usize>(_a: N<W>, _b: N<F>) -> N<{ W + F }> { N }
"#,
    },
    Arm {
        name: "A3",
        what: "comparison of const widths in a where clause: W >= F",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize>;
pub struct Check<const B: bool>;
pub trait True {}
impl True for Check<true> {}
pub fn narrow<const W: usize, const F: usize>(_a: N<W>) -> N<F>
where
    Check<{ W >= F }>: True,
{
    N
}
"#,
    },
    Arm {
        name: "A4",
        what: "arithmetic in an associated const, used as a value",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize, const F: usize>;
impl<const W: usize, const F: usize> N<W, F> {
    pub const INT_BITS: usize = W - F;
    pub const fn int_bits(&self) -> usize { Self::INT_BITS }
}
pub fn use_it(n: &N<13, 4>) -> usize { n.int_bits() }
"#,
    },
    Arm {
        name: "A5",
        what: "an associated const used as a const generic argument",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize>;
pub trait HasWidth { const W: usize; }
pub struct Thirteen;
impl HasWidth for Thirteen { const W: usize = 13; }
pub fn from_trait<T: HasWidth>() -> N<{ T::W }> { N }
"#,
    },
    Arm {
        name: "A6",
        what: "a consumer-facing alias fixing the const (option 5's shape)",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize, const F: usize>;
pub type Q13_4 = N<13, 4>;
pub type Sample = N<14, 12>;
pub fn use_it(_a: Q13_4, _b: Sample) {}
"#,
    },
    Arm {
        name: "A7",
        what: "a const assertion in an associated const (option 7's shape)",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize, const F: usize>;
impl<const W: usize, const F: usize> N<W, F> {
    pub const WIDE_ENOUGH: () = assert!(W >= F, "declared width is narrower than its fraction");
}
pub fn declare<const W: usize, const F: usize>() -> N<W, F> {
    let () = N::<W, F>::WIDE_ENOUGH;
    N
}
pub fn ok() -> N<13, 4> { declare::<13, 4>() }
"#,
    },
    Arm {
        name: "C5",
        what: "option 7's assertion on a VIOLATING width (must be refused)",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize, const F: usize>;
impl<const W: usize, const F: usize> N<W, F> {
    pub const WIDE_ENOUGH: () = assert!(W >= F, "declared width is narrower than its fraction");
}
pub fn declare<const W: usize, const F: usize>() -> N<W, F> {
    let () = N::<W, F>::WIDE_ENOUGH;
    N
}
pub fn bad() -> N<4, 13> { declare::<4, 13>() }
"#,
    },
    Arm {
        name: "C4",
        what: "a deliberately broken arm (must fail in both columns)",
        feature: "generic_const_exprs",
        src: r#"
pub struct N<const W: usize>;
pub fn broken() -> N<13> { this_function_does_not_exist() }
"#,
    },
];

#[derive(Clone, Copy, PartialEq)]
enum Emit {
    Metadata,
    Codegen,
}

fn compile(dir: &std::path::Path, arm: &Arm, gated: bool, emit: Emit) -> (bool, String) {
    let tag = if emit == Emit::Codegen { "cg" } else { "md" };
    let path = dir.join(format!(
        "{}_{}_{tag}.rs",
        arm.name,
        if gated { "gated" } else { "ungated" }
    ));
    let mut f = std::fs::File::create(&path).unwrap();
    if gated {
        writeln!(f, "#![feature({})]", arm.feature).unwrap();
        writeln!(f, "#![allow(incomplete_features)]").unwrap();
    }
    writeln!(f, "#![allow(dead_code, unused_variables)]").unwrap();
    f.write_all(arm.src.as_bytes()).unwrap();
    drop(f);

    let out = Command::new(std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into()))
        .arg("+nightly-2026-05-28")
        .arg("--edition")
        .arg("2021")
        .arg("--crate-type")
        .arg("rlib")
        .arg("--emit")
        .arg(if emit == Emit::Codegen { "link" } else { "metadata" })
        .arg("-o")
        .arg(dir.join(format!(
            "{}_{}_{}.out",
            arm.name,
            gated,
            if emit == Emit::Codegen { "cg" } else { "md" }
        )))
        .arg(&path)
        .output()
        .expect("rustc did not run");
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    let first = err
        .lines()
        .find(|l| l.starts_with("error"))
        .unwrap_or("")
        .to_string();
    (out.status.success(), first)
}

fn main() {
    let dir = std::env::temp_dir().join(format!("arvo-221-p4-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();

    println!("### p4. what a const generic width surface carries on the pinned nightly");
    println!("### the bar is the ungated column: `the_unstable_machinery_does_not_reach_a_consumer`");
    println!();

    // Four cells per arm: gated or not, crossed with which emission.
    let mut results = Vec::new();
    for arm in ARMS {
        let (umd, umde) = compile(&dir, arm, false, Emit::Metadata);
        let (ucg, ucge) = compile(&dir, arm, false, Emit::Codegen);
        let (gmd, _) = compile(&dir, arm, true, Emit::Metadata);
        let (gcg, _) = compile(&dir, arm, true, Emit::Codegen);
        let ue = if umde.is_empty() { ucge.clone() } else { umde };
        // The ungated verdict is the strict one: refused if either emission
        // refuses, because a consumer runs both.
        results.push((arm, umd && ucg, ue, gmd && gcg, umd, ucg));
    }

    println!(
        "{:<4} {:<52} {:<11} {:<11} {:<9}",
        "arm", "what", "ungated cc", "ungated bld", "gated"
    );
    for (arm, _, _, g, umd, ucg) in &results {
        println!(
            "{:<4} {:<52} {:<11} {:<11} {:<9}",
            arm.name,
            arm.what,
            if *umd { "compiles" } else { "REFUSED" },
            if *ucg { "compiles" } else { "REFUSED" },
            if *g { "compiles" } else { "REFUSED" }
        );
    }
    println!();
    println!("  `ungated cc` is `--emit metadata`, what `cargo check` runs.");
    println!("  `ungated bld` is `--emit link`, what `cargo build` runs.");
    println!("  a row differing between them is a check a consumer's `cargo check` misses.");
    println!();

    let find = |n: &str| results.iter().find(|(a, ..)| a.name == n).unwrap();
    let (_, a1u, ..) = find("A1");
    let (_, c4u, _, c4g, _, _) = find("C4");
    let control_names = ["C4", "C5"];
    let any_ungated_failure = results
        .iter()
        .any(|(a, u, ..)| !control_names.contains(&a.name) && !*u);
    let gate_rescues = results
        .iter()
        .any(|(a, u, _, g, ..)| !control_names.contains(&a.name) && !*u && *g);
    let (_, c5u, _, c5g, c5md, c5cg) = find("C5");

    let mut void = false;
    println!("CONTROLS");
    for (label, ok, req) in [
        ("C1  the baseline compiles ungated", *a1u, "compiles"),
        ("C2  some arm is refused ungated", any_ungated_failure, "a refusal"),
        ("C3  some ungated refusal is rescued by the gate", gate_rescues, "rescued"),
        ("C4  the broken arm fails in both columns", !*c4u && !*c4g, "refused twice"),
        ("C5  option 7's assertion refuses a violating width", !*c5u && !*c5g, "refused twice"),
        ("C5b it is refused under codegen specifically", !*c5cg, "refused"),
    ] {
        println!(
            "  {label:<50} {:>11}  required={req}",
            if ok { "as required" } else { "*** VOID ***" }
        );
        void |= !ok;
    }
    println!();
    if void {
        println!("*** A CONTROL DID NOT REPORT ITS REQUIRED VERDICT. NOTHING ABOVE COUNTS. ***");
        std::process::exit(1);
    }

    println!("THE FIRST DIAGNOSTIC OF EACH UNGATED REFUSAL");
    for (arm, u, ue, ..) in &results {
        if !*u {
            println!("  {:<4} {}", arm.name, ue);
        }
    }
    println!();
    println!("THE EMISSION SPLIT, WHICH IS THE PART A DESIGN HAS TO KNOW");
    for (arm, _, _, _, umd, ucg) in &results {
        if umd != ucg {
            println!(
                "  {:<4} `cargo check` {}, `cargo build` {}",
                arm.name,
                if *umd { "accepts" } else { "refuses" },
                if *ucg { "accepts" } else { "refuses" }
            );
        }
    }
    if !results.iter().any(|(_, _, _, _, umd, ucg)| umd != ucg) {
        println!("  none: every arm agrees across the two emissions");
    }
    println!();
    println!("WHAT A CONSUMER MAY WRITE WITH NO FEATURE OF ITS OWN");
    for (arm, u, ..) in &results {
        if *u && !control_names.contains(&arm.name) {
            println!("  {:<4} {}", arm.name, arm.what);
        }
    }

    std::fs::remove_dir_all(&dir).ok();
}
