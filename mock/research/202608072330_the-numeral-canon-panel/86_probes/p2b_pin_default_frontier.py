# PROBE p2b. Pins the default-guard frontier p2 bracketed between k = 12 and
# k = 16: same generated crate as p2 (template embedded in p2_box_frontier.py),
# k = 13, 14, 15, default guard only. Result: ACCEPT at 13 and 14, REFUSE
# (long_running_const_eval) at 15. Toolchain: pinned nightly-2026-05-28.
import subprocess, time

TEMPLATE = open('/tmp/p2_box_k12_deny.rs').read()  # produced by p2_box_frontier.py
for k in [13, 14, 15]:
    src = TEMPLATE.replace('chain_verdict(12, 64)', f'chain_verdict({k}, 64)').replace('k = 12', f'k = {k}')
    path = f'/tmp/p2b_k{k}.rs'
    open(path, 'w').write(src)
    t0 = time.time()
    r = subprocess.run(['rustc', '-O', path, '-o', path + '.bin'], capture_output=True, text=True, timeout=300)
    dt = time.time() - t0
    out = 'ACCEPT' if r.returncode == 0 else ('REFUSE(long_running_const_eval)' if 'long_running_const_eval' in r.stderr else 'REFUSE(other)')
    print(f'k = {k}  grid {2**k:>6}  default guard: {out}  {dt:.1f}s')
