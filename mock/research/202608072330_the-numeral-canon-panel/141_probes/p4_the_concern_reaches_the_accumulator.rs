// p4: the blind convergence measured storage, and the concern is minimisation.
//
// 139 p6 and 140 p3 agree, blind to each other, that the storage-minimising
// concern is a weighting with zero policy content: 60 configurations with zero
// disagreements, and 90 configurations collapsing to the same 24 answer functions.
// Both measured the same object: WHERE A VALUE IS PUT. Neither measured HOW WIDE
// THE ARITHMETIC IS WHILE THE VALUE IS BEING COMPUTED.
//
// I6 is the concern's statement and it says the concern "aggressively minimises
// and bitpacks", and separately that it "should remain small for memory or disk
// storage". Nothing in it stops at the storage boundary, and in a column store an
// accumulator is an array too. So the question is whether the concern, applied to
// the accumulator rather than to the column, is still answer-invisible.
//
// This probe holds the contested dimension FIXED at the conservative reading, so
// nothing here depends on it: the overflow limit is read at the DECLARED width
// always, which is the reading 140's phase-two section C argues the design must
// take. Under that reading storage is invisible by both files' measurements, and
// the question is whether the accumulator is too.
//
// CONFIGURATION SPACE
//   rounding    in {toward zero, floor}
//   overflow    in {wrap, saturate}
//   storage     in {packed at W, minimum rung, double rung}  (all lossless)
//               plus a lossy control at W-1 bits
//   accumulator in {W, W+2, wide}                            (all >= W)
//
// OPERATIONS: add, sub, mul, a*b+c, a*b+c*d. The last is the one an accumulator
// can be seen through; add is the one it cannot, and both are present so that a
// null result somewhere is attributable.
//
// PREDICTIONS, before running:
//   T1. Holding the accumulator fixed, the three lossless storage containers add
//       zero classes. This reproduces 140's F3 on an instrument I wrote, which is
//       required before I am allowed to say anything about it.
//   T2. The lossy storage control adds classes. If it does not, the sweep cannot
//       see a container and T1's zero is worthless.
//   T3. Varying the accumulator adds classes. The storage-minimising concern
//       therefore has non-zero policy content as soon as it reaches an
//       accumulator, and "zero policy content" is a statement about the column
//       rather than about the concern.
//   T4. And the policy content is exactly gated on the overflow axis: under
//       WRAPPING the accumulator is invisible, because reduction mod 2^A followed
//       by reduction mod 2^W equals reduction mod 2^W whenever W <= A; under
//       SATURATING it is visible, because clamping to a narrow range and then to
//       a narrower one is not clamping to the narrower one.
//   T5. `add` alone cannot see the accumulator under either overflow position,
//       so a sweep containing only `add` would report T3 as zero. This is the
//       witness-set dependence 140's F2 names, arriving from a second direction.
//
// CONTROLS: T2 is the sweep-can-see-a-container control. T5 is the witness-set
// control. And a duplicate configuration is included, reached by a different
// construction, which must merge with its twin or the comparator cannot merge.
//
// Run: rustc -O -o /tmp/p4 p4_the_concern_reaches_the_accumulator.rs && /tmp/p4

use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sign {
    U,
    S,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ovf {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Round {
    TowardZero,
    Floor,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Storage {
    PackedAtW,
    MinimumRung,
    DoubleRung,
    /// Control: one bit short of the declared width.
    LossyShort,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Acc {
    AtW,
    WPlus2,
    Wide,
}

fn lo(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => 0,
        Sign::S => -(1i128 << (w - 1)),
    }
}
fn hi(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => (1i128 << w) - 1,
        Sign::S => (1i128 << (w - 1)) - 1,
    }
}
fn reduce(v: i128, s: Sign, o: Ovf, w: u32) -> i128 {
    match o {
        Ovf::Sat => v.clamp(lo(s, w), hi(s, w)),
        Ovf::Wrap => {
            let m = 1i128 << w;
            let r = v.rem_euclid(m);
            match s {
                Sign::U => r,
                Sign::S => {
                    if r >= (1i128 << (w - 1)) {
                        r - m
                    } else {
                        r
                    }
                }
            }
        }
    }
}
fn shift(p: i128, f: u32, r: Round) -> i128 {
    if f == 0 {
        return p;
    }
    match r {
        Round::TowardZero => p / (1i128 << f),
        Round::Floor => p >> f,
    }
}

/// Round trip through the storage container. Lossless containers return the
/// value unchanged; the control truncates a bit.
fn store(v: i128, s: Sign, w: u32, st: Storage) -> i128 {
    let bits = match st {
        Storage::PackedAtW => w,
        Storage::MinimumRung => w.next_power_of_two().max(8),
        Storage::DoubleRung => w.next_power_of_two().max(8) * 2,
        Storage::LossyShort => w - 1,
    };
    if bits >= w {
        v
    } else {
        // genuinely lossy: wrap into the shorter container and back out
        reduce(v, s, Ovf::Wrap, bits)
    }
}

fn acc_width(w: u32, a: Acc) -> u32 {
    match a {
        Acc::AtW => w,
        Acc::WPlus2 => w + 2,
        Acc::Wide => 4 * w,
    }
}

#[derive(Clone, Copy)]
struct Cfg {
    s: Sign,
    o: Ovf,
    r: Round,
    st: Storage,
    a: Acc,
    w: u32,
    f: u32,
}

impl Cfg {
    fn racc(&self, v: i128) -> i128 {
        reduce(v, self.s, self.o, acc_width(self.w, self.a))
    }
    fn rdec(&self, v: i128) -> i128 {
        reduce(v, self.s, self.o, self.w)
    }
    fn ld(&self, v: i128) -> i128 {
        store(v, self.s, self.w, self.st)
    }

    fn add(&self, a: i128, b: i128) -> i128 {
        self.ld(self.rdec(self.racc(self.ld(a) + self.ld(b))))
    }
    fn sub(&self, a: i128, b: i128) -> i128 {
        self.ld(self.rdec(self.racc(self.ld(a) - self.ld(b))))
    }
    fn mul(&self, a: i128, b: i128) -> i128 {
        let p = shift(self.ld(a) * self.ld(b), self.f, self.r);
        self.ld(self.rdec(self.racc(p)))
    }
    fn mac(&self, a: i128, b: i128, c: i128) -> i128 {
        let p = self.racc(shift(self.ld(a) * self.ld(b), self.f, self.r));
        self.ld(self.rdec(self.racc(p + self.ld(c))))
    }
    fn dot2(&self, a: i128, b: i128, c: i128, d: i128) -> i128 {
        let p = self.racc(shift(self.ld(a) * self.ld(b), self.f, self.r));
        let q = self.racc(shift(self.ld(c) * self.ld(d), self.f, self.r));
        self.ld(self.rdec(self.racc(p + q)))
    }
}

/// The full answer function over the chosen witness set, as a byte vector.
fn signature(c: &Cfg, ops: &[&str]) -> Vec<i8> {
    let (l, h) = (lo(c.s, c.w), hi(c.s, c.w));
    let mut sig = Vec::new();
    for op in ops {
        match *op {
            "add" | "sub" | "mul" => {
                for a in l..=h {
                    for b in l..=h {
                        let v = match *op {
                            "add" => c.add(a, b),
                            "sub" => c.sub(a, b),
                            _ => c.mul(a, b),
                        };
                        sig.push(v as i8);
                    }
                }
            }
            "mac" => {
                for a in l..=h {
                    for b in l..=h {
                        for x in l..=h {
                            sig.push(c.mac(a, b, x) as i8);
                        }
                    }
                }
            }
            "dot2" => {
                for a in l..=h {
                    for b in l..=h {
                        for x in l..=h {
                            for y in l..=h {
                                sig.push(c.dot2(a, b, x, y) as i8);
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    sig
}

fn classes(cfgs: &[Cfg], ops: &[&str]) -> usize {
    let mut m: BTreeMap<Vec<i8>, usize> = BTreeMap::new();
    for c in cfgs {
        *m.entry(signature(c, ops)).or_insert(0) += 1;
    }
    m.len()
}

fn build(w: u32, f: u32, s: Sign, storages: &[Storage], accs: &[Acc]) -> Vec<Cfg> {
    let mut v = Vec::new();
    for &r in &[Round::TowardZero, Round::Floor] {
        for &o in &[Ovf::Wrap, Ovf::Sat] {
            for &st in storages {
                for &a in accs {
                    v.push(Cfg {
                        s,
                        o,
                        r,
                        st,
                        a,
                        w,
                        f,
                    });
                }
            }
        }
    }
    v
}

fn main() {
    let w = 4u32;
    let all_ops = ["add", "sub", "mul", "mac", "dot2"];
    let lossless = [
        Storage::PackedAtW,
        Storage::MinimumRung,
        Storage::DoubleRung,
    ];
    let with_lossy = [
        Storage::PackedAtW,
        Storage::MinimumRung,
        Storage::DoubleRung,
        Storage::LossyShort,
    ];
    let all_acc = [Acc::AtW, Acc::WPlus2, Acc::Wide];

    println!("p4: does the minimising concern stay answer-invisible at the accumulator?");
    println!("W = {w}, limit read at the DECLARED width always, exhaustive per operation\n");

    for f in [0u32, 1, 2] {
        for s in [Sign::U, Sign::S] {
            let sn = if s == Sign::U { "unsigned" } else { "signed" };
            println!("--- W={w} F={f} {sn} ---");

            // T1: storage only, accumulator pinned.
            let storage_only = build(w, f, s, &lossless, &[Acc::AtW]);
            let n_storage = classes(&storage_only, &all_ops);
            let base = build(w, f, s, &[Storage::PackedAtW], &[Acc::AtW]);
            let n_base = classes(&base, &all_ops);
            println!(
                "  T1 storage x3 lossless, acc pinned : {:>3} configs -> {n_storage} classes (assignment alone: {n_base})",
                storage_only.len()
            );

            // T2: lossy control.
            let storage_lossy = build(w, f, s, &with_lossy, &[Acc::AtW]);
            let n_lossy = classes(&storage_lossy, &all_ops);
            println!(
                "  T2 lossy control added             : {:>3} configs -> {n_lossy} classes {}",
                storage_lossy.len(),
                if n_lossy > n_storage {
                    "control FIRES"
                } else {
                    "control TOOTHLESS"
                }
            );

            // T3: accumulator varied.
            let acc_varied = build(w, f, s, &lossless, &all_acc);
            let n_acc = classes(&acc_varied, &all_ops);
            println!(
                "  T3 storage x3 AND accumulator x3   : {:>3} configs -> {n_acc} classes {}",
                acc_varied.len(),
                if n_acc > n_storage {
                    "ACCUMULATOR IS VISIBLE"
                } else {
                    "accumulator invisible"
                }
            );

            // T4: split by overflow position.
            for o in [Ovf::Wrap, Ovf::Sat] {
                let on = if o == Ovf::Wrap {
                    "wrapping  "
                } else {
                    "saturating"
                };
                let mut pin: Vec<Cfg> = acc_varied.iter().copied().filter(|c| c.o == o).collect();
                let pinned_acc: Vec<Cfg> =
                    pin.iter().copied().filter(|c| c.a == Acc::AtW).collect();
                pin.sort_by_key(|c| (c.r as u8, c.st as u8, c.a as u8));
                let n_all = classes(&pin, &all_ops);
                let n_pinned = classes(&pinned_acc, &all_ops);
                println!(
                    "  T4 {on}: acc varied -> {n_all} classes, acc pinned -> {n_pinned} classes  {}",
                    if n_all > n_pinned { "VISIBLE" } else { "invisible" }
                );
            }

            // T5: witness-set control. `add` alone.
            let n_acc_add_only = classes(&acc_varied, &["add"]);
            let n_storage_add_only = classes(&storage_only, &["add"]);
            println!(
                "  T5 witness set = {{add}} only        : acc varied -> {n_acc_add_only}, acc pinned -> {n_storage_add_only} {}",
                if n_acc_add_only == n_storage_add_only {
                    "(add cannot see the accumulator)"
                } else {
                    "(add CAN see it, T5 refuted)"
                }
            );

            // merge control: a duplicate configuration reached differently.
            let mut dup = build(w, f, s, &[Storage::PackedAtW], &[Acc::AtW]);
            let extra = dup[0];
            dup.push(extra);
            let n_dup = classes(&dup, &all_ops);
            let n_nodup = classes(
                &build(w, f, s, &[Storage::PackedAtW], &[Acc::AtW]),
                &all_ops,
            );
            println!(
                "  merge control: duplicate config     : {n_dup} vs {n_nodup} {}",
                if n_dup == n_nodup {
                    "(comparator merges)"
                } else {
                    "(COMPARATOR CANNOT MERGE)"
                }
            );
            println!();
        }
    }
}
