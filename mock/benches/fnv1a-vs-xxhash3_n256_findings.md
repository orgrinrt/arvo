# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fnv1a has the worst median (334 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest xxhash3 at 133 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### xxhash3 dominates: 151% faster than the next best (fnv1a)

xxhash3 (133 ns) leads fnv1a (334 ns) by 151%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### xxhash3 beats baseline by 60% (significant)

xxhash3 is -199 ns (60%) faster than baseline fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fnv1a shows warm-up / thermal drift (autocorr +0.92)

fnv1a's per-pass series has lag-1 autocorrelation +0.92, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: xxhash3** at 132.7 ns median (-60.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.51x (fastest 132.7 ns, slowest 333.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 453ns | 455ns | 432ns | 454ns | 473ns | base |
| xxhash3 | 205ns | 204ns | 194ns | 205ns | 217ns | -54.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 334ns | 318ns | 349ns | base | 0.768 |
| xxhash3 | 133ns | 127ns | 140ns | -60.05% | 1.921 |

## Performance model

- Peak throughput: **2.020 Gops/s** (xxhash3; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 0.768 | 38.0% |
| xxhash3 | 1.929 | 95.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 453ns | 453ns | base |
| xxhash3 | 205ns | 205ns | -54.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 334ns | base | --- | [321, 346] | --- | --- | --- | --- |
| xxhash3 | 133ns | -199.8ns (-59.9%) | [-208, -193]ns | [128, 138] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 348ns | -59.8% |
| 2 | 349ns | -60.6% |
| 3 | 348ns | -60.1% |
| 4 | 345ns | -60.6% |
| 5 | 351ns | -60.3% |
| 6 | 346ns | -59.9% |
| 7 | 348ns | -59.9% |
| 8 | 343ns | -59.3% |
| 9 | 346ns | -58.9% |
| 10 | 345ns | -60.8% |
| 11 | 350ns | -60.1% |
| 12 | 347ns | -59.3% |
| 13 | 349ns | -59.7% |
| 14 | 347ns | -60.2% |
| 15 | 347ns | -60.6% |
| 16 | 347ns | -59.7% |
| 17 | 348ns | -60.1% |
| 18 | 349ns | -60.7% |
| 19 | 348ns | -60.7% |
| 20 | 346ns | -59.9% |
| 21 | 321ns | -60.0% |
| 22 | 322ns | -60.5% |
| 23 | 319ns | -59.9% |
| 24 | 324ns | -60.5% |
| 25 | 318ns | -60.4% |
| 26 | 319ns | -60.1% |
| 27 | 319ns | -60.3% |
| 28 | 318ns | -59.1% |
| 29 | 320ns | -60.0% |
| 30 | 315ns | -59.1% |
| 31 | 319ns | -59.3% |
| 32 | 321ns | -60.5% |
| 33 | 320ns | -60.1% |
| 34 | 318ns | -59.9% |
| 35 | 320ns | -59.7% |
| 36 | 319ns | -60.1% |
| 37 | 321ns | -60.1% |
| 38 | 321ns | -60.1% |
| 39 | 321ns | -60.2% |
| 40 | 321ns | -60.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.917 | HIGH+ (drift/warm-up) |
| xxhash3 | 0.851 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **xxhash3**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 2.8ns | 333.5ns | 0.8% |  |
| xxhash3 | 2.7ns | 133.2ns | 2.0% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 318.0-349.0 ns)
    318.0 |##############################
    319.6 |########################################
    321.1 |##########
    322.7 |#####
    324.2 |
    325.8 |
    327.3 |
    328.9 |
    330.4 |
    332.0 |
    333.5 |
    335.1 |
    336.6 |
    338.2 |
    339.7 |
    341.3 |
    342.8 |#####
    344.4 |#########################
    345.9 |####################
    347.5 |##############################
  (3 below, 4 above range)

xxhash3 (n=40, range 126.7-140.3 ns)
    126.7 |##########
    127.4 |########################################
    128.1 |##########
    128.8 |##########
    129.4 |##########
    130.1 |
    130.8 |
    131.5 |
    132.1 |
    132.8 |
    133.5 |
    134.2 |
    134.8 |#####
    135.5 |#####
    136.2 |##########
    136.9 |##########
    137.6 |#####
    138.2 |####################
    138.9 |
    139.6 |##############################
  (4 below, 3 above range)

```

## Diagnostics

- **fnv1a**: autocorrelation=0.92 (measurement drift or warm-up artifact)
- **xxhash3**: autocorrelation=0.85 (measurement drift or warm-up artifact)
