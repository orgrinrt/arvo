# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 24% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (131.78 us) leads bitpack-footprint-dense-alt (163.54 us) by 24%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 5.1x slower than the field

bitpack-footprint-packed-naive (668.51 us) is 5.1x the fastest (131.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed shows alternating (throttle bounce) (autocorr -0.72)

bitpack-footprint-packed's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (308% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 308% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (131.78 us) to slowest bitpack-footprint-packed-naive (668.51 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 131779.3 ns median (-19.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.07x (fastest 131779.3 ns, slowest 668505.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112765842 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 162172ns | 163998ns | 159619ns | 162292ns | 164366ns | base |
| bitpack-footprint-dense-alt | 161976ns | 163693ns | 159617ns | 161889ns | 164595ns | -0.12% |
| bitpack-footprint-packed | 132084ns | 131927ns | 131390ns | 131903ns | 133323ns | -18.55% |
| bitpack-footprint-packed-naive | 674000ns | 668696ns | 668060ns | 671721ns | 686775ns | +315.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 162003ns | 159453ns | 164193ns | base | 6.473 |
| bitpack-footprint-dense-alt | 161812ns | 159453ns | 164428ns | -0.12% | 6.480 |
| bitpack-footprint-packed | 131921ns | 131239ns | 133111ns | -18.57% | 7.949 |
| bitpack-footprint-packed-naive | 673830ns | 667896ns | 686600ns | +315.94% | 1.556 |

## Performance model

- Peak throughput: **7.990 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.400 | 80.1% |
| bitpack-footprint-dense-alt | 6.412 | 80.2% |
| bitpack-footprint-packed | 7.957 | 99.6% |
| bitpack-footprint-packed-naive | 1.569 | 19.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 162172ns | 162172ns | base |
| bitpack-footprint-dense-alt | 161976ns | 161976ns | -0.12% |
| bitpack-footprint-packed | 132084ns | 132084ns | -18.55% |
| bitpack-footprint-packed-naive | 674000ns | 674000ns | +315.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 163830ns | base | --- | [159562, 163867] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 163542ns | no significant difference | [-311, +28]ns | [159528, 163563] | no | 0.2682 | 0.2682 | 0 |
| bitpack-footprint-packed | 131779ns | -30737.9ns (-18.8%) | [-32022, -27975]ns | [131666, 131820] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 668506ns | +506761.6ns (+309.3%) | [+504568, +509000]ns | [668051, 669440] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 163856ns | -0.2% | -19.6% | +307.7% |
| 1 | 2 | 163946ns | -2.7% | -19.7% | +307.6% |
| 1 | 3 | 159460ns | -0.0% | -17.3% | +319.1% |
| 1 | 4 | 159556ns | +2.5% | -17.5% | +330.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.239 | moderate+ |
| bitpack-footprint-dense-alt | -0.250 | moderate- |
| bitpack-footprint-packed | -0.718 | HIGH- (thermal bounce) |
| bitpack-footprint-packed-naive | -0.081 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 1/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 26.8ns | 162003.4ns | 0.0% |  |
| bitpack-footprint-dense-alt | 28.7ns | 161811.6ns | 0.0% |  |
| bitpack-footprint-packed | 27.1ns | 131920.7ns | 0.0% |  |
| bitpack-footprint-packed-naive | 30.6ns | 673830.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 159453.2-164193.1 ns)
  159453.2 |##############################
  159690.2 |
  159927.2 |
  160164.2 |
  160401.2 |
  160638.2 |##
  160875.2 |
  161112.2 |
  161349.2 |
  161586.2 |
  161823.2 |
  162060.2 |
  162297.2 |
  162534.1 |
  162771.1 |
  163008.1 |
  163245.1 |
  163482.1 |
  163719.1 |########################################
  163956.1 |#######
  (5 below, 3 above range)

bitpack-footprint-dense-alt (n=40, range 159453.3-164428.5 ns)
  159453.3 |################################
  159702.0 |##
  159950.8 |
  160199.5 |
  160448.3 |
  160697.1 |
  160945.8 |
  161194.6 |
  161443.3 |
  161692.1 |
  161940.9 |
  162189.6 |
  162438.4 |
  162687.1 |
  162935.9 |
  163184.7 |
  163433.4 |########################################
  163682.2 |##
  163930.9 |##
  164179.7 |
  (4 below, 2 above range)

bitpack-footprint-packed (n=40, range 131239.4-133111.1 ns)
  131239.4 |
  131333.0 |
  131426.6 |
  131520.1 |##################
  131613.7 |########################################
  131707.3 |##################
  131800.9 |####################################
  131894.5 |
  131988.1 |###
  132081.7 |
  132175.3 |
  132268.8 |
  132362.4 |
  132456.0 |###
  132549.6 |###
  132643.2 |
  132736.8 |###
  132830.4 |
  132924.0 |
  133017.5 |###
  (1 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 667896.3-686599.9 ns)
  667896.3 |########################################
  668831.5 |#####
  669766.6 |
  670701.8 |
  671637.0 |#
  672572.2 |
  673507.4 |
  674442.5 |
  675377.7 |
  676312.9 |
  677248.1 |
  678183.3 |
  679118.4 |
  680053.6 |
  680988.8 |
  681924.0 |
  682859.2 |
  683794.3 |
  684729.5 |
  685664.7 |#################
  (3 below, 3 above range)

```
