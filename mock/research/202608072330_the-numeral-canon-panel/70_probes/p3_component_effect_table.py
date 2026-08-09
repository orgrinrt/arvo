# p3: one component moved at a time, four observables measured, exhaustively at 4 bits.
#
# Hypothesis (written before running): moving a single component of the chain
# (D, Q, rho, E, C) produces a SET of observable effects, not one, because later
# components' meanings depend on earlier ones. In particular a Q-change changes
# induced operation results on the shared value set, so no change-test can file
# coverage as a representation-only property. Predicted effect signatures over the
# observables (V = value set, M = config-to-value map on shared configs,
# O = induced op results on shared value pairs, L = container layout):
#   dD:  O only        dQ:  V, M, O       drho: O only
#   dE:  M only        dC:  L only
# If the prediction fails anywhere, the failure is kept and reported.
#
# Baseline: D = (Z, +); Q = [0, 15]; rho = wrap; E = identity binary in the low
# nibble; C = 8-bit container.
#
# The script must re-instantiate later components when an earlier one moves (E's
# config set is re-derived from Q; rho's reduction is re-derived onto Q). Where that
# happens it is printed, because the forced re-instantiation IS the dependency the
# telescope names. Bears on: OPTIONS.md Q19, 65 section 1's change-test, 67 K1/K2.

def make(op, q_lo, q_hi, rho_name, enc_name, c_bits):
    """Build one point of the chain. Returns dict with configs, map, op-on-values, layout."""
    values = list(range(q_lo, q_hi + 1))
    width = (q_hi - q_lo + 1).bit_length() - 1 if (q_hi - q_lo + 1) & (q_hi - q_lo) == 0 else (q_hi - q_lo + 1).bit_length()
    # E: config -> value map over exactly enough bits for Q (re-instantiated from Q)
    n = q_hi - q_lo + 1
    if enc_name == "binary":
        m = {c: q_lo + c for c in range(n)}
    elif enc_name == "revnib":
        # bit-reversed nibble: a different bijection over the same Q (4-bit only)
        assert n == 16, "revnib arm is defined at the 4-bit model width only"
        m = {c: q_lo + int(f"{c:04b}"[::-1], 2) for c in range(n)}
    else:
        raise ValueError(enc_name)

    def induced(a, b):
        # exact ambient op, then rho re-derived onto THIS Q
        exact = op(a, b)
        if rho_name == "wrap":
            return q_lo + (exact - q_lo) % n
        if rho_name == "saturate":
            return max(q_lo, min(q_hi, exact))
        raise ValueError(rho_name)

    return {"values": set(values), "map": m, "op": induced, "layout": c_bits}


def compare(base, var, label, note=""):
    v_changed = base["values"] != var["values"]
    shared_cfg = set(base["map"]) & set(var["map"])
    m_diff = sum(1 for c in shared_cfg if base["map"][c] != var["map"][c])
    shared_vals = sorted(base["values"] & var["values"])
    o_diff = sum(
        1
        for a in shared_vals
        for b in shared_vals
        if base["op"](a, b) != var["op"](a, b)
    )
    l_changed = base["layout"] != var["layout"]
    pairs = len(shared_vals) ** 2
    sig = (int(v_changed), int(m_diff > 0), int(o_diff > 0), int(l_changed))
    print(f"{label}: V={sig[0]} M={sig[1]} (configs differing: {m_diff}/{len(shared_cfg)}) "
          f"O={sig[2]} (pairs differing: {o_diff}/{pairs}) L={sig[3]}"
          + (f"   [{note}]" if note else ""))
    return label, sig


import operator

base = make(operator.add, 0, 15, "wrap", "binary", 8)

results = []
results.append(compare(
    base,
    make(min, 0, 15, "wrap", "binary", 8),
    "dD  (+ -> min)",
    "rho re-derived onto Q; never fires, min is closed on Q",
))
results.append(compare(
    base,
    make(operator.add, 0, 31, "wrap", "binary", 8),
    "dQ  ([0,15] -> [0,31])",
    "E re-instantiated at 5 bits, rho re-derived mod 32: forced by the dependency",
))
results.append(compare(
    base,
    make(operator.add, 0, 15, "saturate", "binary", 8),
    "drho (wrap -> saturate)",
))
results.append(compare(
    base,
    make(operator.add, 0, 15, "wrap", "revnib", 8),
    "dE  (binary -> bit-reversed nibble)",
))
results.append(compare(
    base,
    make(operator.add, 0, 15, "wrap", "binary", 16),
    "dC  (8-bit -> 16-bit container)",
))

print()
prediction = {
    "dD": (0, 0, 1, 0),
    "dQ": (1, 1, 1, 0),
    "drho": (0, 0, 1, 0),
    "dE": (0, 1, 0, 0),
    "dC": (0, 0, 0, 1),
}
all_ok = True
for label, sig in results:
    key = label.split()[0]
    ok = prediction[key] == sig
    all_ok = all_ok and ok
    print(f"prediction {key}: {'CONFIRMED' if ok else 'REFUTED, predicted ' + str(prediction[key]) + ' measured ' + str(sig)}")

print()
distinct = {}
for label, sig in results:
    distinct.setdefault(sig, []).append(label.split()[0])
print(f"distinct effect signatures: {len(distinct)}")
for sig, members in distinct.items():
    print(f"  {sig}: {', '.join(members)}")

print()
print("65's change-test applied to each single-component move:")
for label, sig in results:
    v, m, o, l = sig
    if l and not (v or m or o):
        verdict = "format (container moved, no named value changed)"
    elif m and v and not o:
        verdict = "representation (map and named values moved, ops did not)"
    elif o:
        verdict = "system (what operations mean changed)"
    else:
        verdict = "UNCLASSIFIED by the test's three clauses"
    print(f"  {label.split()[0]}: {verdict}")

print()
print("all predictions confirmed" if all_ok else "AT LEAST ONE PREDICTION REFUTED, kept above")
