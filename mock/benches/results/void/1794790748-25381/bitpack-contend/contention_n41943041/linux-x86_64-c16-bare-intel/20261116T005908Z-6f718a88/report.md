# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed beats baseline by 23% (significant)

bitpack-contend-packed is -158.16 us (23%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 2.6x slower than the field

bitpack-contend-d64 (1.34 ms) is 2.6x the fastest (515.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d16 shows alternating (throttle bounce) (autocorr -0.62)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} vs {bitpack-contend-d64} (92% apart)

The field splits into a fast tier {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 92% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-packed** at 515769.3 ns median (-25.9% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.60x (fastest 515769.3 ns, slowest 1340203.8 ns)

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
| Counter frequency | 2112749291 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 690381ns | 696539ns | 639355ns | 694374ns | 729429ns | base |
| bitpack-contend-d16-control | 687953ns | 674265ns | 641426ns | 688167ns | 733839ns | -0.35% |
| bitpack-contend-d32 | 731693ns | 699111ns | 697919ns | 716414ns | 811302ns | +5.98% |
| bitpack-contend-d64 | 1367663ns | 1341783ns | 1333984ns | 1353935ns | 1442523ns | +98.10% |
| bitpack-contend-packed | 524874ns | 515944ns | 513844ns | 521745ns | 545292ns | -23.97% |
| bitpack-contend-packed-simd | 531761ns | 527538ns | 515737ns | 526494ns | 563585ns | -22.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 689983ns | 638987ns | 729026ns | base | 6.079 |
| bitpack-contend-d16-control | 687665ns | 641232ns | 733518ns | -0.34% | 6.099 |
| bitpack-contend-d32 | 730846ns | 696958ns | 810578ns | +5.92% | 5.739 |
| bitpack-contend-d64 | 1366161ns | 1332187ns | 1441245ns | +98.00% | 3.070 |
| bitpack-contend-packed | 524668ns | 513627ns | 545088ns | -23.96% | 7.994 |
| bitpack-contend-packed-simd | 531530ns | 515570ns | 563258ns | -22.96% | 7.891 |

## Performance model

- Peak throughput: **8.166 Gops/s** (bitpack-contend-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.026 | 73.8% |
| bitpack-contend-d16-control | 6.224 | 76.2% |
| bitpack-contend-d32 | 6.007 | 73.6% |
| bitpack-contend-d64 | 3.130 | 38.3% |
| bitpack-contend-packed | 8.132 | 99.6% |
| bitpack-contend-packed-simd | 7.954 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 690381ns | 690381ns | base |
| bitpack-contend-d16-control | 687953ns | 687953ns | -0.35% |
| bitpack-contend-d32 | 731693ns | 731693ns | +5.98% |
| bitpack-contend-d64 | 1367663ns | 1367663ns | +98.10% |
| bitpack-contend-packed | 524874ns | 524874ns | -23.97% |
| bitpack-contend-packed-simd | 531761ns | 531761ns | -22.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 696081ns | base | --- | [668189, 723506] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 673889ns | +3032.1ns (+0.4%) | [+2007, +10408]ns | [670392, 723058] | YES | 0.0064 | 0.0064 | 0 |
| bitpack-contend-d32 | 698277ns | +45593.7ns (+6.6%) | [+30060, +59116]ns | [697973, 726202] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-d64 | 1340204ns | +694643.1ns (+99.8%) | [+664520, +699788]ns | [1337499, 1341243] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 515769ns | -158159.1ns (-22.7%) | [-190133, -140954]ns | [514963, 530251] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 527344ns | -146089.6ns (-21.0%) | [-161173, -133420]ns | [515710, 534662] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 723282ns | +1.4% | +11.3% | +98.0% | -26.2% | -22.2% |
| 1 | 2 | 668269ns | +0.4% | +4.5% | +99.4% | -22.1% | -20.1% |
| 1 | 3 | 727330ns | -11.8% | -4.1% | +84.3% | -29.4% | -29.1% |
| 1 | 4 | 639065ns | +13.1% | +9.2% | +109.4% | -19.3% | -19.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.616 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | -0.240 | moderate- |
| bitpack-contend-d32 | -0.080 | ok |
| bitpack-contend-d64 | -0.145 | ok |
| bitpack-contend-packed | 0.154 | ok |
| bitpack-contend-packed-simd | 0.202 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 1/4, lost 3/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 33.4ns | 689982.8ns | 0.0% |  |
| bitpack-contend-d16-control | 64.1ns | 687664.9ns | 0.0% |  |
| bitpack-contend-d32 | 72.9ns | 730845.7ns | 0.0% |  |
| bitpack-contend-d64 | 253.5ns | 1366160.5ns | 0.0% |  |
| bitpack-contend-packed | 37.0ns | 524667.9ns | 0.0% |  |
| bitpack-contend-packed-simd | 36.1ns | 531530.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 638986.6-729026.2 ns)
  638986.6 |####################
  643488.5 |####
  647990.5 |
  652492.5 |
  656994.5 |
  661496.5 |
  665998.4 |########################################
  670500.4 |
  675002.4 |
  679504.4 |
  684006.4 |
  688508.4 |
  693010.3 |
  697512.3 |
  702014.3 |
  706516.3 |
  711018.3 |
  715520.2 |
  720022.2 |################################
  724524.2 |####################################
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 641232.0-733517.5 ns)
  641232.0 |############################
  645846.3 |
  650460.5 |
  655074.8 |
  659689.1 |
  664303.4 |
  668917.7 |########################################
  673531.9 |########
  678146.2 |####
  682760.5 |
  687374.8 |####
  691989.0 |
  696603.3 |
  701217.6 |
  705831.9 |
  710446.2 |####
  715060.4 |
  719674.7 |########
  724289.0 |################
  728903.3 |################
  (3 below, 5 above range)

bitpack-contend-d32 (n=40, range 696958.4-810577.6 ns)
  696958.4 |########################################
  702639.4 |
  708320.3 |#
  714001.3 |
  719682.2 |
  725363.2 |
  731044.2 |
  736725.1 |
  742406.1 |#
  748087.1 |#
  753768.0 |
  759449.0 |###
  765129.9 |
  770810.9 |#
  776491.9 |
  782172.8 |
  787853.8 |
  793534.8 |#
  799215.7 |###
  804896.7 |#####
  (4 below, 3 above range)

bitpack-contend-d64 (n=40, range 1332187.0-1441245.2 ns)
  1332187.0 |############################
  1337639.9 |########################################
  1343092.8 |
  1348545.8 |
  1353998.7 |
  1359451.6 |
  1364904.5 |
  1370357.4 |
  1375810.3 |
  1381263.2 |
  1386716.1 |
  1392169.0 |
  1397621.9 |##
  1403074.9 |
  1408527.8 |
  1413980.7 |
  1419433.6 |##
  1424886.5 |##
  1430339.4 |##############
  1435792.3 |########
  (4 below, 1 above range)

bitpack-contend-packed (n=40, range 513626.5-545088.5 ns)
  513626.5 |########################################
  515199.6 |############
  516772.7 |
  518345.8 |###
  519918.9 |
  521492.0 |
  523065.1 |
  524638.2 |###
  526211.3 |
  527784.4 |
  529357.5 |#####################
  530930.6 |######
  532503.7 |############
  534076.8 |
  535649.9 |
  537223.0 |
  538796.1 |
  540369.2 |
  541942.3 |
  543515.4 |
  (4 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 515569.7-563258.3 ns)
  515569.7 |########################################
  517954.1 |
  520338.5 |##
  522723.0 |
  525107.4 |
  527491.8 |
  529876.3 |
  532260.7 |################
  534645.1 |#####
  537029.5 |#####
  539414.0 |##
  541798.4 |##
  544182.8 |
  546567.3 |
  548951.7 |
  551336.1 |
  553720.5 |
  556105.0 |
  558489.4 |##
  560873.8 |########
  (4 below, 4 above range)

```
