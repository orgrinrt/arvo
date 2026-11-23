# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed dominates: 16% faster than the next best (bitpack-carrier-packed-simd)

bitpack-carrier-packed (511.25 us) leads bitpack-carrier-packed-simd (592.39 us) by 16%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-packed beats baseline by 25% (significant)

bitpack-carrier-packed is -170.79 us (25%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d64 is an outlier: 2.7x slower than the field

bitpack-carrier-d64 (1.38 ms) is 2.7x the fastest (511.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-carrier-packed-simd shows alternating (throttle bounce) (autocorr -0.67)

bitpack-carrier-packed-simd's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-packed, bitpack-carrier-packed-simd, bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d32} vs {bitpack-carrier-d64} (97% apart)

The field splits into a fast tier {bitpack-carrier-packed, bitpack-carrier-packed-simd, bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d32} and a slow tier {bitpack-carrier-d64} with a 97% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-packed** at 511246.5 ns median (-26.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.69x (fastest 511246.5 ns, slowest 1375470.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112762950 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 702286ns | 693751ns | 638491ns | 690044ns | 802806ns | base |
| bitpack-carrier-d16-control | 657687ns | 656601ns | 641509ns | 656498ns | 677430ns | -6.35% |
| bitpack-carrier-d32 | 724169ns | 700658ns | 695171ns | 713709ns | 784546ns | +3.12% |
| bitpack-carrier-d64 | 1387641ns | 1376999ns | 1318403ns | 1387594ns | 1457022ns | +97.59% |
| bitpack-carrier-packed | 514559ns | 511405ns | 510957ns | 511862ns | 526249ns | -26.73% |
| bitpack-carrier-packed-simd | 576921ns | 592633ns | 511015ns | 584611ns | 619755ns | -17.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 702060ns | 638328ns | 802577ns | base | 5.974 |
| bitpack-carrier-d16-control | 657445ns | 641220ns | 677207ns | -6.35% | 6.380 |
| bitpack-carrier-d32 | 723323ns | 694239ns | 783833ns | +3.03% | 5.799 |
| bitpack-carrier-d64 | 1386220ns | 1316807ns | 1455746ns | +97.45% | 3.026 |
| bitpack-carrier-packed | 514396ns | 510810ns | 526069ns | -26.73% | 8.154 |
| bitpack-carrier-packed-simd | 576664ns | 510850ns | 619435ns | -17.86% | 7.273 |

## Performance model

- Peak throughput: **8.211 Gops/s** (bitpack-carrier-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.048 | 73.7% |
| bitpack-carrier-d16-control | 6.390 | 77.8% |
| bitpack-carrier-d32 | 5.994 | 73.0% |
| bitpack-carrier-d64 | 3.049 | 37.1% |
| bitpack-carrier-packed | 8.204 | 99.9% |
| bitpack-carrier-packed-simd | 7.080 | 86.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 702286ns | 702286ns | base |
| bitpack-carrier-d16-control | 657687ns | 657687ns | -6.35% |
| bitpack-carrier-d32 | 724169ns | 724169ns | +3.12% |
| bitpack-carrier-d64 | 1387641ns | 1387641ns | +97.59% |
| bitpack-carrier-packed | 514559ns | 514559ns | -26.73% |
| bitpack-carrier-packed-simd | 576921ns | 576921ns | -17.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 693472ns | base | --- | [642053, 719484] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 656336ns | no significant difference | [-77883, +4811]ns | [646606, 663794] | no | 0.8746 | 0.8746 | 0 |
| bitpack-carrier-d32 | 699794ns | no significant difference | [-17209, +66723]ns | [697308, 720801] | no | 0.3352 | 0.2682 | 0 |
| bitpack-carrier-d64 | 1375470ns | +693900.4ns (+100.1%) | [+630101, +709008]ns | [1337318, 1431260] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 511246ns | -170790.5ns (-24.6%) | [-207259, -130982]ns | [510936, 512116] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 592387ns | -126798.4ns (-18.3%) | [-128032, -113209]ns | [572206, 603999] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 642539ns | +0.6% | +8.4% | +108.2% | -20.5% | -6.0% |
| 1 | 2 | 638631ns | +3.9% | +20.2% | +126.6% | -19.9% | -20.0% |
| 1 | 3 | 718892ns | -10.8% | -2.6% | +87.2% | -28.8% | -15.9% |
| 1 | 4 | 786233ns | -14.2% | -11.6% | +72.2% | -34.9% | -27.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.259 | moderate+ |
| bitpack-carrier-d16-control | -0.638 | HIGH- (thermal bounce) |
| bitpack-carrier-d32 | -0.384 | moderate- |
| bitpack-carrier-d64 | -0.515 | HIGH- (thermal bounce) |
| bitpack-carrier-packed | -0.229 | moderate- |
| bitpack-carrier-packed-simd | -0.669 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 2/4, lost 2/4
- **bitpack-carrier-d32**: won 2/4, lost 2/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 45.2ns | 702060.0ns | 0.0% |  |
| bitpack-carrier-d16-control | 76.1ns | 657444.7ns | 0.0% |  |
| bitpack-carrier-d32 | 271.9ns | 723323.3ns | 0.0% |  |
| bitpack-carrier-d64 | 393.6ns | 1386220.0ns | 0.0% |  |
| bitpack-carrier-packed | 39.3ns | 514395.6ns | 0.0% |  |
| bitpack-carrier-packed-simd | 76.4ns | 576663.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 638327.7-802577.4 ns)
  638327.7 |########################################
  646540.2 |##
  654752.7 |
  662965.2 |
  671177.7 |##
  679390.1 |
  687602.6 |
  695815.1 |
  704027.6 |##
  712240.1 |#################
  720452.5 |#####
  728665.0 |
  736877.5 |
  745090.0 |
  753302.5 |
  761515.0 |
  769727.4 |
  777939.9 |###########
  786152.4 |########
  794364.9 |#####
  (4 below, 2 above range)

bitpack-carrier-d16-control (n=40, range 641220.1-677207.0 ns)
  641220.1 |###########
  643019.5 |#####
  644818.8 |############################
  646618.1 |######################
  648417.5 |###########
  650216.8 |
  652016.2 |
  653815.5 |
  655614.9 |
  657414.2 |
  659213.5 |
  661012.9 |#####
  662812.2 |########################################
  664611.6 |
  666410.9 |
  668210.3 |###########
  670009.6 |#################
  671808.9 |#####
  673608.3 |###########
  675407.6 |#####
  (6 below, 3 above range)

bitpack-carrier-d32 (n=40, range 694238.7-783833.5 ns)
  694238.7 |########################################
  698718.4 |##############
  703198.2 |
  707677.9 |##
  712157.7 |##
  716637.4 |##
  721117.1 |##
  725596.9 |
  730076.6 |
  734556.3 |
  739036.1 |#####
  743515.8 |
  747995.6 |
  752475.3 |
  756955.0 |#####
  761434.8 |
  765914.5 |###########
  770394.2 |#####
  774874.0 |
  779353.7 |
  (4 below, 3 above range)

bitpack-carrier-d64 (n=40, range 1316807.0-1455745.7 ns)
  1316807.0 |#####
  1323753.9 |#################
  1330700.9 |########################################
  1337647.8 |
  1344594.8 |###########
  1351541.7 |
  1358488.6 |
  1365435.6 |
  1372382.5 |#################
  1379329.4 |
  1386276.4 |
  1393223.3 |
  1400170.2 |
  1407117.2 |
  1414064.1 |#####
  1421011.0 |###########
  1427958.0 |#################
  1434904.9 |###########
  1441851.8 |##################################
  1448798.8 |#####
  (5 below, 4 above range)

bitpack-carrier-packed (n=40, range 510810.4-526069.0 ns)
  510810.4 |########################################
  511573.3 |#####
  512336.3 |###
  513099.2 |###
  513862.1 |
  514625.1 |
  515388.0 |#
  516150.9 |#
  516913.8 |
  517676.8 |#
  518439.7 |
  519202.6 |
  519965.6 |
  520728.5 |
  521491.4 |#
  522254.3 |
  523017.3 |
  523780.2 |
  524543.1 |
  525306.1 |
  (3 below, 5 above range)

bitpack-carrier-packed-simd (n=40, range 510849.9-619434.6 ns)
  510849.9 |##################
  516279.2 |
  521708.4 |
  527137.6 |
  532566.9 |
  537996.1 |
  543425.3 |
  548854.6 |
  554283.8 |
  559713.0 |
  565142.3 |
  570571.5 |#####################
  576000.7 |
  581430.0 |######
  586859.2 |###
  592288.4 |###
  597717.7 |###
  603146.9 |########################################
  608576.1 |######
  614005.4 |
  (4 below, 3 above range)

```
