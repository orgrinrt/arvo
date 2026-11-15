# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (11.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 56 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 4988% faster than the next best (warm-container-minimum)

warm-container-kernel (56 ns) leads warm-container-minimum (2.87 us) by 4988%, a clear separation rather than a photo finish. CV 26.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 100% (significant)

warm-container-kernel is -11.37 us (100%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 202.3x slower than the field

warm-container-headroom (11.43 us) is 202.3x the fastest (56 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.59)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (4988% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 4988% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 202.3x the fastest

Fastest warm-container-kernel (56 ns) to slowest warm-container-headroom (11.43 us): 202.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.1x its best-20%

warm-container-native's best 20% of batches run at 2.87 us but its worst 20% at 8.92 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 56.5 ns median (-99.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 202.27x (fastest 56.5 ns, slowest 11428.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 11522ns | 11514ns | 11513ns | 11515ns | 11553ns | base |
| warm-container-kernel | 346ns | 307ns | 252ns | 329ns | 488ns | -97.00% |
| warm-container-minimum | 3867ns | 2957ns | 2954ns | 2961ns | 7497ns | -66.44% |
| warm-container-native | 4331ns | 2962ns | 2954ns | 3155ns | 9237ns | -62.41% |
| warm-container-plusone | 11522ns | 11513ns | 11510ns | 11514ns | 11556ns | -0.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 11430ns | 11426ns | 11438ns | base | 2.867 |
| warm-container-kernel | 61ns | 47ns | 86ns | -99.47% | 541.418 |
| warm-container-minimum | 3749ns | 2873ns | 7235ns | -67.20% | 8.740 |
| warm-container-native | 4200ns | 2873ns | 8923ns | -63.25% | 7.802 |
| warm-container-plusone | 11432ns | 11427ns | 11447ns | +0.02% | 2.866 |

## Performance model

- Peak throughput: **702.611 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.867 | 0.4% |
| warm-container-kernel | 579.965 | 82.5% |
| warm-container-minimum | 11.400 | 1.6% |
| warm-container-native | 11.376 | 1.6% |
| warm-container-plusone | 2.867 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 11522ns | 11522ns | base |
| warm-container-kernel | 346ns | 346ns | -97.00% |
| warm-container-minimum | 3867ns | 3867ns | -66.44% |
| warm-container-native | 4331ns | 4331ns | -62.41% |
| warm-container-plusone | 11522ns | 11522ns | -0.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 11428ns | base | --- | [11428, 11428] | --- | --- | --- | --- |
| warm-container-kernel | 56ns | -11373.9ns (-99.5%) | [-11378, -11371]ns | [51, 57] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2874ns | -8553.1ns (-74.8%) | [-8554, -8542]ns | [2873, 2886] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2880ns | -8547.4ns (-74.8%) | [-8554, -8542]ns | [2873, 2886] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 11428ns | no significant difference | [-1, +1]ns | [11427, 11428] | no | 1.0000 | 1.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 11428ns | -99.6% | -74.8% | -74.8% | -0.0% |
| 1 | 2 | 11428ns | -99.3% | -74.9% | -74.8% | -0.0% |
| 1 | 3 | 11428ns | -99.5% | -74.9% | -74.9% | -0.0% |
| 1 | 4 | 11428ns | -99.5% | -74.8% | -74.7% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.491 | moderate- |
| warm-container-kernel | -0.585 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.235 | moderate- |
| warm-container-native | -0.541 | HIGH- (thermal bounce) |
| warm-container-plusone | 0.079 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.5ns | 11429.6ns | 0.2% |  |
| warm-container-kernel | 102.8ns | 60.5ns | 169.9% | HIGH |
| warm-container-minimum | 32.1ns | 3749.1ns | 0.9% |  |
| warm-container-native | 36.0ns | 4200.0ns | 0.9% |  |
| warm-container-plusone | 25.1ns | 11431.9ns | 0.2% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 11425.6-11438.1 ns)
  11425.6 |
  11426.2 |####
  11426.9 |########################################
  11427.5 |####################################
  11428.1 |####################################
  11428.7 |########
  11429.4 |
  11430.0 |####
  11430.6 |
  11431.2 |
  11431.9 |
  11432.5 |
  11433.1 |
  11433.7 |
  11434.4 |
  11435.0 |
  11435.6 |
  11436.2 |
  11436.9 |
  11437.5 |
  (1 below, 7 above range)

warm-container-kernel (n=40, range 46.6-85.9 ns)
     46.6 |###############################
     48.6 |
     50.6 |###################################
     52.5 |
     54.5 |########
     56.4 |########################################
     58.4 |####
     60.4 |
     62.3 |
     64.3 |
     66.2 |
     68.2 |
     70.2 |
     72.1 |
     74.1 |
     76.0 |
     78.0 |
     80.0 |
     81.9 |
     83.9 |##########################
  (3 below, 4 above range)

warm-container-minimum (n=40, range 2873.0-7234.9 ns)
   2873.0 |########################################
   3091.1 |
   3309.2 |
   3527.3 |
   3745.4 |
   3963.5 |
   4181.6 |
   4399.7 |
   4617.8 |
   4835.9 |
   5054.0 |#
   5272.0 |
   5490.1 |#
   5708.2 |
   5926.3 |
   6144.4 |
   6362.5 |
   6580.6 |
   6798.7 |
   7016.8 |#
  (3 below, 4 above range)

warm-container-native (n=40, range 2872.8-8923.4 ns)
   2872.8 |########################################
   3175.3 |
   3477.9 |#
   3780.4 |
   4082.9 |#
   4385.4 |
   4688.0 |
   4990.5 |
   5293.0 |#
   5595.6 |
   5898.1 |
   6200.6 |
   6503.1 |
   6805.7 |
   7108.2 |
   7410.7 |
   7713.3 |###
   8015.8 |
   8318.3 |####
   8620.8 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 11427.0-11446.8 ns)
  11427.0 |########################################
  11428.0 |############
  11428.9 |##
  11429.9 |
  11430.9 |
  11431.9 |
  11432.9 |
  11433.9 |
  11434.9 |
  11435.9 |
  11436.9 |
  11437.9 |##
  11438.9 |######
  11439.8 |
  11440.8 |####
  11441.8 |##
  11442.8 |
  11443.8 |
  11444.8 |
  11445.8 |##
  (3 below, 2 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=24.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=171.3% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=54.7% (high variance, measurements may be unstable)
- **warm-container-native**: CV=58.3% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.1x (possible bimodal distribution)
