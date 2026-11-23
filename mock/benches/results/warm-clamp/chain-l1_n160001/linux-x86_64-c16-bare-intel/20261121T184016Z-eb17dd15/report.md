# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 61% faster than the next best (warm-clamp-minimum)

warm-clamp-min-lanes (86 ns) leads warm-clamp-minimum (138 ns) by 61%, a clear separation rather than a photo finish. CV 19.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 57% (significant)

warm-clamp-min-lanes is -124 ns (57%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit is an outlier: 2.6x slower than the field

warm-clamp-accfit (220 ns) is 2.6x the fastest (86 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.79)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes} vs {warm-clamp-minimum, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit} (61% apart)

The field splits into a fast tier {warm-clamp-min-lanes} and a slow tier {warm-clamp-minimum, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit} with a 61% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-head is inconsistent: worst-20% is 1.9x its best-20%

warm-clamp-head's best 20% of batches run at 184 ns but its worst 20% at 350 ns (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 85.6 ns median (-60.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.57x (fastest 85.6 ns, slowest 220.3 ns)

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
| warm-clamp-acc64 | 511ns | 501ns | 400ns | 508ns | 629ns | base |
| warm-clamp-accfit | 515ns | 503ns | 394ns | 499ns | 682ns | +0.78% |
| warm-clamp-accfit-dyn | 453ns | 444ns | 402ns | 429ns | 578ns | -11.23% |
| warm-clamp-head | 565ns | 501ns | 420ns | 522ns | 839ns | +10.64% |
| warm-clamp-min-lanes | 319ns | 318ns | 245ns | 308ns | 424ns | -37.58% |
| warm-clamp-minimum | 452ns | 425ns | 334ns | 421ns | 662ns | -11.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 220ns | 175ns | 268ns | base | 37.270 |
| warm-clamp-accfit | 226ns | 173ns | 312ns | +2.69% | 36.294 |
| warm-clamp-accfit-dyn | 200ns | 177ns | 259ns | -9.10% | 40.999 |
| warm-clamp-head | 239ns | 184ns | 350ns | +8.93% | 34.214 |
| warm-clamp-min-lanes | 90ns | 72ns | 118ns | -58.95% | 90.790 |
| warm-clamp-minimum | 142ns | 110ns | 189ns | -35.58% | 57.851 |

## Performance model

- Peak throughput: **114.214 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 37.458 | 32.8% |
| warm-clamp-accfit | 37.186 | 32.6% |
| warm-clamp-accfit-dyn | 45.625 | 39.9% |
| warm-clamp-head | 39.632 | 34.7% |
| warm-clamp-min-lanes | 95.701 | 83.8% |
| warm-clamp-minimum | 59.362 | 52.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 511ns | 511ns | base |
| warm-clamp-accfit | 515ns | 515ns | +0.78% |
| warm-clamp-accfit-dyn | 453ns | 453ns | -11.23% |
| warm-clamp-head | 565ns | 565ns | +10.64% |
| warm-clamp-min-lanes | 319ns | 319ns | -37.58% |
| warm-clamp-minimum | 452ns | 452ns | -11.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 219ns | base | --- | [219, 219] | --- | --- | --- | --- |
| warm-clamp-accfit | 220ns | no significant difference | [-26, +33]ns | [210, 221] | no | 0.7948 | 0.6358 | 0 |
| warm-clamp-accfit-dyn | 180ns | -20.9ns (-9.6%) | [-28, -20]ns | [179, 198] | YES | 0.0107 | 0.0064 | 0 |
| warm-clamp-head | 207ns | no significant difference | [-24, +19]ns | [195, 219] | no | 1.0000 | 1.0000 | 1 |
| warm-clamp-min-lanes | 86ns | -124.5ns (-56.9%) | [-131, -120]ns | [79, 95] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 138ns | -61.5ns (-28.1%) | [-108, -36]ns | [114, 158] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 231ns | -4.8% | -22.7% | -5.4% | -65.8% | -48.5% |
| 1 | 2 | 219ns | +15.1% | -9.4% | +59.9% | -56.6% | -15.8% |
| 1 | 3 | 250ns | -29.5% | -16.6% | -22.1% | -53.0% | -55.9% |
| 1 | 4 | 175ns | +25.9% | +2.0% | +9.6% | -58.9% | -9.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.449 | moderate- |
| warm-clamp-accfit | -0.501 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.271 | moderate- |
| warm-clamp-head | -0.300 | moderate- |
| warm-clamp-min-lanes | -0.362 | moderate- |
| warm-clamp-minimum | -0.791 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 108.9ns | 219.8ns | 49.5% | HIGH |
| warm-clamp-accfit | 106.3ns | 225.7ns | 47.1% | HIGH |
| warm-clamp-accfit-dyn | 92.8ns | 199.8ns | 46.5% | HIGH |
| warm-clamp-head | 121.5ns | 239.4ns | 50.7% | HIGH |
| warm-clamp-min-lanes | 84.9ns | 90.2ns | 94.1% | HIGH |
| warm-clamp-minimum | 107.6ns | 141.6ns | 76.0% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 175.0-267.8 ns)
    175.0 |##############
    179.6 |
    184.3 |
    188.9 |
    193.5 |
    198.2 |
    202.8 |
    207.5 |
    212.1 |
    216.8 |########################################
    221.4 |
    226.0 |
    230.7 |
    235.3 |
    240.0 |##
    244.6 |
    249.2 |################
    253.9 |
    258.5 |
    263.2 |
  (3 below, 2 above range)

warm-clamp-accfit (n=40, range 172.6-312.4 ns)
    172.6 |####################
    179.6 |
    186.6 |
    193.6 |##########
    200.6 |
    207.6 |
    214.5 |########################################
    221.5 |
    228.5 |##
    235.5 |
    242.5 |
    249.5 |#################
    256.5 |
    263.5 |
    270.5 |
    277.5 |
    284.5 |
    291.5 |
    298.4 |##
    305.4 |
  (2 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 177.2-259.1 ns)
    177.2 |########################################
    181.3 |
    185.4 |
    189.5 |
    193.6 |########
    197.7 |####################
    201.8 |
    205.8 |
    209.9 |
    214.0 |##
    218.1 |##
    222.2 |####
    226.3 |
    230.4 |
    234.5 |
    238.6 |
    242.7 |
    246.8 |
    250.9 |
    255.0 |
  (1 below, 1 above range)

warm-clamp-head (n=40, range 183.9-349.9 ns)
    183.9 |##
    192.2 |########################################
    200.5 |
    208.8 |
    217.1 |########################
    225.4 |
    233.7 |
    242.0 |
    250.3 |
    258.6 |
    266.9 |
    275.2 |
    283.5 |
    291.8 |
    300.1 |##
    308.4 |
    316.7 |
    325.0 |
    333.3 |
    341.6 |#####################
  (4 below, 2 above range)

warm-clamp-min-lanes (n=40, range 71.7-118.1 ns)
     71.7 |#########################
     74.0 |##########
     76.4 |##########
     78.7 |########################################
     81.0 |
     83.3 |
     85.6 |
     88.0 |
     90.3 |##########
     92.6 |##########
     94.9 |##############################
     97.2 |
     99.5 |#####
    101.9 |#####
    104.2 |
    106.5 |
    108.8 |
    111.1 |
    113.5 |#####
    115.8 |##########
  (3 below, 5 above range)

warm-clamp-minimum (n=40, range 110.0-189.4 ns)
    110.0 |########################################
    113.9 |###
    117.9 |
    121.9 |##############
    125.9 |
    129.8 |
    133.8 |
    137.8 |##############
    141.7 |
    145.7 |
    149.7 |
    153.7 |###
    157.6 |#########################
    161.6 |
    165.6 |
    169.5 |
    173.5 |
    177.5 |
    181.5 |#####################
    185.4 |###
  (3 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: bridge=49.3% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: CV=35.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: bridge=48.6% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: CV=32.1% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: bridge=49.5% of algo (FFI overhead may distort results)
- **warm-clamp-head**: CV=28.0% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=49.4% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=98.1% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: CV=21.9% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: bridge=73.9% of algo (FFI overhead may distort results)
