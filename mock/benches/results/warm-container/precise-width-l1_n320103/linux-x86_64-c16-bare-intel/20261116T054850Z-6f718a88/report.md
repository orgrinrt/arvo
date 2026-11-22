# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 4713% faster than the next best (warm-container-native)

warm-container-kernel (60 ns) leads warm-container-native (2.87 us) by 4713%, a clear separation rather than a photo finish. CV 68.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 98% (significant)

warm-container-kernel is -3.29 us (98%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 68.7x slower than the field

warm-container-plusone (4.10 us) is 68.7x the fastest (60 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} (4713% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 4713% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 68.7x the fastest

Fastest warm-container-kernel (60 ns) to slowest warm-container-plusone (4.10 us): 68.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.6x its best-20%

warm-container-native's best 20% of batches run at 2.35 us but its worst 20% at 8.53 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 59.6 ns median (-98.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 68.66x (fastest 59.6 ns, slowest 4095.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112536030 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4358ns | 3418ns | 3352ns | 3410ns | 8211ns | base |
| warm-container-kernel | 340ns | 343ns | 277ns | 326ns | 445ns | -92.20% |
| warm-container-minimum | 4468ns | 2936ns | 2536ns | 3605ns | 8990ns | +2.52% |
| warm-container-native | 4042ns | 2934ns | 2400ns | 3008ns | 8786ns | -7.26% |
| warm-container-plusone | 4781ns | 4177ns | 3417ns | 3929ns | 8701ns | +9.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4264ns | 3286ns | 8002ns | base | 7.686 |
| warm-container-kernel | 64ns | 50ns | 96ns | -98.49% | 509.413 |
| warm-container-minimum | 4360ns | 2481ns | 8742ns | +2.26% | 7.516 |
| warm-container-native | 3941ns | 2349ns | 8530ns | -7.56% | 8.315 |
| warm-container-plusone | 4678ns | 3351ns | 8480ns | +9.72% | 7.005 |

## Performance model

- Peak throughput: **650.159 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.778 | 1.5% |
| warm-container-kernel | 549.338 | 84.5% |
| warm-container-minimum | 11.412 | 1.8% |
| warm-container-native | 11.415 | 1.8% |
| warm-container-plusone | 8.001 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4358ns | 4358ns | base |
| warm-container-kernel | 340ns | 340ns | -92.20% |
| warm-container-minimum | 4468ns | 4468ns | +2.52% |
| warm-container-native | 4042ns | 4042ns | -7.26% |
| warm-container-plusone | 4781ns | 4781ns | +9.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3351ns | base | --- | [3350, 3361] | --- | --- | --- | --- |
| warm-container-kernel | 60ns | -3294.5ns (-98.3%) | [-3300, -3287]ns | [56, 63] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2871ns | -415.0ns (-12.4%) | [-485, -414]ns | [2871, 2885] | YES | 0.0022 | 0.0022 | 0 |
| warm-container-native | 2871ns | -499.5ns (-14.9%) | [-997, -415]ns | [2859, 2876] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4096ns | +399.4ns (+11.9%) | [+55, +745]ns | [3362, 4096] | YES | 0.0009 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3352ns | -98.1% | -29.7% | -29.6% | +0.1% |
| 1 | 2 | 3352ns | -98.3% | -14.1% | -29.7% | +22.2% |
| 1 | 3 | 3302ns | -98.5% | -12.9% | -13.1% | +24.2% |
| 1 | 4 | 3291ns | -98.1% | -12.5% | -12.6% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.285 | moderate+ |
| warm-container-kernel | -0.289 | moderate- |
| warm-container-minimum | -0.084 | ok |
| warm-container-native | 0.250 | moderate+ |
| warm-container-plusone | -0.250 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.9ns | 4263.5ns | 0.6% |  |
| warm-container-kernel | 102.9ns | 64.3ns | 159.9% | HIGH |
| warm-container-minimum | 38.4ns | 4359.7ns | 0.9% |  |
| warm-container-native | 33.7ns | 3941.0ns | 0.9% |  |
| warm-container-plusone | 28.4ns | 4677.7ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3285.7-8002.1 ns)
   3285.7 |########################################
   3521.5 |
   3757.3 |
   3993.2 |
   4229.0 |#
   4464.8 |
   4700.6 |
   4936.4 |#
   5172.3 |
   5408.1 |
   5643.9 |
   5879.7 |
   6115.5 |
   6351.4 |
   6587.2 |
   6823.0 |
   7058.8 |
   7294.7 |
   7530.5 |
   7766.3 |#
  (4 below, 4 above range)

warm-container-kernel (n=40, range 50.4-95.5 ns)
     50.4 |#####################
     52.7 |
     54.9 |################
     57.2 |
     59.4 |
     61.7 |########################################
     63.9 |####
     66.2 |
     68.4 |
     70.7 |
     72.9 |
     75.2 |
     77.5 |
     79.7 |
     82.0 |
     84.2 |
     86.5 |
     88.7 |
     91.0 |
     93.2 |
  (4 below, 1 above range)

warm-container-minimum (n=40, range 2481.0-8742.0 ns)
   2481.0 |
   2794.0 |########################################
   3107.1 |
   3420.1 |
   3733.2 |
   4046.2 |
   4359.3 |
   4672.3 |#
   4985.4 |
   5298.4 |
   5611.5 |#
   5924.5 |#
   6237.6 |#
   6550.6 |
   6863.7 |#
   7176.7 |#
   7489.8 |#
   7802.8 |
   8115.9 |#
   8428.9 |#
  (6 below, 4 above range)

warm-container-native (n=40, range 2348.7-8530.0 ns)
   2348.7 |####################
   2657.7 |########################################
   2966.8 |##
   3275.9 |
   3584.9 |
   3894.0 |
   4203.1 |
   4512.1 |
   4821.2 |
   5130.3 |
   5439.3 |
   5748.4 |
   6057.5 |
   6366.6 |##
   6675.6 |
   6984.7 |####
   7293.8 |####
   7602.8 |
   7911.9 |####
   8221.0 |
  (3 below, 2 above range)

warm-container-plusone (n=40, range 3350.7-8479.9 ns)
   3350.7 |############################
   3607.2 |
   3863.6 |########################################
   4120.1 |
   4376.5 |
   4633.0 |
   4889.4 |
   5145.9 |
   5402.4 |##
   5658.8 |
   5915.3 |
   6171.7 |##
   6428.2 |
   6684.6 |
   6941.1 |##
   7197.6 |
   7454.0 |
   7710.5 |
   7966.9 |
   8223.4 |
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=54.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=63.4% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=182.5% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=57.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-native**: CV=63.5% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=50.2% (high variance, measurements may be unstable)
