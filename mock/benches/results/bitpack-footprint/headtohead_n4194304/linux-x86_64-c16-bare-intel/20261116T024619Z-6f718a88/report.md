# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 32% faster than the next best (bitpack-footprint-dense)

bitpack-footprint-packed (530.10 us) leads bitpack-footprint-dense (699.89 us) by 32%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed beats baseline by 26% (significant)

bitpack-footprint-packed is -183.46 us (26%) faster than baseline bitpack-footprint-dense, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-footprint-packed-naive is an outlier: 5.1x slower than the field

bitpack-footprint-packed-naive (2.70 ms) is 5.1x the fastest (530.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} vs {bitpack-footprint-packed-naive} (279% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} and a slow tier {bitpack-footprint-packed-naive} with a 279% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (530.10 us) to slowest bitpack-footprint-packed-naive (2.70 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 530097.6 ns median (-24.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.09x (fastest 530097.6 ns, slowest 2696724.9 ns)

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
| bitpack-footprint-dense | 741914ns | 700192ns | 667337ns | 706882ns | 921589ns | base |
| bitpack-footprint-dense-alt | 702115ns | 711279ns | 666868ns | 699339ns | 745690ns | -5.36% |
| bitpack-footprint-packed | 525522ns | 530248ns | 515479ns | 526361ns | 533049ns | -29.17% |
| bitpack-footprint-packed-naive | 2697575ns | 2696942ns | 2695504ns | 2697081ns | 2701129ns | +263.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 741600ns | 667062ns | 921236ns | base | 5.656 |
| bitpack-footprint-dense-alt | 701747ns | 666533ns | 745176ns | -5.37% | 5.977 |
| bitpack-footprint-packed | 525360ns | 515323ns | 532887ns | -29.16% | 7.984 |
| bitpack-footprint-packed-naive | 2697350ns | 2695259ns | 2700893ns | +263.72% | 1.555 |

## Performance model

- Peak throughput: **8.139 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.993 | 73.6% |
| bitpack-footprint-dense-alt | 5.899 | 72.5% |
| bitpack-footprint-packed | 7.912 | 97.2% |
| bitpack-footprint-packed-naive | 1.555 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 741914ns | 741914ns | base |
| bitpack-footprint-dense-alt | 702115ns | 702115ns | -5.36% |
| bitpack-footprint-packed | 525522ns | 525522ns | -29.17% |
| bitpack-footprint-packed-naive | 2697575ns | 2697575ns | +263.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 699895ns | base | --- | [691804, 715357] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 711039ns | -29299.4ns (-4.2%) | [-35503, -2983]ns | [678947, 714970] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-footprint-packed | 530098ns | -183463.6ns (-26.2%) | [-196795, -176133]ns | [518578, 530312] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 2696725ns | +1997521.8ns (+285.4%) | [+1980332, +2004697]ns | [2696430, 2697331] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 921115ns | -20.6% | -42.4% | +192.8% |
| 1 | 2 | 692581ns | -3.7% | -25.6% | +289.4% |
| 1 | 3 | 715219ns | -5.1% | -27.1% | +277.0% |
| 1 | 4 | 668015ns | +6.9% | -20.3% | +303.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.125 | ok |
| bitpack-footprint-dense-alt | -0.283 | moderate- |
| bitpack-footprint-packed | -0.241 | moderate- |
| bitpack-footprint-packed-naive | -0.278 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 1/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 50.9ns | 741599.8ns | 0.0% |  |
| bitpack-footprint-dense-alt | 63.8ns | 701747.5ns | 0.0% |  |
| bitpack-footprint-packed | 29.6ns | 525360.0ns | 0.0% |  |
| bitpack-footprint-packed-naive | 63.6ns | 2697350.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 667061.8-921235.5 ns)
  667061.8 |########################################
  679770.5 |########
  692479.2 |########################
  705187.9 |################################
  717896.6 |####
  730605.3 |
  743313.9 |
  756022.6 |####
  768731.3 |
  781440.0 |
  794148.7 |
  806857.4 |
  819566.0 |
  832274.7 |####
  844983.4 |
  857692.1 |
  870400.8 |
  883109.5 |
  895818.1 |
  908526.8 |################
  (3 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 666533.0-745175.6 ns)
  666533.0 |#########################
  670465.1 |
  674397.2 |#####
  678329.4 |########################################
  682261.5 |
  686193.6 |
  690125.8 |
  694057.9 |
  697990.0 |
  701922.1 |
  705854.3 |#####
  709786.4 |###############
  713718.5 |##############################
  717650.7 |
  721582.8 |
  725514.9 |#####
  729447.1 |##############################
  733379.2 |
  737311.3 |#####
  741243.4 |
  (5 below, 3 above range)

bitpack-footprint-packed (n=40, range 515323.3-532887.2 ns)
  515323.3 |########################################
  516201.5 |
  517079.7 |
  517957.9 |
  518836.1 |
  519714.3 |
  520592.5 |###
  521470.7 |
  522348.9 |
  523227.1 |
  524105.2 |
  524983.4 |
  525861.6 |
  526739.8 |###
  527618.0 |###
  528496.2 |
  529374.4 |#####################
  530252.6 |#####################
  531130.8 |#######
  532009.0 |#########################
  (3 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 2695259.0-2700892.8 ns)
  2695259.0 |#############
  2695540.7 |#############
  2695822.4 |####################
  2696104.1 |######
  2696385.8 |########################################
  2696667.4 |##########################
  2696949.1 |#############
  2697230.8 |##########################
  2697512.5 |#############
  2697794.2 |#############
  2698075.9 |
  2698357.6 |
  2698639.3 |######
  2698921.0 |######
  2699202.7 |
  2699484.3 |######
  2699766.0 |
  2700047.7 |
  2700329.4 |
  2700611.1 |######
  (4 below, 4 above range)

```
