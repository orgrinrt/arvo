// t1. What does each candidate test procedure actually certify?
//
// Method: seed a packed accessor with one plausible implementation defect at a time,
// run every candidate test procedure against it, and record which procedures observe
// the defect. The output is a procedure-by-defect matrix. A procedure whose row is
// empty certifies nothing about layout, whatever it looks like on the page.
//
// The oracle is deliberately dumb: element k occupies bits [k*W, k*W + W), written and
// read one bit at a time. That is the specification restated in the most obvious way
// available, and it is written from the intent rather than from the mechanism. The
// thing under test is the word-load form an implementer would actually write.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O t1_defect_matrix.rs -o bin/t1 && ./bin/t1
//
// Nothing here is timed. No bench harness has run in this panel. Every number below is
// a count of wrong elements or a compile-time size.
//
// Spike. Presume it flawed. In particular the defect list is a list I chose, so the
// matrix's columns are as good as my imagination and no better; the rows are the part
// worth reading.

use std::panic;

// ============================================================================
// The specification. Dumb on purpose.
// ============================================================================

#[derive(Copy, Clone, PartialEq, Debug)]
struct Decl {
    w: usize,
    signed: bool,
}

fn spec_bytes(d: Decl, n: usize) -> usize {
    (n * d.w).div_ceil(8)
}

fn spec_write(buf: &mut [u8], d: Decl, k: usize, v: i64) {
    let base = k * d.w;
    for i in 0..d.w {
        let b = base + i;
        let set = (v >> i) & 1 == 1;
        if set {
            buf[b / 8] |= 1 << (b % 8);
        } else {
            buf[b / 8] &= !(1 << (b % 8));
        }
    }
}

fn spec_read(buf: &[u8], d: Decl, k: usize) -> i64 {
    let base = k * d.w;
    let mut acc: u64 = 0;
    for i in 0..d.w {
        let b = base + i;
        if buf[b / 8] >> (b % 8) & 1 == 1 {
            acc |= 1u64 << i;
        }
    }
    if d.signed && d.w > 0 && (acc >> (d.w - 1)) & 1 == 1 {
        // sign extend from bit w-1
        (acc | (!0u64 << d.w)) as i64
    } else {
        acc as i64
    }
}

// ============================================================================
// The defects. Each is a plausible thing an implementer writes.
// ============================================================================

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Defect {
    None,
    /// access width taken from the carrier rather than from the declared width
    AccessFromCarrier,
    /// the write mask is off by one, so a write spills one bit into the next element
    WriteSpillsOneBit,
    /// stride rounded up to whole bytes: self-consistent, and not packed
    StrideRoundedToBytes,
    /// signed read forgets to sign extend from bit w-1
    NoSignExtend,
    /// phase dropped: the element is assumed byte aligned
    PhaseDropped,
    /// the tail element's access window is not clamped to the buffer
    TailUnguarded,
}

impl Defect {
    fn name(self) -> &'static str {
        match self {
            Defect::None => "none (control)",
            Defect::AccessFromCarrier => "access width from carrier",
            Defect::WriteSpillsOneBit => "write spills one bit",
            Defect::StrideRoundedToBytes => "stride rounded to bytes",
            Defect::NoSignExtend => "no sign extension",
            Defect::PhaseDropped => "phase dropped",
            Defect::TailUnguarded => "tail unguarded",
        }
    }
}

const DEFECTS: [Defect; 7] = [
    Defect::None,
    Defect::AccessFromCarrier,
    Defect::WriteSpillsOneBit,
    Defect::StrideRoundedToBytes,
    Defect::NoSignExtend,
    Defect::PhaseDropped,
    Defect::TailUnguarded,
];

/// the carrier a declaration lowers to, in bytes: next native rung at or above w
fn carrier_bytes(w: usize) -> usize {
    match w {
        0..=8 => 1,
        9..=16 => 2,
        17..=32 => 4,
        33..=64 => 8,
        _ => 16,
    }
}

/// the byte span a w-bit field can occupy at an unknown phase
fn spec_access_bytes(w: usize) -> usize {
    (w + 6) / 8 + 1
}

struct Store {
    buf: Vec<u8>,
    d: Decl,
    defect: Defect,
}

impl Store {
    fn stride(&self) -> usize {
        match self.defect {
            Defect::StrideRoundedToBytes => self.d.w.div_ceil(8) * 8,
            _ => self.d.w,
        }
    }

    fn new(d: Decl, defect: Defect, n: usize, fill: u8) -> Self {
        let mut s = Store {
            buf: Vec::new(),
            d,
            defect,
        };
        let bytes = (n * s.stride()).div_ceil(8);
        s.buf = vec![fill; bytes];
        s
    }

    fn write(&mut self, k: usize, v: i64) {
        let stride = self.stride();
        let base = k * stride;
        // the spill defect writes one extra bit beyond the declared width
        let bits = match self.defect {
            Defect::WriteSpillsOneBit => self.d.w + 1,
            _ => self.d.w,
        };
        for i in 0..bits {
            let b = base + i;
            if b / 8 >= self.buf.len() {
                break;
            }
            let set = (v >> i) & 1 == 1;
            if set {
                self.buf[b / 8] |= 1 << (b % 8);
            } else {
                self.buf[b / 8] &= !(1 << (b % 8));
            }
        }
    }

    fn read(&self, k: usize) -> i64 {
        let stride = self.stride();
        let base = k * stride;
        let byte = base / 8;
        let phase = match self.defect {
            Defect::PhaseDropped => 0,
            _ => base % 8,
        };
        let access = match self.defect {
            Defect::AccessFromCarrier => carrier_bytes(self.d.w),
            _ => spec_access_bytes(self.d.w),
        };
        let mut acc: u64 = 0;
        // Clamping the access window at the buffer end is what a correct implementation
        // must do (or it must over-allocate). Leaving it unclamped is the TailUnguarded
        // defect, and it is a fault rather than a wrong value, so it is what the panic
        // hook in `catches` observes.
        for i in 0..access.min(8) {
            if matches!(self.defect, Defect::TailUnguarded) {
                acc |= (self.buf[byte + i] as u64) << (8 * i);
            } else if byte + i < self.buf.len() {
                acc |= (self.buf[byte + i] as u64) << (8 * i);
            }
        }
        let mask = if self.d.w >= 64 {
            !0u64
        } else {
            (1u64 << self.d.w) - 1
        };
        let raw = (acc >> phase) & mask;
        let sign_ok = !matches!(self.defect, Defect::NoSignExtend);
        if self.d.signed && sign_ok && self.d.w > 0 && (raw >> (self.d.w - 1)) & 1 == 1 {
            (raw | (!0u64 << self.d.w)) as i64
        } else {
            raw as i64
        }
    }
}

// ============================================================================
// The candidate test procedures. Each returns true when it OBSERVES the defect.
// ============================================================================

fn catches(f: impl FnOnce() -> bool + panic::UnwindSafe) -> bool {
    // a procedure that faults has observed something, which counts as catching
    match panic::catch_unwind(f) {
        Ok(v) => v,
        Err(_) => true,
    }
}

/// P1. round trip through the carrier alone: no array anywhere
fn p1_carrier_round_trip(d: Decl, _defect: Defect) -> bool {
    let mask: u64 = if d.w >= 64 { !0 } else { (1u64 << d.w) - 1 };
    for r in 0..=mask.min(4095) {
        let stored = r & mask;
        if stored != r & mask {
            return true;
        }
    }
    false
}

/// P2. size_of agrees with the ladder
fn p2_size_of_matches_ladder(d: Decl, _defect: Defect) -> bool {
    // asks the derivation to agree with itself: the carrier IS the ladder's output
    carrier_bytes(d.w) != carrier_bytes(d.w)
}

/// P3. write the whole run ascending, read it back, with small values
///
/// Note on the data. A plain 0..n counter overflows the declared range at narrow widths
/// (at W = 5 signed the range is -16..=15, and 20 is not representable), so a correct
/// implementation fails it. The natural repair is to bring the counter into range, which
/// is what this does, and the repair is precisely what removes the high bits. So the
/// blindness in the P3 row below is not carelessness: it is what a careful person gets
/// when they fix the obvious problem with the obvious data.
fn p3_ascending_small(d: Decl, defect: Defect) -> bool {
    let n = 64;
    let mut s = Store::new(d, defect, n, 0);
    let hi: i64 = if d.signed {
        if d.w >= 2 {
            1i64 << (d.w - 1)
        } else {
            1
        }
    } else if d.w >= 63 {
        i64::MAX
    } else {
        1i64 << d.w
    };
    let vals: Vec<i64> = (0..n as i64).map(|k| k % hi).collect();
    for (k, &v) in vals.iter().enumerate() {
        s.write(k, v);
    }
    (0..n).any(|k| s.read(k) != vals[k])
}

/// P4. same, with values that fill the declared width
fn p4_ascending_width_filling(d: Decl, defect: Defect) -> bool {
    let n = 64;
    let mut s = Store::new(d, defect, n, 0);
    let vals: Vec<i64> = (0..n).map(|k| fill_value(d, k)).collect();
    for (k, &v) in vals.iter().enumerate() {
        s.write(k, v);
    }
    (0..n).any(|k| s.read(k) != vals[k])
}

/// P5. write ONE element into a poisoned buffer, then check it and both neighbours
fn p5_single_write_poisoned(d: Decl, defect: Defect) -> bool {
    let n = 64;
    for (k, poison) in (1..9usize).flat_map(|k| [(k, 0x00u8), (k, 0xFFu8)]) {
        let mut s = Store::new(d, defect, n, poison);
        // establish the reference: what the whole buffer should look like after
        let stride = s.stride();
        let mut expect = vec![poison; (n * stride).div_ceil(8)];
        let v = fill_value(d, k);
        s.write(k, v);
        // the spec's own write over the same buffer shape
        {
            let base = k * stride;
            for i in 0..d.w {
                let b = base + i;
                let set = (v >> i) & 1 == 1;
                if set {
                    expect[b / 8] |= 1 << (b % 8);
                } else {
                    expect[b / 8] &= !(1 << (b % 8));
                }
            }
        }
        if s.buf != expect {
            return true;
        }
    }
    false
}

/// P6. the aggregate extent assertion: N elements occupy N*W bits
fn p6_aggregate_extent(d: Decl, defect: Defect) -> bool {
    let n = 1000;
    let s = Store::new(d, defect, n, 0);
    s.buf.len() != spec_bytes(d, n)
}

/// P7. values spanning the sign domain, written ascending and read back
fn p7_signed_span(d: Decl, defect: Defect) -> bool {
    if !d.signed {
        return false;
    }
    let n = 64;
    let mut s = Store::new(d, defect, n, 0);
    let vals: Vec<i64> = (0..n).map(|k| signed_span_value(d, k)).collect();
    for (k, &v) in vals.iter().enumerate() {
        s.write(k, v);
    }
    (0..n).any(|k| s.read(k) != vals[k])
}

/// P8. the LAST element of an exactly-sized run
fn p8_tail_element(d: Decl, defect: Defect) -> bool {
    let n = 64;
    let mut s = Store::new(d, defect, n, 0);
    let v = fill_value(d, n - 1);
    s.write(n - 1, v);
    s.read(n - 1) != v
}

/// a value that fills the declared width: high bit set, and a varying low pattern
fn fill_value(d: Decl, k: usize) -> i64 {
    let mask: u64 = if d.w >= 64 { !0 } else { (1u64 << d.w) - 1 };
    let raw = (0xA5A5_5A5Au64.wrapping_mul(k as u64 + 1) | (1u64 << (d.w - 1))) & mask;
    if d.signed && (raw >> (d.w - 1)) & 1 == 1 {
        (raw | (!0u64 << d.w)) as i64
    } else {
        raw as i64
    }
}

/// a value spanning the sign domain: alternates negative and positive
fn signed_span_value(d: Decl, k: usize) -> i64 {
    let mask: u64 = if d.w >= 64 { !0 } else { (1u64 << d.w) - 1 };
    let raw = if k % 2 == 0 {
        (0x3C3Cu64.wrapping_mul(k as u64 + 1) | (1u64 << (d.w - 1))) & mask
    } else {
        (0x3C3Cu64.wrapping_mul(k as u64 + 1)) & mask & !(1u64 << (d.w - 1))
    };
    if (raw >> (d.w - 1)) & 1 == 1 {
        (raw | (!0u64 << d.w)) as i64
    } else {
        raw as i64
    }
}

type Proc = (&'static str, fn(Decl, Defect) -> bool);

const PROCS: [Proc; 8] = [
    ("P1 carrier round trip", p1_carrier_round_trip),
    ("P2 size_of vs ladder", p2_size_of_matches_ladder),
    ("P3 run, ascending, small values", p3_ascending_small),
    (
        "P4 run, ascending, width-filling",
        p4_ascending_width_filling,
    ),
    (
        "P5 one write into poisoned buffer",
        p5_single_write_poisoned,
    ),
    ("P6 aggregate extent N*W", p6_aggregate_extent),
    ("P7 values spanning the sign domain", p7_signed_span),
    ("P8 tail element of an exact run", p8_tail_element),
];

fn run_matrix(d: Decl) {
    println!(
        "\ndeclaration: W = {}, {}",
        d.w,
        if d.signed { "signed" } else { "unsigned" }
    );
    print!("{:36}", "procedure \\ defect");
    for x in DEFECTS.iter().skip(1) {
        print!("{:>10}", short(*x));
    }
    println!("   catches");
    println!("{}", "-".repeat(36 + 10 * (DEFECTS.len() - 1) + 10));

    for (name, f) in PROCS.iter() {
        // the control must be quiet, or the procedure is reporting noise
        let control = catches(move || f(d, Defect::None));
        print!("{:36}", name);
        let mut caught = 0;
        for x in DEFECTS.iter().skip(1) {
            if *x == Defect::NoSignExtend && !d.signed {
                print!("{:>10}", "n/a");
                continue;
            }
            let c = catches(move || f(d, *x));
            print!("{:>10}", if c { "CATCH" } else { "." });
            if c {
                caught += 1;
            }
        }
        println!(
            "{:>10}{}",
            caught,
            if control { "  <- FALSE POSITIVE" } else { "" }
        );
    }
}

fn short(x: Defect) -> &'static str {
    match x {
        Defect::None => "none",
        Defect::AccessFromCarrier => "access",
        Defect::WriteSpillsOneBit => "spill",
        Defect::StrideRoundedToBytes => "stride",
        Defect::NoSignExtend => "signext",
        Defect::PhaseDropped => "phase",
        Defect::TailUnguarded => "tail",
    }
}

// ============================================================================
// Residual analysis. An escape is only a hole if the defect was observable at all.
// ============================================================================

/// Is the defected store observably different from the specification at this
/// declaration, on ANY (index, value, poison) it is exercised with? A defect that is
/// vacuous at a declaration cannot be missed, because there is nothing to miss.
fn observable(d: Decl, defect: Defect) -> bool {
    let n = 64;
    for poison in [0x00u8, 0xFFu8] {
        for k in 0..n {
            for v in candidate_values(d) {
                // buffer state after a single write, against the spec's own write
                let mut s = Store::new(d, defect, n, poison);
                let stride_ok = s.stride() == d.w;
                let mut expect = vec![poison; (n * d.w).div_ceil(8)];
                s.write(k, v);
                if stride_ok {
                    spec_write(&mut expect, d, k, v);
                    if s.buf != expect {
                        return true;
                    }
                } else if s.buf.len() != expect.len() {
                    return true;
                }
                // read back, against the spec's read of the spec's buffer
                let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| s.read(k)));
                match got {
                    Err(_) => return true,
                    Ok(g) => {
                        if stride_ok && g != spec_read(&expect, d, k) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn candidate_values(d: Decl) -> Vec<i64> {
    let mask: u64 = if d.w >= 64 { !0 } else { (1u64 << d.w) - 1 };
    let mut v: Vec<i64> = Vec::new();
    for raw in [
        0u64,
        1,
        mask,
        mask >> 1,
        mask ^ (mask >> 1),
        0xA5A5_A5A5_A5A5_A5A5 & mask,
    ] {
        let x = if d.signed && d.w > 0 && (raw >> (d.w - 1)) & 1 == 1 {
            (raw | (!0u64 << d.w)) as i64
        } else {
            raw as i64
        };
        v.push(x);
    }
    v
}

fn residual_analysis() {
    println!("\nresidual: of the defect instances the full eight-procedure suite does not");
    println!("observe, how many were observable at all?");
    let mut vacuous = 0usize;
    let mut genuinely_missed: Vec<(Decl, Defect)> = Vec::new();
    let mut total = 0usize;
    for w in 1..=48usize {
        for signed in [false, true] {
            let d = Decl { w, signed };
            for x in DEFECTS.iter().skip(1) {
                if *x == Defect::NoSignExtend && !signed {
                    continue;
                }
                total += 1;
                let caught = (0..PROCS.len()).any(|i| catches(move || (PROCS[i].1)(d, *x)));
                if caught {
                    continue;
                }
                if observable(d, *x) {
                    genuinely_missed.push((d, *x));
                } else {
                    vacuous += 1;
                }
            }
        }
    }
    println!("  total defect instances          : {total}");
    println!(
        "  observed by the suite           : {}",
        total - vacuous - genuinely_missed.len()
    );
    println!("  vacuous at that declaration     : {vacuous}");
    println!(
        "  OBSERVABLE AND MISSED           : {}",
        genuinely_missed.len()
    );
    if !genuinely_missed.is_empty() {
        println!("\n  the misses, which are the holes the suite still has:");
        for (d, x) in genuinely_missed.iter().take(40) {
            println!(
                "    W = {:3} {:8}  {}",
                d.w,
                if d.signed { "signed" } else { "unsigned" },
                x.name()
            );
        }
        if genuinely_missed.len() > 40 {
            println!("    ... and {} more", genuinely_missed.len() - 40);
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {}));

    println!("t1. which test procedure observes which packed-layout defect");
    println!("CATCH means the procedure observed the defect. A dot means it did not.");
    println!("The oracle is a bit-at-a-time reference; the thing under test is a word load.");

    run_matrix(Decl {
        w: 13,
        signed: false,
    });
    run_matrix(Decl {
        w: 13,
        signed: true,
    });
    run_matrix(Decl { w: 5, signed: true });

    println!("\nsweep: how many of the five defects each procedure catches, summed over");
    println!("every declaration W = 1..=48 in both sign domains");
    let mut totals = [0usize; PROCS.len()];
    let mut possible = 0usize;
    for w in 1..=48usize {
        for signed in [false, true] {
            let d = Decl { w, signed };
            for x in DEFECTS.iter().skip(1) {
                // a defect that is not applicable to this declaration is not counted
                if *x == Defect::NoSignExtend && !signed {
                    continue;
                }
                possible += 1;
                for (i, (_, f)) in PROCS.iter().enumerate() {
                    if catches(move || f(d, *x)) {
                        totals[i] += 1;
                    }
                }
            }
        }
    }
    println!("\n{:36}{:>10}{:>12}", "procedure", "caught", "of possible");
    println!("{}", "-".repeat(58));
    for (i, (name, _)) in PROCS.iter().enumerate() {
        println!("{:36}{:>10}{:>12}", name, totals[i], possible);
    }

    // the union of the procedures that a person writes without already believing in a
    // second output: P1, P2, P3
    let mut naive_union = 0usize;
    let mut full_union = 0usize;
    for w in 1..=48usize {
        for signed in [false, true] {
            let d = Decl { w, signed };
            for x in DEFECTS.iter().skip(1) {
                if *x == Defect::NoSignExtend && !signed {
                    continue;
                }
                let naive = [0usize, 1, 2]
                    .iter()
                    .any(|&i| catches(move || (PROCS[i].1)(d, *x)));
                let full = (0..PROCS.len()).any(|i| catches(move || (PROCS[i].1)(d, *x)));
                if naive {
                    naive_union += 1;
                }
                if full {
                    full_union += 1;
                }
            }
        }
    }
    println!(
        "\nunion of P1+P2+P3, the suite written without believing in a second output: \
         {naive_union} of {possible}"
    );
    println!("union of all eight procedures: {full_union} of {possible}");

    residual_analysis();
}
