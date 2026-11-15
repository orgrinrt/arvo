# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 24% faster than the next best (bitpack-footprint-dense)

bitpack-footprint-packed (2.47 us) leads bitpack-footprint-dense (3.05 us) by 24%, a clear separation rather than a photo finish. CV 97.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 5.1x slower than the field

bitpack-footprint-packed-naive (12.68 us) is 5.1x the fastest (2.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed is fastest but the noisiest (CV 97.6%)

bitpack-footprint-packed wins on median (2.47 us) yet has the highest variance (CV 97.6%), while bitpack-footprint-packed-naive is the steadiest (CV 7.9%, 12.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} vs {bitpack-footprint-packed-naive} (315% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} and a slow tier {bitpack-footprint-packed-naive} with a 315% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (2.47 us) to slowest bitpack-footprint-packed-naive (12.68 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-packed is inconsistent: worst-20% is 3.8x its best-20%

bitpack-footprint-packed's best 20% of batches run at 2.16 us but its worst 20% at 8.21 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-footprint-packed** at 2467.2 ns median (-19.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.14x (fastest 2467.2 ns, slowest 12675.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 4401ns | 3117ns | 2762ns | 3430ns | 8953ns | base |
| bitpack-footprint-dense-alt | 4639ns | 3118ns | 2623ns | 3548ns | 9930ns | +5.41% |
| bitpack-footprint-packed | 4081ns | 2530ns | 2217ns | 3235ns | 8483ns | -7.27% |
| bitpack-footprint-packed-naive | 12184ns | 12745ns | 10432ns | 12558ns | 12816ns | +176.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 4298ns | 2705ns | 8711ns | base | 3.812 |
| bitpack-footprint-dense-alt | 4530ns | 2568ns | 9668ns | +5.40% | 3.617 |
| bitpack-footprint-packed | 3965ns | 2160ns | 8207ns | -7.76% | 4.133 |
| bitpack-footprint-packed-naive | 12112ns | 10370ns | 12730ns | +181.80% | 1.353 |

## Performance model

- Peak throughput: **7.585 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.371 | 70.8% |
| bitpack-footprint-dense-alt | 5.371 | 70.8% |
| bitpack-footprint-packed | 6.641 | 87.5% |
| bitpack-footprint-packed-naive | 1.293 | 17.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 4401ns | 4401ns | base |
| bitpack-footprint-dense-alt | 4639ns | 4639ns | +5.41% |
| bitpack-footprint-packed | 4081ns | 4081ns | -7.27% |
| bitpack-footprint-packed-naive | 12184ns | 12184ns | +176.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 3051ns | base | --- | [3050, 3063] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 3051ns | no significant difference | [-0, +1]ns | [3051, 3063] | no | 0.6271 | 0.6271 | 2 |
| bitpack-footprint-packed | 2467ns | -583.3ns (-19.1%) | [-608, -548]ns | [2453, 2851] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 12675ns | +9622.2ns (+315.4%) | [+7875, +9624]ns | [12673, 12680] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 3031ns | -17.5% | +22.9% | +242.2% |
| 1 | 2 | 3051ns | +0.4% | -19.5% | +315.9% |
| 1 | 3 | 3051ns | +0.2% | -19.1% | +315.5% |
| 1 | 4 | 3056ns | -0.2% | -19.3% | +314.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.031 | ok |
| bitpack-footprint-dense-alt | -0.095 | ok |
| bitpack-footprint-packed | -0.090 | ok |
| bitpack-footprint-packed-naive | -0.087 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 2/4
- **bitpack-footprint-packed**: won 3/4, lost 1/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 33.3ns | 4298.0ns | 0.8% |  |
| bitpack-footprint-dense-alt | 35.2ns | 4530.3ns | 0.8% |  |
| bitpack-footprint-packed | 38.1ns | 3964.5ns | 1.0% |  |
| bitpack-footprint-packed-naive | 22.9ns | 12111.6ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 2705.1-8711.0 ns)
   2705.1 |
   3005.4 |########################################
   3305.7 |#
   3606.0 |
   3906.3 |
   4206.6 |
   4506.9 |
   4807.2 |
   5107.5 |
   5407.8 |
   5708.1 |#
   6008.4 |
   6308.7 |
   6609.0 |###
   6909.3 |#
   7209.5 |#
   7509.8 |#
   7810.1 |
   8110.4 |
   8410.7 |
  (5 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 2567.7-9668.4 ns)
   2567.7 |
   2922.7 |########################################
   3277.7 |
   3632.8 |
   3987.8 |
   4342.9 |
   4697.9 |#
   5052.9 |#
   5408.0 |
   5763.0 |#
   6118.0 |#
   6473.1 |
   6828.1 |
   7183.1 |
   7538.2 |
   7893.2 |#
   8248.2 |
   8603.3 |###
   8958.3 |
   9313.3 |#
  (7 below, 4 above range)

bitpack-footprint-packed (n=40, range 2159.9-8206.6 ns)
   2159.9 |########################################
   2462.3 |####################################
   2764.6 |
   3066.9 |###
   3369.3 |
   3671.6 |
   3973.9 |
   4276.3 |
   4578.6 |###
   4880.9 |
   5183.3 |#######
   5485.6 |
   5787.9 |
   6090.3 |#######
   6392.6 |
   6694.9 |###
   6997.3 |###
   7299.6 |
   7601.9 |
   7904.3 |###
  (5 below, 5 above range)

bitpack-footprint-packed-naive (n=40, range 10370.2-12730.5 ns)
  10370.2 |######
  10488.2 |
  10606.2 |
  10724.2 |
  10842.2 |
  10960.3 |
  11078.3 |
  11196.3 |
  11314.3 |
  11432.3 |
  11550.3 |
  11668.3 |
  11786.3 |
  11904.4 |
  12022.4 |
  12140.4 |
  12258.4 |
  12376.4 |
  12494.4 |
  12612.4 |########################################
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: CV=56.9% (high variance, measurements may be unstable)
- **bitpack-footprint-dense**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **bitpack-footprint-dense-alt**: CV=60.3% (high variance, measurements may be unstable)
- **bitpack-footprint-dense-alt**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bitpack-footprint-packed**: CV=60.8% (high variance, measurements may be unstable)
- **bitpack-footprint-packed**: worst_20/best_20 = 3.8x (possible bimodal distribution)
