# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 102% (significant)

warm-container-kernel is -8.61 us (102%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 46.4x slower than the field

warm-container-plusone (8.76 us) is 46.4x the fastest (189 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (189 ns) and warm-container-native (190 ns) differ by 0.74%, inside the noise, even though the wider field spreads 4539.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows warm-up / thermal drift (autocorr +0.89)

warm-container-native's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-headroom, warm-container-plusone} (4289% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-headroom, warm-container-plusone} with a 4289% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 46.4x the fastest

Fastest warm-container-minimum (189 ns) to slowest warm-container-plusone (8.76 us): 46.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 2.4x its best-20%

warm-container-kernel's best 20% of batches run at 185 ns but its worst 20% at 446 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (4 ns, 0.05%)

warm-container-plusone differs from baseline warm-container-headroom by 4 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-minimum** at 188.8 ns median (-97.8% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 46.39x (fastest 188.8 ns, slowest 8759.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 10312ns | 8490ns | 8347ns | 8530ns | 17624ns | base |
| warm-container-kernel | 348ns | 254ns | 247ns | 283ns | 646ns | -96.62% |
| warm-container-lanes-deferred | 256ns | 257ns | 247ns | 256ns | 266ns | -97.52% |
| warm-container-minimum | 253ns | 252ns | 246ns | 253ns | 262ns | -97.54% |
| warm-container-native | 344ns | 251ns | 247ns | 281ns | 629ns | -96.67% |
| warm-container-plusone | 9356ns | 8822ns | 8387ns | 8991ns | 11422ns | -9.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 10234ns | 8286ns | 17491ns | base | 3.202 |
| warm-container-kernel | 252ns | 185ns | 446ns | -97.54% | 130.060 |
| warm-container-lanes-deferred | 193ns | 185ns | 201ns | -98.12% | 170.031 |
| warm-container-minimum | 190ns | 185ns | 197ns | -98.14% | 172.450 |
| warm-container-native | 241ns | 186ns | 397ns | -97.64% | 135.733 |
| warm-container-plusone | 9281ns | 8326ns | 11307ns | -9.32% | 3.531 |

## Performance model

- Peak throughput: **177.388 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.891 | 2.2% |
| warm-container-kernel | 171.920 | 96.9% |
| warm-container-lanes-deferred | 170.756 | 96.3% |
| warm-container-minimum | 173.559 | 97.8% |
| warm-container-native | 172.282 | 97.1% |
| warm-container-plusone | 3.741 | 2.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 10312ns | 10312ns | base |
| warm-container-kernel | 348ns | 348ns | -96.62% |
| warm-container-lanes-deferred | 256ns | 256ns | -97.52% |
| warm-container-minimum | 253ns | 253ns | -97.54% |
| warm-container-native | 344ns | 344ns | -96.67% |
| warm-container-plusone | 9356ns | 9356ns | -9.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8422ns | base | --- | [8344, 8518] | --- | --- | --- | --- |
| warm-container-kernel | 191ns | -8229.3ns (-97.7%) | [-8301, -8152]ns | [189, 228] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 192ns | -8226.6ns (-97.7%) | [-8325, -8153]ns | [189, 196] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 189ns | -8234.1ns (-97.8%) | [-8327, -8154]ns | [187, 191] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 190ns | -8231.9ns (-97.7%) | [-8328, -8148]ns | [189, 194] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8759ns | +169.6ns (+2.0%) | [+29, +485]ns | [8485, 9400] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 14895ns | -97.8% | -98.7% | -98.7% | -97.3% | -10.9% |
| 2 | 8284ns | -95.9% | -97.8% | -97.6% | -95.2% | +61.2% |
| 3 | 11835ns | -97.2% | -98.4% | -98.4% | -96.6% | +15.3% |
| 4 | 22850ns | -98.4% | -99.2% | -99.1% | -98.3% | -58.1% |
| 5 | 20360ns | -94.8% | -99.1% | -99.0% | -98.1% | -57.1% |
| 6 | 16770ns | -98.0% | -98.9% | -98.8% | -97.6% | -47.7% |
| 7 | 8482ns | -96.1% | -97.8% | -97.6% | -95.3% | +3.1% |
| 8 | 14076ns | -97.7% | -98.7% | -98.6% | -97.2% | -36.4% |
| 9 | 20290ns | -98.4% | -99.1% | -99.0% | -98.1% | -57.2% |
| 10 | 18848ns | -97.4% | -99.0% | -99.0% | -97.9% | -55.1% |
| 11 | 8287ns | -97.7% | -97.6% | -97.8% | -97.8% | +19.7% |
| 12 | 8283ns | -97.8% | -97.6% | -97.8% | -97.7% | +2.0% |
| 13 | 8287ns | -97.7% | -97.6% | -97.8% | -97.6% | +2.3% |
| 14 | 8419ns | -97.8% | -97.7% | -97.8% | -97.7% | +11.2% |
| 15 | 8672ns | -97.8% | -97.7% | -97.8% | -97.8% | +8.7% |
| 16 | 8422ns | -97.8% | -97.6% | -97.7% | -97.8% | +11.9% |
| 17 | 8354ns | -97.8% | -97.7% | -97.8% | -97.7% | +14.9% |
| 18 | 8389ns | -97.7% | -97.8% | -97.8% | -97.8% | +12.3% |
| 19 | 8286ns | -97.8% | -97.7% | -97.7% | -97.6% | +15.6% |
| 20 | 9080ns | -98.0% | -97.9% | -97.9% | -97.9% | +3.3% |
| 21 | 8528ns | -97.3% | -97.7% | -97.8% | -97.8% | -2.8% |
| 22 | 8460ns | -97.3% | -97.7% | -97.7% | -97.8% | -2.1% |
| 23 | 8526ns | -97.3% | -97.7% | -97.8% | -97.7% | -1.1% |
| 24 | 8682ns | -97.4% | -97.8% | -97.9% | -97.8% | +15.8% |
| 25 | 8463ns | -97.3% | -97.7% | -97.8% | -97.8% | +18.9% |
| 26 | 8309ns | -97.7% | -97.6% | -97.8% | -97.7% | +21.1% |
| 27 | 8328ns | -97.8% | -97.6% | -97.8% | -97.6% | +21.1% |
| 28 | 8486ns | -97.8% | -97.6% | -97.8% | -97.7% | +3.6% |
| 29 | 8330ns | -97.8% | -97.6% | -97.7% | -97.8% | -0.3% |
| 30 | 8285ns | -97.7% | -97.6% | -97.7% | -97.7% | +1.2% |
| 31 | 8291ns | -97.6% | -97.6% | -97.7% | -97.7% | +3.0% |
| 32 | 8288ns | -97.7% | -97.6% | -97.7% | -97.8% | +1.6% |
| 33 | 8289ns | -97.7% | -97.6% | -97.7% | -97.7% | +2.1% |
| 34 | 8610ns | -97.8% | -97.9% | -97.8% | -97.7% | +1.6% |
| 35 | 8328ns | -97.7% | -97.7% | -97.7% | -97.7% | +2.0% |
| 36 | 8347ns | -97.7% | -97.7% | -97.7% | -97.8% | -0.7% |
| 37 | 8341ns | -97.7% | -97.7% | -97.7% | -97.7% | +0.1% |
| 38 | 8379ns | -97.8% | -97.8% | -97.8% | -97.8% | -1.1% |
| 39 | 8423ns | -97.7% | -97.7% | -97.8% | -97.8% | +0.6% |
| 40 | 8509ns | -97.7% | -97.8% | -97.7% | -97.8% | +7.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.620 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.355 | moderate+ |
| warm-container-lanes-deferred | 0.606 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.650 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.886 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.673 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 13/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.3ns | 10234.3ns | 0.0% |  |
| warm-container-kernel | 4.3ns | 251.9ns | 1.7% |  |
| warm-container-lanes-deferred | 2.6ns | 192.7ns | 1.3% |  |
| warm-container-minimum | 2.7ns | 190.0ns | 1.4% |  |
| warm-container-native | 4.2ns | 241.4ns | 1.7% |  |
| warm-container-plusone | 3.5ns | 9280.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8286.2-17490.5 ns)
   8286.2 |########################################
   8746.4 |#
   9206.6 |
   9666.8 |
  10127.1 |
  10587.3 |
  11047.5 |
  11507.7 |#
  11967.9 |
  12428.1 |
  12888.4 |
  13348.6 |
  13808.8 |#
  14269.0 |
  14729.2 |#
  15189.4 |
  15649.7 |
  16109.9 |
  16570.1 |#
  17030.3 |
  (3 below, 4 above range)

warm-container-kernel (n=40, range 184.7-445.7 ns)
    184.7 |########################################
    197.8 |##
    210.8 |
    223.9 |##########
    236.9 |
    250.0 |
    263.0 |
    276.1 |
    289.1 |
    302.1 |
    315.2 |####
    328.2 |##########
    341.3 |
    354.3 |
    367.4 |##
    380.4 |
    393.5 |
    406.5 |
    419.6 |
    432.6 |
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 184.8-201.0 ns)
    184.8 |################
    185.6 |################
    186.4 |
    187.2 |########################
    188.0 |########################
    188.9 |################
    189.7 |################
    190.5 |########
    191.3 |################
    192.1 |########
    192.9 |########
    193.7 |
    194.5 |########
    195.4 |
    196.2 |########################################
    197.0 |
    197.8 |########
    198.6 |########
    199.4 |################################
    200.2 |########
  (4 below, 4 above range)

warm-container-minimum (n=40, range 184.8-196.9 ns)
    184.8 |
    185.4 |######################
    186.0 |#####
    186.6 |########################################
    187.2 |###########
    187.8 |#####
    188.4 |#################
    189.0 |
    189.6 |#####
    190.2 |############################
    190.8 |
    191.4 |
    192.0 |###########
    192.6 |#####
    193.2 |
    193.8 |#####
    194.4 |#####
    195.0 |#####
    195.6 |######################
    196.2 |
  (3 below, 3 above range)

warm-container-native (n=40, range 186.0-396.9 ns)
    186.0 |########################################
    196.5 |###
    207.1 |
    217.6 |
    228.2 |
    238.7 |
    249.2 |
    259.8 |
    270.3 |
    280.9 |
    291.4 |
    302.0 |
    312.5 |
    323.1 |
    333.6 |
    344.1 |
    354.7 |
    365.2 |
    375.8 |
    386.3 |###########
  (4 below, 3 above range)

warm-container-plusone (n=40, range 8326.2-11307.2 ns)
   8326.2 |########################################
   8475.2 |###############
   8624.3 |#########################
   8773.3 |#####
   8922.4 |#####
   9071.4 |#####
   9220.5 |#####
   9369.6 |####################
   9518.6 |###############
   9667.7 |
   9816.7 |#####
   9965.8 |####################
  10114.8 |
  10263.9 |
  10412.9 |
  10562.0 |
  10711.0 |
  10860.1 |
  11009.1 |
  11158.2 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=38.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=57.9% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-container-native**: CV=37.1% (high variance, measurements may be unstable)
- **warm-container-native**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.67 (measurement drift or warm-up artifact)
