# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (48.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-native at 19.41 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-native dominates: 28% faster than the next best (bitpack-mac-simd)

bitpack-mac-native (19.41 us) leads bitpack-mac-simd (24.91 us) by 28%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-mac-native beats baseline by 61% (significant)

bitpack-mac-native is -29.64 us (61%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-mac-naive is an outlier: 2.5x slower than the field

bitpack-mac-naive (48.78 us) is 2.5x the fastest (19.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-mac-narrow shows warm-up / thermal drift (autocorr +0.73)

bitpack-mac-narrow's per-pass series has lag-1 autocorrelation +0.73, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-mac-native, bitpack-mac-simd} vs {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} (58% apart)

The field splits into a fast tier {bitpack-mac-native, bitpack-mac-simd} and a slow tier {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} with a 58% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-native** at 19408.6 ns median (-60.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.51x (fastest 19408.6 ns, slowest 48776.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 49410ns | 48955ns | 47341ns | 49170ns | 52197ns | base |
| bitpack-mac-narrow | 40776ns | 39659ns | 38480ns | 39681ns | 46360ns | -17.47% |
| bitpack-mac-native | 19755ns | 19528ns | 19422ns | 19554ns | 20691ns | -60.02% |
| bitpack-mac-simd | 25235ns | 25056ns | 24459ns | 25068ns | 26508ns | -48.93% |
| bitpack-mac-windowed | 42675ns | 39937ns | 39273ns | 40087ns | 53842ns | -13.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 49229ns | 47172ns | 51996ns | base | 1.997 |
| bitpack-mac-narrow | 40607ns | 38343ns | 46150ns | -17.51% | 2.421 |
| bitpack-mac-native | 19626ns | 19302ns | 20537ns | -60.13% | 5.009 |
| bitpack-mac-simd | 25092ns | 24333ns | 26328ns | -49.03% | 3.918 |
| bitpack-mac-windowed | 42486ns | 39114ns | 53568ns | -13.70% | 2.314 |

## Performance model

- Peak throughput: **5.093 Gops/s** (bitpack-mac-native; best 20% batches)
- Ops per call: 98304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 2.015 | 39.6% |
| bitpack-mac-narrow | 2.491 | 48.9% |
| bitpack-mac-native | 5.065 | 99.4% |
| bitpack-mac-simd | 3.947 | 77.5% |
| bitpack-mac-windowed | 2.472 | 48.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 49410ns | 49410ns | base |
| bitpack-mac-narrow | 40776ns | 40776ns | -17.47% |
| bitpack-mac-native | 19755ns | 19755ns | -60.02% |
| bitpack-mac-simd | 25235ns | 25235ns | -48.93% |
| bitpack-mac-windowed | 42675ns | 42675ns | -13.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 48777ns | base | --- | [48069, 49601] | --- | --- | --- | --- |
| bitpack-mac-narrow | 39468ns | -8902.2ns (-18.3%) | [-10023, -8350]ns | [39415, 39566] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 19409ns | -29078.9ns (-59.6%) | [-29731, -28563]ns | [19342, 19478] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 24909ns | -23709.3ns (-48.6%) | [-24587, -23060]ns | [24874, 24962] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 39772ns | -8441.7ns (-17.3%) | [-9277, -8084]ns | [39722, 39964] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|
| 1 | 48314ns | +4.3% | -58.9% | -47.0% | +76.1% |
| 2 | 51475ns | +19.6% | -61.9% | -51.7% | +18.6% |
| 3 | 52915ns | +3.7% | -59.0% | -52.8% | -0.1% |
| 4 | 49848ns | -15.0% | -56.3% | -48.8% | -1.6% |
| 5 | 50078ns | -20.2% | -59.9% | -50.3% | -9.6% |
| 6 | 51339ns | -22.8% | -62.3% | -50.9% | -22.6% |
| 7 | 49021ns | -25.1% | -60.6% | -48.4% | -18.0% |
| 8 | 48550ns | -21.9% | -60.2% | -48.8% | +6.8% |
| 9 | 51174ns | -23.0% | -61.6% | -51.4% | -20.7% |
| 10 | 48981ns | -18.1% | -58.5% | -49.2% | -16.7% |
| 11 | 47816ns | -16.3% | -59.1% | -47.8% | -16.9% |
| 12 | 51989ns | -23.3% | -62.6% | -52.2% | -23.7% |
| 13 | 47766ns | -16.4% | -59.6% | -47.6% | -16.4% |
| 14 | 47750ns | -17.4% | -56.3% | -47.7% | -15.4% |
| 15 | 48190ns | -23.8% | -60.0% | -48.2% | -17.4% |
| 16 | 51844ns | -27.0% | -62.8% | -51.9% | -23.3% |
| 17 | 47948ns | -17.9% | -59.7% | -47.2% | -17.2% |
| 18 | 52255ns | -24.6% | -62.9% | -41.1% | -24.1% |
| 19 | 52213ns | -24.4% | -62.3% | -49.4% | -23.9% |
| 20 | 51935ns | -23.9% | -62.8% | -52.1% | -20.8% |
| 21 | 47755ns | -17.4% | -59.1% | -47.7% | -17.0% |
| 22 | 48138ns | -18.1% | -59.8% | -48.1% | -17.6% |
| 23 | 49353ns | -20.1% | -60.0% | -49.1% | -19.5% |
| 24 | 48572ns | -18.9% | -59.9% | -48.8% | -18.4% |
| 25 | 48354ns | -18.5% | -60.1% | -48.5% | -18.0% |
| 26 | 47915ns | -17.8% | -59.4% | -48.1% | -22.1% |
| 27 | 47241ns | -16.6% | -59.0% | -44.7% | -20.1% |
| 28 | 44305ns | -11.0% | -56.1% | -43.9% | -10.4% |
| 29 | 47748ns | -17.5% | -59.5% | -47.9% | -17.0% |
| 30 | 48000ns | -17.9% | -59.8% | -48.1% | -17.3% |
| 31 | 47058ns | -15.9% | -59.0% | -47.1% | -15.6% |
| 32 | 47796ns | -16.7% | -58.4% | -48.0% | -11.6% |
| 33 | 47750ns | -17.2% | -59.2% | -46.7% | -15.5% |
| 34 | 50782ns | -22.0% | -62.0% | -51.7% | -21.7% |
| 35 | 49022ns | -19.7% | -60.3% | -53.3% | -18.6% |
| 36 | 47793ns | -17.3% | -59.6% | -52.0% | -16.3% |
| 37 | 50623ns | -22.2% | -61.8% | -50.8% | -21.5% |
| 38 | 49169ns | -19.7% | -60.5% | -49.4% | -19.0% |
| 39 | 51260ns | -22.8% | -62.3% | -51.5% | -22.7% |
| 40 | 49109ns | -19.0% | -60.5% | -49.4% | -18.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | 0.263 | moderate+ |
| bitpack-mac-narrow | 0.725 | HIGH+ (drift/warm-up) |
| bitpack-mac-native | 0.433 | moderate+ |
| bitpack-mac-simd | 0.311 | moderate+ |
| bitpack-mac-windowed | 0.472 | moderate+ |

**Consistency summary:**

- **bitpack-mac-narrow**: won 37/40, lost 3/40
- **bitpack-mac-native**: won 40/40, lost 0/40
- **bitpack-mac-simd**: won 40/40, lost 0/40
- **bitpack-mac-windowed**: won 37/40, lost 3/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 4.0ns | 49228.7ns | 0.0% |  |
| bitpack-mac-narrow | 3.5ns | 40607.1ns | 0.0% |  |
| bitpack-mac-native | 2.7ns | 19626.3ns | 0.0% |  |
| bitpack-mac-simd | 3.4ns | 25092.5ns | 0.0% |  |
| bitpack-mac-windowed | 5.7ns | 42486.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 47171.7-51995.7 ns)
  47171.7 |#####
  47412.9 |
  47654.1 |########################################
  47895.3 |###############
  48136.5 |####################
  48377.7 |##########
  48618.9 |
  48860.1 |###############
  49101.3 |##########
  49342.5 |#####
  49583.7 |
  49824.9 |#####
  50066.1 |#####
  50307.3 |
  50548.5 |##########
  50789.7 |
  51030.9 |##########
  51272.1 |##########
  51513.3 |
  51754.5 |###############
  (2 below, 3 above range)

bitpack-mac-narrow (n=40, range 38343.5-46149.9 ns)
  38343.5 |
  38733.8 |
  39124.1 |########################################
  39514.4 |######################
  39904.8 |########
  40295.1 |
  40685.4 |
  41075.7 |
  41466.0 |
  41856.4 |
  42246.7 |##
  42637.0 |
  43027.3 |
  43417.6 |
  43807.9 |
  44198.3 |
  44588.6 |
  44978.9 |
  45369.2 |
  45759.5 |
  (4 below, 3 above range)

bitpack-mac-native (n=40, range 19301.6-20537.1 ns)
  19301.6 |########################################
  19363.4 |########
  19425.2 |##############
  19486.9 |#####
  19548.7 |##
  19610.5 |#####
  19672.3 |##
  19734.0 |##
  19795.8 |##
  19857.6 |##
  19919.4 |
  19981.2 |
  20042.9 |##
  20104.7 |
  20166.5 |
  20228.3 |
  20290.0 |##
  20351.8 |
  20413.6 |
  20475.4 |
  (4 below, 3 above range)

bitpack-mac-simd (n=40, range 24333.5-26328.3 ns)
  24333.5 |
  24433.2 |##
  24533.0 |
  24632.7 |
  24732.5 |
  24832.2 |########################################
  24931.9 |##############
  25031.7 |##
  25131.4 |##
  25231.2 |##
  25330.9 |##
  25430.6 |####
  25530.4 |##
  25630.1 |
  25729.9 |
  25829.6 |
  25929.3 |
  26029.1 |
  26128.8 |##
  26228.6 |
  (2 below, 2 above range)

bitpack-mac-windowed (n=40, range 39113.9-53568.0 ns)
  39113.9 |########################################
  39836.6 |##################
  40559.3 |####
  41282.0 |
  42004.7 |##
  42727.4 |
  43450.1 |
  44172.8 |
  44895.5 |##
  45618.2 |
  46340.9 |
  47063.7 |
  47786.4 |
  48509.1 |##
  49231.8 |
  49954.5 |
  50677.2 |
  51399.9 |##
  52122.6 |
  52845.3 |##
  (2 below, 2 above range)

```

## Diagnostics

- **bitpack-mac-narrow**: autocorrelation=0.73 (measurement drift or warm-up artifact)
