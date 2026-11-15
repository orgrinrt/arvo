# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (7.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-native at 3.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-native dominates: 28% faster than the next best (bitpack-mac-simd)

bitpack-mac-native (3.23 us) leads bitpack-mac-simd (4.14 us) by 28%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-mac-native beats baseline by 64% (significant)

bitpack-mac-native is -5.08 us (64%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-mac-naive is an outlier: 2.5x slower than the field

bitpack-mac-naive (7.97 us) is 2.5x the fastest (3.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-mac-simd shows warm-up / thermal drift (autocorr +0.86)

bitpack-mac-simd's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-mac-native, bitpack-mac-simd} vs {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} (59% apart)

The field splits into a fast tier {bitpack-mac-native, bitpack-mac-simd} and a slow tier {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} with a 59% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-native** at 3225.2 ns median (-59.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.47x (fastest 3225.2 ns, slowest 7973.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 8126ns | 8058ns | 7709ns | 8085ns | 8665ns | base |
| bitpack-mac-narrow | 6559ns | 6650ns | 6129ns | 6634ns | 6761ns | -19.29% |
| bitpack-mac-native | 3346ns | 3305ns | 3229ns | 3308ns | 3574ns | -58.83% |
| bitpack-mac-simd | 4186ns | 4220ns | 3889ns | 4165ns | 4545ns | -48.49% |
| bitpack-mac-windowed | 6570ns | 6704ns | 6168ns | 6597ns | 6894ns | -19.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 8039ns | 7632ns | 8557ns | base | 2.038 |
| bitpack-mac-narrow | 6478ns | 6054ns | 6671ns | -19.42% | 2.529 |
| bitpack-mac-native | 3263ns | 3151ns | 3486ns | -59.41% | 5.020 |
| bitpack-mac-simd | 4106ns | 3814ns | 4459ns | -48.93% | 3.991 |
| bitpack-mac-windowed | 6488ns | 6093ns | 6803ns | -19.29% | 2.525 |

## Performance model

- Peak throughput: **5.199 Gops/s** (bitpack-mac-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 2.055 | 39.5% |
| bitpack-mac-narrow | 2.493 | 48.0% |
| bitpack-mac-native | 5.080 | 97.7% |
| bitpack-mac-simd | 3.957 | 76.1% |
| bitpack-mac-windowed | 2.474 | 47.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 8126ns | 8126ns | base |
| bitpack-mac-narrow | 6559ns | 6559ns | -19.29% |
| bitpack-mac-native | 3346ns | 3346ns | -58.83% |
| bitpack-mac-simd | 4186ns | 4186ns | -48.49% |
| bitpack-mac-windowed | 6570ns | 6570ns | -19.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 7974ns | base | --- | [7964, 7991] | --- | --- | --- | --- |
| bitpack-mac-narrow | 6571ns | -1418.3ns (-17.8%) | [-1649, -1402]ns | [6566, 6579] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 3225ns | -4758.1ns (-59.7%) | [-4791, -4742]ns | [3224, 3228] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 4141ns | -3843.9ns (-48.2%) | [-4124, -3824]ns | [4132, 4143] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 6622ns | -1371.4ns (-17.2%) | [-1810, -1344]ns | [6593, 6628] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|
| 1 | 7964ns | -23.9% | -62.7% | -46.2% | -16.9% |
| 2 | 7964ns | -24.0% | -62.7% | -46.4% | -10.9% |
| 3 | 7961ns | -23.9% | -60.2% | -47.9% | -11.1% |
| 4 | 7963ns | -20.4% | -59.6% | -48.0% | -16.6% |
| 5 | 7975ns | -17.6% | -59.6% | -50.9% | -12.6% |
| 6 | 7969ns | -17.6% | -59.5% | -52.2% | -16.8% |
| 7 | 7962ns | -17.5% | -59.6% | -52.2% | -16.5% |
| 8 | 7962ns | -17.4% | -59.5% | -52.1% | -16.8% |
| 9 | 8019ns | -17.9% | -59.8% | -52.4% | -17.3% |
| 10 | 7982ns | -17.6% | -59.6% | -52.1% | -16.2% |
| 11 | 7970ns | -17.5% | -59.6% | -48.1% | -16.9% |
| 12 | 7973ns | -17.5% | -59.6% | -48.0% | -16.9% |
| 13 | 7965ns | -17.6% | -59.5% | -48.0% | -16.9% |
| 14 | 7979ns | -17.7% | -59.6% | -48.1% | -16.9% |
| 15 | 7979ns | -17.7% | -59.6% | -48.0% | -17.0% |
| 16 | 7971ns | -17.6% | -59.6% | -48.0% | -16.7% |
| 17 | 7989ns | -17.8% | -59.7% | -48.3% | -17.0% |
| 18 | 8239ns | -20.3% | -60.9% | -49.7% | -19.7% |
| 19 | 8978ns | -26.5% | -64.1% | -53.9% | -26.1% |
| 20 | 8716ns | -24.6% | -63.0% | -52.6% | -24.0% |
| 21 | 7332ns | -17.4% | -56.0% | -43.6% | -9.8% |
| 22 | 7332ns | -17.5% | -56.1% | -43.6% | -9.9% |
| 23 | 7401ns | -18.2% | -56.4% | -43.9% | -17.7% |
| 24 | 7331ns | -17.5% | -53.0% | -42.5% | -16.9% |
| 25 | 7791ns | -22.4% | -54.7% | -42.0% | -21.8% |
| 26 | 7958ns | -20.2% | -55.7% | -43.1% | -23.4% |
| 27 | 7957ns | -11.8% | -55.7% | -43.2% | -23.4% |
| 28 | 7975ns | -17.7% | -55.8% | -43.4% | -23.6% |
| 29 | 7958ns | -17.2% | -55.5% | -43.2% | -23.4% |
| 30 | 7957ns | -17.4% | -55.2% | -43.2% | -23.2% |
| 31 | 7998ns | -17.8% | -59.5% | -48.2% | -23.8% |
| 32 | 7993ns | -17.6% | -59.6% | -48.2% | -23.7% |
| 33 | 8087ns | -18.5% | -60.0% | -48.8% | -24.5% |
| 34 | 8599ns | -23.4% | -62.5% | -54.6% | -28.8% |
| 35 | 8256ns | -20.2% | -60.9% | -53.8% | -26.1% |
| 36 | 8434ns | -21.9% | -61.6% | -54.6% | -22.0% |
| 37 | 8365ns | -21.2% | -61.4% | -54.4% | -20.8% |
| 38 | 8556ns | -20.8% | -62.2% | -55.4% | -22.5% |
| 39 | 8430ns | -21.7% | -61.7% | -54.7% | -21.1% |
| 40 | 8378ns | -21.1% | -61.3% | -54.4% | -20.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | 0.627 | HIGH+ (drift/warm-up) |
| bitpack-mac-narrow | 0.671 | HIGH+ (drift/warm-up) |
| bitpack-mac-native | 0.790 | HIGH+ (drift/warm-up) |
| bitpack-mac-simd | 0.859 | HIGH+ (drift/warm-up) |
| bitpack-mac-windowed | 0.824 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-mac-narrow**: won 40/40, lost 0/40
- **bitpack-mac-native**: won 40/40, lost 0/40
- **bitpack-mac-simd**: won 40/40, lost 0/40
- **bitpack-mac-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 2.0ns | 8039.2ns | 0.0% |  |
| bitpack-mac-narrow | 2.2ns | 6477.9ns | 0.0% |  |
| bitpack-mac-native | 2.2ns | 3263.5ns | 0.1% |  |
| bitpack-mac-simd | 2.0ns | 4105.5ns | 0.0% |  |
| bitpack-mac-windowed | 2.3ns | 6488.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 7632.2-8556.9 ns)
   7632.2 |
   7678.5 |
   7724.7 |
   7770.9 |#
   7817.2 |
   7863.4 |
   7909.6 |
   7955.9 |########################################
   8002.1 |#
   8048.3 |#
   8094.6 |
   8140.8 |
   8187.0 |
   8233.3 |###
   8279.5 |
   8325.8 |#
   8372.0 |#
   8418.2 |###
   8464.5 |
   8510.7 |#
  (4 below, 3 above range)

bitpack-mac-narrow (n=40, range 6054.4-6671.2 ns)
   6054.4 |##########
   6085.3 |
   6116.1 |
   6146.9 |
   6177.8 |
   6208.6 |
   6239.4 |
   6270.3 |
   6301.1 |
   6332.0 |#####
   6362.8 |
   6393.6 |
   6424.5 |
   6455.3 |
   6486.1 |
   6517.0 |
   6547.8 |########################################
   6578.7 |##############################
   6609.5 |
   6640.3 |
  (4 below, 2 above range)

bitpack-mac-native (n=40, range 3151.3-3486.0 ns)
   3151.3 |
   3168.0 |#
   3184.8 |
   3201.5 |
   3218.2 |########################################
   3235.0 |######
   3251.7 |
   3268.4 |
   3285.2 |
   3301.9 |
   3318.6 |
   3335.4 |
   3352.1 |
   3368.8 |
   3385.6 |
   3402.3 |
   3419.0 |
   3435.8 |#
   3452.5 |
   3469.2 |
  (2 below, 6 above range)

bitpack-mac-simd (n=40, range 3814.5-4459.1 ns)
   3814.5 |####################
   3846.7 |
   3878.9 |##
   3911.2 |##
   3943.4 |
   3975.6 |
   4007.9 |
   4040.1 |
   4072.3 |
   4104.5 |###########
   4136.8 |########################################
   4169.0 |
   4201.2 |##
   4233.5 |
   4265.7 |#####
   4297.9 |
   4330.1 |
   4362.4 |
   4394.6 |
   4426.8 |
  (4 below, 6 above range)

bitpack-mac-windowed (n=40, range 6093.2-6802.6 ns)
   6093.2 |######################
   6128.7 |
   6164.1 |
   6199.6 |
   6235.1 |
   6270.6 |
   6306.0 |
   6341.5 |
   6377.0 |
   6412.4 |
   6447.9 |
   6483.4 |
   6518.8 |
   6554.3 |##
   6589.8 |######################
   6625.3 |########################################
   6660.7 |##
   6696.2 |
   6731.7 |
   6767.1 |
  (5 below, 3 above range)

```

## Diagnostics

- **bitpack-mac-naive**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **bitpack-mac-narrow**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **bitpack-mac-native**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **bitpack-mac-simd**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **bitpack-mac-windowed**: autocorrelation=0.82 (measurement drift or warm-up artifact)
