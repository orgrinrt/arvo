# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum dominates: 16% faster than the next best (warm-clamp-min-lanes)

warm-clamp-minimum (179 ns) leads warm-clamp-min-lanes (207 ns) by 16%, a clear separation rather than a photo finish. CV 19.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 51% (significant)

warm-clamp-minimum is -174 ns (51%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-head is an outlier: 2.3x slower than the field

warm-clamp-head (409 ns) is 2.3x the fastest (179 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.78)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-head} (64% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-head} with a 64% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-acc64 is inconsistent: worst-20% is 2.1x its best-20%

warm-clamp-acc64's best 20% of batches run at 254 ns but its worst 20% at 536 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-accfit's edge over baseline is significant but tiny (3 ns, 0.94%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 3 ns (0.94%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-minimum** at 178.7 ns median (-47.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.29x (fastest 178.7 ns, slowest 409.4 ns)

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
| warm-clamp-acc64 | 652ns | 592ns | 444ns | 628ns | 932ns | base |
| warm-clamp-accfit | 661ns | 664ns | 578ns | 646ns | 789ns | +1.38% |
| warm-clamp-accfit-dyn | 672ns | 661ns | 586ns | 660ns | 792ns | +3.05% |
| warm-clamp-head | 755ns | 765ns | 531ns | 757ns | 972ns | +15.77% |
| warm-clamp-min-lanes | 543ns | 560ns | 404ns | 527ns | 729ns | -16.72% |
| warm-clamp-minimum | 422ns | 445ns | 290ns | 425ns | 547ns | -35.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 365ns | 254ns | 536ns | base | 22.423 |
| warm-clamp-accfit | 372ns | 329ns | 436ns | +1.83% | 22.020 |
| warm-clamp-accfit-dyn | 377ns | 336ns | 434ns | +3.25% | 21.716 |
| warm-clamp-head | 424ns | 305ns | 530ns | +16.06% | 19.320 |
| warm-clamp-min-lanes | 230ns | 177ns | 327ns | -37.01% | 35.596 |
| warm-clamp-minimum | 177ns | 127ns | 221ns | -51.53% | 46.265 |

## Performance model

- Peak throughput: **64.727 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 24.158 | 37.3% |
| warm-clamp-accfit | 21.451 | 33.1% |
| warm-clamp-accfit-dyn | 21.666 | 33.5% |
| warm-clamp-head | 20.007 | 30.9% |
| warm-clamp-min-lanes | 39.623 | 61.2% |
| warm-clamp-minimum | 45.855 | 70.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 652ns | 652ns | base |
| warm-clamp-accfit | 661ns | 661ns | +1.38% |
| warm-clamp-accfit-dyn | 672ns | 672ns | +3.05% |
| warm-clamp-head | 755ns | 755ns | +15.77% |
| warm-clamp-min-lanes | 543ns | 543ns | -16.72% |
| warm-clamp-minimum | 422ns | 422ns | -35.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 339ns | base | --- | [339, 340] | --- | --- | --- | --- |
| warm-clamp-accfit | 382ns | +3.2ns (+0.9%) | [+2, +43]ns | [342, 383] | YES | 0.0207 | 0.0166 | 0 |
| warm-clamp-accfit-dyn | 378ns | +39.6ns (+11.7%) | [+26, +49]ns | [378, 379] | YES | 0.0385 | 0.0385 | 0 |
| warm-clamp-head | 409ns | +50.6ns (+14.9%) | [+42, +52]ns | [381, 508] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-min-lanes | 207ns | -111.4ns (-32.9%) | [-144, -103]ns | [195, 232] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 179ns | -173.8ns (-51.2%) | [-213, -118]ns | [161, 197] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 339ns | +0.6% | +11.7% | +12.5% | -31.9% | -38.5% |
| 1 | 2 | 340ns | +12.6% | +11.3% | +49.6% | -42.5% | -62.7% |
| 1 | 3 | 510ns | -14.1% | -15.2% | -0.2% | -50.8% | -57.9% |
| 1 | 4 | 254ns | +45.8% | +32.5% | +20.2% | -30.9% | -36.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.537 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.138 | ok |
| warm-clamp-accfit-dyn | -0.528 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.227 | moderate- |
| warm-clamp-min-lanes | -0.695 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.783 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 104.3ns | 365.3ns | 28.6% | HIGH |
| warm-clamp-accfit | 103.1ns | 372.0ns | 27.7% | HIGH |
| warm-clamp-accfit-dyn | 107.5ns | 377.2ns | 28.5% | HIGH |
| warm-clamp-head | 130.4ns | 424.0ns | 30.8% | HIGH |
| warm-clamp-min-lanes | 111.8ns | 230.1ns | 48.6% | HIGH |
| warm-clamp-minimum | 85.9ns | 177.1ns | 48.5% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 254.0-536.1 ns)
    254.0 |#################
    268.1 |
    282.2 |
    296.3 |##
    310.4 |##
    324.5 |########
    338.7 |########################################
    352.8 |
    366.9 |
    381.0 |
    395.1 |
    409.2 |
    423.3 |##
    437.4 |##
    451.5 |
    465.6 |
    479.7 |
    493.8 |##
    507.9 |#################
    522.0 |
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 329.2-435.9 ns)
    329.2 |
    334.5 |
    339.9 |########################################
    345.2 |
    350.5 |
    355.9 |##
    361.2 |
    366.5 |
    371.9 |
    377.2 |#####
    382.5 |#############################
    387.9 |
    393.2 |
    398.5 |
    403.9 |
    409.2 |
    414.5 |#####
    419.9 |
    425.2 |
    430.5 |
  (3 below, 6 above range)

warm-clamp-accfit-dyn (n=40, range 335.7-433.7 ns)
    335.7 |#########################
    340.6 |
    345.5 |
    350.4 |##
    355.3 |
    360.2 |
    365.1 |
    370.0 |
    374.9 |########################################
    379.8 |##
    384.7 |
    389.6 |
    394.5 |#####
    399.4 |##
    404.3 |
    409.2 |
    414.1 |
    419.0 |
    423.9 |
    428.8 |############
  (2 below, 2 above range)

warm-clamp-head (n=40, range 305.4-529.9 ns)
    305.4 |##############
    316.6 |
    327.8 |#####
    339.0 |##
    350.3 |
    361.5 |##
    372.7 |####################
    383.9 |
    395.2 |
    406.4 |
    417.6 |
    428.8 |##
    440.1 |##
    451.3 |##
    462.5 |
    473.7 |
    485.0 |
    496.2 |##
    507.4 |########################################
    518.6 |
  (4 below, 2 above range)

warm-clamp-min-lanes (n=40, range 176.8-327.1 ns)
    176.8 |
    184.3 |####
    191.8 |########################################
    199.3 |########
    206.9 |####
    214.4 |####
    221.9 |
    229.4 |########################
    236.9 |
    244.4 |########################
    251.9 |####
    259.5 |
    267.0 |
    274.5 |
    282.0 |########
    289.5 |####
    297.0 |
    304.6 |
    312.1 |
    319.6 |
  (7 below, 2 above range)

warm-clamp-minimum (n=40, range 126.6-221.4 ns)
    126.6 |################
    131.3 |########
    136.0 |
    140.8 |
    145.5 |
    150.3 |
    155.0 |
    159.7 |########################################
    164.5 |
    169.2 |
    174.0 |
    178.7 |
    183.4 |
    188.2 |
    192.9 |####################################
    197.7 |
    202.4 |
    207.1 |####
    211.9 |
    216.6 |################################
  (4 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=28.4% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: bridge=28.1% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=27.2% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: bridge=28.0% of algo (FFI overhead may distort results)
- **warm-clamp-head**: CV=21.4% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=30.7% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: CV=33.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: bridge=53.3% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=48.5% of algo (FFI overhead may distort results)
