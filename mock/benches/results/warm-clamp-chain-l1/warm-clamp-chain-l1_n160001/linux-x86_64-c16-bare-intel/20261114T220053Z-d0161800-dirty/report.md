# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum beats baseline by 37% (significant)

warm-clamp-minimum is -65 ns (37%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.78)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-minimum} vs {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn} (42% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-minimum} and a slow tier {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-minimum is inconsistent: worst-20% is 1.9x its best-20%

warm-clamp-minimum's best 20% of batches run at 105 ns but its worst 20% at 197 ns (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 114.2 ns median (-34.8% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.94x (fastest 114.2 ns, slowest 221.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 478ns | 402ns | 364ns | 438ns | 709ns | base |
| warm-clamp-accfit | 436ns | 445ns | 335ns | 426ns | 564ns | -8.83% |
| warm-clamp-accfit-dyn | 561ns | 571ns | 400ns | 541ns | 779ns | +17.37% |
| warm-clamp-head | 492ns | 459ns | 446ns | 476ns | 588ns | +3.04% |
| warm-clamp-min-lanes | 465ns | 399ns | 374ns | 433ns | 654ns | -2.62% |
| warm-clamp-minimum | 471ns | 411ns | 346ns | 437ns | 695ns | -1.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 198ns | 159ns | 291ns | base | 41.369 |
| warm-clamp-accfit | 183ns | 147ns | 221ns | -7.45% | 44.702 |
| warm-clamp-accfit-dyn | 225ns | 174ns | 290ns | +13.66% | 36.396 |
| warm-clamp-head | 203ns | 173ns | 240ns | +2.29% | 40.443 |
| warm-clamp-min-lanes | 122ns | 106ns | 148ns | -38.15% | 66.891 |
| warm-clamp-minimum | 137ns | 105ns | 197ns | -30.90% | 59.871 |

## Performance model

- Peak throughput: **78.280 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 46.798 | 59.8% |
| warm-clamp-accfit | 43.925 | 56.1% |
| warm-clamp-accfit-dyn | 36.909 | 47.2% |
| warm-clamp-head | 41.353 | 52.8% |
| warm-clamp-min-lanes | 71.765 | 91.7% |
| warm-clamp-minimum | 66.602 | 85.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 478ns | 478ns | base |
| warm-clamp-accfit | 436ns | 436ns | -8.83% |
| warm-clamp-accfit-dyn | 561ns | 561ns | +17.37% |
| warm-clamp-head | 492ns | 492ns | +3.04% |
| warm-clamp-min-lanes | 465ns | 465ns | -2.62% |
| warm-clamp-minimum | 471ns | 471ns | -1.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 175ns | base | --- | [175, 176] | --- | --- | --- | --- |
| warm-clamp-accfit | 186ns | no significant difference | [-28, +13]ns | [160, 196] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-accfit-dyn | 222ns | no significant difference | [-0, +35]ns | [194, 249] | no | 1.0000 | 0.8746 | 0 |
| warm-clamp-head | 198ns | +21.9ns (+12.5%) | [+14, +23]ns | [197, 199] | YES | 0.0037 | 0.0022 | 0 |
| warm-clamp-min-lanes | 114ns | -64.2ns (-36.6%) | [-69, -60]ns | [112, 127] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 123ns | -64.7ns (-36.9%) | [-69, -58]ns | [115, 135] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 159ns | +3.6% | +22.1% | +13.2% | -23.1% | -15.9% |
| 1 | 2 | 291ns | -24.3% | -14.4% | -32.0% | -49.2% | -33.8% |
| 1 | 3 | 175ns | -16.1% | +42.4% | +27.4% | -35.3% | -38.9% |
| 1 | 4 | 175ns | +11.9% | -0.3% | +13.1% | -36.8% | -34.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.479 | moderate- |
| warm-clamp-accfit | -0.780 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.239 | moderate- |
| warm-clamp-head | -0.053 | ok |
| warm-clamp-min-lanes | -0.170 | ok |
| warm-clamp-minimum | -0.261 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 99.6ns | 198.0ns | 50.3% | HIGH |
| warm-clamp-accfit | 93.7ns | 183.3ns | 51.1% | HIGH |
| warm-clamp-accfit-dyn | 113.5ns | 225.1ns | 50.4% | HIGH |
| warm-clamp-head | 103.0ns | 202.6ns | 50.9% | HIGH |
| warm-clamp-min-lanes | 121.2ns | 122.5ns | 99.0% | HIGH |
| warm-clamp-minimum | 119.3ns | 136.8ns | 87.2% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 158.9-291.4 ns)
    158.9 |############
    165.5 |
    172.2 |########################################
    178.8 |
    185.4 |
    192.0 |
    198.7 |
    205.3 |
    211.9 |
    218.5 |
    225.2 |
    231.8 |
    238.4 |
    245.0 |####
    251.7 |
    258.3 |
    264.9 |
    271.5 |
    278.2 |
    284.8 |######
  (4 below, 5 above range)

warm-clamp-accfit (n=40, range 146.8-221.5 ns)
    146.8 |##########################
    150.6 |
    154.3 |
    158.0 |######################
    161.8 |
    165.5 |####
    169.2 |
    173.0 |#############
    176.7 |####
    180.4 |
    184.2 |
    187.9 |
    191.6 |
    195.4 |########################################
    199.1 |
    202.8 |
    206.6 |
    210.3 |####
    214.0 |
    217.8 |###############################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 174.5-289.6 ns)
    174.5 |##################
    180.2 |
    186.0 |
    191.7 |#############################
    197.5 |
    203.3 |
    209.0 |
    214.8 |
    220.5 |##########
    226.3 |###
    232.0 |
    237.8 |
    243.6 |##################
    249.3 |########################################
    255.1 |
    260.8 |
    266.6 |
    272.4 |
    278.1 |
    283.9 |
  (5 below, 2 above range)

warm-clamp-head (n=40, range 173.0-239.7 ns)
    173.0 |
    176.3 |
    179.7 |###########
    183.0 |
    186.3 |
    189.7 |
    193.0 |####
    196.3 |########################################
    199.7 |
    203.0 |
    206.3 |
    209.7 |
    213.0 |
    216.3 |
    219.7 |###########
    223.0 |########
    226.3 |
    229.7 |##
    233.0 |
    236.3 |
  (4 below, 1 above range)

warm-clamp-min-lanes (n=40, range 105.7-148.5 ns)
    105.7 |
    107.9 |
    110.0 |########################################
    112.1 |#########
    114.3 |######
    116.4 |###
    118.5 |
    120.7 |
    122.8 |
    125.0 |#########
    127.1 |######
    129.2 |###
    131.4 |######
    133.5 |
    135.6 |
    137.8 |
    139.9 |
    142.0 |
    144.2 |
    146.3 |#####################
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 104.7-196.7 ns)
    104.7 |########################################
    109.3 |#################
    113.9 |############################
    118.5 |###########
    123.1 |###########
    127.7 |###########
    132.3 |#################
    136.9 |
    141.5 |
    146.1 |###########
    150.7 |#####
    155.3 |#####
    159.9 |
    164.5 |
    169.1 |#####
    173.7 |
    178.3 |#####
    182.9 |
    187.5 |###########
    192.1 |#####
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=25.4% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: bridge=50.2% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=49.6% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: CV=25.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: bridge=49.5% of algo (FFI overhead may distort results)
- **warm-clamp-head**: bridge=50.4% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=94.7% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: CV=24.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: bridge=86.7% of algo (FFI overhead may distort results)
