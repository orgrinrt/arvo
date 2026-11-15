# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (31.83 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-native at 12.90 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-native dominates: 29% faster than the next best (bitpack-mac-simd)

bitpack-mac-native (12.90 us) leads bitpack-mac-simd (16.65 us) by 29%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-mac-native beats baseline by 59% (significant)

bitpack-mac-native is -18.77 us (59%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-mac-naive is an outlier: 2.5x slower than the field

bitpack-mac-naive (31.83 us) is 2.5x the fastest (12.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-mac-native shows warm-up / thermal drift (autocorr +0.61)

bitpack-mac-native's per-pass series has lag-1 autocorrelation +0.61, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-mac-native, bitpack-mac-simd} vs {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} (59% apart)

The field splits into a fast tier {bitpack-mac-native, bitpack-mac-simd} and a slow tier {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} with a 59% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-mac-naive is inconsistent: worst-20% is 2.0x its best-20%

bitpack-mac-naive's best 20% of batches run at 30.97 us but its worst 20% at 61.11 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-mac-native** at 12900.6 ns median (-59.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.47x (fastest 12900.6 ns, slowest 31828.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 37662ns | 31939ns | 31061ns | 31982ns | 61305ns | base |
| bitpack-mac-narrow | 27182ns | 26568ns | 26360ns | 26806ns | 29132ns | -27.83% |
| bitpack-mac-native | 13214ns | 13038ns | 13013ns | 13044ns | 13925ns | -64.91% |
| bitpack-mac-simd | 17045ns | 16739ns | 16713ns | 16789ns | 18145ns | -54.74% |
| bitpack-mac-windowed | 26599ns | 26564ns | 26297ns | 26567ns | 26999ns | -29.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 37543ns | 30967ns | 61111ns | base | 1.746 |
| bitpack-mac-narrow | 27065ns | 26270ns | 28974ns | -27.91% | 2.421 |
| bitpack-mac-native | 13061ns | 12887ns | 13689ns | -65.21% | 5.018 |
| bitpack-mac-simd | 16953ns | 16628ns | 18041ns | -54.85% | 3.866 |
| bitpack-mac-windowed | 26499ns | 26208ns | 26863ns | -29.42% | 2.473 |

## Performance model

- Peak throughput: **5.085 Gops/s** (bitpack-mac-native; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 2.059 | 40.5% |
| bitpack-mac-narrow | 2.477 | 48.7% |
| bitpack-mac-native | 5.080 | 99.9% |
| bitpack-mac-simd | 3.936 | 77.4% |
| bitpack-mac-windowed | 2.475 | 48.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 37662ns | 37662ns | base |
| bitpack-mac-narrow | 27182ns | 27182ns | -27.83% |
| bitpack-mac-native | 13214ns | 13214ns | -64.91% |
| bitpack-mac-simd | 17045ns | 17045ns | -54.74% |
| bitpack-mac-windowed | 26599ns | 26599ns | -29.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 31828ns | base | --- | [31079, 32463] | --- | --- | --- | --- |
| bitpack-mac-narrow | 26461ns | -5142.7ns (-16.2%) | [-5617, -4683]ns | [26308, 26932] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 12901ns | -18640.4ns (-58.6%) | [-19450, -18097]ns | [12898, 12922] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 16651ns | -14973.7ns (-47.0%) | [-15785, -14338]ns | [16639, 16670] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 26475ns | -5458.7ns (-17.2%) | [-5982, -4593]ns | [26446, 26500] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|
| 1 | 31299ns | -15.4% | -58.7% | -46.8% | -15.3% |
| 2 | 31114ns | -14.9% | -55.3% | -45.8% | -14.6% |
| 3 | 31996ns | -17.8% | -59.6% | -47.4% | -17.1% |
| 4 | 243796ns | -89.0% | -94.7% | -93.2% | -89.1% |
| 5 | 38102ns | -28.6% | -65.8% | -56.3% | -30.5% |
| 6 | 34493ns | -23.6% | -60.3% | -51.8% | -23.1% |
| 7 | 31937ns | -17.2% | -55.3% | -47.8% | -17.0% |
| 8 | 32793ns | -19.8% | -56.8% | -49.2% | -19.2% |
| 9 | 32435ns | -16.3% | -56.1% | -48.6% | -18.2% |
| 10 | 31695ns | -16.4% | -59.2% | -47.4% | -15.6% |
| 11 | 31045ns | -15.3% | -58.5% | -45.3% | -14.7% |
| 12 | 31551ns | -16.7% | -59.1% | -47.3% | -16.2% |
| 13 | 30965ns | -15.1% | -58.3% | -44.7% | -12.2% |
| 14 | 30970ns | -9.3% | -58.3% | -46.3% | -14.6% |
| 15 | 33623ns | -21.9% | -61.6% | -50.5% | -21.3% |
| 16 | 32607ns | -19.3% | -60.4% | -49.0% | -17.3% |
| 17 | 30966ns | -15.1% | -58.4% | -46.3% | -14.6% |
| 18 | 37118ns | -28.6% | -65.3% | -55.2% | -28.2% |
| 19 | 32493ns | -17.1% | -60.2% | -48.8% | -18.4% |
| 20 | 30975ns | -15.1% | -58.4% | -46.3% | -14.6% |
| 21 | 30991ns | -15.0% | -58.4% | -41.3% | -14.6% |
| 22 | 32458ns | -19.0% | -60.3% | -44.0% | -18.5% |
| 23 | 33768ns | -22.2% | -61.8% | -49.3% | -21.7% |
| 24 | 30993ns | -15.2% | -58.3% | -46.3% | -14.7% |
| 25 | 32228ns | -0.7% | -60.0% | -48.4% | -17.9% |
| 26 | 33868ns | -17.5% | -61.9% | -50.9% | -21.9% |
| 27 | 33889ns | -20.6% | -62.0% | -50.9% | -21.9% |
| 28 | 32468ns | -12.6% | -60.3% | -48.7% | -18.5% |
| 29 | 30975ns | -7.3% | -58.4% | -46.3% | -14.5% |
| 30 | 33399ns | -14.0% | -61.4% | -50.2% | -20.6% |
| 31 | 30983ns | -15.2% | -58.4% | -41.3% | -20.5% |
| 32 | 30977ns | -8.7% | -58.4% | -41.1% | -11.2% |
| 33 | 30962ns | -7.1% | -58.3% | -44.9% | -14.6% |
| 34 | 30991ns | -7.3% | -58.4% | -46.4% | -14.7% |
| 35 | 30964ns | -9.8% | -58.3% | -46.2% | -14.6% |
| 36 | 30960ns | -8.9% | -58.3% | -46.3% | -14.6% |
| 37 | 31958ns | -17.8% | -59.6% | -47.9% | -17.3% |
| 38 | 33855ns | -22.4% | -60.9% | -50.8% | -21.8% |
| 39 | 31357ns | -16.2% | -58.8% | -43.3% | -14.8% |
| 40 | 31720ns | -17.2% | -59.2% | -38.5% | -16.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | 0.001 | ok |
| bitpack-mac-narrow | 0.305 | moderate+ |
| bitpack-mac-native | 0.611 | HIGH+ (drift/warm-up) |
| bitpack-mac-simd | 0.401 | moderate+ |
| bitpack-mac-windowed | -0.376 | moderate- |

**Consistency summary:**

- **bitpack-mac-narrow**: won 40/40, lost 0/40
- **bitpack-mac-native**: won 40/40, lost 0/40
- **bitpack-mac-simd**: won 40/40, lost 0/40
- **bitpack-mac-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 5.4ns | 37543.4ns | 0.0% |  |
| bitpack-mac-narrow | 5.3ns | 27064.7ns | 0.0% |  |
| bitpack-mac-native | 2.6ns | 13060.7ns | 0.0% |  |
| bitpack-mac-simd | 3.7ns | 16952.6ns | 0.0% |  |
| bitpack-mac-windowed | 3.5ns | 26499.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 30967.2-61111.1 ns)
  30967.2 |########################################
  32474.4 |################
  33981.6 |#
  35488.8 |
  36996.0 |###
  38503.2 |
  40010.4 |
  41517.6 |
  43024.8 |
  44532.0 |
  46039.2 |
  47546.4 |
  49053.6 |
  50560.8 |
  52068.0 |
  53575.2 |
  55082.4 |
  56589.6 |
  58096.7 |
  59603.9 |
  (5 below, 1 above range)

bitpack-mac-narrow (n=40, range 26269.6-28973.8 ns)
  26269.6 |########################################
  26404.8 |############
  26540.0 |
  26675.2 |##
  26810.5 |#####
  26945.7 |
  27080.9 |#####
  27216.1 |
  27351.3 |
  27486.5 |
  27621.7 |
  27756.9 |
  27892.1 |#####
  28027.3 |##
  28162.5 |#####
  28297.7 |##
  28432.9 |
  28568.1 |
  28703.3 |##########
  28838.5 |
  (3 below, 1 above range)

bitpack-mac-native (n=40, range 12886.9-13689.1 ns)
  12886.9 |########################################
  12927.0 |######
  12967.1 |#
  13007.3 |#
  13047.4 |
  13087.5 |
  13127.6 |
  13167.7 |
  13207.8 |#
  13247.9 |
  13288.0 |
  13328.1 |
  13368.2 |
  13408.3 |
  13448.4 |
  13488.6 |
  13528.7 |
  13568.8 |
  13608.9 |
  13649.0 |
  (4 below, 5 above range)

bitpack-mac-simd (n=40, range 16628.4-18041.4 ns)
  16628.4 |########################################
  16699.0 |
  16769.7 |#
  16840.3 |#
  16911.0 |
  16981.6 |#
  17052.3 |###
  17122.9 |#
  17193.6 |
  17264.3 |
  17334.9 |
  17405.6 |
  17476.2 |
  17546.9 |
  17617.5 |
  17688.2 |
  17758.8 |#
  17829.5 |
  17900.1 |
  17970.8 |
  (3 below, 5 above range)

bitpack-mac-windowed (n=40, range 26207.5-26862.7 ns)
  26207.5 |
  26240.3 |
  26273.0 |
  26305.8 |
  26338.5 |
  26371.3 |
  26404.1 |#####################
  26436.8 |########################################
  26469.6 |################################
  26502.3 |#####################
  26535.1 |
  26567.9 |###
  26600.6 |
  26633.4 |###
  26666.1 |
  26698.9 |###
  26731.7 |###
  26764.4 |
  26797.2 |
  26830.0 |
  (1 below, 3 above range)

```

## Diagnostics

- **bitpack-mac-naive**: CV=88.1% (high variance, measurements may be unstable)
- **bitpack-mac-native**: autocorrelation=0.61 (measurement drift or warm-up artifact)
