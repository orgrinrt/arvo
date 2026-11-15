# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (123 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 56 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-minimum dominates: 25% faster than the next best (warm-clamp-min-lanes)

warm-clamp-minimum (56 ns) leads warm-clamp-min-lanes (70 ns) by 25%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 55% (significant)

warm-clamp-minimum is -67 ns (55%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.2x slower than the field

warm-clamp-acc64 (123 ns) is 2.2x the fastest (56 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.54)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-min-lanes is inconsistent: worst-20% is 2.6x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 56 ns but its worst 20% at 145 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 55.9 ns median (-54.6% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 2.20x (fastest 55.9 ns, slowest 123.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 412ns | 375ns | 300ns | 406ns | 542ns | base |
| warm-clamp-accfit | 365ns | 359ns | 294ns | 355ns | 464ns | -11.51% |
| warm-clamp-accfit-dyn | 376ns | 348ns | 262ns | 354ns | 560ns | -8.66% |
| warm-clamp-head | 388ns | 316ns | 313ns | 357ns | 553ns | -5.98% |
| warm-clamp-min-lanes | 367ns | 353ns | 282ns | 347ns | 510ns | -11.05% |
| warm-clamp-minimum | 308ns | 284ns | 280ns | 291ns | 390ns | -25.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 135ns | 96ns | 185ns | base | 60.690 |
| warm-clamp-accfit | 107ns | 90ns | 128ns | -20.65% | 76.480 |
| warm-clamp-accfit-dyn | 101ns | 73ns | 145ns | -25.36% | 81.310 |
| warm-clamp-head | 104ns | 88ns | 149ns | -22.63% | 78.441 |
| warm-clamp-min-lanes | 81ns | 56ns | 145ns | -39.94% | 101.048 |
| warm-clamp-minimum | 58ns | 55ns | 62ns | -57.40% | 142.470 |

## Performance model

- Peak throughput: **148.037 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 66.548 | 45.0% |
| warm-clamp-accfit | 76.063 | 51.4% |
| warm-clamp-accfit-dyn | 87.335 | 59.0% |
| warm-clamp-head | 92.097 | 62.2% |
| warm-clamp-min-lanes | 117.364 | 79.3% |
| warm-clamp-minimum | 146.547 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 412ns | 412ns | base |
| warm-clamp-accfit | 365ns | 365ns | -11.51% |
| warm-clamp-accfit-dyn | 376ns | 376ns | -8.66% |
| warm-clamp-head | 388ns | 388ns | -5.98% |
| warm-clamp-min-lanes | 367ns | 367ns | -11.05% |
| warm-clamp-minimum | 308ns | 308ns | -25.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 123ns | base | --- | [123, 138] | --- | --- | --- | --- |
| warm-clamp-accfit | 108ns | -31.0ns (-25.2%) | [-45, -14]ns | [100, 110] | YES | 0.0011 | 0.0007 | 0 |
| warm-clamp-accfit-dyn | 94ns | -49.2ns (-40.0%) | [-59, -12]ns | [88, 98] | YES | 0.0028 | 0.0022 | 0 |
| warm-clamp-head | 89ns | -34.0ns (-27.6%) | [-41, -11]ns | [89, 89] | YES | 0.0064 | 0.0064 | 0 |
| warm-clamp-min-lanes | 70ns | -67.0ns (-54.5%) | [-82, -53]ns | [70, 70] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 56ns | -67.3ns (-54.7%) | [-82, -67]ns | [56, 56] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 123ns | -26.3% | -40.4% | +20.3% | -54.5% | -54.7% |
| 1 | 2 | 123ns | -11.0% | +18.7% | -28.0% | -43.2% | -54.6% |
| 1 | 3 | 158ns | -36.4% | -38.1% | -43.8% | -55.8% | -60.8% |
| 1 | 4 | 101ns | +25.8% | -12.4% | -11.6% | -30.5% | -44.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.538 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.263 | moderate- |
| warm-clamp-accfit-dyn | -0.459 | moderate- |
| warm-clamp-head | -0.087 | ok |
| warm-clamp-min-lanes | -0.084 | ok |
| warm-clamp-minimum | -0.424 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 102.3ns | 135.0ns | 75.8% | HIGH |
| warm-clamp-accfit | 92.9ns | 107.1ns | 86.7% | HIGH |
| warm-clamp-accfit-dyn | 99.4ns | 100.8ns | 98.7% | HIGH |
| warm-clamp-head | 100.2ns | 104.4ns | 95.9% | HIGH |
| warm-clamp-min-lanes | 101.8ns | 81.1ns | 125.5% | HIGH |
| warm-clamp-minimum | 88.0ns | 57.5ns | 153.0% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 96.2-184.8 ns)
     96.2 |########
    100.7 |#################
    105.1 |
    109.5 |
    113.9 |
    118.4 |#################
    122.8 |########################################
    127.2 |
    131.7 |####
    136.1 |######################
    140.5 |
    144.9 |
    149.4 |
    153.8 |###################################
    158.2 |########
    162.7 |
    167.1 |
    171.5 |
    176.0 |
    180.4 |
  (3 below, 2 above range)

warm-clamp-accfit (n=40, range 89.6-127.5 ns)
     89.6 |######################
     91.5 |###########
     93.4 |###########
     95.3 |
     97.2 |######################
     99.1 |###########
    101.0 |###########
    102.9 |#####
    104.8 |
    106.7 |#################
    108.6 |############################
    110.5 |#################
    112.4 |
    114.3 |
    116.2 |
    118.1 |
    120.0 |
    121.8 |
    123.7 |#####
    125.6 |########################################
  (2 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 73.4-144.7 ns)
     73.4 |#################
     76.9 |####
     80.5 |
     84.0 |####
     87.6 |########################################
     91.2 |
     94.7 |########################################
     98.3 |
    101.9 |
    105.4 |
    109.0 |
    112.6 |
    116.1 |####
    119.7 |
    123.3 |#############
    126.8 |
    130.4 |
    134.0 |
    137.5 |
    141.1 |
  (5 below, 7 above range)

warm-clamp-head (n=40, range 88.2-149.2 ns)
     88.2 |########################################
     91.2 |#
     94.3 |
     97.4 |#
    100.4 |
    103.5 |#
    106.5 |
    109.6 |
    112.6 |
    115.7 |
    118.7 |
    121.8 |
    124.8 |
    127.9 |
    130.9 |
    134.0 |
    137.0 |
    140.1 |
    143.1 |
    146.2 |##############
  (2 below, 1 above range)

warm-clamp-min-lanes (n=40, range 55.9-145.4 ns)
     55.9 |########
     60.4 |######
     64.9 |
     69.3 |########################################
     73.8 |
     78.3 |#
     82.8 |
     87.2 |
     91.7 |
     96.2 |
    100.7 |
    105.1 |
    109.6 |
    114.1 |
    118.6 |
    123.0 |
    127.5 |
    132.0 |
    136.5 |
    140.9 |
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 55.3-62.2 ns)
     55.3 |#############
     55.7 |########################################
     56.0 |####
     56.4 |
     56.7 |
     57.1 |##
     57.4 |
     57.8 |
     58.1 |
     58.4 |
     58.8 |
     59.1 |
     59.5 |
     59.8 |
     60.2 |
     60.5 |
     60.9 |
     61.2 |
     61.5 |######
     61.9 |###########
  (2 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=27.7% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: bridge=79.7% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=88.5% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: CV=25.4% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: bridge=96.1% of algo (FFI overhead may distort results)
- **warm-clamp-head**: CV=24.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=96.1% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: CV=72.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: bridge=153.4% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=153.0% of algo (FFI overhead may distort results)
