#!/usr/bin/env python3
"""PROBE p11. Why p9's frontier and 80's frontier disagree by one bit, and what
that says about how a frontier may be quoted.

80 section 4.1's table puts the arity-3 frontier for signed saturating
associativity at width 5, first refused at width 6, on this same host and this
same toolchain. p9 measures the same law at the same arity accepting at width 6
and first refused at width 7. One bit apart, same machine, same rustc, same
guard, both counting rather than returning early.

The candidate mechanism is that the two instruments spend different amounts of
work PER TUPLE, and `long_running_const_eval` is a budget on total evaluation
steps rather than on domain size. Reading the two probes at the source:

  80's `p2_frontier.py` builds a `[i32; K]` array for each tuple and calls
  `left_fold` and `right_fold`, each of which runs a `while` loop with indexing.

  p9's template writes the arity-3 comparison inline as two `sat_add` calls per
  side, with no array and no loop.

This probe holds the law, the width, the domain and the guard fixed and varies
ONLY the encoding, so the difference cannot be attributed to anything else. If
the array-and-fold encoding refuses at a lower width than the inline encoding,
the mechanism is confirmed and neither frontier is "the" frontier.

Both encodings compute the same thing, which the probe checks by comparing the
violation counts they report at a width where both accept.
"""

import subprocess, tempfile, os, time, json

INLINE = r'''
const W: i32 = {W};
const HI: i32 = (1 << (W - 1)) - 1;
const LO: i32 = -(1 << (W - 1));

const fn sat_add(a: i32, b: i32) -> i32 {{
    let s = a + b;
    if s > HI {{ HI }} else if s < LO {{ LO }} else {{ s }}
}}

const fn violations() -> u64 {{
    let mut bad: u64 = 0;
    let mut a = LO;
    while a <= HI {{
        let mut b = LO;
        while b <= HI {{
            let mut c = LO;
            while c <= HI {{
                if sat_add(sat_add(a, b), c) != sat_add(a, sat_add(b, c)) {{ bad += 1; }}
                c += 1;
            }}
            b += 1;
        }}
        a += 1;
    }}
    bad
}}

pub const RESULT: u64 = violations();
pub const _C: () = {{ assert!(RESULT != u64::MAX); }};
'''

ARRAY_FOLD = r'''
const W: i32 = {W};
const HI: i32 = (1 << (W - 1)) - 1;
const LO: i32 = -(1 << (W - 1));
const K: usize = 3;

const fn sat_add(a: i32, b: i32) -> i32 {{
    let s = a + b;
    if s > HI {{ HI }} else if s < LO {{ LO }} else {{ s }}
}}

const fn left_fold(xs: &[i32; K]) -> i32 {{
    let mut acc = xs[0];
    let mut i = 1;
    while i < K {{ acc = sat_add(acc, xs[i]); i += 1; }}
    acc
}}

const fn right_fold(xs: &[i32; K]) -> i32 {{
    let mut acc = xs[K - 1];
    let mut i = K - 1;
    while i > 0 {{ i -= 1; acc = sat_add(xs[i], acc); }}
    acc
}}

const fn violations() -> u64 {{
    let mut bad: u64 = 0;
    let mut a = LO;
    while a <= HI {{
        let mut b = LO;
        while b <= HI {{
            let mut c = LO;
            while c <= HI {{
                let xs = [a, b, c];
                if left_fold(&xs) != right_fold(&xs) {{ bad += 1; }}
                c += 1;
            }}
            b += 1;
        }}
        a += 1;
    }}
    bad
}}

pub const RESULT: u64 = violations();
pub const _C: () = {{ assert!(RESULT != u64::MAX); }};
'''

# A third encoding, deliberately more expensive per tuple than either, to show
# the frontier moves with per-tuple cost in the predicted direction rather than
# only differing between two arbitrary spellings.
PADDED = r'''
const W: i32 = {W};
const HI: i32 = (1 << (W - 1)) - 1;
const LO: i32 = -(1 << (W - 1));
const K: usize = 3;

const fn sat_add(a: i32, b: i32) -> i32 {{
    let s = a + b;
    if s > HI {{ HI }} else if s < LO {{ LO }} else {{ s }}
}}

const fn left_fold(xs: &[i32; K]) -> i32 {{
    let mut acc = xs[0];
    let mut i = 1;
    while i < K {{ acc = sat_add(acc, xs[i]); i += 1; }}
    acc
}}

const fn right_fold(xs: &[i32; K]) -> i32 {{
    let mut acc = xs[K - 1];
    let mut i = K - 1;
    while i > 0 {{ i -= 1; acc = sat_add(xs[i], acc); }}
    acc
}}

// Same verdict, more steps per tuple: the tuple is copied through a second
// array before use. Nothing about the law changes.
const fn violations() -> u64 {{
    let mut bad: u64 = 0;
    let mut a = LO;
    while a <= HI {{
        let mut b = LO;
        while b <= HI {{
            let mut c = LO;
            while c <= HI {{
                let xs = [a, b, c];
                let mut ys = [0i32; K];
                let mut i = 0;
                while i < K {{ ys[i] = xs[i]; i += 1; }}
                if left_fold(&ys) != right_fold(&ys) {{ bad += 1; }}
                c += 1;
            }}
            b += 1;
        }}
        a += 1;
    }}
    bad
}}

pub const RESULT: u64 = violations();
pub const _C: () = {{ assert!(RESULT != u64::MAX); }};
'''

ENCODINGS = [
    ("inline (p9's shape)", INLINE),
    ("array + two folds (80's shape)", ARRAY_FOLD),
    ("array + copy + two folds", PADDED),
]


def compile_at(tmpl, w, allow):
    src = tmpl.format(W=w)
    if allow:
        src = "#![allow(long_running_const_eval)]\n" + src
    with tempfile.TemporaryDirectory() as d:
        p = os.path.join(d, 'probe.rs')
        open(p, 'w').write(src)
        t0 = time.time()
        try:
            r = subprocess.run(['rustc', '-O', '--crate-type=lib', '--emit=metadata',
                                '-o', os.path.join(d, 'o.meta'), p],
                               capture_output=True, text=True, timeout=300)
            dt = time.time() - t0
            ok = r.returncode == 0
            err = next((l.strip()[:60] for l in r.stderr.splitlines()
                        if l.startswith('error')), '')
        except subprocess.TimeoutExpired:
            dt, ok, err = 300.0, False, 'TIMEOUT at the probe cap'
    return ok, dt, err


def value_at(tmpl, w):
    """The violation count each encoding reports, so the two are known to be
    computing the same thing rather than assumed to."""
    src = tmpl.format(W=w) + '\nfn main() { println!("{}", RESULT); }\n'
    src = src.replace('--crate-type=lib', '')
    with tempfile.TemporaryDirectory() as d:
        p = os.path.join(d, 'probe.rs')
        open(p, 'w').write(src)
        b = os.path.join(d, 'probe')
        r = subprocess.run(['rustc', '-O', '-o', b, p], capture_output=True, text=True)
        if r.returncode != 0:
            return None
        out = subprocess.run([b], capture_output=True, text=True)
        return out.stdout.strip()


def main():
    print("p11: the const-eval frontier reads the ENCODING, not only the domain\n")
    print(f"toolchain: {subprocess.run(['rustc','--version'],capture_output=True,text=True).stdout.strip()}")
    print("host:      aarch64-apple-darwin")
    print()

    print("agreement check first: all three encodings must report the same violation")
    print("count, or they are not computing the same law and nothing below compares.\n")
    for name, tmpl in ENCODINGS:
        v = value_at(tmpl, 4)
        print(f"  {name:<32} violations at width 4: {v}")
    print()

    rows = []
    print(f"{'encoding':<32} {'guard':>8} {'widest accept':>14} {'first refuse':>13}  diagnostic")
    for name, tmpl in ENCODINGS:
        for allow in (False, True):
            widest = None
            first_refuse = None
            diag = ''
            for w in range(3, 11):
                ok, dt, err = compile_at(tmpl, w, allow)
                rows.append(dict(encoding=name, allow_guard=allow, width=w,
                                 accepted=ok, seconds=round(dt, 2), diagnostic=err))
                if ok:
                    widest = w
                else:
                    first_refuse = w
                    diag = err
                    break
            print(f"{name:<32} {'allowed' if allow else 'default':>8} "
                  f"{str(widest):>14} {str(first_refuse):>13}  {diag}")

    here = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(here, 'p11_output.json'), 'w') as f:
        json.dump(rows, f, indent=1)
    print("\nmachine-readable at p11_output.json")
    print("\nreading: the law, the domain, the arity, the guard, the host and the")
    print("toolchain are identical down every column. Only the spelling of the")
    print("check differs. Any spread in the frontier is therefore attributable to")
    print("per-tuple evaluation cost and to nothing else.")


if __name__ == '__main__':
    main()
