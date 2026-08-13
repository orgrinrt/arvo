# PROBE p2. The grid procedure's own frontier, measured as accept/refuse on
# the pinned nightly, default guard and --cfg allow_guard, at width 64.
#
# The question 84 left open (its O-J costing names 2^k for a length-k
# multilinear chain law and stops there): where does the box actually stop
# being const-evaluable? A chain-assoc law of length k over the wrapping
# fragment is multilinear, so its degree grid is {0,1}^k, 2^k points, each
# costing ~2k wrapping multiplies. The law is TRUE (mul is associative), so
# there is no early exit and the full box is walked: this is the expensive
# positive-verdict case, which is exactly the verdict that licenses an arm.
#
# For each k, a tiny crate is generated: a const fn walks the grid streaming
# (no allocation, no tensor, per file 86 section 3's grid form), a crate-level
# const asserts the verdict, rustc compiles it. Outcome recorded as ACCEPT,
# REFUSE(long_running_const_eval), or TIMEOUT at the probe's own cap.
# Wall-clock seconds are an ad-hoc quick spike with no substance; the
# accept/refuse outcomes are the results.
#
# Toolchain: pinned nightly-2026-05-28 via the repo's rust-toolchain.toml.

import subprocess, sys, time, os

TEMPLATE = """
#![allow(dead_code)]
{allow}
// left-assoc against right-assoc wrapping product of (C_i + x_i), x in {{0,1}}^k:
// multilinear, true at every width, so the whole 2^k grid must be walked.
const fn chain_verdict(k: u32, w: u32) -> bool {{
    let m: u64 = if w >= 64 {{ u64::MAX }} else {{ (1u64 << w) - 1 }};
    let n: u64 = 1u64 << k;
    let mut pt: u64 = 0;
    while pt < n {{
        let mut l: u64 = 1;
        let mut i: u32 = 0;
        while i < k {{
            let opnd = (3 + i as u64).wrapping_add((pt >> i) & 1) & m;
            l = l.wrapping_mul(opnd) & m;
            i += 1;
        }}
        let mut r: u64 = 1;
        let mut j: u32 = k;
        while j > 0 {{
            j -= 1;
            let opnd = (3 + j as u64).wrapping_add((pt >> j) & 1) & m;
            r = opnd.wrapping_mul(r) & m;
        }}
        if l != r {{
            return false;
        }}
        pt += 1;
    }}
    true
}}

const VERDICT: bool = chain_verdict({k}, 64);
const _: () = assert!(VERDICT, "chain law false at width 64, which cannot happen");

fn main() {{
    println!("k = {k}: verdict {{}}", VERDICT);
}}
"""

CAP = 300  # seconds per compile, the probe's own cap


def run(k, allow):
    src = f"/tmp/p2_box_k{k}_{'allow' if allow else 'deny'}.rs"
    with open(src, "w") as f:
        f.write(TEMPLATE.format(k=k, allow="#![allow(long_running_const_eval)]" if allow else ""))
    t0 = time.time()
    try:
        r = subprocess.run(
            ["rustc", "-O", src, "-o", src + ".bin"],
            capture_output=True, text=True, timeout=CAP,
        )
        dt = time.time() - t0
        if r.returncode == 0:
            return "ACCEPT", dt, ""
        err = r.stderr
        if "long_running_const_eval" in err:
            return "REFUSE(long_running_const_eval)", dt, ""
        return "REFUSE(other)", dt, err.splitlines()[0] if err else ""
    except subprocess.TimeoutExpired:
        return f"TIMEOUT({CAP}s)", CAP, ""


def main():
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    print("p2: const-eval frontier of the 2^k grid at width 64, chain-assoc (true verdict, no early exit)")
    print(f"{'k':>4} {'grid':>12} {'default guard':>32} {'#![allow(long_running_const_eval)]':>36}")
    stop_default = False
    stop_allow = False
    for k in [8, 12, 16, 18, 20, 22, 24]:
        d = ("skipped", 0, "") if stop_default else run(k, allow=False)
        a = ("skipped", 0, "") if stop_allow else run(k, allow=True)
        print(f"{k:>4} {2**k:>12} {d[0]:>25} {d[1]:>5.1f}s {a[0]:>28} {a[1]:>6.1f}s")
        sys.stdout.flush()
        if d[0].startswith(("REFUSE", "TIMEOUT")):
            stop_default = True
        if a[0].startswith("TIMEOUT"):
            stop_allow = True


if __name__ == "__main__":
    main()
