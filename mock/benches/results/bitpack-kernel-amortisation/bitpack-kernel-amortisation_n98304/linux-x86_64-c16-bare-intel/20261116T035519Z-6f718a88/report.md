# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (71.72 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-simd at 46.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-simd beats baseline by 34% (significant)

bitpack-mac-simd is -24.52 us (34%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-mac-simd shows alternating (throttle bounce) (autocorr -0.71)

bitpack-mac-simd's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-mac-simd, bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-native} vs {bitpack-mac-naive} (37% apart)

The field splits into a fast tier {bitpack-mac-simd, bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-simd** at 46851.8 ns median (-34.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.53x (fastest 46851.8 ns, slowest 71722.0 ns)

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
| bitpack-mac-naive | 71595ns | 71792ns | 70569ns | 71803ns | 71995ns | base |
| bitpack-mac-narrow | 48257ns | 48421ns | 46910ns | 48126ns | 49997ns | -32.60% |
| bitpack-mac-native | 52415ns | 52369ns | 52308ns | 52378ns | 52635ns | -26.79% |
| bitpack-mac-simd | 47490ns | 46922ns | 46895ns | 47347ns | 48514ns | -33.67% |
| bitpack-mac-windowed | 48235ns | 48494ns | 47230ns | 48470ns | 48539ns | -32.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 71520ns | 70497ns | 71917ns | base | 1.374 |
| bitpack-mac-narrow | 48183ns | 46841ns | 49909ns | -32.63% | 2.040 |
| bitpack-mac-native | 52338ns | 52233ns | 52551ns | -26.82% | 1.878 |
| bitpack-mac-simd | 47411ns | 46812ns | 48436ns | -33.71% | 2.073 |
| bitpack-mac-windowed | 48162ns | 47160ns | 48455ns | -32.66% | 2.041 |

## Performance model

- Peak throughput: **2.100 Gops/s** (bitpack-mac-simd; best 20% batches)
- Ops per call: 98304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.371 | 65.3% |
| bitpack-mac-narrow | 2.033 | 96.8% |
| bitpack-mac-native | 1.880 | 89.5% |
| bitpack-mac-simd | 2.098 | 99.9% |
| bitpack-mac-windowed | 2.030 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 71595ns | 71595ns | base |
| bitpack-mac-narrow | 48257ns | 48257ns | -32.60% |
| bitpack-mac-native | 52415ns | 52415ns | -26.79% |
| bitpack-mac-simd | 47490ns | 47490ns | -33.67% |
| bitpack-mac-windowed | 48235ns | 48235ns | -32.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 71722ns | base | --- | [71714, 71736] | --- | --- | --- | --- |
| bitpack-mac-narrow | 48354ns | -23354.6ns (-32.6%) | [-23458, -23279]ns | [48157, 48440] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 52294ns | -19435.3ns (-27.1%) | [-19460, -19380]ns | [52280, 52318] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 46852ns | -24521.1ns (-34.2%) | [-24888, -23314]ns | [46843, 48074] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 48418ns | -23323.4ns (-32.5%) | [-23367, -23287]ns | [48366, 48436] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 71718ns | -32.5% | -27.1% | -32.5% | -32.4% |
| 1 | 2 | 71806ns | -32.6% | -27.1% | -34.8% | -32.5% |
| 1 | 3 | 71720ns | -33.6% | -27.1% | -32.8% | -32.5% |
| 1 | 4 | 71378ns | -34.4% | -26.7% | -34.4% | -32.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | 0.011 | ok |
| bitpack-mac-narrow | 0.252 | moderate+ |
| bitpack-mac-native | -0.570 | HIGH- (thermal bounce) |
| bitpack-mac-simd | -0.710 | HIGH- (thermal bounce) |
| bitpack-mac-windowed | 0.296 | moderate+ |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 19.5ns | 71519.9ns | 0.0% |  |
| bitpack-mac-narrow | 19.5ns | 48183.4ns | 0.0% |  |
| bitpack-mac-native | 19.0ns | 52338.0ns | 0.0% |  |
| bitpack-mac-simd | 19.2ns | 47411.0ns | 0.0% |  |
| bitpack-mac-windowed | 19.3ns | 48162.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 70496.9-71917.4 ns)
  70496.9 |
  70568.0 |
  70639.0 |
  70710.0 |
  70781.0 |
  70852.0 |
  70923.1 |
  70994.1 |#
  71065.1 |
  71136.1 |
  71207.1 |
  71278.2 |
  71349.2 |
  71420.2 |
  71491.2 |
  71562.3 |
  71633.3 |###
  71704.3 |########################################
  71775.3 |#######
  71846.3 |
  (4 below, 2 above range)

bitpack-mac-narrow (n=40, range 46841.1-49908.8 ns)
  46841.1 |########################
  46994.5 |
  47147.9 |
  47301.3 |
  47454.7 |
  47608.1 |
  47761.4 |
  47914.8 |##
  48068.2 |
  48221.6 |###############
  48375.0 |########################################
  48528.4 |
  48681.7 |
  48835.1 |
  48988.5 |
  49141.9 |
  49295.3 |
  49448.7 |
  49602.0 |
  49755.4 |
  (2 below, 1 above range)

bitpack-mac-native (n=40, range 52232.8-52550.9 ns)
  52232.8 |######
  52248.7 |####################
  52264.6 |########################################
  52280.5 |########################################
  52296.4 |####################
  52312.3 |####################
  52328.2 |#############
  52344.1 |####################
  52360.0 |
  52375.9 |
  52391.8 |######
  52407.7 |######
  52423.6 |######
  52439.6 |
  52455.5 |
  52471.4 |
  52487.3 |####################
  52503.2 |
  52519.1 |
  52535.0 |
  (5 below, 2 above range)

bitpack-mac-simd (n=40, range 46811.6-48435.8 ns)
  46811.6 |########################################
  46892.8 |
  46974.0 |
  47055.2 |
  47136.4 |
  47217.6 |
  47298.9 |
  47380.1 |
  47461.3 |
  47542.5 |
  47623.7 |
  47704.9 |
  47786.1 |
  47867.3 |
  47948.6 |
  48029.8 |###
  48111.0 |
  48192.2 |
  48273.4 |###
  48354.6 |##############
  (3 below, 3 above range)

bitpack-mac-windowed (n=40, range 47160.4-48455.4 ns)
  47160.4 |
  47225.2 |
  47289.9 |
  47354.7 |
  47419.4 |
  47484.2 |
  47548.9 |
  47613.7 |#
  47678.4 |
  47743.2 |
  47807.9 |
  47872.7 |
  47937.4 |
  48002.2 |
  48066.9 |
  48131.7 |
  48196.4 |
  48261.2 |#######
  48325.9 |###########
  48390.7 |########################################
  (6 below, 2 above range)

```
