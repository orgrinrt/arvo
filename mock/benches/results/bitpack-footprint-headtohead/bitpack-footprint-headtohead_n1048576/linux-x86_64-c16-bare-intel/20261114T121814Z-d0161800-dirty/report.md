# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 23% faster than the next best (bitpack-footprint-dense)

bitpack-footprint-packed (132.61 us) leads bitpack-footprint-dense (162.52 us) by 23%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 5.0x slower than the field

bitpack-footprint-packed-naive (667.64 us) is 5.0x the fastest (132.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-dense shows alternating (throttle bounce) (autocorr -0.72)

bitpack-footprint-dense's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} vs {bitpack-footprint-packed-naive} (308% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} and a slow tier {bitpack-footprint-packed-naive} with a 308% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.0x the fastest

Fastest bitpack-footprint-packed (132.61 us) to slowest bitpack-footprint-packed-naive (667.64 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 132612.5 ns median (-18.4% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.03x (fastest 132612.5 ns, slowest 667637.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 161774ns | 162677ns | 159554ns | 161727ns | 164135ns | base |
| bitpack-footprint-dense-alt | 163198ns | 163759ns | 161025ns | 163678ns | 163929ns | +0.88% |
| bitpack-footprint-packed | 131878ns | 132754ns | 129097ns | 132486ns | 132837ns | -18.48% |
| bitpack-footprint-packed-naive | 675953ns | 667790ns | 667087ns | 674975ns | 687753ns | +317.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 161602ns | 159418ns | 163875ns | base | 6.489 |
| bitpack-footprint-dense-alt | 163052ns | 160891ns | 163773ns | +0.90% | 6.431 |
| bitpack-footprint-packed | 131735ns | 128957ns | 132687ns | -18.48% | 7.960 |
| bitpack-footprint-packed-naive | 675798ns | 666930ns | 687605ns | +318.19% | 1.552 |

## Performance model

- Peak throughput: **8.131 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.452 | 79.4% |
| bitpack-footprint-dense-alt | 6.409 | 78.8% |
| bitpack-footprint-packed | 7.907 | 97.2% |
| bitpack-footprint-packed-naive | 1.571 | 19.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 161774ns | 161774ns | base |
| bitpack-footprint-dense-alt | 163198ns | 163198ns | +0.88% |
| bitpack-footprint-packed | 131878ns | 131878ns | -18.48% |
| bitpack-footprint-packed-naive | 675953ns | 675953ns | +317.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 162516ns | base | --- | [159448, 163357] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 163616ns | +1037.9ns (+0.6%) | [+323, +3949]ns | [163382, 163648] | YES | 0.0007 | 0.0007 | 0 |
| bitpack-footprint-packed | 132612ns | -30066.9ns (-18.5%) | [-31256, -26816]ns | [132601, 132634] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 667638ns | +507886.3ns (+312.5%) | [+507484, +523246]ns | [667130, 686859] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 163866ns | -1.5% | -19.1% | +319.5% |
| 1 | 2 | 159418ns | +2.5% | -16.8% | +330.8% |
| 1 | 3 | 163351ns | +0.2% | -21.1% | +308.4% |
| 1 | 4 | 159445ns | +2.6% | -16.8% | +318.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.716 | HIGH- (thermal bounce) |
| bitpack-footprint-dense-alt | 0.015 | ok |
| bitpack-footprint-packed | -0.420 | moderate- |
| bitpack-footprint-packed-naive | 0.260 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 3/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 28.5ns | 161602.0ns | 0.0% |  |
| bitpack-footprint-dense-alt | 28.0ns | 163051.7ns | 0.0% |  |
| bitpack-footprint-packed | 31.0ns | 131734.8ns | 0.0% |  |
| bitpack-footprint-packed-naive | 30.0ns | 675798.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 159417.9-163875.0 ns)
  159417.9 |########################################
  159640.8 |###
  159863.6 |###
  160086.5 |
  160309.3 |
  160532.2 |
  160755.0 |
  160977.9 |
  161200.8 |
  161423.6 |
  161646.5 |###
  161869.3 |
  162092.2 |
  162315.0 |
  162537.9 |
  162760.7 |
  162983.6 |
  163206.5 |#################################
  163429.3 |
  163652.2 |####################
  (5 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 160890.8-163773.3 ns)
  160890.8 |
  161034.9 |
  161179.0 |
  161323.1 |
  161467.3 |
  161611.4 |
  161755.5 |
  161899.7 |
  162043.8 |
  162187.9 |
  162332.0 |
  162476.2 |
  162620.3 |
  162764.4 |
  162908.6 |
  163052.7 |##
  163196.8 |
  163340.9 |################################
  163485.1 |#############
  163629.2 |########################################
  (5 below, 2 above range)

bitpack-footprint-packed (n=40, range 128957.0-132687.0 ns)
  128957.0 |##
  129143.5 |
  129330.0 |
  129516.5 |#
  129703.0 |
  129889.5 |
  130076.0 |
  130262.5 |
  130449.0 |
  130635.5 |
  130822.0 |
  131008.5 |
  131195.0 |
  131381.5 |
  131568.0 |
  131754.5 |
  131941.0 |
  132127.5 |
  132314.0 |
  132500.5 |########################################
  (7 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 666930.2-687605.4 ns)
  666930.2 |########################################
  667964.0 |
  668997.7 |
  670031.5 |
  671065.2 |
  672099.0 |
  673132.8 |
  674166.5 |
  675200.3 |
  676234.0 |
  677267.8 |##
  678301.6 |
  679335.3 |##
  680369.1 |
  681402.8 |
  682436.6 |
  683470.4 |
  684504.1 |
  685537.9 |
  686571.7 |############################
  (4 below, 3 above range)

```
