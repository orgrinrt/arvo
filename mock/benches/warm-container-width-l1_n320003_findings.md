# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.72 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 802 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 86% (significant)

warm-container-minimum is -4.93 us (86%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 7.1x slower than the field

warm-container-headroom (5.72 us) is 7.1x the fastest (802 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.53)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-plusone, warm-container-headroom} (537% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-plusone, warm-container-headroom} with a 537% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.1x the fastest

Fastest warm-container-native (802 ns) to slowest warm-container-headroom (5.72 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 801.7 ns median (-86.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 7.13x (fastest 801.7 ns, slowest 5717.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5791ns | 5782ns | 5701ns | 5771ns | 5943ns | base |
| warm-container-kernel | 965ns | 958ns | 954ns | 960ns | 990ns | -83.34% |
| warm-container-lanes-deferred | 967ns | 959ns | 953ns | 960ns | 1004ns | -83.30% |
| warm-container-minimum | 916ns | 878ns | 862ns | 882ns | 1074ns | -84.18% |
| warm-container-native | 883ns | 866ns | 860ns | 871ns | 943ns | -84.76% |
| warm-container-plusone | 5790ns | 5777ns | 5699ns | 5774ns | 5930ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5729ns | 5642ns | 5879ns | base | 5.720 |
| warm-container-kernel | 904ns | 894ns | 928ns | -84.22% | 36.258 |
| warm-container-lanes-deferred | 906ns | 893ns | 939ns | -84.19% | 36.175 |
| warm-container-minimum | 844ns | 799ns | 973ns | -85.26% | 38.808 |
| warm-container-native | 819ns | 799ns | 876ns | -85.70% | 40.007 |
| warm-container-plusone | 5728ns | 5643ns | 5864ns | -0.00% | 5.720 |

## Performance model

- Peak throughput: **41.028 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.731 | 14.0% |
| warm-container-kernel | 36.553 | 89.1% |
| warm-container-lanes-deferred | 36.510 | 89.0% |
| warm-container-minimum | 40.290 | 98.2% |
| warm-container-native | 40.873 | 99.6% |
| warm-container-plusone | 5.734 | 14.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5791ns | 5791ns | base |
| warm-container-kernel | 965ns | 965ns | -83.34% |
| warm-container-lanes-deferred | 967ns | 967ns | -83.30% |
| warm-container-minimum | 916ns | 916ns | -84.18% |
| warm-container-native | 883ns | 883ns | -84.76% |
| warm-container-plusone | 5790ns | 5790ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5718ns | base | --- | [5650, 5737] | --- | --- | --- | --- |
| warm-container-kernel | 896ns | -4818.8ns (-84.3%) | [-4828, -4751]ns | [895, 902] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 898ns | -4808.6ns (-84.1%) | [-4827, -4751]ns | [896, 899] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 813ns | -4910.6ns (-85.9%) | [-4930, -4838]ns | [811, 815] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 802ns | -4895.5ns (-85.6%) | [-4925, -4851]ns | [801, 812] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 5715ns | no significant difference | [-37, +12]ns | [5667, 5742] | no | 1.0000 | 1.0000 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5696ns | -84.3% | -84.3% | -85.2% | -86.0% | +0.6% |
| 2 | 5642ns | -84.1% | -84.2% | -85.1% | -83.4% | +0.3% |
| 3 | 5645ns | -84.2% | -84.1% | -85.1% | -85.8% | +3.5% |
| 4 | 5641ns | -84.1% | -84.1% | -85.1% | -85.8% | +0.1% |
| 5 | 5642ns | -84.1% | -84.1% | -85.1% | -83.8% | +0.0% |
| 6 | 5643ns | -84.1% | -84.2% | -85.1% | -85.8% | +0.0% |
| 7 | 5644ns | -84.2% | -84.2% | -85.1% | -85.8% | +0.0% |
| 8 | 5645ns | -84.2% | -84.1% | -85.1% | -85.8% | +3.5% |
| 9 | 5716ns | -84.3% | -84.3% | -83.0% | -86.0% | +1.9% |
| 10 | 5713ns | -84.3% | -84.3% | -85.8% | -86.0% | -0.8% |
| 11 | 5644ns | -84.1% | -84.2% | -85.6% | -85.8% | +0.0% |
| 12 | 5719ns | -84.3% | -84.3% | -86.0% | -86.0% | -0.2% |
| 13 | 5772ns | -84.4% | -84.4% | -86.2% | -86.2% | -2.2% |
| 14 | 5769ns | -84.2% | -84.6% | -86.2% | -86.1% | -1.8% |
| 15 | 5956ns | -84.7% | -85.0% | -86.6% | -86.6% | -4.7% |
| 16 | 5654ns | -83.9% | -84.2% | -85.9% | -85.9% | -0.1% |
| 17 | 5641ns | -83.9% | -84.1% | -85.8% | -85.8% | +0.1% |
| 18 | 5680ns | -83.2% | -84.2% | -85.8% | -85.9% | +0.6% |
| 19 | 5773ns | -84.3% | -84.4% | -86.0% | -86.1% | -2.3% |
| 20 | 5732ns | -84.1% | -84.3% | -86.0% | -86.0% | -1.6% |
| 21 | 5646ns | -84.1% | -84.1% | -85.6% | -83.9% | +2.3% |
| 22 | 5646ns | -84.1% | -84.1% | -85.7% | -85.9% | +1.7% |
| 23 | 5642ns | -84.1% | -84.1% | -85.6% | -85.8% | +2.1% |
| 24 | 5692ns | -84.3% | -84.2% | -85.7% | -85.7% | +2.1% |
| 25 | 5646ns | -84.2% | -84.1% | -85.6% | -85.8% | +2.3% |
| 26 | 5643ns | -84.1% | -84.1% | -74.1% | -85.8% | +3.3% |
| 27 | 5860ns | -84.7% | -84.6% | -86.4% | -86.3% | -2.7% |
| 28 | 5756ns | -84.5% | -84.4% | -85.7% | -86.0% | +1.7% |
| 29 | 5810ns | -84.6% | -82.7% | -86.0% | -85.8% | -1.2% |
| 30 | 5755ns | -84.5% | -82.3% | -85.8% | -86.1% | +1.3% |
| 31 | 5736ns | -84.2% | -84.2% | -85.8% | -85.3% | -1.5% |
| 32 | 5736ns | -84.1% | -84.0% | -85.8% | -85.1% | -0.4% |
| 33 | 5846ns | -84.4% | -84.2% | -86.1% | -85.2% | -1.5% |
| 34 | 5737ns | -84.1% | -84.2% | -85.8% | -85.3% | +0.1% |
| 35 | 5737ns | -84.2% | -84.1% | -85.9% | -85.3% | -0.2% |
| 36 | 5808ns | -84.4% | -84.3% | -86.0% | -85.7% | -1.2% |
| 37 | 5794ns | -82.6% | -84.3% | -86.0% | -85.6% | -1.3% |
| 38 | 5734ns | -84.4% | -84.2% | -85.8% | -85.8% | -1.5% |
| 39 | 6112ns | -85.3% | -85.2% | -81.4% | -86.7% | -0.5% |
| 40 | 5845ns | -84.7% | -84.4% | -85.9% | -86.0% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.345 | moderate+ |
| warm-container-kernel | 0.090 | ok |
| warm-container-lanes-deferred | 0.527 | HIGH+ (drift/warm-up) |
| warm-container-minimum | -0.065 | ok |
| warm-container-native | -0.022 | ok |
| warm-container-plusone | 0.093 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 19/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.6ns | 5728.7ns | 0.0% |  |
| warm-container-kernel | 2.0ns | 903.8ns | 0.2% |  |
| warm-container-lanes-deferred | 2.4ns | 905.8ns | 0.3% |  |
| warm-container-minimum | 2.8ns | 844.4ns | 0.3% |  |
| warm-container-native | 2.5ns | 819.1ns | 0.3% |  |
| warm-container-plusone | 2.9ns | 5728.4ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5642.2-5879.0 ns)
   5642.2 |########################################
   5654.0 |
   5665.9 |
   5677.7 |###
   5689.5 |#######
   5701.4 |###
   5713.2 |#######
   5725.1 |##################
   5736.9 |###
   5748.7 |#######
   5760.6 |#######
   5772.4 |###
   5784.3 |###
   5796.1 |###
   5808.0 |###
   5819.8 |
   5831.6 |
   5843.5 |#######
   5855.3 |###
   5867.2 |
  (4 below, 2 above range)

warm-container-kernel (n=40, range 893.5-927.8 ns)
    893.5 |########################################
    895.3 |########################################
    897.0 |#############
    898.7 |########
    900.4 |
    902.1 |
    903.8 |####
    905.5 |####
    907.2 |#############
    909.0 |######################
    910.7 |########
    912.4 |
    914.1 |
    915.8 |
    917.5 |
    919.2 |
    921.0 |
    922.7 |
    924.4 |
    926.1 |
  (3 below, 2 above range)

warm-container-lanes-deferred (n=40, range 893.4-938.6 ns)
    893.4 |#########################
    895.6 |########################################
    897.9 |#####################
    900.1 |###
    902.4 |
    904.7 |
    906.9 |##############
    909.2 |##########
    911.4 |###
    913.7 |
    916.0 |
    918.2 |###
    920.5 |###
    922.7 |
    925.0 |
    927.2 |
    929.5 |
    931.8 |
    934.0 |
    936.3 |
  (3 below, 2 above range)

warm-container-minimum (n=40, range 799.3-972.8 ns)
    799.3 |########
    808.0 |########################################
    816.6 |####
    825.3 |
    834.0 |#################
    842.7 |
    851.3 |
    860.0 |
    868.7 |
    877.4 |
    886.0 |
    894.7 |
    903.4 |
    912.1 |
    920.7 |
    929.4 |
    938.1 |
    946.8 |
    955.4 |
    964.1 |##
  (5 below, 2 above range)

warm-container-native (n=40, range 798.7-875.8 ns)
    798.7 |########################################
    802.5 |###
    806.4 |
    810.2 |###
    814.1 |###
    818.0 |
    821.8 |#
    825.7 |
    829.5 |###
    833.4 |
    837.2 |
    841.1 |#####
    844.9 |
    848.8 |
    852.7 |
    856.5 |#
    860.4 |#
    864.2 |
    868.1 |
    871.9 |
  (2 below, 3 above range)

warm-container-plusone (n=40, range 5643.3-5864.3 ns)
   5643.3 |########################################
   5654.4 |##########
   5665.4 |##########
   5676.5 |
   5687.5 |
   5698.6 |##########
   5709.6 |###############
   5720.7 |##########
   5731.7 |###############
   5742.8 |#####
   5753.8 |##########
   5764.9 |#####
   5775.9 |##########
   5787.0 |
   5798.0 |
   5809.1 |#####
   5820.1 |###############
   5831.2 |##########
   5842.2 |#####
   5853.3 |
  (4 below, 1 above range)

```

## Diagnostics

- **warm-container-lanes-deferred**: autocorrelation=0.53 (measurement drift or warm-up artifact)
