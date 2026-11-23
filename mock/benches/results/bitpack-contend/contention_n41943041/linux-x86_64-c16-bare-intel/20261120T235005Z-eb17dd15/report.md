# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd beats baseline by 26% (significant)

bitpack-contend-packed-simd is -182.10 us (26%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 2.6x slower than the field

bitpack-contend-d64 (1.34 ms) is 2.6x the fastest (516.24 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} vs {bitpack-contend-d64} (73% apart)

The field splits into a fast tier {bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 73% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader bitpack-contend-packed-simd vs stability leader bitpack-contend-packed (+3% speed for 3.2x steadier)

bitpack-contend-packed-simd is fastest (516.24 us, CV 6.5%); bitpack-contend-packed gives up 3.1% median for 3.2x lower variance (CV 2.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-packed-simd** at 516238.3 ns median (-27.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.60x (fastest 516238.3 ns, slowest 1344746.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 713713ns | 709709ns | 640313ns | 709366ns | 800155ns | base |
| bitpack-contend-d16-control | 670818ns | 663235ns | 642026ns | 661804ns | 726649ns | -6.01% |
| bitpack-contend-d32 | 775926ns | 778805ns | 714593ns | 779137ns | 827626ns | +8.72% |
| bitpack-contend-d64 | 1366152ns | 1346108ns | 1323654ns | 1345729ns | 1469922ns | +91.41% |
| bitpack-contend-packed | 533589ns | 532756ns | 516377ns | 534129ns | 549181ns | -25.24% |
| bitpack-contend-packed-simd | 530142ns | 516406ns | 515945ns | 518141ns | 580339ns | -25.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 713342ns | 640121ns | 799863ns | base | 5.880 |
| bitpack-contend-d16-control | 670556ns | 641739ns | 726378ns | -6.00% | 6.255 |
| bitpack-contend-d32 | 775205ns | 713742ns | 826824ns | +8.67% | 5.411 |
| bitpack-contend-d64 | 1364912ns | 1322492ns | 1468595ns | +91.34% | 3.073 |
| bitpack-contend-packed | 533374ns | 516216ns | 548917ns | -25.23% | 7.864 |
| bitpack-contend-packed-simd | 529967ns | 515775ns | 580170ns | -25.71% | 7.914 |

## Performance model

- Peak throughput: **8.132 Gops/s** (bitpack-contend-packed-simd; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.914 | 72.7% |
| bitpack-contend-d16-control | 6.327 | 77.8% |
| bitpack-contend-d32 | 5.389 | 66.3% |
| bitpack-contend-d64 | 3.119 | 38.4% |
| bitpack-contend-packed | 7.877 | 96.9% |
| bitpack-contend-packed-simd | 8.125 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 713713ns | 713713ns | base |
| bitpack-contend-d16-control | 670818ns | 670818ns | -6.01% |
| bitpack-contend-d32 | 775926ns | 775926ns | +8.72% |
| bitpack-contend-d64 | 1366152ns | 1366152ns | +91.41% |
| bitpack-contend-packed | 533589ns | 533589ns | -25.24% |
| bitpack-contend-packed-simd | 530142ns | 530142ns | -25.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 709234ns | base | --- | [698059, 714982] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 662891ns | no significant difference | [-39146, +90]ns | [645406, 676902] | no | 0.0807 | 0.0807 | 0 |
| bitpack-contend-d32 | 778254ns | +49560.9ns (+7.0%) | [+23052, +92132]ns | [767730, 789745] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-d64 | 1344746ns | +637421.8ns (+89.9%) | [+627861, +672864]ns | [1340076, 1350239] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 532494ns | -169046.2ns (-23.8%) | [-183523, -149503]ns | [531474, 534035] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 516238ns | -182099.9ns (-25.7%) | [-187980, -172586]ns | [516067, 516879] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 640640ns | +0.2% | +23.0% | +111.9% | -16.7% | -19.4% |
| 1 | 2 | 698371ns | +4.4% | +8.8% | +92.6% | -21.5% | -26.1% |
| 1 | 3 | 714018ns | -5.3% | +6.8% | +88.3% | -25.6% | -26.1% |
| 1 | 4 | 793367ns | -18.6% | +1.1% | +66.9% | -35.0% | -34.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.093 | ok |
| bitpack-contend-d16-control | -0.344 | moderate- |
| bitpack-contend-d32 | -0.217 | moderate- |
| bitpack-contend-d64 | 0.004 | ok |
| bitpack-contend-packed | 0.048 | ok |
| bitpack-contend-packed-simd | -0.419 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 2/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 41.3ns | 713342.5ns | 0.0% |  |
| bitpack-contend-d16-control | 31.2ns | 670556.5ns | 0.0% |  |
| bitpack-contend-d32 | 92.2ns | 775205.0ns | 0.0% |  |
| bitpack-contend-d64 | 223.4ns | 1364912.1ns | 0.0% |  |
| bitpack-contend-packed | 31.9ns | 533373.8ns | 0.0% |  |
| bitpack-contend-packed-simd | 29.9ns | 529967.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 640121.3-799863.3 ns)
  640121.3 |######################
  648108.4 |
  656095.5 |
  664082.6 |
  672069.7 |
  680056.8 |
  688043.9 |####
  696031.0 |###################################
  704018.1 |####
  712005.2 |########################################
  719992.3 |####
  727979.4 |
  735966.5 |
  743953.6 |
  751940.7 |
  759927.8 |
  767914.9 |
  775902.0 |
  783889.1 |####
  791876.2 |##########################
  (5 below, 3 above range)

bitpack-contend-d16-control (n=40, range 641738.6-726377.8 ns)
  641738.6 |########################################
  645970.6 |##############
  650202.5 |###
  654434.5 |
  658666.4 |
  662898.4 |
  667130.4 |
  671362.3 |##########
  675594.3 |#############################
  679826.3 |#######
  684058.2 |
  688290.2 |
  692522.1 |
  696754.1 |
  700986.1 |
  705218.0 |
  709450.0 |
  713682.0 |
  717913.9 |
  722145.9 |
  (4 below, 7 above range)

bitpack-contend-d32 (n=40, range 713742.0-826823.6 ns)
  713742.0 |
  719396.1 |
  725050.1 |######
  730704.2 |######
  736358.3 |######
  742012.4 |
  747666.5 |#############
  753320.5 |######
  758974.6 |######
  764628.7 |####################
  770282.8 |##########################
  775936.8 |#############
  781590.9 |######
  787245.0 |########################################
  792899.1 |
  798553.2 |##########################
  804207.2 |######
  809861.3 |#############
  815515.4 |#############
  821169.5 |
  (5 below, 3 above range)

bitpack-contend-d64 (n=40, range 1322492.1-1468595.2 ns)
  1322492.1 |#################
  1329797.3 |#############
  1337102.4 |###################################
  1344407.6 |########################################
  1351712.7 |#############
  1359017.9 |####
  1366323.1 |
  1373628.2 |
  1380933.4 |
  1388238.5 |
  1395543.7 |
  1402848.8 |####
  1410154.0 |
  1417459.1 |
  1424764.3 |
  1432069.5 |
  1439374.6 |
  1446679.8 |
  1453984.9 |
  1461290.1 |
  (4 below, 7 above range)

bitpack-contend-packed (n=40, range 516215.8-548917.1 ns)
  516215.8 |
  517850.9 |
  519485.9 |
  521121.0 |
  522756.1 |
  524391.1 |###
  526026.2 |
  527661.2 |
  529296.3 |
  530931.4 |########################################
  532566.4 |#####################
  534201.5 |###
  535836.6 |###
  537471.6 |
  539106.7 |
  540741.7 |
  542376.8 |
  544011.9 |
  545646.9 |
  547282.0 |##################
  (7 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 515775.0-580170.1 ns)
  515775.0 |########################################
  518994.8 |
  522214.5 |#
  525434.3 |
  528654.0 |#####
  531873.8 |#
  535093.5 |#
  538313.3 |
  541533.0 |
  544752.8 |
  547972.5 |
  551192.3 |
  554412.0 |
  557631.8 |
  560851.5 |
  564071.3 |
  567291.0 |
  570510.8 |
  573730.5 |
  576950.3 |
  (2 below, 4 above range)

```
