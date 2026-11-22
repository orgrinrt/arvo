# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed-simd beats baseline by 23% (significant)

bitpack-carrier-packed-simd is -152.24 us (23%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d64 is an outlier: 2.6x slower than the field

bitpack-carrier-d64 (1.33 ms) is 2.6x the fastest (515.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-carrier-packed-simd, bitpack-carrier-packed) are a dead heat (<1%)

bitpack-carrier-packed-simd (515.67 us) and bitpack-carrier-packed (518.78 us) differ by 0.60%, inside the noise, even though the wider field spreads 158.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-carrier-d64 shows alternating (throttle bounce) (autocorr -0.58)

bitpack-carrier-d64's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d32} vs {bitpack-carrier-d64} (90% apart)

The field splits into a fast tier {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d32} and a slow tier {bitpack-carrier-d64} with a 90% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-packed-simd** at 515670.0 ns median (-22.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.58x (fastest 515670.0 ns, slowest 1332956.0 ns)

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
| Counter frequency | 2112721357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 677844ns | 666836ns | 639008ns | 667790ns | 746843ns | base |
| bitpack-carrier-d16-control | 655244ns | 647442ns | 638257ns | 648427ns | 692681ns | -3.33% |
| bitpack-carrier-d32 | 727745ns | 702256ns | 697270ns | 716517ns | 791904ns | +7.36% |
| bitpack-carrier-d64 | 1337300ns | 1334432ns | 1326730ns | 1335326ns | 1353793ns | +97.29% |
| bitpack-carrier-packed | 530088ns | 519086ns | 517007ns | 524848ns | 558889ns | -21.80% |
| bitpack-carrier-packed-simd | 521113ns | 515827ns | 514577ns | 519321ns | 533025ns | -23.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 677589ns | 638821ns | 746443ns | base | 6.190 |
| bitpack-carrier-d16-control | 655010ns | 638082ns | 692336ns | -3.33% | 6.403 |
| bitpack-carrier-d32 | 726857ns | 696105ns | 791548ns | +7.27% | 5.770 |
| bitpack-carrier-d64 | 1335651ns | 1325119ns | 1352008ns | +97.12% | 3.140 |
| bitpack-carrier-packed | 529796ns | 516667ns | 558618ns | -21.81% | 7.917 |
| bitpack-carrier-packed-simd | 520911ns | 514401ns | 532756ns | -23.12% | 8.052 |

## Performance model

- Peak throughput: **8.154 Gops/s** (bitpack-carrier-packed-simd; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.292 | 77.2% |
| bitpack-carrier-d16-control | 6.480 | 79.5% |
| bitpack-carrier-d32 | 5.982 | 73.4% |
| bitpack-carrier-d64 | 3.147 | 38.6% |
| bitpack-carrier-packed | 8.085 | 99.2% |
| bitpack-carrier-packed-simd | 8.134 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 677844ns | 677844ns | base |
| bitpack-carrier-d16-control | 655244ns | 655244ns | -3.33% |
| bitpack-carrier-d32 | 727745ns | 727745ns | +7.36% |
| bitpack-carrier-d64 | 1337300ns | 1337300ns | +97.29% |
| bitpack-carrier-packed | 530088ns | 530088ns | -21.80% |
| bitpack-carrier-packed-simd | 521113ns | 521113ns | -23.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 666646ns | base | --- | [654212, 674684] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 647231ns | no significant difference | [-36212, +7319]ns | [638371, 654840] | no | 0.4296 | 0.4296 | 0 |
| bitpack-carrier-d32 | 701202ns | +58709.7ns (+8.8%) | [+42767, +83542]ns | [700076, 732295] | YES | 0.0028 | 0.0022 | 0 |
| bitpack-carrier-d64 | 1332956ns | +675800.9ns (+101.4%) | [+650700, +690127]ns | [1331478, 1336188] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 518776ns | -123393.0ns (-18.5%) | [-135615, -118320]ns | [518181, 531482] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 515670ns | -152242.1ns (-22.8%) | [-159511, -125261]ns | [515270, 522148] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 742715ns | -14.1% | -5.7% | +79.3% | -30.3% | -28.4% |
| 1 | 2 | 654212ns | +0.1% | +6.6% | +103.8% | -20.7% | -21.2% |
| 1 | 3 | 674486ns | -5.4% | +13.2% | +96.5% | -17.4% | -23.6% |
| 1 | 4 | 639505ns | +6.0% | +13.3% | +109.3% | -17.6% | -19.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.213 | moderate- |
| bitpack-carrier-d16-control | -0.404 | moderate- |
| bitpack-carrier-d32 | -0.140 | ok |
| bitpack-carrier-d64 | -0.579 | HIGH- (thermal bounce) |
| bitpack-carrier-packed | -0.244 | moderate- |
| bitpack-carrier-packed-simd | -0.071 | ok |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 2/4, lost 1/4
- **bitpack-carrier-d32**: won 1/4, lost 3/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 71.3ns | 677589.4ns | 0.0% |  |
| bitpack-carrier-d16-control | 47.3ns | 655010.2ns | 0.0% |  |
| bitpack-carrier-d32 | 215.4ns | 726857.0ns | 0.0% |  |
| bitpack-carrier-d64 | 306.3ns | 1335650.7ns | 0.0% |  |
| bitpack-carrier-packed | 45.6ns | 529796.2ns | 0.0% |  |
| bitpack-carrier-packed-simd | 47.1ns | 520911.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 638820.8-746442.9 ns)
  638820.8 |########################################
  644201.9 |
  649583.0 |####################
  654964.1 |########
  660345.2 |
  665726.3 |
  671107.4 |########################################
  676488.5 |
  681869.6 |
  687250.7 |
  692631.8 |
  698012.9 |
  703394.1 |
  708775.2 |
  714156.3 |
  719537.4 |
  724918.5 |
  730299.6 |
  735680.7 |####
  741061.8 |################################
  (3 below, 1 above range)

bitpack-carrier-d16-control (n=40, range 638081.7-692336.2 ns)
  638081.7 |########################################
  640794.4 |
  643507.1 |
  646219.9 |
  648932.6 |
  651645.3 |##
  654358.0 |######################
  657070.8 |
  659783.5 |
  662496.2 |
  665208.9 |
  667921.7 |
  670634.4 |#######
  673347.1 |##
  676059.8 |#####
  678772.6 |
  681485.3 |
  684198.0 |
  686910.7 |
  689623.5 |
  (4 below, 4 above range)

bitpack-carrier-d32 (n=40, range 696105.0-791548.1 ns)
  696105.0 |########################################
  700877.2 |###########
  705649.3 |##
  710421.5 |
  715193.7 |##
  719965.8 |
  724738.0 |##
  729510.1 |
  734282.3 |##
  739054.4 |
  743826.6 |
  748598.7 |#######
  753370.9 |
  758143.0 |####
  762915.2 |###########
  767687.4 |
  772459.5 |
  777231.7 |
  782003.8 |
  786776.0 |
  (1 below, 3 above range)

bitpack-carrier-d64 (n=40, range 1325118.9-1352008.3 ns)
  1325118.9 |##########################
  1326463.4 |
  1327807.9 |######
  1329152.3 |######
  1330496.8 |#################################
  1331841.3 |########################################
  1333185.8 |##########################
  1334530.2 |######
  1335874.7 |######
  1337219.2 |####################
  1338563.6 |##########################
  1339908.1 |
  1341252.6 |######
  1342597.0 |#############
  1343941.5 |
  1345286.0 |######
  1346630.4 |
  1347974.9 |
  1349319.4 |
  1350663.9 |
  (4 below, 2 above range)

bitpack-carrier-packed (n=40, range 516666.9-558617.8 ns)
  516666.9 |########################################
  518764.4 |#######
  520862.0 |####
  522959.5 |
  525057.1 |
  527154.6 |
  529252.2 |
  531349.7 |###########
  533447.2 |
  535544.8 |
  537642.3 |
  539739.9 |
  541837.4 |
  543935.0 |
  546032.5 |
  548130.1 |
  550227.6 |
  552325.2 |
  554422.7 |#########
  556520.3 |#########
  (3 below, 2 above range)

bitpack-carrier-packed-simd (n=40, range 514400.6-532756.1 ns)
  514400.6 |########################################
  515318.4 |##############################
  516236.2 |
  517153.9 |###
  518071.7 |
  518989.5 |
  519907.3 |
  520825.0 |
  521742.8 |
  522660.6 |
  523578.4 |
  524496.1 |
  525413.9 |
  526331.7 |###
  527249.4 |
  528167.2 |
  529085.0 |###
  530002.8 |##########
  530920.5 |################
  531838.3 |######
  (4 below, 2 above range)

```
