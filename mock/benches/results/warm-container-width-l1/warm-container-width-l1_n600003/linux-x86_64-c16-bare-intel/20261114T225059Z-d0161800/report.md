# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred dominates: 21% faster than the next best (warm-container-minimum)

warm-container-lanes-deferred (1.55 us) leads warm-container-minimum (1.87 us) by 21%, a clear separation rather than a photo finish. CV 136.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-lanes-deferred beats baseline by 55% (significant)

warm-container-lanes-deferred is -1.88 us (55%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-kernel is an outlier: 2.8x slower than the field

warm-container-kernel (4.31 us) is 2.8x the fastest (1.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred is fastest but the noisiest (CV 136.5%)

warm-container-lanes-deferred wins on median (1.55 us) yet has the highest variance (CV 136.5%), while warm-container-kernel is the steadiest (CV 43.8%, 4.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {warm-container-lanes-deferred, warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone, warm-container-kernel} (75% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone, warm-container-kernel} with a 75% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.2x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.54 us but its worst 20% at 6.42 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1549.2 ns median (-54.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.78x (fastest 1549.2 ns, slowest 4309.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4429ns | 3520ns | 2962ns | 3540ns | 8564ns | base |
| warm-container-kernel | 3734ns | 4541ns | 1560ns | 3574ns | 6390ns | -15.69% |
| warm-container-lanes-deferred | 3249ns | 1632ns | 1626ns | 2601ns | 6816ns | -26.65% |
| warm-container-minimum | 3790ns | 1952ns | 1687ns | 3481ns | 6822ns | -14.42% |
| warm-container-native | 3910ns | 2047ns | 2026ns | 3400ns | 7324ns | -11.72% |
| warm-container-plusone | 4002ns | 4289ns | 1674ns | 3924ns | 6564ns | -9.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4312ns | 2891ns | 8306ns | base | 7.599 |
| warm-container-kernel | 3533ns | 1481ns | 6005ns | -18.07% | 9.275 |
| warm-container-lanes-deferred | 3076ns | 1545ns | 6420ns | -28.67% | 10.653 |
| warm-container-minimum | 3615ns | 1612ns | 6472ns | -16.18% | 9.065 |
| warm-container-native | 3737ns | 1942ns | 6964ns | -13.34% | 8.768 |
| warm-container-plusone | 3817ns | 1602ns | 6220ns | -11.49% | 8.585 |

## Performance model

- Peak throughput: **22.133 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.539 | 43.1% |
| warm-container-kernel | 7.604 | 34.4% |
| warm-container-lanes-deferred | 21.152 | 95.6% |
| warm-container-minimum | 17.534 | 79.2% |
| warm-container-native | 16.695 | 75.4% |
| warm-container-plusone | 7.973 | 36.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4429ns | 4429ns | base |
| warm-container-kernel | 3734ns | 3734ns | -15.69% |
| warm-container-lanes-deferred | 3249ns | 3249ns | -26.65% |
| warm-container-minimum | 3790ns | 3790ns | -14.42% |
| warm-container-native | 3910ns | 3910ns | -11.72% |
| warm-container-plusone | 4002ns | 4002ns | -9.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3435ns | base | --- | [3435, 3448] | --- | --- | --- | --- |
| warm-container-kernel | 4309ns | no significant difference | [-1867, +902]ns | [1590, 4608] | no | 0.2682 | 0.2682 | 0 |
| warm-container-lanes-deferred | 1549ns | -1883.2ns (-54.8%) | [-1890, -1262]ns | [1549, 3134] | YES | 0.0002 | 0.0000 | 0 |
| warm-container-minimum | 1869ns | -1575.6ns (-45.9%) | [-1614, -508]ns | [1822, 5430] | YES (adj: no) | 0.1345 | 0.0807 | 0 |
| warm-container-native | 1963ns | -1475.4ns (-43.0%) | [-1490, -844]ns | [1952, 5326] | YES | 0.0415 | 0.0166 | 0 |
| warm-container-plusone | 4110ns | no significant difference | [-1559, +1213]ns | [1867, 5564] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3442ns | +30.2% | -55.0% | -46.2% | -43.3% | +44.6% |
| 1 | 2 | 3435ns | -54.8% | -30.0% | -10.0% | +50.9% | +57.4% |
| 1 | 3 | 3442ns | +31.6% | -6.2% | -13.3% | +18.7% | -33.4% |
| 1 | 4 | 2816ns | +54.3% | -45.0% | +4.4% | -30.0% | -45.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.089 | ok |
| warm-container-kernel | -0.457 | moderate- |
| warm-container-lanes-deferred | -0.294 | moderate- |
| warm-container-minimum | -0.164 | ok |
| warm-container-native | -0.271 | moderate- |
| warm-container-plusone | 0.256 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 30.4ns | 4312.3ns | 0.7% |  |
| warm-container-kernel | 56.0ns | 3533.0ns | 1.6% |  |
| warm-container-lanes-deferred | 53.1ns | 3075.8ns | 1.7% |  |
| warm-container-minimum | 48.6ns | 3614.7ns | 1.3% |  |
| warm-container-native | 46.4ns | 3737.2ns | 1.2% |  |
| warm-container-plusone | 50.7ns | 3816.7ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2891.1-8305.6 ns)
   2891.1 |
   3161.8 |
   3432.6 |########################################
   3703.3 |###
   3974.0 |#
   4244.7 |
   4515.5 |
   4786.2 |
   5056.9 |
   5327.6 |
   5598.4 |#
   5869.1 |
   6139.8 |
   6410.5 |
   6681.3 |
   6952.0 |
   7222.7 |
   7493.4 |
   7764.2 |
   8034.9 |
  (7 below, 5 above range)

warm-container-kernel (n=40, range 1480.5-6004.6 ns)
   1480.5 |########################################
   1706.7 |
   1932.9 |
   2159.1 |##
   2385.3 |
   2611.5 |
   2837.8 |##
   3064.0 |
   3290.2 |
   3516.4 |
   3742.6 |##
   3968.8 |
   4195.0 |##############
   4421.2 |#####
   4647.4 |###########
   4873.6 |########
   5099.8 |########
   5326.0 |
   5552.2 |#####
   5778.4 |
  (2 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1544.8-6419.8 ns)
   1544.8 |########################################
   1788.5 |
   2032.3 |
   2276.0 |
   2519.8 |
   2763.5 |
   3007.3 |#
   3251.0 |#
   3494.8 |
   3738.5 |
   3982.3 |
   4226.0 |
   4469.8 |
   4713.5 |#
   4957.3 |
   5201.1 |###
   5444.8 |#########
   5688.6 |#
   5932.3 |
   6176.1 |
  (4 below, 4 above range)

warm-container-minimum (n=40, range 1612.4-6472.4 ns)
   1612.4 |########################################
   1855.4 |#####
   2098.4 |
   2341.4 |
   2584.4 |
   2827.4 |
   3070.4 |
   3313.4 |
   3556.4 |##
   3799.4 |
   4042.4 |##
   4285.4 |#####
   4528.4 |
   4771.4 |
   5014.4 |
   5257.4 |#####
   5500.4 |########
   5743.4 |##
   5986.4 |###########
   6229.4 |##
  (5 below, 4 above range)

warm-container-native (n=40, range 1941.9-6963.8 ns)
   1941.9 |########################################
   2193.0 |
   2444.1 |
   2695.1 |
   2946.2 |##
   3197.3 |
   3448.4 |
   3699.5 |
   3950.6 |##
   4201.7 |
   4452.8 |
   4703.9 |##
   4955.0 |
   5206.1 |####
   5457.2 |########
   5708.3 |####
   5959.4 |
   6210.5 |####
   6461.6 |##
   6712.7 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1601.6-6220.3 ns)
   1601.6 |####
   1832.6 |########################################
   2063.5 |
   2294.4 |
   2525.4 |########
   2756.3 |
   2987.2 |
   3218.2 |####
   3449.1 |
   3680.0 |
   3911.0 |
   4141.9 |
   4372.8 |
   4603.8 |
   4834.7 |########
   5065.6 |############
   5296.6 |
   5527.5 |############################
   5758.4 |########
   5989.4 |############
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=53.4% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=68.8% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=57.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=57.5% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=51.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
