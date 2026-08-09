// Probe 3: a value-keyed digest cannot take the tier-1 free shortcut, categorically, regardless
// of the column's mutation history. Two datum-distinct, value-equal carriers (a NaN-payload
// pair) must digest identically under a value-keyed pairing, so the digest must consume the
// class-collapsed canonical datum (Encoding::Canonical, the V -> D projection), which 82:536-539
// already classes as genuinely data-dependent and therefore not a const-position site: no mask
// undoes it, because it is not a padding tier, it is a real per-element decision.

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

// model datum: a decimal-shaped cohort member (mantissa, exponent) or a NaN payload.
#[derive(Copy, Clone)]
enum D {
    Finite { mantissa: u16, exponent: i8 },
    Nan { payload: u16 },
}

fn value_of(d: D) -> Option<i64> {
    match d {
        D::Finite { mantissa, exponent } => {
            let m = mantissa as i64;
            if exponent >= 0 {
                Some(m * 10i64.pow(exponent as u32))
            } else {
                let p = 10i64.pow((-exponent) as u32);
                if m % p == 0 {
                    Some(m / p)
                } else {
                    None
                } // model: only exact quotients compared here
            }
        }
        D::Nan { .. } => None, // NaN carries no value; every NaN payload is one value
    }
}

// Encoding::Canonical, modelled: NaN collapses to one representative; a finite cohort collapses
// to its preferred (here: smallest-exponent) member. This is the genuinely data-dependent step.
fn canonical(d: D) -> D {
    match d {
        D::Nan { .. } => D::Nan { payload: 0 },
        D::Finite { mantissa, exponent } => {
            let mut m = mantissa;
            let mut e = exponent;
            while m % 10 == 0 && m != 0 {
                m /= 10;
                e += 1;
            }
            D::Finite {
                mantissa: m,
                exponent: e,
            }
        }
    }
}

fn digest_bits(d: D) -> [u8; 3] {
    match d {
        D::Finite { mantissa, exponent } => {
            let mut buf = [0u8; 3];
            buf[0..2].copy_from_slice(&mantissa.to_le_bytes());
            buf[2] = exponent as u8;
            buf
        }
        D::Nan { payload } => {
            let mut buf = [0u8; 3];
            buf[0] = 0xFF;
            buf[1..3].copy_from_slice(&payload.to_le_bytes());
            buf
        }
    }
}

// datum-keyed digest: no canonicalisation, raw bits.
fn digest_datum(d: D) -> u64 {
    fnv1a(&digest_bits(d))
}

// value-keyed digest: MUST canonicalise first. No mask can substitute for this step.
fn digest_value(d: D) -> u64 {
    fnv1a(&digest_bits(canonical(d)))
}

fn main() {
    // Two data denoting the same value: 120 at exponent -1 (mantissa 1200, exp -1) and
    // 120 at exponent 0 directly (mantissa 120, exp 0). Same value, different cohort member.
    let a = D::Finite {
        mantissa: 1200,
        exponent: -1,
    };
    let b = D::Finite {
        mantissa: 120,
        exponent: 0,
    };
    assert_eq!(
        value_of(a),
        value_of(b),
        "same value, different cohort member"
    );
    assert_ne!(
        digest_datum(a),
        digest_datum(b),
        "datum-keyed digest correctly separates two distinct data even though they share a value"
    );
    assert_eq!(
        digest_value(a),
        digest_value(b),
        "value-keyed digest agrees: the canonicalisation step, not a mask, is what makes it agree"
    );

    // Two NaN payloads, one value at the value layer.
    let n1 = D::Nan { payload: 7 };
    let n2 = D::Nan { payload: 200 };
    assert_ne!(
        digest_datum(n1),
        digest_datum(n2),
        "datum-keyed separates distinct NaN payloads"
    );
    assert_eq!(
        digest_value(n1),
        digest_value(n2),
        "value-keyed collapses NaN payloads to one value"
    );

    // The categorical point: unlike padding, canonicalisation reads the datum's own content
    // (the mantissa's trailing zeros, the NaN discriminant) to decide what to do. A digest that
    // tried to get this "for free" by masking a fixed bit range would be wrong on both counts
    // (it would neither separate NaN correctly nor collapse a variable-length cohort correctly),
    // because there is no fixed bit range: the mask.rs shape in probes 1 and 2 has no analogue
    // here, confirming 82:536-539's classification of V -> D as a non-site for the fourth
    // design rule's const-position test.
    println!("value-keyed digest requires Encoding::Canonical per element; no masking shortcut exists, at any construction discipline, confirming V -> D is a non-site for the const-position test");
}
