# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum dominates: 18% faster than the next best (warm-clamp-min-lanes)

warm-clamp-minimum (185 ns) leads warm-clamp-min-lanes (219 ns) by 18%, a clear separation rather than a photo finish. CV 11.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 45% (significant)

warm-clamp-minimum is -141 ns (45%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.73)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-head} (39% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-head} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-head is inconsistent: worst-20% is 2.0x its best-20%

warm-clamp-head's best 20% of batches run at 302 ns but its worst 20% at 616 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 184.8 ns median (-41.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.86x (fastest 184.8 ns, slowest 343.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 613ns | 590ns | 500ns | 606ns | 747ns | base |
| warm-clamp-accfit | 597ns | 591ns | 502ns | 586ns | 722ns | -2.71% |
| warm-clamp-accfit-dyn | 555ns | 530ns | 486ns | 542ns | 662ns | -9.53% |
| warm-clamp-head | 751ns | 715ns | 528ns | 711ns | 1092ns | +22.41% |
| warm-clamp-min-lanes | 504ns | 500ns | 446ns | 488ns | 612ns | -17.79% |
| warm-clamp-minimum | 446ns | 444ns | 369ns | 438ns | 545ns | -27.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 333ns | 281ns | 407ns | base | 24.621 |
| warm-clamp-accfit | 337ns | 289ns | 408ns | +1.14% | 24.343 |
| warm-clamp-accfit-dyn | 310ns | 279ns | 350ns | -6.73% | 26.396 |
| warm-clamp-head | 405ns | 302ns | 616ns | +21.86% | 20.205 |
| warm-clamp-min-lanes | 214ns | 195ns | 241ns | -35.70% | 38.291 |
| warm-clamp-minimum | 188ns | 161ns | 219ns | -43.49% | 43.568 |

## Performance model

- Peak throughput: **50.835 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 25.969 | 51.1% |
| warm-clamp-accfit | 26.052 | 51.2% |
| warm-clamp-accfit-dyn | 27.010 | 53.1% |
| warm-clamp-head | 23.883 | 47.0% |
| warm-clamp-min-lanes | 37.449 | 73.7% |
| warm-clamp-minimum | 44.329 | 87.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 613ns | 613ns | base |
| warm-clamp-accfit | 597ns | 597ns | -2.71% |
| warm-clamp-accfit-dyn | 555ns | 555ns | -9.53% |
| warm-clamp-head | 751ns | 751ns | +22.41% |
| warm-clamp-min-lanes | 504ns | 504ns | -17.79% |
| warm-clamp-minimum | 446ns | 446ns | -27.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 315ns | base | --- | [306, 339] | --- | --- | --- | --- |
| warm-clamp-accfit | 314ns | no significant difference | [-10, +45]ns | [307, 341] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-accfit-dyn | 303ns | -12.5ns (-3.9%) | [-20, -2]ns | [303, 304] | YES | 0.0003 | 0.0002 | 0 |
| warm-clamp-head | 343ns | no significant difference | [-0, +81]ns | [320, 388] | no | 0.1009 | 0.0807 | 0 |
| warm-clamp-min-lanes | 219ns | -103.2ns (-32.7%) | [-140, -94]ns | [196, 219] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 185ns | -141.4ns (-44.8%) | [-164, -118]ns | [175, 194] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 339ns | +13.3% | -0.5% | -0.8% | -42.4% | -48.4% |
| 1 | 2 | 302ns | -3.1% | -8.8% | +28.0% | -27.6% | -27.7% |
| 1 | 3 | 382ns | -19.5% | -15.4% | +58.5% | -47.4% | -49.2% |
| 1 | 4 | 291ns | +17.2% | +4.3% | +4.1% | -24.5% | -45.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.726 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.272 | moderate- |
| warm-clamp-accfit-dyn | -0.694 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.420 | moderate- |
| warm-clamp-min-lanes | -0.646 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.179 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 99.6ns | 332.7ns | 29.9% | HIGH |
| warm-clamp-accfit | 94.9ns | 336.5ns | 28.2% | HIGH |
| warm-clamp-accfit-dyn | 91.4ns | 310.3ns | 29.5% | HIGH |
| warm-clamp-head | 127.0ns | 405.4ns | 31.3% | HIGH |
| warm-clamp-min-lanes | 112.9ns | 213.9ns | 52.8% | HIGH |
| warm-clamp-minimum | 101.6ns | 188.0ns | 54.1% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 281.2-406.6 ns)
    281.2 |
    287.4 |############################
    293.7 |
    300.0 |######################
    306.2 |###########
    312.5 |############################
    318.8 |
    325.1 |#####
    331.4 |
    337.6 |########################################
    343.9 |
    350.2 |#####
    356.4 |
    362.7 |
    369.0 |
    375.3 |############################
    381.5 |###########
    387.8 |
    394.1 |
    400.4 |
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 289.3-407.9 ns)
    289.3 |
    295.2 |
    301.2 |########################################
    307.1 |##########################
    313.0 |
    318.9 |####
    324.9 |
    330.8 |
    336.7 |###################################
    342.7 |
    348.6 |
    354.5 |
    360.5 |
    366.4 |
    372.3 |
    378.2 |###############################
    384.2 |
    390.1 |
    396.0 |
    402.0 |
  (5 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 278.7-349.6 ns)
    278.7 |
    282.3 |
    285.8 |
    289.4 |
    292.9 |
    296.4 |
    300.0 |########################################
    303.5 |######
    307.1 |##
    310.6 |
    314.2 |
    317.7 |
    321.3 |
    324.8 |
    328.4 |
    331.9 |
    335.4 |####################
    339.0 |##
    342.5 |
    346.1 |
  (7 below, 1 above range)

warm-clamp-head (n=40, range 302.4-616.0 ns)
    302.4 |########################################
    318.1 |
    333.8 |#########################
    349.5 |
    365.1 |###
    380.8 |#############################
    396.5 |
    412.2 |
    427.9 |
    443.5 |
    459.2 |
    474.9 |
    490.6 |
    506.2 |
    521.9 |
    537.6 |
    553.3 |
    569.0 |
    584.6 |
    600.3 |################################
  (3 below, 1 above range)

warm-clamp-min-lanes (n=40, range 194.8-241.3 ns)
    194.8 |############################
    197.1 |
    199.4 |
    201.7 |
    204.1 |##
    206.4 |
    208.7 |
    211.1 |####
    213.4 |
    215.7 |
    218.0 |########################################
    220.4 |##
    222.7 |
    225.0 |
    227.4 |
    229.7 |
    232.0 |##
    234.3 |
    236.7 |
    239.0 |
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 161.2-218.8 ns)
    161.2 |
    164.0 |
    166.9 |
    169.8 |
    172.7 |########################################
    175.6 |#######
    178.4 |
    181.3 |
    184.2 |
    187.1 |
    189.9 |
    192.8 |################################
    195.7 |
    198.6 |
    201.5 |###
    204.3 |
    207.2 |
    210.1 |
    213.0 |
    215.9 |#####################
  (7 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: bridge=31.7% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=28.2% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: bridge=28.7% of algo (FFI overhead may distort results)
- **warm-clamp-head**: CV=30.8% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=32.1% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=49.9% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=52.8% of algo (FFI overhead may distort results)
