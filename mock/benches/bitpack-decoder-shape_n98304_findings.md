# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (50.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-native at 10.94 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-native dominates: 50% faster than the next best (bitpack-plan-windowed)

bitpack-plan-native (10.94 us) leads bitpack-plan-windowed (16.41 us) by 50%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-plan-native beats baseline by 79% (significant)

bitpack-plan-native is -40.16 us (79%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.6x slower than the field

bitpack-plan-naive (50.75 us) is 4.6x the fastest (10.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-plan-naive shows warm-up / thermal drift (autocorr +0.60)

bitpack-plan-naive's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} vs {bitpack-plan-naive} (147% apart)

The field splits into a fast tier {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} and a slow tier {bitpack-plan-naive} with a 147% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-plan-native (10.94 us) to slowest bitpack-plan-naive (50.75 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-native** at 10944.0 ns median (-78.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.64x (fastest 10944.0 ns, slowest 50754.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 52613ns | 50891ns | 48396ns | 51117ns | 61319ns | base |
| bitpack-plan-native | 11030ns | 11068ns | 10437ns | 11070ns | 11503ns | -79.04% |
| bitpack-plan-simd | 20763ns | 20710ns | 19740ns | 20752ns | 21818ns | -60.54% |
| bitpack-plan-windowed | 16297ns | 16535ns | 15261ns | 16408ns | 16997ns | -69.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 52424ns | 48223ns | 61035ns | base | 1.875 |
| bitpack-plan-native | 10907ns | 10324ns | 11374ns | -79.19% | 9.013 |
| bitpack-plan-simd | 20622ns | 19612ns | 21654ns | -60.66% | 4.767 |
| bitpack-plan-windowed | 16170ns | 15145ns | 16854ns | -69.16% | 6.079 |

## Performance model

- Peak throughput: **9.522 Gops/s** (bitpack-plan-native; best 20% batches)
- Ops per call: 98304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.937 | 20.3% |
| bitpack-plan-native | 8.982 | 94.3% |
| bitpack-plan-simd | 4.781 | 50.2% |
| bitpack-plan-windowed | 5.990 | 62.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 52613ns | 52613ns | base |
| bitpack-plan-native | 11030ns | 11030ns | -79.04% |
| bitpack-plan-simd | 20763ns | 20763ns | -60.54% |
| bitpack-plan-windowed | 16297ns | 16297ns | -69.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 50754ns | base | --- | [49929, 52129] | --- | --- | --- | --- |
| bitpack-plan-native | 10944ns | -39940.4ns (-78.7%) | [-41240, -39179]ns | [10942, 10946] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 20559ns | -30066.1ns (-59.2%) | [-31220, -29224]ns | [20557, 20645] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 16412ns | -35130.7ns (-69.2%) | [-35617, -33999]ns | [16408, 16443] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|
| 1 | 68902ns | -84.1% | -65.5% | -74.1% |
| 2 | 77052ns | -85.8% | -72.9% | -78.6% |
| 3 | 56370ns | -80.5% | -63.2% | -69.9% |
| 4 | 52192ns | -79.0% | -60.0% | -68.1% |
| 5 | 62876ns | -82.6% | -67.7% | -73.7% |
| 6 | 61215ns | -82.1% | -69.1% | -73.1% |
| 7 | 55896ns | -80.4% | -66.0% | -70.6% |
| 8 | 52509ns | -79.0% | -62.1% | -66.9% |
| 9 | 52290ns | -79.7% | -60.5% | -69.7% |
| 10 | 51549ns | -80.4% | -59.8% | -69.4% |
| 11 | 47914ns | -77.2% | -56.5% | -68.4% |
| 12 | 47965ns | -77.2% | -57.1% | -68.5% |
| 13 | 49872ns | -78.1% | -58.8% | -68.4% |
| 14 | 52044ns | -78.9% | -60.5% | -68.5% |
| 15 | 49691ns | -78.0% | -58.6% | -67.0% |
| 16 | 50260ns | -78.2% | -58.4% | -67.3% |
| 17 | 49985ns | -78.1% | -58.7% | -67.2% |
| 18 | 50269ns | -78.2% | -58.0% | -67.4% |
| 19 | 49574ns | -77.9% | -58.5% | -66.8% |
| 20 | 51102ns | -78.6% | -59.8% | -67.9% |
| 21 | 52186ns | -79.0% | -56.4% | -68.5% |
| 22 | 52212ns | -79.0% | -58.0% | -68.4% |
| 23 | 53266ns | -79.5% | -61.4% | -69.1% |
| 24 | 52583ns | -79.1% | -60.9% | -68.8% |
| 25 | 49350ns | -77.8% | -58.1% | -66.7% |
| 26 | 51236ns | -78.6% | -59.8% | -67.9% |
| 27 | 52701ns | -78.9% | -60.8% | -68.8% |
| 28 | 52073ns | -77.0% | -60.5% | -68.4% |
| 29 | 48188ns | -75.2% | -57.4% | -65.9% |
| 30 | 51698ns | -76.8% | -60.2% | -68.2% |
| 31 | 50250ns | -79.9% | -61.3% | -69.9% |
| 32 | 48024ns | -79.0% | -60.2% | -66.9% |
| 33 | 48022ns | -79.0% | -59.1% | -68.1% |
| 34 | 49420ns | -79.6% | -58.4% | -69.4% |
| 35 | 50406ns | -78.9% | -59.2% | -70.0% |
| 36 | 49612ns | -77.9% | -58.6% | -69.5% |
| 37 | 50328ns | -78.3% | -58.4% | -70.0% |
| 38 | 48342ns | -77.4% | -57.5% | -68.2% |
| 39 | 47982ns | -77.2% | -57.2% | -65.8% |
| 40 | 49554ns | -77.9% | -57.9% | -66.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | 0.604 | HIGH+ (drift/warm-up) |
| bitpack-plan-native | 0.578 | HIGH+ (drift/warm-up) |
| bitpack-plan-simd | 0.402 | moderate+ |
| bitpack-plan-windowed | 0.591 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-plan-native**: won 40/40, lost 0/40
- **bitpack-plan-simd**: won 40/40, lost 0/40
- **bitpack-plan-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 5.5ns | 52424.0ns | 0.0% |  |
| bitpack-plan-native | 2.0ns | 10907.4ns | 0.0% |  |
| bitpack-plan-simd | 10.7ns | 20621.5ns | 0.1% |  |
| bitpack-plan-windowed | 3.0ns | 16169.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 48223.4-61034.7 ns)
  48223.4 |#####
  48864.0 |##########
  49504.6 |##############################
  50145.1 |#########################
  50785.7 |##########
  51426.3 |###############
  52066.8 |########################################
  52707.4 |#####
  53348.0 |
  53988.5 |
  54629.1 |
  55269.7 |#####
  55910.2 |#####
  56550.8 |
  57191.3 |
  57831.9 |
  58472.5 |
  59113.0 |
  59753.6 |
  60394.2 |
  (6 below, 4 above range)

bitpack-plan-native (n=40, range 10323.7-11373.8 ns)
  10323.7 |
  10376.2 |
  10428.7 |
  10481.2 |
  10533.7 |
  10586.2 |###
  10638.7 |
  10691.2 |
  10743.7 |
  10796.2 |
  10848.8 |
  10901.3 |########################################
  10953.8 |##########
  11006.3 |#
  11058.8 |#
  11111.3 |
  11163.8 |
  11216.3 |
  11268.8 |
  11321.3 |
  (5 below, 3 above range)

bitpack-plan-simd (n=40, range 19612.1-21653.5 ns)
  19612.1 |##
  19714.2 |
  19816.3 |##
  19918.3 |
  20020.4 |
  20122.5 |
  20224.6 |##
  20326.6 |
  20428.7 |
  20530.8 |########################################
  20632.8 |######
  20734.9 |######
  20837.0 |########
  20939.0 |##
  21041.1 |##
  21143.2 |
  21245.2 |
  21347.3 |
  21449.4 |
  21551.5 |
  (4 below, 3 above range)

bitpack-plan-windowed (n=40, range 15144.6-16854.1 ns)
  15144.6 |
  15230.1 |
  15315.6 |#####
  15401.1 |
  15486.5 |
  15572.0 |
  15657.5 |##
  15742.9 |##
  15828.4 |#####
  15913.9 |
  15999.4 |
  16084.8 |
  16170.3 |
  16255.8 |
  16341.2 |######################
  16426.7 |########################################
  16512.2 |##
  16597.6 |##
  16683.1 |
  16768.6 |
  (7 below, 3 above range)

```

## Diagnostics

- **bitpack-plan-naive**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **bitpack-plan-native**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **bitpack-plan-windowed**: autocorrelation=0.59 (measurement drift or warm-up artifact)
