# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 410% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (130.70 us) leads bitpack-footprint-packed-naive (667.07 us) by 410%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive shows alternating (throttle bounce) (autocorr -0.50)

bitpack-footprint-packed-naive's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (130.70 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (130.70 us) to slowest bitpack-footprint-packed-naive (667.07 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 130699.6 ns median
- 1 variant significantly slower than baseline
- Spread: 5.10x (fastest 130699.6 ns, slowest 667070.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 130859ns | 130871ns | 129070ns | 130854ns | 132660ns | base |
| bitpack-footprint-packed-naive | 675108ns | 667236ns | 667067ns | 667352ns | 706417ns | +415.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 130710ns | 128930ns | 132506ns | base | 8.022 |
| bitpack-footprint-packed-naive | 674945ns | 666911ns | 706254ns | +416.37% | 1.554 |

## Performance model

- Peak throughput: **8.133 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 8.023 | 98.6% |
| bitpack-footprint-packed-naive | 1.572 | 19.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 130859ns | 130859ns | base |
| bitpack-footprint-packed-naive | 675108ns | 675108ns | +415.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 130700ns | base | --- | [128954, 132458] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 667071ns | +537941.2ns (+411.6%) | [+534860, +538119]ns | [667001, 667321] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 128935ns | +417.4% |
| 1 | 2 | 132454ns | +403.6% |
| 1 | 3 | 132475ns | +403.6% |
| 1 | 4 | 128952ns | +417.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.250 | moderate- |
| bitpack-footprint-packed-naive | -0.501 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 29.6ns | 130709.8ns | 0.0% |  |
| bitpack-footprint-packed-naive | 32.6ns | 674944.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 128929.8-132506.0 ns)
  128929.8 |########################################
  129108.6 |
  129287.4 |
  129466.2 |
  129645.0 |
  129823.8 |
  130002.6 |
  130181.5 |
  130360.3 |
  130539.1 |
  130717.9 |
  130896.7 |
  131075.5 |
  131254.3 |
  131433.1 |
  131612.0 |
  131790.8 |
  131969.6 |
  132148.4 |
  132327.2 |########################################
  (2 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 666910.9-706253.6 ns)
  666910.9 |########################################
  668878.0 |#
  670845.1 |#
  672812.3 |#
  674779.4 |
  676746.5 |
  678713.7 |
  680680.8 |
  682647.9 |
  684615.1 |
  686582.2 |
  688549.4 |
  690516.5 |
  692483.6 |
  694450.8 |
  696417.9 |
  698385.0 |
  700352.2 |
  702319.3 |
  704286.4 |
  (2 below, 2 above range)

```
