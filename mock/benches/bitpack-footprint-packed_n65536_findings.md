# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 347% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (10.95 us) leads bitpack-footprint-packed-naive (48.88 us) by 347%, a clear separation rather than a photo finish. CV 12.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed is fastest but the noisiest (CV 12.4%)

bitpack-footprint-packed wins on median (10.95 us) yet has the highest variance (CV 12.4%), while bitpack-footprint-packed-naive is the steadiest (CV 6.9%, 48.88 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (10.95 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.5x the fastest

Fastest bitpack-footprint-packed (10.95 us) to slowest bitpack-footprint-packed-naive (48.88 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 10945.2 ns median
- 1 variant significantly slower than baseline
- Spread: 4.47x (fastest 10945.2 ns, slowest 48875.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 11337ns | 11041ns | 10667ns | 11069ns | 12811ns | base |
| bitpack-footprint-packed-naive | 49335ns | 48975ns | 46368ns | 49025ns | 53230ns | +335.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 11228ns | 10577ns | 12645ns | base | 5.837 |
| bitpack-footprint-packed-naive | 49204ns | 46248ns | 53051ns | +338.21% | 1.332 |

## Performance model

- Peak throughput: **6.196 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 5.988 | 96.6% |
| bitpack-footprint-packed-naive | 1.341 | 21.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 11337ns | 11337ns | base |
| bitpack-footprint-packed-naive | 49335ns | 49335ns | +335.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 10945ns | base | --- | [10944, 10989] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 48876ns | +37903.0ns (+346.3%) | [+37754, +38068]ns | [48820, 49009] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|
| 1 | 11036ns | +311.0% |
| 2 | 10945ns | +310.6% |
| 3 | 11013ns | +316.2% |
| 4 | 10944ns | +346.5% |
| 5 | 11495ns | +324.5% |
| 6 | 10943ns | +340.9% |
| 7 | 10946ns | +317.4% |
| 8 | 10945ns | +352.5% |
| 9 | 11021ns | +344.0% |
| 10 | 10946ns | +427.7% |
| 11 | 10944ns | +337.2% |
| 12 | 10546ns | +363.2% |
| 13 | 10083ns | +390.3% |
| 14 | 10081ns | +384.8% |
| 15 | 10145ns | +381.4% |
| 16 | 18857ns | +161.0% |
| 17 | 11400ns | +328.1% |
| 18 | 11135ns | +339.7% |
| 19 | 10946ns | +345.8% |
| 20 | 10943ns | +351.5% |
| 21 | 11209ns | +337.2% |
| 22 | 10939ns | +342.9% |
| 23 | 11007ns | +323.8% |
| 24 | 14169ns | +269.5% |
| 25 | 10944ns | +351.0% |
| 26 | 10942ns | +349.5% |
| 27 | 11180ns | +338.1% |
| 28 | 10942ns | +503.7% |
| 29 | 10941ns | +338.9% |
| 30 | 10941ns | +351.3% |
| 31 | 11691ns | +331.9% |
| 32 | 10943ns | +346.3% |
| 33 | 11162ns | +340.8% |
| 34 | 10944ns | +317.0% |
| 35 | 10971ns | +346.5% |
| 36 | 10943ns | +347.9% |
| 37 | 10945ns | +346.6% |
| 38 | 11101ns | +339.7% |
| 39 | 10961ns | +347.3% |
| 40 | 10942ns | +346.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.054 | ok |
| bitpack-footprint-packed-naive | -0.016 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 3.5ns | 11228.3ns | 0.0% |  |
| bitpack-footprint-packed-naive | 4.4ns | 49203.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 10577.3-12645.4 ns)
  10577.3 |
  10680.7 |
  10784.1 |
  10887.5 |########################################
  10990.9 |#######
  11094.3 |#######
  11197.7 |#
  11301.1 |#
  11404.5 |#
  11507.9 |
  11611.3 |#
  11714.7 |
  11818.1 |
  11921.5 |
  12025.0 |
  12128.4 |
  12231.8 |
  12335.2 |
  12438.6 |
  12542.0 |
  (4 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 46248.1-53050.7 ns)
  46248.1 |
  46588.2 |###
  46928.3 |
  47268.4 |
  47608.6 |###
  47948.7 |######
  48288.8 |###
  48629.0 |########################################
  48969.1 |########################
  49309.2 |###############
  49649.4 |
  49989.5 |
  50329.6 |###
  50669.8 |
  51009.9 |
  51350.0 |
  51690.1 |
  52030.3 |###
  52370.4 |
  52710.5 |
  (5 below, 2 above range)

```
