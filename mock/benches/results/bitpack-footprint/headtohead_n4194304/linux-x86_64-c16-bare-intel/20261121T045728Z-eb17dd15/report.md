# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 26% faster than the next best (bitpack-footprint-dense)

bitpack-footprint-packed (528.28 us) leads bitpack-footprint-dense (666.13 us) by 26%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed beats baseline by 22% (significant)

bitpack-footprint-packed is -144.45 us (22%) faster than baseline bitpack-footprint-dense, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-footprint-packed-naive is an outlier: 5.1x slower than the field

bitpack-footprint-packed-naive (2.70 ms) is 5.1x the fastest (528.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} vs {bitpack-footprint-packed-naive} (297% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} and a slow tier {bitpack-footprint-packed-naive} with a 297% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (528.28 us) to slowest bitpack-footprint-packed-naive (2.70 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 528281.1 ns median (-20.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.11x (fastest 528281.1 ns, slowest 2701030.6 ns)

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
| bitpack-footprint-dense | 671478ns | 666396ns | 638797ns | 668122ns | 714227ns | base |
| bitpack-footprint-dense-alt | 676905ns | 680430ns | 651149ns | 678314ns | 698433ns | +0.81% |
| bitpack-footprint-packed | 531254ns | 528550ns | 510887ns | 524077ns | 573151ns | -20.88% |
| bitpack-footprint-packed-naive | 2709159ns | 2701230ns | 2699146ns | 2701415ns | 2742403ns | +303.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 671232ns | 638600ns | 714015ns | base | 6.249 |
| bitpack-footprint-dense-alt | 676607ns | 650995ns | 698071ns | +0.80% | 6.199 |
| bitpack-footprint-packed | 531054ns | 510711ns | 572874ns | -20.88% | 7.898 |
| bitpack-footprint-packed-naive | 2708943ns | 2698943ns | 2742163ns | +303.58% | 1.548 |

## Performance model

- Peak throughput: **8.213 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.297 | 76.7% |
| bitpack-footprint-dense-alt | 6.167 | 75.1% |
| bitpack-footprint-packed | 7.940 | 96.7% |
| bitpack-footprint-packed-naive | 1.553 | 18.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 671478ns | 671478ns | base |
| bitpack-footprint-dense-alt | 676905ns | 676905ns | +0.81% |
| bitpack-footprint-packed | 531254ns | 531254ns | -20.88% |
| bitpack-footprint-packed-naive | 2709159ns | 2709159ns | +303.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 666128ns | base | --- | [656302, 674956] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 680153ns | no significant difference | [-2669, +26554]ns | [676913, 682730] | no | 0.8746 | 0.8746 | 0 |
| bitpack-footprint-packed | 528281ns | -144449.7ns (-21.7%) | [-145633, -131278]ns | [511829, 528757] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 2701031ns | +2042250.1ns (+306.6%) | [+2026764, +2048466]ns | [2699866, 2701974] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 674562ns | +2.8% | -21.6% | +300.5% |
| 1 | 2 | 713775ns | -4.8% | -21.3% | +278.3% |
| 1 | 3 | 638444ns | +7.0% | -20.0% | +322.9% |
| 1 | 4 | 656311ns | -0.0% | -22.0% | +311.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.244 | moderate- |
| bitpack-footprint-dense-alt | -0.099 | ok |
| bitpack-footprint-packed | -0.172 | ok |
| bitpack-footprint-packed-naive | -0.278 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 2/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 67.5ns | 671232.1ns | 0.0% |  |
| bitpack-footprint-dense-alt | 77.5ns | 676607.1ns | 0.0% |  |
| bitpack-footprint-packed | 29.3ns | 531054.3ns | 0.0% |  |
| bitpack-footprint-packed-naive | 62.2ns | 2708942.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 638599.8-714015.2 ns)
  638599.8 |#########################
  642370.5 |
  646141.3 |
  649912.1 |
  653682.8 |##############################
  657453.6 |####################
  661224.4 |
  664995.2 |
  668765.9 |
  672536.7 |########################################
  676307.5 |##########
  680078.2 |
  683849.0 |
  687619.8 |
  691390.5 |
  695161.3 |
  698932.1 |
  702702.9 |
  706473.6 |
  710244.4 |#########################
  (5 below, 5 above range)

bitpack-footprint-dense-alt (n=40, range 650994.7-698070.9 ns)
  650994.7 |
  653348.5 |
  655702.3 |########################################
  658056.1 |
  660409.9 |
  662763.7 |
  665117.5 |
  667471.3 |
  669825.2 |
  672179.0 |#################
  674532.8 |#####
  676886.6 |######################
  679240.4 |########################################
  681594.2 |########################################
  683948.0 |#####
  686301.8 |###########
  688655.6 |
  691009.4 |
  693363.2 |
  695717.1 |
  (3 below, 5 above range)

bitpack-footprint-packed (n=40, range 510711.2-572873.6 ns)
  510711.2 |########################################
  513819.3 |
  516927.4 |
  520035.5 |
  523143.7 |
  526251.8 |##############################
  529359.9 |
  532468.0 |
  535576.2 |
  538684.3 |
  541792.4 |
  544900.5 |
  548008.6 |
  551116.8 |
  554224.9 |##########
  557333.0 |##
  560441.1 |
  563549.3 |##
  566657.4 |
  569765.5 |
  (2 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 2698942.8-2742162.7 ns)
  2698942.8 |########################################
  2701103.8 |####################
  2703264.8 |####
  2705425.7 |####
  2707586.7 |
  2709747.7 |##
  2711908.7 |
  2714069.7 |
  2716230.7 |
  2718391.7 |
  2720552.7 |
  2722713.7 |##
  2724874.7 |
  2727035.7 |
  2729196.7 |
  2731357.7 |
  2733518.7 |
  2735679.7 |
  2737840.7 |
  2740001.7 |
  (3 below, 4 above range)

```
