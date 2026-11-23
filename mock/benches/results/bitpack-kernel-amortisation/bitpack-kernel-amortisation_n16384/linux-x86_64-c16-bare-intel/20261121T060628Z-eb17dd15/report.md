# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (11.72 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-windowed at 7.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-windowed beats baseline by 33% (significant)

bitpack-mac-windowed is -3.87 us (33%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-mac-windowed, bitpack-mac-simd) are a dead heat (<1%)

bitpack-mac-windowed (7.85 us) and bitpack-mac-simd (7.85 us) differ by 0.09%, inside the noise, even though the wider field spreads 49.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-narrow, bitpack-mac-native} vs {bitpack-mac-naive} (36% apart)

The field splits into a fast tier {bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-narrow, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-windowed** at 7846.1 ns median (-33.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.49x (fastest 7846.1 ns, slowest 11720.7 ns)

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
| bitpack-mac-naive | 11785ns | 11775ns | 11769ns | 11776ns | 11829ns | base |
| bitpack-mac-narrow | 8590ns | 7906ns | 7893ns | 7905ns | 11344ns | -27.11% |
| bitpack-mac-native | 8651ns | 8643ns | 8637ns | 8643ns | 8687ns | -26.60% |
| bitpack-mac-simd | 7908ns | 7905ns | 7893ns | 7904ns | 7937ns | -32.90% |
| bitpack-mac-windowed | 7903ns | 7898ns | 7891ns | 7901ns | 7923ns | -32.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 11728ns | 11714ns | 11762ns | base | 1.397 |
| bitpack-mac-narrow | 8530ns | 7842ns | 11255ns | -27.27% | 1.921 |
| bitpack-mac-native | 8595ns | 8585ns | 8616ns | -26.71% | 1.906 |
| bitpack-mac-simd | 7853ns | 7841ns | 7871ns | -33.04% | 2.086 |
| bitpack-mac-windowed | 7848ns | 7839ns | 7857ns | -33.08% | 2.088 |

## Performance model

- Peak throughput: **2.090 Gops/s** (bitpack-mac-windowed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.398 | 66.9% |
| bitpack-mac-narrow | 2.086 | 99.8% |
| bitpack-mac-native | 1.907 | 91.2% |
| bitpack-mac-simd | 2.086 | 99.8% |
| bitpack-mac-windowed | 2.088 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 11785ns | 11785ns | base |
| bitpack-mac-narrow | 8590ns | 8590ns | -27.11% |
| bitpack-mac-native | 8651ns | 8651ns | -26.60% |
| bitpack-mac-simd | 7908ns | 7908ns | -32.90% |
| bitpack-mac-windowed | 7903ns | 7903ns | -32.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 11721ns | base | --- | [11718, 11723] | --- | --- | --- | --- |
| bitpack-mac-narrow | 7854ns | -3869.3ns (-33.0%) | [-3872, -3867]ns | [7846, 7854] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 8592ns | -3128.8ns (-26.7%) | [-3132, -3126]ns | [8591, 8592] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 7853ns | -3871.1ns (-33.0%) | [-3876, -3866]ns | [7846, 7854] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 7846ns | -3872.8ns (-33.0%) | [-3877, -3869]ns | [7845, 7853] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 11720ns | -33.0% | -26.7% | -33.0% | -33.0% |
| 1 | 2 | 11719ns | -29.0% | -26.7% | -33.0% | -33.1% |
| 1 | 3 | 11721ns | -33.1% | -26.7% | -33.0% | -33.1% |
| 1 | 4 | 11722ns | -33.0% | -26.7% | -33.1% | -33.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | 0.271 | moderate+ |
| bitpack-mac-narrow | -0.422 | moderate- |
| bitpack-mac-native | -0.424 | moderate- |
| bitpack-mac-simd | -0.154 | ok |
| bitpack-mac-windowed | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 19.5ns | 11727.8ns | 0.2% |  |
| bitpack-mac-narrow | 20.7ns | 8529.9ns | 0.2% |  |
| bitpack-mac-native | 18.8ns | 8595.4ns | 0.2% |  |
| bitpack-mac-simd | 19.0ns | 7852.8ns | 0.2% |  |
| bitpack-mac-windowed | 19.0ns | 7848.1ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 11714.0-11762.5 ns)
  11714.0 |############################
  11716.4 |##################################
  11718.9 |########################################
  11721.3 |############################
  11723.7 |######################
  11726.1 |###########
  11728.6 |###########
  11731.0 |#####
  11733.4 |
  11735.8 |
  11738.2 |#####
  11740.7 |#####
  11743.1 |
  11745.5 |
  11747.9 |
  11750.4 |
  11752.8 |
  11755.2 |
  11757.6 |
  11760.1 |
  (4 below, 2 above range)

bitpack-mac-narrow (n=40, range 7841.7-11254.7 ns)
   7841.7 |########################################
   8012.3 |
   8183.0 |
   8353.6 |
   8524.3 |
   8694.9 |#
   8865.6 |
   9036.2 |
   9206.9 |
   9377.5 |
   9548.2 |
   9718.8 |
   9889.5 |
  10060.1 |
  10230.8 |#
  10401.4 |
  10572.1 |
  10742.7 |
  10913.4 |
  11084.0 |
  (2 below, 3 above range)

bitpack-mac-native (n=40, range 8584.9-8616.2 ns)
   8584.9 |
   8586.5 |
   8588.1 |
   8589.6 |#################
   8591.2 |########################################
   8592.7 |####################
   8594.3 |##
   8595.9 |
   8597.4 |
   8599.0 |##
   8600.6 |
   8602.1 |
   8603.7 |
   8605.3 |
   8606.8 |
   8608.4 |
   8610.0 |
   8611.5 |
   8613.1 |
   8614.6 |
  (5 below, 2 above range)

bitpack-mac-simd (n=40, range 7840.6-7870.9 ns)
   7840.6 |
   7842.1 |######
   7843.6 |#####################
   7845.1 |#########
   7846.6 |###
   7848.2 |
   7849.7 |
   7851.2 |######
   7852.7 |########################################
   7854.2 |###############
   7855.8 |###
   7857.3 |
   7858.8 |
   7860.3 |###
   7861.8 |
   7863.4 |
   7864.9 |
   7866.4 |
   7867.9 |
   7869.4 |###
  (3 below, 1 above range)

bitpack-mac-windowed (n=40, range 7839.4-7856.8 ns)
   7839.4 |
   7840.3 |
   7841.2 |
   7842.0 |
   7842.9 |###########
   7843.8 |##################################
   7844.6 |######################
   7845.5 |########################################
   7846.4 |
   7847.2 |
   7848.1 |#####
   7849.0 |
   7849.8 |
   7850.7 |
   7851.6 |#####
   7852.4 |#####
   7853.3 |######################
   7854.2 |#################
   7855.0 |######################
   7855.9 |#####
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-mac-narrow**: CV=25.9% (high variance, measurements may be unstable)
