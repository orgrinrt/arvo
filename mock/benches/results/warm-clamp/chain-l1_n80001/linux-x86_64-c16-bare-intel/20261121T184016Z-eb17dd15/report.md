# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (136 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 55 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-min-lanes beats baseline by 58% (significant)

warm-clamp-min-lanes is -79 ns (58%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.5x slower than the field

warm-clamp-acc64 (136 ns) is 2.5x the fastest (55 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-minimum, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-minimum (55 ns) and warm-clamp-min-lanes (56 ns) differ by 0.54%, inside the noise, even though the wider field spreads 145.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.52)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-accfit, warm-clamp-acc64} (60% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-accfit, warm-clamp-acc64} with a 60% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 1.5x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 88 ns but its worst 20% at 135 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 55.3 ns median (-59.3% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 2.46x (fastest 55.3 ns, slowest 135.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112832026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 406ns | 417ns | 335ns | 400ns | 496ns | base |
| warm-clamp-accfit | 374ns | 366ns | 325ns | 360ns | 465ns | -7.98% |
| warm-clamp-accfit-dyn | 363ns | 316ns | 313ns | 338ns | 486ns | -10.77% |
| warm-clamp-head | 340ns | 335ns | 279ns | 327ns | 441ns | -16.37% |
| warm-clamp-min-lanes | 301ns | 285ns | 280ns | 286ns | 366ns | -26.03% |
| warm-clamp-minimum | 306ns | 283ns | 255ns | 296ns | 385ns | -24.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 130ns | 109ns | 151ns | base | 62.985 |
| warm-clamp-accfit | 113ns | 101ns | 131ns | -12.85% | 72.269 |
| warm-clamp-accfit-dyn | 100ns | 88ns | 135ns | -23.08% | 81.881 |
| warm-clamp-head | 108ns | 92ns | 138ns | -17.25% | 76.111 |
| warm-clamp-min-lanes | 59ns | 55ns | 71ns | -54.84% | 139.486 |
| warm-clamp-minimum | 57ns | 50ns | 70ns | -55.80% | 142.488 |

## Performance model

- Peak throughput: **163.554 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 60.324 | 36.9% |
| warm-clamp-accfit | 73.868 | 45.2% |
| warm-clamp-accfit-dyn | 91.993 | 56.2% |
| warm-clamp-head | 77.174 | 47.2% |
| warm-clamp-min-lanes | 147.338 | 90.1% |
| warm-clamp-minimum | 148.137 | 90.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 406ns | 406ns | base |
| warm-clamp-accfit | 374ns | 374ns | -7.98% |
| warm-clamp-accfit-dyn | 363ns | 363ns | -10.77% |
| warm-clamp-head | 340ns | 340ns | -16.37% |
| warm-clamp-min-lanes | 301ns | 301ns | -26.03% |
| warm-clamp-minimum | 306ns | 306ns | -24.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 136ns | base | --- | [122, 138] | --- | --- | --- | --- |
| warm-clamp-accfit | 111ns | -7.9ns (-5.8%) | [-18, -6]ns | [106, 116] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 89ns | -27.9ns (-20.5%) | [-34, -21]ns | [89, 90] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 106ns | -29.5ns (-21.8%) | [-35, -0]ns | [92, 110] | YES | 0.0064 | 0.0064 | 0 |
| warm-clamp-min-lanes | 56ns | -78.6ns (-57.9%) | [-82, -66]ns | [56, 56] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 55ns | -69.4ns (-51.1%) | [-82, -60]ns | [55, 56] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 138ns | -6.2% | -13.1% | -33.2% | -59.7% | -59.9% |
| 1 | 2 | 123ns | -5.2% | -27.6% | -25.3% | -54.7% | -43.7% |
| 1 | 3 | 138ns | -24.6% | -35.8% | -0.7% | -59.8% | -60.0% |
| 1 | 4 | 110ns | -4.6% | -19.0% | +0.4% | -46.6% | -54.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.519 | HIGH- (thermal bounce) |
| warm-clamp-accfit | 0.240 | moderate+ |
| warm-clamp-accfit-dyn | -0.084 | ok |
| warm-clamp-head | -0.113 | ok |
| warm-clamp-min-lanes | -0.071 | ok |
| warm-clamp-minimum | -0.174 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 109.9ns | 130.1ns | 84.5% | HIGH |
| warm-clamp-accfit | 91.9ns | 113.4ns | 81.1% | HIGH |
| warm-clamp-accfit-dyn | 92.3ns | 100.0ns | 92.2% | HIGH |
| warm-clamp-head | 84.4ns | 107.6ns | 78.4% | HIGH |
| warm-clamp-min-lanes | 86.2ns | 58.7ns | 146.8% | HIGH |
| warm-clamp-minimum | 87.4ns | 57.5ns | 152.0% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 109.2-151.0 ns)
    109.2 |########################
    111.3 |
    113.3 |
    115.4 |
    117.5 |
    119.6 |######
    121.7 |##################
    123.8 |
    125.9 |
    128.0 |
    130.1 |
    132.2 |###
    134.3 |###
    136.4 |########################################
    138.5 |###############
    140.6 |###
    142.6 |
    144.7 |
    146.8 |
    148.9 |
  (2 below, 1 above range)

warm-clamp-accfit (n=40, range 101.1-131.3 ns)
    101.1 |################################
    102.6 |########
    104.1 |########################################
    105.6 |################################
    107.1 |########################
    108.6 |########
    110.1 |
    111.6 |########
    113.2 |
    114.7 |########################################
    116.2 |################################
    117.7 |
    119.2 |########
    120.7 |########
    122.2 |########
    123.7 |
    125.3 |########
    126.8 |
    128.3 |########
    129.8 |################
  (2 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 88.4-135.4 ns)
     88.4 |########################################
     90.8 |#
     93.1 |#
     95.5 |
     97.8 |
    100.2 |
    102.5 |
    104.9 |
    107.2 |
    109.6 |#######
    111.9 |#
    114.3 |
    116.6 |
    118.9 |
    121.3 |
    123.6 |
    126.0 |#######
    128.3 |#
    130.7 |
    133.0 |
  (5 below, 1 above range)

warm-clamp-head (n=40, range 91.5-138.0 ns)
     91.5 |########################################
     93.9 |
     96.2 |
     98.5 |
    100.8 |###
    103.2 |
    105.5 |
    107.8 |###############
    110.1 |###############
    112.4 |
    114.8 |
    117.1 |
    119.4 |
    121.7 |###
    124.1 |
    126.4 |###
    128.7 |
    131.0 |
    133.4 |
    135.7 |###############
  (6 below, 3 above range)

warm-clamp-min-lanes (n=40, range 55.2-71.4 ns)
     55.2 |########################################
     56.0 |#
     56.8 |
     57.6 |
     58.4 |
     59.3 |
     60.1 |
     60.9 |#
     61.7 |#####
     62.5 |
     63.3 |
     64.1 |
     64.9 |
     65.8 |
     66.6 |
     67.4 |
     68.2 |
     69.0 |
     69.8 |
     70.6 |
  (3 below, 1 above range)

warm-clamp-minimum (n=40, range 50.1-69.9 ns)
     50.1 |####################
     51.1 |
     52.1 |
     53.1 |
     54.1 |#####
     55.0 |########################################
     56.0 |#####
     57.0 |
     58.0 |
     59.0 |
     60.0 |
     61.0 |##
     62.0 |
     63.0 |
     64.0 |
     65.0 |
     66.0 |
     66.9 |
     67.9 |#####
     68.9 |############
  (2 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: bridge=78.7% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=81.4% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: CV=23.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: bridge=92.9% of algo (FFI overhead may distort results)
- **warm-clamp-head**: bridge=78.2% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: CV=25.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: bridge=153.1% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=152.3% of algo (FFI overhead may distort results)
