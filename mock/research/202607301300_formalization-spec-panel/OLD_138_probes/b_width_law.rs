//! 138 probe B. Is the stored width DERIVABLE from the numeral's four
//! coordinates, at every radix, or must it be declared (110:3248)?
//!
//! Candidate law:  W_S = sign + ceil(log2( M * span ))
//!   M    = R^(P-1) where a leading digit is hidden (radix two only)
//!        = R^P     otherwise
//!   span = EMAX - EMIN + 1, the count of normal binades
//!
//! Checked over the WHOLE matrix of formats the design's own chapters name:
//! every IEEE 754-2019 interchange format, binary and decimal, plus the two
//! OCP 8-bit formats and bfloat16. No sampling.

/// ceil(log2(n)) for an exact big integer given as (radix, exp, mult):
/// value = radix^exp * mult. Computed exactly by repeated doubling.
fn ceil_log2_exact(radix: u32, exp: u32, mult: u64) -> u32 {
    // build radix^exp * mult as a little-endian base-2^32 bignum
    let mut v: Vec<u64> = vec![mult];
    for _ in 0..exp {
        let mut carry = 0u64;
        for w in v.iter_mut() {
            let t = *w * radix as u64 + carry;
            *w = t & 0xffff_ffff;
            carry = t >> 32;
        }
        while carry > 0 {
            v.push(carry & 0xffff_ffff);
            carry >>= 32;
        }
    }
    while v.len() > 1 && *v.last().unwrap() == 0 {
        v.pop();
    }
    let top = *v.last().unwrap();
    let bl = (v.len() as u32 - 1) * 32 + (32 - (top as u32).leading_zeros());
    // ceil(log2 n) is bl for a non-power of two and bl - 1 for an exact one
    let mut is_pow2 = (top & (top - 1)) == 0;
    for w in &v[..v.len() - 1] {
        if *w != 0 {
            is_pow2 = false;
        }
    }
    if is_pow2 {
        bl - 1
    } else {
        bl
    }
}

struct Fmt {
    name: &'static str,
    radix: u32,
    p: u32,
    emin: i64,
    emax: i64,
    hidden: bool,
    width: u32,
}

fn main() {
    let fs = [
        // IEEE 754-2019 binary interchange formats
        Fmt {
            name: "binary16",
            radix: 2,
            p: 11,
            emin: -14,
            emax: 15,
            hidden: true,
            width: 16,
        },
        Fmt {
            name: "binary32",
            radix: 2,
            p: 24,
            emin: -126,
            emax: 127,
            hidden: true,
            width: 32,
        },
        Fmt {
            name: "binary64",
            radix: 2,
            p: 53,
            emin: -1022,
            emax: 1023,
            hidden: true,
            width: 64,
        },
        Fmt {
            name: "binary128",
            radix: 2,
            p: 113,
            emin: -16382,
            emax: 16383,
            hidden: true,
            width: 128,
        },
        Fmt {
            name: "binary256",
            radix: 2,
            p: 237,
            emin: -262142,
            emax: 262143,
            hidden: true,
            width: 256,
        },
        // non-IEEE binary the design names
        Fmt {
            name: "bfloat16",
            radix: 2,
            p: 8,
            emin: -126,
            emax: 127,
            hidden: true,
            width: 16,
        },
        Fmt {
            name: "E4M3(OCP)",
            radix: 2,
            p: 4,
            emin: -6,
            emax: 8,
            hidden: true,
            width: 8,
        },
        Fmt {
            name: "E5M2(OCP)",
            radix: 2,
            p: 3,
            emin: -14,
            emax: 15,
            hidden: true,
            width: 8,
        },
        // IEEE 754-2019 decimal interchange formats
        Fmt {
            name: "decimal32",
            radix: 10,
            p: 7,
            emin: -95,
            emax: 96,
            hidden: false,
            width: 32,
        },
        Fmt {
            name: "decimal64",
            radix: 10,
            p: 16,
            emin: -383,
            emax: 384,
            hidden: false,
            width: 64,
        },
        Fmt {
            name: "decimal128",
            radix: 10,
            p: 34,
            emin: -6143,
            emax: 6144,
            hidden: false,
            width: 128,
        },
    ];
    let mut fail = 0;
    println!(
        "{:<11} {:>5} {:>4} {:>8} {:>8} {:>6} {:>6} {:>6}",
        "format", "radix", "p", "span", "sig_exp", "pred", "actual", "ok"
    );
    for f in &fs {
        let span = (f.emax - f.emin + 1) as u64;
        let sig_exp = if f.hidden { f.p - 1 } else { f.p };
        let pred = 1 + ceil_log2_exact(f.radix, sig_exp, span);
        let ok = pred == f.width;
        if !ok {
            fail += 1;
        }
        println!(
            "{:<11} {:>5} {:>4} {:>8} {:>8} {:>6} {:>6} {:>6}",
            f.name, f.radix, f.p, span, sig_exp, pred, f.width, ok
        );
    }
    println!("\nfailures: {}", fail);

    // the per-field sum form, which is the radix-two specialisation
    println!("\nper-field sum form  W = 1 + sig_exp*log2(R) rounded up + bitlen(span):");
    let mut sumfail = 0;
    for f in &fs {
        let span = (f.emax - f.emin + 1) as u64;
        let sig_exp = if f.hidden { f.p - 1 } else { f.p };
        let sigbits = ceil_log2_exact(f.radix, sig_exp, 1);
        let expbits = ceil_log2_exact(2, 0, span);
        let pred = 1 + sigbits + expbits;
        let ok = pred == f.width;
        if !ok {
            sumfail += 1;
        }
        println!(
            "  {:<11} sig={:>4} exp={:>3} sum={:>4} actual={:>4} {}",
            f.name,
            sigbits,
            expbits,
            pred,
            f.width,
            if ok { "ok" } else { "OVERSHOOT" }
        );
    }
    println!("per-field failures: {}", sumfail);

    println!("\nexponent-field slack: codes the bit length leaves after the normal binades");
    for f in &fs {
        if f.radix != 2 {
            continue;
        }
        let span = (f.emax - f.emin + 1) as u64;
        let bits = ceil_log2_exact(2, 0, span);
        let slack = (1u64 << bits) - span;
        println!(
            "  {:<11} span={:>7} expbits={:>3} codes={:>8} slack={}",
            f.name,
            span,
            bits,
            1u64 << bits,
            slack
        );
    }
}
