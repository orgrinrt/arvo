# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 25% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (131.07 us) leads bitpack-footprint-dense-alt (163.40 us) by 25%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed beats baseline by 20% (significant)

bitpack-footprint-packed is -33.18 us (20%) faster than baseline bitpack-footprint-dense, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-footprint-packed-naive is an outlier: 5.1x slower than the field

bitpack-footprint-packed-naive (667.48 us) is 5.1x the fastest (131.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed-naive shows alternating (throttle bounce) (autocorr -0.76)

bitpack-footprint-packed-naive's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (308% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 308% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (131.07 us) to slowest bitpack-footprint-packed-naive (667.48 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 131072.5 ns median (-19.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.09x (fastest 131072.5 ns, slowest 667478.8 ns)

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
| Counter frequency | 2112837581 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 162961ns | 163839ns | 159625ns | 163525ns | 164603ns | base |
| bitpack-footprint-dense-alt | 162890ns | 163561ns | 159908ns | 163581ns | 163796ns | -0.04% |
| bitpack-footprint-packed | 131019ns | 131233ns | 129086ns | 131047ns | 132866ns | -19.60% |
| bitpack-footprint-packed-naive | 676299ns | 667632ns | 667075ns | 674912ns | 689685ns | +315.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 162798ns | 159459ns | 164450ns | base | 6.441 |
| bitpack-footprint-dense-alt | 162682ns | 159761ns | 163457ns | -0.07% | 6.446 |
| bitpack-footprint-packed | 130846ns | 128926ns | 132692ns | -19.63% | 8.014 |
| bitpack-footprint-packed-naive | 676137ns | 666918ns | 689532ns | +315.32% | 1.551 |

## Performance model

- Peak throughput: **8.133 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.406 | 78.8% |
| bitpack-footprint-dense-alt | 6.417 | 78.9% |
| bitpack-footprint-packed | 8.000 | 98.4% |
| bitpack-footprint-packed-naive | 1.571 | 19.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 162961ns | 162961ns | base |
| bitpack-footprint-dense-alt | 162890ns | 162890ns | -0.04% |
| bitpack-footprint-packed | 131019ns | 131019ns | -19.60% |
| bitpack-footprint-packed-naive | 676299ns | 676299ns | +315.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 163681ns | base | --- | [163670, 163692] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 163402ns | -276.0ns (-0.2%) | [-477, -241]ns | [163372, 163418] | YES | 0.0022 | 0.0022 | 0 |
| bitpack-footprint-packed | 131073ns | -33177.6ns (-20.3%) | [-34721, -31008]ns | [128977, 132668] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 667479ns | +507547.7ns (+310.1%) | [+507185, +523240]ns | [667003, 686993] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 163685ns | -2.6% | -18.9% | +319.7% |
| 1 | 2 | 159460ns | +2.5% | -16.8% | +318.3% |
| 1 | 3 | 163688ns | -0.2% | -21.2% | +320.3% |
| 1 | 4 | 163780ns | -0.3% | -21.3% | +307.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.399 | moderate- |
| bitpack-footprint-dense-alt | -0.086 | ok |
| bitpack-footprint-packed | 0.250 | moderate+ |
| bitpack-footprint-packed-naive | -0.761 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 1/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 27.2ns | 162798.2ns | 0.0% |  |
| bitpack-footprint-dense-alt | 27.1ns | 162681.5ns | 0.0% |  |
| bitpack-footprint-packed | 28.0ns | 130845.8ns | 0.0% |  |
| bitpack-footprint-packed-naive | 32.2ns | 676137.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 159458.6-164449.8 ns)
  159458.6 |##########
  159708.1 |####
  159957.7 |
  160207.3 |
  160456.8 |
  160706.4 |
  160955.9 |
  161205.5 |
  161455.1 |
  161704.6 |
  161954.2 |
  162203.7 |
  162453.3 |
  162702.9 |
  162952.4 |
  163202.0 |
  163451.5 |########################################
  163701.1 |########
  163950.6 |##
  164200.2 |##
  (3 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 159761.0-163456.5 ns)
  159761.0 |
  159945.7 |
  160130.5 |
  160315.3 |
  160500.1 |
  160684.9 |
  160869.6 |
  161054.4 |
  161239.2 |
  161424.0 |
  161608.7 |
  161793.5 |
  161978.3 |#
  162163.1 |
  162347.8 |
  162532.6 |
  162717.4 |
  162902.2 |
  163087.0 |
  163271.7 |########################################
  (7 below, 2 above range)

bitpack-footprint-packed (n=40, range 128925.5-132691.6 ns)
  128925.5 |################################
  129113.8 |##
  129302.1 |
  129490.4 |#####
  129678.7 |
  129867.0 |
  130055.4 |
  130243.7 |
  130432.0 |
  130620.3 |
  130808.6 |
  130996.9 |
  131185.2 |
  131373.5 |
  131561.8 |
  131750.1 |
  131938.4 |
  132126.7 |
  132315.0 |
  132503.3 |########################################
  (4 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 666917.9-689531.9 ns)
  666917.9 |########################################
  668048.6 |
  669179.3 |##
  670310.0 |
  671440.7 |
  672571.4 |
  673702.1 |
  674832.8 |
  675963.5 |
  677094.2 |
  678224.9 |
  679355.6 |
  680486.3 |
  681617.0 |
  682747.7 |
  683878.4 |
  685009.1 |
  686139.8 |#############
  687270.5 |#################
  688401.2 |##
  (4 below, 2 above range)

```
