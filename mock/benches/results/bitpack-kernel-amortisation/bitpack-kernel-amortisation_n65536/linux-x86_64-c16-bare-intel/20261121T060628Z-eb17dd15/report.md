# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (48.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-narrow at 32.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-narrow beats baseline by 33% (significant)

bitpack-mac-narrow is -16.16 us (33%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-mac-narrow, bitpack-mac-simd) are a dead heat (<1%)

bitpack-mac-narrow (32.08 us) and bitpack-mac-simd (32.15 us) differ by 0.21%, inside the noise, even though the wider field spreads 50.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-mac-narrow shows alternating (throttle bounce) (autocorr -0.75)

bitpack-mac-narrow's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-mac-narrow, bitpack-mac-simd, bitpack-mac-windowed, bitpack-mac-native} vs {bitpack-mac-naive} (39% apart)

The field splits into a fast tier {bitpack-mac-narrow, bitpack-mac-simd, bitpack-mac-windowed, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-narrow** at 32082.3 ns median (-33.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.51x (fastest 32082.3 ns, slowest 48304.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked decode against per-element consumer work |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112613062 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 48038ns | 48375ns | 47116ns | 48204ns | 48461ns | base |
| bitpack-mac-narrow | 31890ns | 32152ns | 31428ns | 31929ns | 32234ns | -33.61% |
| bitpack-mac-native | 34926ns | 34890ns | 34877ns | 34893ns | 35075ns | -27.29% |
| bitpack-mac-simd | 32218ns | 32221ns | 32144ns | 32211ns | 32311ns | -32.93% |
| bitpack-mac-windowed | 32255ns | 32223ns | 31608ns | 32207ns | 33044ns | -32.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 47964ns | 47045ns | 48380ns | base | 1.366 |
| bitpack-mac-narrow | 31819ns | 31356ns | 32162ns | -33.66% | 2.060 |
| bitpack-mac-native | 34855ns | 34810ns | 34998ns | -27.33% | 1.880 |
| bitpack-mac-simd | 32147ns | 32076ns | 32233ns | -32.98% | 2.039 |
| bitpack-mac-windowed | 32183ns | 31542ns | 32967ns | -32.90% | 2.036 |

## Performance model

- Peak throughput: **2.090 Gops/s** (bitpack-mac-narrow; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.357 | 64.9% |
| bitpack-mac-narrow | 2.043 | 97.7% |
| bitpack-mac-native | 1.882 | 90.0% |
| bitpack-mac-simd | 2.038 | 97.5% |
| bitpack-mac-windowed | 2.038 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 48038ns | 48038ns | base |
| bitpack-mac-narrow | 31890ns | 31890ns | -33.61% |
| bitpack-mac-native | 34926ns | 34926ns | -27.29% |
| bitpack-mac-simd | 32218ns | 32218ns | -32.93% |
| bitpack-mac-windowed | 32255ns | 32255ns | -32.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 48304ns | base | --- | [48292, 48313] | --- | --- | --- | --- |
| bitpack-mac-narrow | 32082ns | -16164.1ns (-33.5%) | [-16209, -16148]ns | [31382, 32140] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 34821ns | -13481.8ns (-27.9%) | [-13491, -13445]ns | [34815, 34825] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 32149ns | -16152.6ns (-33.4%) | [-16166, -16019]ns | [32138, 32157] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 32154ns | -16156.9ns (-33.4%) | [-16171, -16136]ns | [32120, 32158] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 48306ns | -33.4% | -27.9% | -33.5% | -33.4% |
| 1 | 2 | 48309ns | -35.1% | -27.9% | -33.4% | -35.0% |
| 1 | 3 | 48319ns | -33.5% | -27.9% | -33.5% | -33.5% |
| 1 | 4 | 47052ns | -33.3% | -26.0% | -31.7% | -31.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | -0.090 | ok |
| bitpack-mac-narrow | -0.752 | HIGH- (thermal bounce) |
| bitpack-mac-native | -0.329 | moderate- |
| bitpack-mac-simd | -0.095 | ok |
| bitpack-mac-windowed | -0.414 | moderate- |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 20.9ns | 47963.9ns | 0.0% |  |
| bitpack-mac-narrow | 19.9ns | 31819.2ns | 0.1% |  |
| bitpack-mac-native | 20.2ns | 34854.6ns | 0.1% |  |
| bitpack-mac-simd | 20.0ns | 32146.7ns | 0.1% |  |
| bitpack-mac-windowed | 20.2ns | 32183.3ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 47045.4-48379.8 ns)
  47045.4 |######################
  47112.2 |
  47178.9 |
  47245.6 |
  47312.3 |
  47379.0 |
  47445.8 |
  47512.5 |
  47579.2 |
  47645.9 |
  47712.6 |
  47779.3 |##
  47846.1 |
  47912.8 |
  47979.5 |
  48046.2 |
  48112.9 |
  48179.6 |
  48246.4 |########################################
  48313.1 |##################################
  (3 below, 2 above range)

bitpack-mac-narrow (n=40, range 31356.3-32161.9 ns)
  31356.3 |##################################
  31396.6 |
  31436.9 |
  31477.2 |
  31517.4 |
  31557.7 |
  31598.0 |
  31638.3 |
  31678.6 |
  31718.8 |
  31759.1 |##
  31799.4 |
  31839.7 |
  31879.9 |
  31920.2 |
  31960.5 |
  32000.8 |
  32041.1 |########
  32081.3 |########
  32121.6 |########################################
  (4 below, 3 above range)

bitpack-mac-native (n=40, range 34810.4-34998.2 ns)
  34810.4 |########################################
  34819.8 |#############################
  34829.2 |##
  34838.6 |#####
  34848.0 |
  34857.3 |#####
  34866.7 |
  34876.1 |
  34885.5 |
  34894.9 |
  34904.3 |
  34913.7 |
  34923.1 |
  34932.5 |##
  34941.9 |
  34951.2 |
  34960.6 |
  34970.0 |
  34979.4 |
  34988.8 |##
  (4 below, 3 above range)

bitpack-mac-simd (n=40, range 32075.8-32233.3 ns)
  32075.8 |##############
  32083.7 |###
  32091.6 |###
  32099.4 |
  32107.3 |##########
  32115.2 |
  32123.1 |
  32130.9 |#######
  32138.8 |##########
  32146.7 |##############
  32154.6 |########################################
  32162.5 |##################
  32170.3 |
  32178.2 |
  32186.1 |
  32194.0 |
  32201.8 |
  32209.7 |
  32217.6 |
  32225.5 |
  (4 below, 2 above range)

bitpack-mac-windowed (n=40, range 31541.7-32967.0 ns)
  31541.7 |
  31612.9 |
  31684.2 |
  31755.5 |
  31826.7 |
  31898.0 |#
  31969.3 |
  32040.5 |############
  32111.8 |########################################
  32183.1 |###
  32254.3 |
  32325.6 |
  32396.8 |
  32468.1 |
  32539.4 |
  32610.6 |
  32681.9 |
  32753.2 |
  32824.4 |
  32895.7 |
  (6 below, 1 above range)

```
