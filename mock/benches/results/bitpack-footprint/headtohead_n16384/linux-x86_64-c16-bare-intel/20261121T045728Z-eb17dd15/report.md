# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 25% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (2.45 us) leads bitpack-footprint-dense-alt (3.05 us) by 25%, a clear separation rather than a photo finish. CV 102.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 4.7x slower than the field

bitpack-footprint-packed-naive (11.60 us) is 4.7x the fastest (2.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed is fastest but the noisiest (CV 102.9%)

bitpack-footprint-packed wins on median (2.45 us) yet has the highest variance (CV 102.9%), while bitpack-footprint-packed-naive is the steadiest (CV 9.9%, 11.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (280% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 280% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest bitpack-footprint-packed (2.45 us) to slowest bitpack-footprint-packed-naive (11.60 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-packed is inconsistent: worst-20% is 4.1x its best-20%

bitpack-footprint-packed's best 20% of batches run at 2.00 us but its worst 20% at 8.27 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-footprint-packed** at 2445.2 ns median (-19.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.75x (fastest 2445.2 ns, slowest 11603.0 ns)

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
| bitpack-footprint-dense | 4346ns | 3114ns | 2627ns | 3233ns | 9407ns | base |
| bitpack-footprint-dense-alt | 4357ns | 3116ns | 2690ns | 3364ns | 9003ns | +0.24% |
| bitpack-footprint-packed | 4001ns | 2508ns | 2051ns | 3136ns | 8544ns | -7.96% |
| bitpack-footprint-packed-naive | 11591ns | 11687ns | 10419ns | 11592ns | 12761ns | +166.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 4248ns | 2574ns | 9166ns | base | 3.857 |
| bitpack-footprint-dense-alt | 4258ns | 2635ns | 8769ns | +0.25% | 3.848 |
| bitpack-footprint-packed | 3888ns | 2000ns | 8270ns | -8.46% | 4.214 |
| bitpack-footprint-packed-naive | 11527ns | 10367ns | 12683ns | +171.38% | 1.421 |

## Performance model

- Peak throughput: **8.190 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.369 | 65.6% |
| bitpack-footprint-dense-alt | 5.370 | 65.6% |
| bitpack-footprint-packed | 6.700 | 81.8% |
| bitpack-footprint-packed-naive | 1.412 | 17.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 4346ns | 4346ns | base |
| bitpack-footprint-dense-alt | 4357ns | 4357ns | +0.24% |
| bitpack-footprint-packed | 4001ns | 4001ns | -7.96% |
| bitpack-footprint-packed-naive | 11591ns | 11591ns | +166.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 3051ns | base | --- | [3051, 3064] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 3051ns | no significant difference | [-12, +0]ns | [3051, 3063] | no | 0.1433 | 0.1433 | 2 |
| bitpack-footprint-packed | 2445ns | -606.5ns (-19.9%) | [-799, -605]ns | [2444, 2458] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 11603ns | +7316.2ns (+239.8%) | [+7314, +9583]ns | [10370, 12672] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 3058ns | -0.2% | -34.4% | +239.1% |
| 1 | 2 | 3051ns | +0.4% | -19.9% | +239.8% |
| 1 | 3 | 3058ns | -0.2% | -19.6% | +314.4% |
| 1 | 4 | 2507ns | -0.4% | -19.8% | +405.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.090 | ok |
| bitpack-footprint-dense-alt | -0.072 | ok |
| bitpack-footprint-packed | -0.250 | moderate- |
| bitpack-footprint-packed-naive | 0.250 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 1/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 33.6ns | 4247.5ns | 0.8% |  |
| bitpack-footprint-dense-alt | 32.7ns | 4258.0ns | 0.8% |  |
| bitpack-footprint-packed | 37.1ns | 3888.2ns | 1.0% |  |
| bitpack-footprint-packed-naive | 21.4ns | 11527.0ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 2574.0-9165.9 ns)
   2574.0 |
   2903.6 |########################################
   3233.2 |#
   3562.8 |
   3892.4 |
   4222.0 |
   4551.6 |
   4881.2 |
   5210.8 |#
   5540.4 |
   5870.0 |#
   6199.6 |
   6529.2 |
   6858.8 |
   7188.4 |
   7518.0 |
   7847.6 |#
   8177.2 |
   8506.7 |#
   8836.3 |
  (7 below, 5 above range)

bitpack-footprint-dense-alt (n=40, range 2635.0-8768.8 ns)
   2635.0 |
   2941.7 |########################################
   3248.4 |
   3555.1 |
   3861.8 |
   4168.4 |
   4475.1 |
   4781.8 |
   5088.5 |
   5395.2 |#
   5701.9 |
   6008.6 |
   6315.3 |#
   6622.0 |#
   6928.7 |
   7235.4 |#
   7542.1 |
   7848.8 |#
   8155.5 |
   8462.1 |#
  (6 below, 4 above range)

bitpack-footprint-packed (n=40, range 2000.5-8269.7 ns)
   2000.5 |###############################
   2313.9 |########################################
   2627.4 |
   2940.8 |
   3254.3 |
   3567.8 |
   3881.2 |
   4194.7 |
   4508.1 |##
   4821.6 |
   5135.1 |
   5448.5 |#####
   5762.0 |##
   6075.4 |
   6388.9 |
   6702.4 |
   7015.8 |
   7329.3 |##
   7642.7 |########
   7956.2 |##
  (2 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 10366.8-12682.6 ns)
  10366.8 |###################################
  10482.6 |##
  10598.4 |
  10714.2 |
  10829.9 |
  10945.7 |
  11061.5 |
  11177.3 |
  11293.1 |
  11408.9 |
  11524.7 |
  11640.5 |
  11756.3 |
  11872.1 |
  11987.9 |
  12103.7 |
  12219.5 |
  12335.2 |
  12451.0 |
  12566.8 |########################################
  (3 below, 2 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: CV=61.0% (high variance, measurements may be unstable)
- **bitpack-footprint-dense**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bitpack-footprint-dense-alt**: CV=57.1% (high variance, measurements may be unstable)
- **bitpack-footprint-dense-alt**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-footprint-packed**: CV=64.7% (high variance, measurements may be unstable)
- **bitpack-footprint-packed**: worst_20/best_20 = 4.1x (possible bimodal distribution)
