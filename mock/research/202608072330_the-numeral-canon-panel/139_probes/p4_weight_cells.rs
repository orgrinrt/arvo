// p4: the weighting half of a strategy is a continuum. How much of it is
// OBSERVABLE, and can the observable part be named once for all targets?
//
// A weighting is a vector over cost coordinates. Vectors are continuous, so
// there are uncountably many. But a weighting is only ever USED to pick one
// arm out of a finite set, so its entire observable content is the selection
// function it induces, and a linear objective over a finite point set has
// finitely many distinct argmins. The weight space partitions into cells, one
// per arm that can ever win, and two weightings in the same cell are the same
// strategy in every way anything can detect.
//
// Two questions follow, and they are the ones the design needs answered:
//   how many cells are there, and
//   is the cell structure the same on two different targets?
//
// PREDICTIONS, recorded before the first run:
//   S1 the number of distinct winners is strictly less than the number of
//      arms, because some arms are dominated and can never win.
//   S2 the number of distinct winners equals the number of arms on the lower
//      Pareto frontier. (I expect this to be WRONG in one direction: an arm
//      can be Pareto-nondominated and still never win a LINEAR objective, if
//      it sits inside the convex hull of two others. So I expect
//      winners <= pareto, with strict inequality somewhere.)
//   S3 the two cost tables produce different cell counts, so the cell
//      structure is target-dependent and cannot be named once in a document.
//   S4 the three named concerns that ARE weightings land in fewer cells than
//      exist, so naming them does not cover the space.
//
// CONTROLS:
//   C1 a strictly dominated arm must win ZERO times. If it wins, the argmin is
//      broken and every count here is noise.
//   C2 an arm deliberately constructed to win only in a narrow band must win at
//      least once. If it wins zero times, the grid is too coarse and the cell
//      count is an undercount rather than a count.
//   C3 an exact duplicate of an existing arm must win zero times under
//      index-order tie-breaking, so duplicates cannot inflate the cell count.

#[derive(Clone, Copy)]
struct Arm {
    name: &'static str,
    // cost coordinates, normalised to comparable units:
    //   0 = time, 1 = code bytes, 2 = data bytes
    c: [f64; 3],
}

fn argmin(arms: &[Arm], w: [f64; 3]) -> usize {
    let mut best = 0usize;
    let mut bestv = f64::INFINITY;
    for (i, a) in arms.iter().enumerate() {
        let v = w[0] * a.c[0] + w[1] * a.c[1] + w[2] * a.c[2];
        // strict less-than means ties go to the lower index, so a duplicate
        // arm placed after its twin can never win.
        if v < bestv {
            bestv = v;
            best = i;
        }
    }
    best
}

fn dominated(arms: &[Arm], i: usize) -> bool {
    arms.iter().enumerate().any(|(j, b)| {
        j != i
            && (0..3).all(|k| b.c[k] <= arms[i].c[k])
            && (0..3).any(|k| b.c[k] < arms[i].c[k])
    })
}

// Sweep the 2-simplex at resolution 1/n.
fn sweep(arms: &[Arm], n: usize) -> Vec<u64> {
    let mut wins = vec![0u64; arms.len()];
    for i in 0..=n {
        for j in 0..=(n - i) {
            let k = n - i - j;
            let w = [i as f64 / n as f64, j as f64 / n as f64, k as f64 / n as f64];
            wins[argmin(arms, w)] += 1;
        }
    }
    wins
}

fn report(label: &str, arms: &[Arm], n: usize, failures: &mut usize) -> usize {
    let wins = sweep(arms, n);
    let winners: Vec<usize> = (0..arms.len()).filter(|&i| wins[i] > 0).collect();
    let pareto: Vec<usize> = (0..arms.len()).filter(|&i| !dominated(arms, i)).collect();
    println!("{label}");
    println!(
        "  arms={} pareto-nondominated={} distinct winners (cells)={}  grid points={}",
        arms.len(),
        pareto.len(),
        winners.len(),
        (n + 1) * (n + 2) / 2
    );
    for (i, a) in arms.iter().enumerate() {
        println!(
            "    {:<22} cost=[{:>6.1},{:>6.1},{:>6.1}]  wins={:<7} {}{}",
            a.name,
            a.c[0],
            a.c[1],
            a.c[2],
            wins[i],
            if dominated(arms, i) { "DOMINATED " } else { "" },
            if wins[i] == 0 { "never optimal" } else { "" }
        );
    }
    // C1
    let dom_wins: u64 = (0..arms.len())
        .filter(|&i| dominated(arms, i))
        .map(|i| wins[i])
        .sum();
    if dom_wins == 0 {
        println!("  C1 dominated arms win zero times: PASS");
    } else {
        println!("  C1 dominated arms win {dom_wins} times: FAIL");
        *failures += 1;
    }
    // C2
    let narrow = arms.iter().position(|a| a.name.contains("narrow-band"));
    if let Some(ix) = narrow {
        if wins[ix] > 0 {
            println!(
                "  C2 the narrow-band arm wins {} times, so the grid resolves it: PASS",
                wins[ix]
            );
        } else {
            println!("  C2 the narrow-band arm never wins: FAIL, grid too coarse");
            *failures += 1;
        }
    }
    // C3
    let dup = arms.iter().position(|a| a.name.contains("[dup]"));
    if let Some(ix) = dup {
        if wins[ix] == 0 {
            println!("  C3 the duplicate arm wins zero times: PASS");
        } else {
            println!("  C3 the duplicate arm wins {} times: FAIL", wins[ix]);
            *failures += 1;
        }
    }
    winners.len()
}

fn main() {
    let mut failures = 0usize;

    // Target 1: a machine with a wide multiplier and cheap wide loads. The
    // packed arm pays real unpack time; the widened arm pays real data.
    let t1 = [
        Arm { name: "scalar-widened", c: [10.0, 40.0, 32.0] },
        Arm { name: "packed", c: [26.0, 90.0, 13.0] },
        Arm { name: "packed-simd", c: [14.0, 240.0, 13.0] },
        Arm { name: "table-lookup", c: [8.0, 60.0, 512.0] },
        Arm { name: "narrow-band-compromise", c: [15.5, 55.0, 20.0] },
        Arm { name: "naive-loop", c: [40.0, 95.0, 32.0] },
        Arm { name: "scalar-widened [dup]", c: [10.0, 40.0, 32.0] },
    ];

    // Target 2: the same arms on a machine with no wide multiplier and an
    // expensive instruction cache. Only the cost table changed; the arms and
    // their semantics are identical.
    let t2 = [
        Arm { name: "scalar-widened", c: [34.0, 40.0, 32.0] },
        Arm { name: "packed", c: [30.0, 90.0, 13.0] },
        Arm { name: "packed-simd", c: [11.0, 240.0, 13.0] },
        Arm { name: "table-lookup", c: [9.0, 60.0, 512.0] },
        Arm { name: "narrow-band-compromise", c: [15.5, 55.0, 20.0] },
        Arm { name: "naive-loop", c: [40.0, 95.0, 32.0] },
        Arm { name: "scalar-widened [dup]", c: [34.0, 40.0, 32.0] },
    ];

    let n = 400;
    let c1 = report("TARGET 1 (wide multiplier, cheap wide loads)", &t1, n, &mut failures);
    println!();
    let c2 = report("TARGET 2 (no wide multiplier, expensive icache)", &t2, n, &mut failures);

    println!();
    if c1 != c2 {
        println!("  S3 the cell count differs between targets ({c1} vs {c2}): the observable weighting");
        println!("     structure is target-dependent and cannot be fixed in a document");
    } else {
        println!("  S3 the cell counts coincide ({c1}): no target-dependence found in this pair");
    }

    println!();
    println!("WHERE THE NAMED CONCERNS LAND");
    println!("  Only three of the four concerns are points on this simplex at all.");
    println!("  The accuracy-first concern is not a weighting: it asks for a different");
    println!("  ANSWER, not a cheaper route to the same one, so it has no weight vector.");
    let named: [(&str, [f64; 3]); 3] = [
        ("speed-first          w=(1,0,0)", [1.0, 0.0, 0.0]),
        ("storage-minimising   w=(0,0,1)", [0.0, 0.0, 1.0]),
        ("balanced             w=(1/3,1/3,1/3)", [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]),
    ];
    for (label, tbl) in [("target 1", &t1[..]), ("target 2", &t2[..])] {
        let mut hit = Vec::new();
        for (nm, w) in named {
            let ix = argmin(tbl, w);
            println!("  {label}  {nm} -> {}", tbl[ix].name);
            if !hit.contains(&ix) {
                hit.push(ix);
            }
        }
        let total = (0..tbl.len())
            .filter(|&i| sweep(tbl, 60)[i] > 0)
            .count();
        println!(
            "  {label}  the three named weightings reach {} of {} cells",
            hit.len(),
            total
        );
        if hit.len() < total {
            println!("  {label}  S4 holds here: naming three points does not cover the space");
        } else {
            println!("  {label}  S4 fails here: the named points happen to cover every cell");
        }
    }

    println!();
    println!("S3 RESTATED. The cell COUNT coinciding refutes S3 as I wrote it. The claim");
    println!("that survives is about the MAPPING, so it is tested rather than assumed:");
    {
        let mut moved = 0;
        let mut same = 0;
        let n2 = 60;
        for i in 0..=n2 {
            for j in 0..=(n2 - i) {
                let k = n2 - i - j;
                let w = [
                    i as f64 / n2 as f64,
                    j as f64 / n2 as f64,
                    k as f64 / n2 as f64,
                ];
                if t1[argmin(&t1, w)].name == t2[argmin(&t2, w)].name {
                    same += 1;
                } else {
                    moved += 1;
                }
            }
        }
        println!(
            "  the same weight vector selects a DIFFERENT arm at {moved} of {} grid points ({:.1}%)",
            moved + same,
            100.0 * moved as f64 / (moved + same) as f64
        );
    }

    println!();
    println!("S2 WITNESS: a Pareto-optimal arm that NO linear weighting can select.");
    println!("  Two coordinates, three arms. The compromise arm is dominated by neither");
    println!("  endpoint, and sits above the line joining them, so every weight vector");
    println!("  prefers an endpoint. If the weighting is linear, this arm is unreachable.");
    {
        let hull = [
            Arm { name: "endpoint-A", c: [0.0, 10.0, 0.0] },
            Arm { name: "endpoint-B", c: [10.0, 0.0, 0.0] },
            Arm { name: "compromise-C", c: [6.0, 6.0, 0.0] },
        ];
        let n3 = 2000;
        let mut wins = [0u64; 3];
        for i in 0..=n3 {
            let w = [i as f64 / n3 as f64, 1.0 - i as f64 / n3 as f64, 0.0];
            wins[argmin(&hull, w)] += 1;
        }
        for (i, a) in hull.iter().enumerate() {
            println!(
                "    {:<14} cost=[{:>4.1},{:>4.1}] pareto-nondominated={} wins={}",
                a.name,
                a.c[0],
                a.c[1],
                !dominated(&hull, i),
                wins[i]
            );
        }
        if !dominated(&hull, 2) && wins[2] == 0 {
            println!("    S2 strict inequality witnessed: nondominated and never selectable: PASS");
        } else {
            println!("    S2 not witnessed here: FAIL");
            failures += 1;
        }
        // control: the same arm made genuinely better MUST become selectable,
        // otherwise the zero above is a property of the sweep and not the arm.
        let hull2 = [
            Arm { name: "endpoint-A", c: [0.0, 10.0, 0.0] },
            Arm { name: "endpoint-B", c: [10.0, 0.0, 0.0] },
            Arm { name: "compromise-C-improved", c: [4.0, 4.0, 0.0] },
        ];
        let mut wins2 = [0u64; 3];
        for i in 0..=n3 {
            let w = [i as f64 / n3 as f64, 1.0 - i as f64 / n3 as f64, 0.0];
            wins2[argmin(&hull2, w)] += 1;
        }
        if wins2[2] > 0 {
            println!(
                "    control: pulling the same arm inside the hull makes it win {} times: PASS",
                wins2[2]
            );
        } else {
            println!("    control: the improved arm still never wins: FAIL, the sweep is broken");
            failures += 1;
        }
    }

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
