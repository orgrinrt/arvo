// Probe 03: does unsigned saturating addition satisfy BOTH associativity
// and translation-monotonicity at once, where section 3's proof argues it
// must (a one-sided clamp on a domain bounded below by 0 cannot hit the
// two-sided escape that breaks signed saturating addition's associativity)?
// And does unsigned wraparound still fail monotonicity, confirming the
// torsion-group argument is about the abstract group and not about which
// integers get called negative?
//
// Model: representable [0, 7] (an 8-value unsigned range, UFixed-shaped).
//
// Run: rustc -O 03_unsigned_saturate_both.rs -o /tmp/u3 && /tmp/u3

const LO: i64 = 0;
const HI: i64 = 7;

fn sat(x: i64) -> i64 {
    if x < LO {
        LO
    } else if x > HI {
        HI
    } else {
        x
    }
}

fn wrap(x: i64) -> i64 {
    let range = HI - LO + 1;
    let m = ((x - LO) % range + range) % range;
    m + LO
}

fn assoc(name: &str, f: fn(i64) -> i64) {
    let mut ok = true;
    let mut ex = None;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                let l = f(f(a + b) + c);
                let r = f(a + f(b + c));
                if l != r {
                    ok = false;
                    ex = Some((a, b, c));
                }
            }
        }
    }
    println!("{name} unsigned [{LO},{HI}] associative: {ok} {ex:?}");
}

fn monotone(name: &str, f: fn(i64) -> i64) {
    // is x -> f(x+c) monotone in x, for every representable c? this is the
    // property (D) of section 3 needs: translation by every representable
    // constant is order-preserving.
    let mut ok = true;
    let mut ex = None;
    'outer: for c in LO..=HI {
        for x in LO..=(2 * HI) {
            for y in x..=(2 * HI) {
                if f(x + c) > f(y + c) {
                    ok = false;
                    ex = Some((c, x, y));
                    break 'outer;
                }
            }
        }
    }
    println!("{name} unsigned [{LO},{HI}] translation-monotone: {ok} {ex:?}");
}

fn main() {
    assoc("Saturate", sat);
    assoc("Wrap", wrap);
    monotone("Saturate", sat);
    monotone("Wrap", wrap);
}
