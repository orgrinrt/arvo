# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (6.64 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 58 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 4876% faster than the next best (warm-container-minimum)

warm-container-kernel (58 ns) leads warm-container-minimum (2.87 us) by 4876%, a clear separation rather than a photo finish. CV 29.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -6.57 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 115.1x slower than the field

warm-container-headroom (6.64 us) is 115.1x the fastest (58 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (4876% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 4876% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 115.1x the fastest

Fastest warm-container-kernel (58 ns) to slowest warm-container-headroom (6.64 us): 115.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 3.8x its best-20%

warm-container-minimum's best 20% of batches run at 2.35 us but its worst 20% at 8.95 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 57.7 ns median (-99.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 115.07x (fastest 57.7 ns, slowest 6639.6 ns)

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
| warm-container-headroom | 6771ns | 6720ns | 5626ns | 6713ns | 8089ns | base |
| warm-container-kernel | 332ns | 310ns | 223ns | 328ns | 453ns | -95.10% |
| warm-container-minimum | 4099ns | 2935ns | 2401ns | 2968ns | 9190ns | -39.46% |
| warm-container-native | 4570ns | 3126ns | 3111ns | 3275ns | 9917ns | -32.50% |
| warm-container-plusone | 4851ns | 4649ns | 4492ns | 4684ns | 5712ns | -28.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6688ns | 5552ns | 8007ns | base | 4.900 |
| warm-container-kernel | 60ns | 42ns | 84ns | -99.10% | 543.506 |
| warm-container-minimum | 4002ns | 2349ns | 8946ns | -40.17% | 8.188 |
| warm-container-native | 4467ns | 3048ns | 9662ns | -33.21% | 7.336 |
| warm-container-plusone | 4784ns | 4432ns | 5624ns | -28.47% | 6.849 |

## Performance model

- Peak throughput: **787.929 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.935 | 0.6% |
| warm-container-kernel | 567.903 | 72.1% |
| warm-container-minimum | 11.412 | 1.4% |
| warm-container-native | 10.700 | 1.4% |
| warm-container-plusone | 7.139 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6771ns | 6771ns | base |
| warm-container-kernel | 332ns | 332ns | -95.10% |
| warm-container-minimum | 4099ns | 4099ns | -39.46% |
| warm-container-native | 4570ns | 4570ns | -32.50% |
| warm-container-plusone | 4851ns | 4851ns | -28.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 6640ns | base | --- | [6213, 7029] | --- | --- | --- | --- |
| warm-container-kernel | 58ns | -6573.8ns (-99.0%) | [-6941, -6164]ns | [57, 58] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2871ns | -3703.0ns (-55.8%) | [-4140, -2872]ns | [2488, 2884] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-native | 3063ns | -3105.4ns (-46.8%) | [-3578, -2743]ns | [3062, 3070] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-plusone | 4590ns | -1883.8ns (-28.4%) | [-2331, -1577]ns | [4545, 4725] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 6928ns | -99.2% | -66.0% | -55.8% | -34.3% |
| 1 | 2 | 6406ns | -99.3% | -63.3% | -52.1% | -27.4% |
| 1 | 3 | 5954ns | -98.8% | -50.1% | -48.6% | -19.7% |
| 1 | 4 | 7425ns | -99.2% | -61.3% | -58.7% | -39.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.338 | moderate- |
| warm-container-kernel | -0.496 | moderate- |
| warm-container-minimum | 0.192 | ok |
| warm-container-native | -0.399 | moderate- |
| warm-container-plusone | -0.376 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 23.8ns | 6688.0ns | 0.4% |  |
| warm-container-kernel | 98.0ns | 60.3ns | 162.6% | HIGH |
| warm-container-minimum | 33.2ns | 4001.7ns | 0.8% |  |
| warm-container-native | 36.2ns | 4466.9ns | 0.8% |  |
| warm-container-plusone | 21.2ns | 4784.1ns | 0.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5551.8-8007.1 ns)
   5551.8 |####################
   5674.6 |####################
   5797.3 |##########
   5920.1 |##############################
   6042.8 |####################
   6165.6 |##############################
   6288.4 |####################
   6411.1 |##########
   6533.9 |##########
   6656.7 |####################
   6779.4 |##########
   6902.2 |##############################
   7024.9 |##############################
   7147.7 |####################
   7270.5 |
   7393.2 |
   7516.0 |##########
   7638.8 |########################################
   7761.5 |
   7884.3 |##########
  (3 below, 3 above range)

warm-container-kernel (n=40, range 41.6-84.4 ns)
     41.6 |##########
     43.7 |##
     45.9 |
     48.0 |
     50.1 |
     52.3 |
     54.4 |
     56.6 |########################################
     58.7 |
     60.8 |
     63.0 |
     65.1 |
     67.3 |
     69.4 |
     71.5 |######
     73.7 |##########
     75.8 |
     78.0 |
     80.1 |
     82.2 |##
  (4 below, 2 above range)

warm-container-minimum (n=40, range 2349.4-8945.6 ns)
   2349.4 |####################################
   2679.2 |########################################
   3009.0 |######
   3338.8 |
   3668.6 |
   3998.4 |
   4328.2 |###
   4658.0 |
   4987.9 |###
   5317.7 |
   5647.5 |
   5977.3 |
   6307.1 |
   6636.9 |
   6966.7 |
   7296.5 |
   7626.3 |######
   7956.1 |###
   8286.0 |###
   8615.8 |
  (3 below, 4 above range)

warm-container-native (n=40, range 3047.6-9662.0 ns)
   3047.6 |########################################
   3378.3 |
   3709.1 |#
   4039.8 |#
   4370.5 |#
   4701.2 |
   5031.9 |
   5362.7 |
   5693.4 |
   6024.1 |
   6354.8 |
   6685.5 |
   7016.3 |
   7347.0 |
   7677.7 |
   8008.4 |#
   8339.1 |
   8669.9 |#
   9000.6 |#
   9331.3 |
  (3 below, 5 above range)

warm-container-plusone (n=40, range 4432.2-5623.7 ns)
   4432.2 |########################################
   4491.8 |#################################
   4551.4 |########################################
   4610.9 |##########################
   4670.5 |######
   4730.1 |#################################
   4789.7 |####################
   4849.2 |##########################
   4908.8 |######
   4968.4 |
   5028.0 |
   5087.5 |
   5147.1 |
   5206.7 |
   5266.3 |
   5325.8 |
   5385.4 |
   5445.0 |
   5504.6 |
   5564.1 |
  (4 below, 1 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=28.1% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=167.8% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=64.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-native**: CV=59.1% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=21.1% (high variance, measurements may be unstable)
