# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (7.80 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 2.71 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 60% (significant)

warm-container-minimum is -4.71 us (60%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.9x slower than the field

warm-container-headroom (7.80 us) is 2.9x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 89.0%)

warm-container-kernel wins on median (2.71 us) yet has the highest variance (CV 89.0%), while warm-container-headroom is the steadiest (CV 8.0%, 7.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {warm-container-kernel, warm-container-minimum, warm-container-native} vs {warm-container-plusone, warm-container-headroom} (134% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-minimum, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom} with a 134% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-native is inconsistent: worst-20% is 3.5x its best-20%

warm-container-native's best 20% of batches run at 2.47 us but its worst 20% at 8.72 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 2714.9 ns median (-65.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.87x (fastest 2714.9 ns, slowest 7799.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 7545ns | 7886ns | 6454ns | 7769ns | 7963ns | base |
| warm-container-kernel | 4464ns | 2796ns | 2424ns | 3795ns | 8510ns | -40.83% |
| warm-container-minimum | 4165ns | 3022ns | 2569ns | 3159ns | 8781ns | -44.79% |
| warm-container-native | 4255ns | 3105ns | 2535ns | 3243ns | 9010ns | -43.61% |
| warm-container-plusone | 7166ns | 7183ns | 6438ns | 7165ns | 7897ns | -5.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 7459ns | 6383ns | 7863ns | base | 5.491 |
| warm-container-kernel | 4324ns | 2352ns | 8216ns | -42.04% | 9.473 |
| warm-container-minimum | 4047ns | 2503ns | 8502ns | -45.74% | 10.121 |
| warm-container-native | 4133ns | 2467ns | 8723ns | -44.59% | 9.910 |
| warm-container-plusone | 7084ns | 6368ns | 7801ns | -5.03% | 5.782 |

## Performance model

- Peak throughput: **17.412 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.252 | 30.2% |
| warm-container-kernel | 15.087 | 86.6% |
| warm-container-minimum | 13.933 | 80.0% |
| warm-container-native | 13.544 | 77.8% |
| warm-container-plusone | 5.783 | 33.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 7545ns | 7545ns | base |
| warm-container-kernel | 4464ns | 4464ns | -40.83% |
| warm-container-minimum | 4165ns | 4165ns | -44.79% |
| warm-container-native | 4255ns | 4255ns | -43.61% |
| warm-container-plusone | 7166ns | 7166ns | -5.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7799ns | base | --- | [7791, 7807] | --- | --- | --- | --- |
| warm-container-kernel | 2715ns | -3678.6ns (-47.2%) | [-5112, -3340]ns | [2701, 4472] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2940ns | -4709.8ns (-60.4%) | [-4797, -3882]ns | [2534, 3084] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 3024ns | -4698.9ns (-60.3%) | [-4771, -3920]ns | [3012, 3099] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-plusone | 7082ns | -718.8ns (-9.2%) | [-1422, -2]ns | [6378, 7790] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 7803ns | -55.8% | -60.4% | -61.4% | -0.1% |
| 1 | 2 | 6385ns | -57.4% | -60.5% | -61.4% | +22.1% |
| 1 | 3 | 7801ns | -66.8% | -67.7% | -61.3% | -18.3% |
| 1 | 4 | 7813ns | -65.2% | -62.4% | -60.3% | -18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.411 | moderate- |
| warm-container-kernel | -0.009 | ok |
| warm-container-minimum | -0.238 | moderate- |
| warm-container-native | -0.297 | moderate- |
| warm-container-plusone | 0.250 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 23.3ns | 7459.4ns | 0.3% |  |
| warm-container-kernel | 39.3ns | 4323.7ns | 0.9% |  |
| warm-container-minimum | 32.4ns | 4047.1ns | 0.8% |  |
| warm-container-native | 33.1ns | 4133.1ns | 0.8% |  |
| warm-container-plusone | 22.3ns | 7084.2ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 6383.0-7863.3 ns)
   6383.0 |##########
   6457.0 |
   6531.0 |
   6605.1 |
   6679.1 |
   6753.1 |
   6827.1 |
   6901.1 |
   6975.1 |
   7049.1 |
   7123.1 |
   7197.2 |
   7271.2 |
   7345.2 |
   7419.2 |
   7493.2 |
   7567.2 |
   7641.2 |
   7715.2 |##
   7789.3 |########################################
  (3 below, 1 above range)

warm-container-kernel (n=40, range 2352.4-8216.5 ns)
   2352.4 |#################
   2645.6 |########################################
   2938.8 |
   3232.0 |
   3525.2 |
   3818.4 |
   4111.6 |##
   4404.8 |##
   4698.0 |
   4991.2 |
   5284.4 |
   5577.7 |
   5870.9 |
   6164.1 |##
   6457.3 |#####
   6750.5 |#####
   7043.7 |##
   7336.9 |##
   7630.1 |#####
   7923.3 |##
  (5 below, 3 above range)

warm-container-minimum (n=40, range 2503.0-8501.8 ns)
   2503.0 |################################
   2802.9 |########################################
   3102.9 |
   3402.8 |
   3702.7 |##
   4002.7 |
   4302.6 |
   4602.6 |
   4902.5 |
   5202.5 |
   5502.4 |
   5802.3 |
   6102.3 |
   6402.2 |
   6702.2 |
   7002.1 |#####
   7302.1 |#####
   7602.0 |
   7901.9 |
   8201.9 |
  (3 below, 5 above range)

warm-container-native (n=40, range 2467.3-8722.8 ns)
   2467.3 |#####
   2780.1 |########################################
   3092.9 |###############
   3405.7 |
   3718.4 |
   4031.2 |
   4344.0 |#####
   4656.7 |
   4969.5 |
   5282.3 |
   5595.1 |##
   5907.8 |
   6220.6 |
   6533.4 |
   6846.1 |
   7158.9 |
   7471.7 |##
   7784.4 |
   8097.2 |
   8410.0 |##
  (6 below, 5 above range)

warm-container-plusone (n=40, range 6368.3-7800.9 ns)
   6368.3 |#####################################
   6439.9 |
   6511.6 |
   6583.2 |
   6654.8 |
   6726.5 |
   6798.1 |
   6869.7 |
   6941.3 |
   7013.0 |
   7084.6 |
   7156.2 |
   7227.8 |
   7299.5 |
   7371.1 |
   7442.7 |
   7514.4 |
   7586.0 |
   7657.6 |
   7729.2 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=55.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-minimum**: CV=58.9% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-native**: CV=58.3% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.5x (possible bimodal distribution)
