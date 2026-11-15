# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (7.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 5603% faster than the next best (warm-container-minimum)

warm-container-kernel (52 ns) leads warm-container-minimum (2.94 us) by 5603%, a clear separation rather than a photo finish. CV 17.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -6.98 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 136.8x slower than the field

warm-container-headroom (7.05 us) is 136.8x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (5603% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 5603% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 136.8x the fastest

Fastest warm-container-kernel (52 ns) to slowest warm-container-headroom (7.05 us): 136.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.9x its best-20%

warm-container-native's best 20% of batches run at 2.44 us but its worst 20% at 9.44 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 51.5 ns median (-99.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 136.83x (fastest 51.5 ns, slowest 7047.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 7038ns | 7119ns | 5784ns | 7081ns | 8162ns | base |
| warm-container-kernel | 321ns | 278ns | 252ns | 299ns | 457ns | -95.43% |
| warm-container-minimum | 4084ns | 3000ns | 2954ns | 3044ns | 8332ns | -41.98% |
| warm-container-native | 4312ns | 3047ns | 2488ns | 3123ns | 9699ns | -38.74% |
| warm-container-plusone | 6563ns | 6707ns | 5443ns | 6735ns | 7167ns | -6.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6964ns | 5719ns | 8084ns | base | 4.705 |
| warm-container-kernel | 56ns | 47ns | 71ns | -99.20% | 589.538 |
| warm-container-minimum | 3988ns | 2891ns | 8106ns | -42.74% | 8.217 |
| warm-container-native | 4211ns | 2437ns | 9444ns | -39.53% | 7.781 |
| warm-container-plusone | 6488ns | 5384ns | 7074ns | -6.83% | 5.050 |

## Performance model

- Peak throughput: **700.545 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.650 | 0.7% |
| warm-container-kernel | 636.272 | 90.8% |
| warm-container-minimum | 11.157 | 1.6% |
| warm-container-native | 10.980 | 1.6% |
| warm-container-plusone | 4.938 | 0.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 7038ns | 7038ns | base |
| warm-container-kernel | 321ns | 321ns | -95.43% |
| warm-container-minimum | 4084ns | 4084ns | -41.98% |
| warm-container-native | 4312ns | 4312ns | -38.74% |
| warm-container-plusone | 6563ns | 6563ns | -6.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7047ns | base | --- | [6993, 7148] | --- | --- | --- | --- |
| warm-container-kernel | 52ns | -6978.9ns (-99.0%) | [-7097, -6918]ns | [51, 55] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2937ns | -4011.8ns (-56.9%) | [-4119, -2856]ns | [2909, 3060] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2984ns | -4047.2ns (-57.4%) | [-4112, -3315]ns | [2965, 3033] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-plusone | 6636ns | -423.1ns (-6.0%) | [-1205, -164]ns | [6596, 6750] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 7126ns | -99.3% | -57.0% | -57.8% | -23.4% |
| 1 | 2 | 8014ns | -99.4% | -63.7% | -62.6% | -15.5% |
| 1 | 3 | 5730ns | -99.2% | -49.2% | -57.3% | +15.7% |
| 1 | 4 | 7004ns | -99.0% | -58.4% | -56.9% | -4.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.442 | moderate- |
| warm-container-kernel | -0.242 | moderate- |
| warm-container-minimum | -0.070 | ok |
| warm-container-native | -0.423 | moderate- |
| warm-container-plusone | -0.164 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 23.8ns | 6964.0ns | 0.3% |  |
| warm-container-kernel | 93.8ns | 55.6ns | 168.8% | HIGH |
| warm-container-minimum | 32.8ns | 3987.7ns | 0.8% |  |
| warm-container-native | 34.7ns | 4211.4ns | 0.8% |  |
| warm-container-plusone | 24.4ns | 6488.3ns | 0.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5718.7-8084.1 ns)
   5718.7 |########################
   5836.9 |
   5955.2 |
   6073.5 |
   6191.7 |
   6310.0 |
   6428.3 |
   6546.5 |
   6664.8 |
   6783.1 |########
   6901.4 |################
   7019.6 |########################################
   7137.9 |############
   7256.2 |
   7374.4 |####
   7492.7 |####
   7611.0 |
   7729.2 |####
   7847.5 |
   7965.8 |####################
  (4 below, 3 above range)

warm-container-kernel (n=40, range 46.8-71.2 ns)
     46.8 |####################
     48.0 |
     49.2 |
     50.4 |########################################
     51.7 |#############
     52.9 |
     54.1 |
     55.3 |
     56.5 |######
     57.7 |###
     59.0 |
     60.2 |###
     61.4 |
     62.6 |###
     63.8 |##########
     65.1 |
     66.3 |
     67.5 |
     68.7 |###
     69.9 |
  (4 below, 5 above range)

warm-container-minimum (n=40, range 2891.0-8105.9 ns)
   2891.0 |########################################
   3151.7 |
   3412.5 |
   3673.2 |
   3934.0 |
   4194.7 |#
   4455.5 |
   4716.2 |#
   4977.0 |
   5237.7 |
   5498.5 |
   5759.2 |
   6019.9 |
   6280.7 |
   6541.4 |#
   6802.2 |
   7062.9 |#
   7323.7 |
   7584.4 |
   7845.2 |
  (3 below, 4 above range)

warm-container-native (n=40, range 2436.7-9443.7 ns)
   2436.7 |#######
   2787.0 |########################################
   3137.4 |
   3487.7 |
   3838.1 |
   4188.4 |
   4538.8 |
   4889.1 |#
   5239.5 |
   5589.8 |
   5940.2 |
   6290.6 |
   6640.9 |
   6991.3 |
   7341.6 |
   7692.0 |
   8042.3 |
   8392.7 |###
   8743.0 |###
   9093.4 |#
  (5 below, 3 above range)

warm-container-plusone (n=40, range 5384.4-7073.8 ns)
   5384.4 |###
   5468.9 |######
   5553.4 |
   5637.8 |###
   5722.3 |
   5806.8 |
   5891.2 |
   5975.7 |
   6060.2 |
   6144.6 |
   6229.1 |
   6313.6 |
   6398.0 |
   6482.5 |##########
   6567.0 |########################################
   6651.4 |######
   6735.9 |##############################
   6820.4 |##########
   6904.9 |
   6989.3 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-kernel**: bridge=169.3% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=59.1% (high variance, measurements may be unstable)
- **warm-container-native**: CV=63.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.9x (possible bimodal distribution)
