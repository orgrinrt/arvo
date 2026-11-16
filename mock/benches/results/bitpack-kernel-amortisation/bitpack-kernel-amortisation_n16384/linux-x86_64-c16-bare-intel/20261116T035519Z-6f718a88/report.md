# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (11.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-narrow at 7.82 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-windowed beats baseline by 33% (significant)

bitpack-mac-windowed is -3.78 us (33%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-mac-narrow, bitpack-mac-windowed) are a dead heat (<1%)

bitpack-mac-narrow (7.82 us) and bitpack-mac-windowed (7.82 us) differ by 0.00%, inside the noise, even though the wider field spreads 48.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-native} vs {bitpack-mac-naive} (34% apart)

The field splits into a fast tier {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 34% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-narrow** at 7815.2 ns median (-32.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.48x (fastest 7815.2 ns, slowest 11593.7 ns)

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
| Counter frequency | 2112768841 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 11669ns | 11658ns | 11615ns | 11664ns | 11738ns | base |
| bitpack-mac-narrow | 7874ns | 7871ns | 7861ns | 7871ns | 7896ns | -32.52% |
| bitpack-mac-native | 8733ns | 8684ns | 8650ns | 8686ns | 8955ns | -25.16% |
| bitpack-mac-simd | 7875ns | 7871ns | 7864ns | 7871ns | 7898ns | -32.51% |
| bitpack-mac-windowed | 7881ns | 7873ns | 7861ns | 7872ns | 7929ns | -32.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 11605ns | 11559ns | 11666ns | base | 1.412 |
| bitpack-mac-narrow | 7814ns | 7805ns | 7821ns | -32.67% | 2.097 |
| bitpack-mac-native | 8671ns | 8595ns | 8894ns | -25.28% | 1.889 |
| bitpack-mac-simd | 7816ns | 7807ns | 7828ns | -32.65% | 2.096 |
| bitpack-mac-windowed | 7821ns | 7805ns | 7855ns | -32.61% | 2.095 |

## Performance model

- Peak throughput: **2.099 Gops/s** (bitpack-mac-narrow; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.413 | 67.3% |
| bitpack-mac-narrow | 2.096 | 99.9% |
| bitpack-mac-native | 1.900 | 90.5% |
| bitpack-mac-simd | 2.096 | 99.9% |
| bitpack-mac-windowed | 2.096 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 11669ns | 11669ns | base |
| bitpack-mac-narrow | 7874ns | 7874ns | -32.52% |
| bitpack-mac-native | 8733ns | 8733ns | -25.16% |
| bitpack-mac-simd | 7875ns | 7875ns | -32.51% |
| bitpack-mac-windowed | 7881ns | 7881ns | -32.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 11594ns | base | --- | [11581, 11622] | --- | --- | --- | --- |
| bitpack-mac-narrow | 7815ns | -3778.9ns (-32.6%) | [-3805, -3771]ns | [7812, 7817] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 8622ns | -2975.4ns (-25.7%) | [-3000, -2945]ns | [8617, 8625] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 7816ns | -3778.2ns (-32.6%) | [-3798, -3760]ns | [7814, 7817] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 7815ns | -3781.1ns (-32.6%) | [-3800, -3768]ns | [7814, 7817] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 11581ns | -32.6% | -25.5% | -32.5% | -32.5% |
| 1 | 2 | 11588ns | -32.5% | -25.4% | -32.5% | -32.6% |
| 1 | 3 | 11587ns | -32.5% | -25.6% | -32.5% | -32.5% |
| 1 | 4 | 11646ns | -32.9% | -26.0% | -32.9% | -32.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | -0.063 | ok |
| bitpack-mac-narrow | -0.082 | ok |
| bitpack-mac-native | -0.085 | ok |
| bitpack-mac-simd | -0.048 | ok |
| bitpack-mac-windowed | -0.021 | ok |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 19.7ns | 11605.0ns | 0.2% |  |
| bitpack-mac-narrow | 19.2ns | 7813.9ns | 0.2% |  |
| bitpack-mac-native | 19.2ns | 8671.2ns | 0.2% |  |
| bitpack-mac-simd | 19.3ns | 7816.4ns | 0.2% |  |
| bitpack-mac-windowed | 19.3ns | 7820.9ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 11559.0-11666.5 ns)
  11559.0 |####################
  11564.3 |########################################
  11569.7 |##########
  11575.1 |##############################
  11580.5 |##############################
  11585.8 |##############################
  11591.2 |##########
  11596.6 |####################
  11602.0 |####################
  11607.4 |
  11612.7 |
  11618.1 |####################
  11623.5 |
  11628.9 |####################
  11634.2 |##########
  11639.6 |####################
  11645.0 |
  11650.4 |##########
  11655.7 |########################################
  11661.1 |
  (4 below, 3 above range)

bitpack-mac-narrow (n=40, range 7804.8-7821.4 ns)
   7804.8 |
   7805.6 |##########
   7806.5 |##########
   7807.3 |
   7808.1 |##############################
   7808.9 |##############################
   7809.8 |##########
   7810.6 |####################
   7811.4 |##########
   7812.3 |##########
   7813.1 |##########
   7813.9 |####################
   7814.8 |####################
   7815.6 |####################
   7816.4 |##############################
   7817.3 |##############################
   7818.1 |########################################
   7818.9 |##############################
   7819.8 |##############################
   7820.6 |
  (3 below, 1 above range)

bitpack-mac-native (n=40, range 8594.7-8894.3 ns)
   8594.7 |#################################
   8609.7 |########################################
   8624.7 |#############
   8639.7 |##########
   8654.6 |######
   8669.6 |
   8684.6 |
   8699.6 |###
   8714.6 |######
   8729.5 |
   8744.5 |
   8759.5 |
   8774.5 |
   8789.5 |
   8804.5 |
   8819.4 |
   8834.4 |
   8849.4 |
   8864.4 |
   8879.4 |
  (3 below, 3 above range)

bitpack-mac-simd (n=40, range 7807.4-7828.1 ns)
   7807.4 |###########
   7808.4 |#################
   7809.5 |#####
   7810.5 |#####
   7811.6 |######################
   7812.6 |#####
   7813.6 |#################
   7814.7 |#####
   7815.7 |########################################
   7816.7 |###########
   7817.8 |##################################
   7818.8 |######################
   7819.9 |###########
   7820.9 |
   7821.9 |
   7823.0 |
   7824.0 |
   7825.0 |
   7826.1 |
   7827.1 |
  (2 below, 1 above range)

bitpack-mac-windowed (n=40, range 7804.8-7855.4 ns)
   7804.8 |#############
   7807.3 |######################
   7809.9 |########
   7812.4 |######################
   7814.9 |########################################
   7817.5 |########################################
   7820.0 |########
   7822.5 |
   7825.0 |
   7827.6 |
   7830.1 |
   7832.6 |
   7835.2 |####
   7837.7 |
   7840.2 |
   7842.8 |
   7845.3 |
   7847.8 |
   7850.3 |
   7852.9 |
  (3 below, 1 above range)

```
