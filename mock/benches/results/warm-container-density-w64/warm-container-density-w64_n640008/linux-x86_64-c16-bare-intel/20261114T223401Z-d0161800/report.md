# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 55% (significant)

warm-container-native is -5.39 us (55%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.8x slower than the field

warm-container-plusone (9.76 us) is 2.8x the fastest (3.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 75.2%)

warm-container-kernel wins on median (3.46 us) yet has the highest variance (CV 75.2%), while warm-container-headroom is the steadiest (CV 7.7%, 9.76 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (3.46 us) and warm-container-native (3.49 us) differ by 0.87%, inside the noise, even though the wider field spreads 182.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (180% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 180% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 3.3x its best-20%

warm-container-kernel's best 20% of batches run at 2.91 us but its worst 20% at 9.50 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 3456.3 ns median (-64.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.82x (fastest 3456.3 ns, slowest 9762.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 9284ns | 9847ns | 8057ns | 9488ns | 9898ns | base |
| warm-container-kernel | 4769ns | 3540ns | 2977ns | 3693ns | 9788ns | -48.63% |
| warm-container-minimum | 4466ns | 3571ns | 2923ns | 3459ns | 9029ns | -51.90% |
| warm-container-native | 4652ns | 3568ns | 3552ns | 3582ns | 8960ns | -49.89% |
| warm-container-plusone | 10284ns | 9846ns | 8052ns | 9699ns | 14274ns | +10.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 9195ns | 7987ns | 9793ns | base | 8.018 |
| warm-container-kernel | 4646ns | 2908ns | 9503ns | -49.47% | 15.868 |
| warm-container-minimum | 4354ns | 2856ns | 8779ns | -52.65% | 16.932 |
| warm-container-native | 4534ns | 3470ns | 8701ns | -50.69% | 16.261 |
| warm-container-plusone | 10193ns | 7984ns | 14142ns | +10.85% | 7.233 |

## Performance model

- Peak throughput: **25.816 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.555 | 29.3% |
| warm-container-kernel | 21.331 | 82.6% |
| warm-container-minimum | 21.127 | 81.8% |
| warm-container-native | 21.148 | 81.9% |
| warm-container-plusone | 7.552 | 29.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 9284ns | 9284ns | base |
| warm-container-kernel | 4769ns | 4769ns | -48.63% |
| warm-container-minimum | 4466ns | 4466ns | -51.90% |
| warm-container-native | 4652ns | 4652ns | -49.89% |
| warm-container-plusone | 10284ns | 10284ns | +10.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 9759ns | base | --- | [8705, 9767] | --- | --- | --- | --- |
| warm-container-kernel | 3456ns | -5243.2ns (-53.7%) | [-6300, -5162]ns | [3455, 3468] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3490ns | -5150.2ns (-52.8%) | [-6217, -5086]ns | [2901, 3555] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 3486ns | -5389.5ns (-55.2%) | [-6223, -4867]ns | [3481, 3532] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 9762ns | no significant difference | [-6, +6]ns | [9758, 9765] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 7988ns | -64.5% | -63.6% | -56.4% | -0.0% |
| 1 | 2 | 8705ns | -60.2% | -59.2% | -60.0% | +12.2% |
| 1 | 3 | 9769ns | -64.6% | -70.7% | -63.7% | -0.1% |
| 1 | 4 | 9768ns | -64.5% | -70.7% | -64.4% | +8.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.277 | moderate+ |
| warm-container-kernel | -0.087 | ok |
| warm-container-minimum | -0.374 | moderate- |
| warm-container-native | -0.421 | moderate- |
| warm-container-plusone | -0.016 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.5ns | 9195.4ns | 0.3% |  |
| warm-container-kernel | 34.8ns | 4646.3ns | 0.7% |  |
| warm-container-minimum | 31.6ns | 4354.5ns | 0.7% |  |
| warm-container-native | 33.4ns | 4534.0ns | 0.7% |  |
| warm-container-plusone | 27.0ns | 10193.2ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7987.3-9792.8 ns)
   7987.3 |#########
   8077.6 |
   8167.9 |
   8258.1 |
   8348.4 |
   8438.7 |
   8529.0 |
   8619.2 |##########
   8709.5 |
   8799.8 |
   8890.1 |
   8980.3 |
   9070.6 |
   9160.9 |
   9251.2 |
   9341.4 |#
   9431.7 |#
   9522.0 |
   9612.3 |
   9702.5 |########################################
  (4 below, 1 above range)

warm-container-kernel (n=40, range 2908.3-9502.8 ns)
   2908.3 |
   3238.0 |########################################
   3567.8 |
   3897.5 |
   4227.2 |
   4556.9 |
   4886.7 |
   5216.4 |
   5546.1 |
   5875.8 |
   6205.6 |
   6535.3 |
   6865.0 |#
   7194.7 |#
   7524.4 |#
   7854.2 |
   8183.9 |#
   8513.6 |
   8843.3 |#
   9173.1 |
  (7 below, 4 above range)

warm-container-minimum (n=40, range 2855.9-8779.4 ns)
   2855.9 |########################################
   3152.1 |##
   3448.3 |#########################
   3744.4 |
   4040.6 |
   4336.8 |
   4633.0 |
   4929.1 |
   5225.3 |
   5521.5 |
   5817.6 |
   6113.8 |
   6410.0 |
   6706.2 |##
   7002.3 |
   7298.5 |
   7594.7 |
   7890.8 |####
   8187.0 |
   8483.2 |####
  (2 below, 4 above range)

warm-container-native (n=40, range 3469.8-8700.7 ns)
   3469.8 |########################################
   3731.3 |
   3992.9 |#
   4254.4 |
   4516.0 |
   4777.5 |
   5039.0 |
   5300.6 |
   5562.1 |
   5823.7 |
   6085.2 |
   6346.8 |
   6608.3 |
   6869.9 |
   7131.4 |
   7393.0 |
   7654.5 |#
   7916.1 |#
   8177.6 |
   8439.1 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 7984.0-14142.0 ns)
   7984.0 |############
   8291.9 |
   8599.8 |
   8907.7 |
   9215.6 |
   9523.5 |########################################
   9831.4 |
  10139.3 |
  10447.2 |
  10755.1 |
  11063.0 |
  11370.9 |#
  11678.8 |
  11986.7 |
  12294.6 |
  12602.5 |
  12910.4 |
  13218.3 |
  13526.2 |
  13834.1 |
  (2 below, 4 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=55.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-minimum**: CV=53.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-container-native**: CV=55.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=27.1% (high variance, measurements may be unstable)
