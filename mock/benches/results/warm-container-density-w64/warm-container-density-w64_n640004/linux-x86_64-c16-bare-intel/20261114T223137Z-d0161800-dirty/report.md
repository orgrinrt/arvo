# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 43% faster than the next best (warm-container-minimum)

warm-container-kernel (1.93 us) leads warm-container-minimum (2.76 us) by 43%, a clear separation rather than a photo finish. CV 117.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 56% (significant)

warm-container-kernel is -3.11 us (56%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.9x slower than the field

warm-container-plusone (5.54 us) is 2.9x the fastest (1.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 117.9%)

warm-container-kernel wins on median (1.93 us) yet has the highest variance (CV 117.9%), while warm-container-headroom is the steadiest (CV 40.1%, 5.52 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel is inconsistent: worst-20% is 4.6x its best-20%

warm-container-kernel's best 20% of batches run at 1.57 us but its worst 20% at 7.25 us (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1927.0 ns median (-65.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.87x (fastest 1927.0 ns, slowest 5539.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6022ns | 5603ns | 4582ns | 5560ns | 8848ns | base |
| warm-container-kernel | 3732ns | 2009ns | 1638ns | 3139ns | 7605ns | -38.03% |
| warm-container-minimum | 3909ns | 2878ns | 1956ns | 3477ns | 7159ns | -35.08% |
| warm-container-native | 3931ns | 4198ns | 1682ns | 3805ns | 6557ns | -34.72% |
| warm-container-plusone | 6472ns | 5622ns | 4099ns | 5918ns | 10506ns | +7.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5923ns | 4514ns | 8669ns | base | 6.915 |
| warm-container-kernel | 3569ns | 1571ns | 7249ns | -39.76% | 11.478 |
| warm-container-minimum | 3737ns | 1876ns | 6826ns | -36.92% | 10.962 |
| warm-container-native | 3762ns | 1613ns | 6255ns | -36.49% | 10.888 |
| warm-container-plusone | 6369ns | 4031ns | 10324ns | +7.52% | 6.432 |

## Performance model

- Peak throughput: **26.075 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.421 | 28.5% |
| warm-container-kernel | 21.256 | 81.5% |
| warm-container-minimum | 14.836 | 56.9% |
| warm-container-native | 10.170 | 39.0% |
| warm-container-plusone | 7.394 | 28.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6022ns | 6022ns | base |
| warm-container-kernel | 3732ns | 3732ns | -38.03% |
| warm-container-minimum | 3909ns | 3909ns | -35.08% |
| warm-container-native | 3931ns | 3931ns | -34.72% |
| warm-container-plusone | 6472ns | 6472ns | +7.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5520ns | base | --- | [5511, 5524] | --- | --- | --- | --- |
| warm-container-kernel | 1927ns | -3108.0ns (-56.3%) | [-3606, -2124]ns | [1920, 4740] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-minimum | 2761ns | -2596.8ns (-47.0%) | [-3588, -1161]ns | [1921, 4836] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-native | 4028ns | -2485.9ns (-45.0%) | [-3586, -881]ns | [1931, 5070] | YES | 0.0009 | 0.0007 | 0 |
| warm-container-plusone | 5539ns | no significant difference | [-984, +31]ns | [4539, 5605] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 5523ns | -71.5% | -59.8% | -22.2% | -27.1% |
| 1 | 2 | 5524ns | -65.2% | -65.1% | -17.7% | +0.6% |
| 1 | 3 | 4516ns | -27.8% | -57.5% | +14.9% | +115.8% |
| 1 | 4 | 5524ns | -21.5% | -15.3% | -64.8% | -17.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.417 | moderate- |
| warm-container-kernel | 0.282 | moderate+ |
| warm-container-minimum | -0.111 | ok |
| warm-container-native | -0.268 | moderate- |
| warm-container-plusone | -0.304 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.4ns | 5923.5ns | 0.5% |  |
| warm-container-kernel | 47.6ns | 3568.6ns | 1.3% |  |
| warm-container-minimum | 49.8ns | 3736.5ns | 1.3% |  |
| warm-container-native | 50.0ns | 3761.9ns | 1.3% |  |
| warm-container-plusone | 30.7ns | 6368.7ns | 0.5% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 4513.9-8668.8 ns)
   4513.9 |#####
   4721.6 |
   4929.3 |
   5137.1 |
   5344.8 |########################################
   5552.6 |
   5760.3 |
   5968.1 |
   6175.8 |
   6383.6 |
   6591.3 |
   6799.1 |
   7006.8 |
   7214.6 |
   7422.3 |#
   7630.1 |
   7837.8 |
   8045.6 |
   8253.3 |
   8461.1 |
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1570.9-7249.3 ns)
   1570.9 |###############
   1854.8 |########################################
   2138.7 |
   2422.6 |
   2706.6 |
   2990.5 |
   3274.4 |
   3558.3 |###
   3842.3 |
   4126.2 |###
   4410.1 |###
   4694.0 |######
   4978.0 |######
   5261.9 |###
   5545.8 |###
   5829.7 |
   6113.6 |###
   6397.6 |###
   6681.5 |###
   6965.4 |######
  (5 below, 3 above range)

warm-container-minimum (n=40, range 1875.5-6825.7 ns)
   1875.5 |########################################
   2123.0 |
   2370.5 |##
   2618.0 |
   2865.6 |##
   3113.1 |##
   3360.6 |
   3608.1 |
   3855.6 |
   4103.1 |##
   4350.6 |##
   4598.1 |####
   4845.6 |####
   5093.2 |####
   5340.7 |
   5588.2 |####
   5835.7 |##
   6083.2 |##
   6330.7 |
   6578.2 |##
  (1 below, 5 above range)

warm-container-native (n=40, range 1613.1-6255.1 ns)
   1613.1 |
   1845.2 |########################################
   2077.3 |####
   2309.4 |####
   2541.5 |
   2773.6 |
   3005.7 |
   3237.8 |####
   3469.9 |####
   3702.0 |
   3934.1 |
   4166.2 |
   4398.3 |########
   4630.4 |####
   4862.5 |#################
   5094.6 |#############
   5326.7 |########
   5558.8 |########
   5790.9 |########
   6023.0 |
  (7 below, 4 above range)

warm-container-plusone (n=40, range 4031.0-10323.6 ns)
   4031.0 |############
   4345.6 |################################
   4660.2 |####
   4974.9 |
   5289.5 |################################
   5604.1 |########
   5918.8 |
   6233.4 |
   6548.0 |
   6862.6 |
   7177.3 |
   7491.9 |
   7806.5 |
   8121.2 |
   8435.8 |
   8750.4 |
   9065.1 |
   9379.7 |
   9694.3 |########################################
  10009.0 |####
  (6 below, 1 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=37.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=63.6% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-minimum**: CV=53.9% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-native**: CV=49.9% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=40.9% (high variance, measurements may be unstable)
