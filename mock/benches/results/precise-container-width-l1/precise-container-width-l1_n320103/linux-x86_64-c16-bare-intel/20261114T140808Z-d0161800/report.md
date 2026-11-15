# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 56 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 5054% faster than the next best (warm-container-native)

warm-container-kernel (56 ns) leads warm-container-native (2.87 us) by 5054%, a clear separation rather than a photo finish. CV 19.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -4.05 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 73.7x slower than the field

warm-container-headroom (4.11 us) is 73.7x the fastest (56 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-native, warm-container-minimum, warm-container-plusone, warm-container-headroom} (5054% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-native, warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 5054% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 73.7x the fastest

Fastest warm-container-kernel (56 ns) to slowest warm-container-headroom (4.11 us): 73.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.9x its best-20%

warm-container-native's best 20% of batches run at 2.41 us but its worst 20% at 9.46 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 55.7 ns median (-98.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 73.74x (fastest 55.7 ns, slowest 4107.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5416ns | 4188ns | 3420ns | 4856ns | 9090ns | base |
| warm-container-kernel | 342ns | 306ns | 275ns | 316ns | 488ns | -93.69% |
| warm-container-minimum | 4109ns | 2934ns | 2400ns | 3138ns | 8733ns | -24.13% |
| warm-container-native | 4369ns | 2934ns | 2468ns | 3217ns | 9726ns | -19.33% |
| warm-container-plusone | 5506ns | 4179ns | 3420ns | 4336ns | 11101ns | +1.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5300ns | 3353ns | 8863ns | base | 6.183 |
| warm-container-kernel | 59ns | 50ns | 76ns | -98.88% | 551.302 |
| warm-container-minimum | 4008ns | 2349ns | 8482ns | -24.37% | 8.175 |
| warm-container-native | 4263ns | 2415ns | 9456ns | -19.57% | 7.687 |
| warm-container-plusone | 5386ns | 3353ns | 10822ns | +1.62% | 6.084 |

## Performance model

- Peak throughput: **654.378 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.978 | 1.2% |
| warm-container-kernel | 588.294 | 89.9% |
| warm-container-minimum | 11.413 | 1.7% |
| warm-container-native | 11.414 | 1.7% |
| warm-container-plusone | 7.999 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5416ns | 5416ns | base |
| warm-container-kernel | 342ns | 342ns | -93.69% |
| warm-container-minimum | 4109ns | 4109ns | -24.13% |
| warm-container-native | 4369ns | 4369ns | -19.33% |
| warm-container-plusone | 5506ns | 5506ns | +1.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4107ns | base | --- | [4094, 4448] | --- | --- | --- | --- |
| warm-container-kernel | 56ns | -4050.6ns (-98.6%) | [-4397, -4038]ns | [51, 58] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2871ns | -1222.6ns (-29.8%) | [-1236, -1006]ns | [2359, 2877] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-native | 2871ns | -1217.1ns (-29.6%) | [-1237, -1037]ns | [2871, 2882] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-plusone | 4097ns | no significant difference | [-6, +737]ns | [4096, 4109] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 7292ns | -99.0% | -67.8% | -60.5% | -54.0% |
| 1 | 2 | 3351ns | -98.5% | -29.9% | -29.9% | +22.4% |
| 1 | 3 | 4107ns | -98.8% | -29.8% | -30.1% | +0.0% |
| 1 | 4 | 4100ns | -98.6% | -29.8% | -29.9% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.251 | moderate- |
| warm-container-kernel | -0.144 | ok |
| warm-container-minimum | 0.247 | moderate+ |
| warm-container-native | -0.420 | moderate- |
| warm-container-plusone | -0.077 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 31.4ns | 5300.1ns | 0.6% |  |
| warm-container-kernel | 101.7ns | 59.4ns | 171.0% | HIGH |
| warm-container-minimum | 34.2ns | 4008.2ns | 0.9% |  |
| warm-container-native | 36.3ns | 4262.6ns | 0.9% |  |
| warm-container-plusone | 32.4ns | 5386.0ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3352.6-8863.1 ns)
   3352.6 |#######
   3628.1 |
   3903.7 |########################################
   4179.2 |
   4454.7 |
   4730.2 |##
   5005.8 |
   5281.3 |
   5556.8 |
   5832.3 |
   6107.9 |
   6383.4 |
   6658.9 |
   6934.4 |
   7210.0 |#######################
   7485.5 |
   7761.0 |
   8036.5 |
   8312.1 |
   8587.6 |
  (6 below, 3 above range)

warm-container-kernel (n=40, range 50.1-75.7 ns)
     50.1 |########################################
     51.4 |###
     52.6 |
     53.9 |
     55.2 |##################
     56.5 |######
     57.8 |
     59.1 |###
     60.3 |
     61.6 |
     62.9 |
     64.2 |###
     65.5 |
     66.7 |
     68.0 |
     69.3 |
     70.6 |
     71.9 |########################
     73.2 |###
     74.4 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 2349.0-8482.0 ns)
   2349.0 |########################################
   2655.6 |################################
   2962.3 |
   3268.9 |
   3575.6 |
   3882.2 |
   4188.9 |
   4495.5 |
   4802.2 |
   5108.8 |
   5415.5 |##
   5722.1 |##
   6028.8 |
   6335.4 |##
   6642.1 |
   6948.7 |
   7255.4 |
   7562.0 |#####
   7868.7 |##
   8175.3 |#####
  (2 below, 3 above range)

warm-container-native (n=40, range 2415.0-9455.6 ns)
   2415.0 |
   2767.0 |########################################
   3119.0 |
   3471.1 |
   3823.1 |#
   4175.1 |
   4527.2 |
   4879.2 |
   5231.2 |###
   5583.3 |
   5935.3 |
   6287.3 |
   6639.4 |
   6991.4 |
   7343.4 |
   7695.5 |#
   8047.5 |
   8399.5 |#
   8751.6 |#
   9103.6 |#
  (7 below, 4 above range)

warm-container-plusone (n=40, range 3353.4-10822.0 ns)
   3353.4 |######
   3726.9 |########################################
   4100.3 |###########################
   4473.7 |
   4847.1 |
   5220.6 |
   5594.0 |###
   5967.4 |######
   6340.9 |
   6714.3 |
   7087.7 |
   7461.1 |
   7834.6 |###
   8208.0 |
   8581.4 |
   8954.8 |
   9328.3 |
   9701.7 |
  10075.1 |
  10448.5 |###
  (6 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=43.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=173.7% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=60.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-native**: CV=63.7% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=55.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.2x (possible bimodal distribution)
