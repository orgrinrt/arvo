# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 is an outlier: 3.2x slower than the field

bitpack-contend-d64 (1.47 ms) is 3.2x the fastest (461.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-contend-packed, bitpack-contend-packed-simd) are a dead heat (<1%)

bitpack-contend-packed (461.53 us) and bitpack-contend-packed-simd (464.00 us) differ by 0.53%, inside the noise, even though the wider field spreads 219.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d32 shows alternating (throttle bounce) (autocorr -0.61)

bitpack-contend-d32's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest bitpack-contend-packed (461.53 us) to slowest bitpack-contend-d64 (1.47 ms): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-contend-packed** at 461532.6 ns median (-13.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.19x (fastest 461532.6 ns, slowest 1474237.4 ns)

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
| bitpack-contend-d16 | 559032ns | 532787ns | 516300ns | 554332ns | 615863ns | base |
| bitpack-contend-d16-control | 577721ns | 579225ns | 535622ns | 576670ns | 622973ns | +3.34% |
| bitpack-contend-d32 | 839913ns | 839272ns | 821793ns | 841450ns | 853424ns | +50.24% |
| bitpack-contend-d64 | 1494263ns | 1476161ns | 1461442ns | 1485416ns | 1553624ns | +167.29% |
| bitpack-contend-packed | 464047ns | 461947ns | 444948ns | 463171ns | 485773ns | -16.99% |
| bitpack-contend-packed-simd | 467002ns | 464503ns | 457604ns | 465029ns | 482318ns | -16.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 558321ns | 515640ns | 615133ns | base | 15.025 |
| bitpack-contend-d16-control | 577060ns | 535066ns | 622115ns | +3.36% | 14.537 |
| bitpack-contend-d32 | 838491ns | 820328ns | 852124ns | +50.18% | 10.004 |
| bitpack-contend-d64 | 1492490ns | 1459802ns | 1551724ns | +167.32% | 5.621 |
| bitpack-contend-packed | 463555ns | 444403ns | 485182ns | -16.97% | 18.096 |
| bitpack-contend-packed-simd | 466466ns | 457090ns | 481677ns | -16.45% | 17.983 |

## Performance model

- Peak throughput: **18.876 Gops/s** (bitpack-contend-packed; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 15.764 | 83.5% |
| bitpack-contend-d16-control | 14.499 | 76.8% |
| bitpack-contend-d32 | 10.013 | 53.0% |
| bitpack-contend-d64 | 5.690 | 30.1% |
| bitpack-contend-packed | 18.176 | 96.3% |
| bitpack-contend-packed-simd | 18.079 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 559032ns | 559032ns | base |
| bitpack-contend-d16-control | 577721ns | 577721ns | +3.34% |
| bitpack-contend-d32 | 839913ns | 839913ns | +50.24% |
| bitpack-contend-d64 | 1494263ns | 1494263ns | +167.29% |
| bitpack-contend-packed | 464047ns | 464047ns | -16.99% |
| bitpack-contend-packed-simd | 467002ns | 467002ns | -16.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 532131ns | base | --- | [530409, 593583] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 578556ns | +15624.6ns (+2.9%) | [+3407, +67732]ns | [544092, 598875] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-contend-d32 | 837738ns | +293245.0ns (+55.1%) | [+253053, +298622]ns | [833144, 846938] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 1474237ns | +943326.8ns (+177.3%) | [+938317, +945975]ns | [1469805, 1492428] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 461533ns | -71562.9ns (-13.4%) | [-124967, -68573]ns | [460230, 466499] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 464000ns | -74789.9ns (-14.1%) | [-122139, -71254]ns | [463676, 465601] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 598911ns | -9.5% | +41.9% | +158.4% | -22.0% | -22.1% |
| 1 | 2 | 529256ns | +15.0% | +57.3% | +177.4% | -12.9% | -12.3% |
| 1 | 3 | 532595ns | +12.3% | +58.9% | +174.5% | -16.5% | -12.9% |
| 1 | 4 | 530866ns | +1.0% | +56.1% | +177.7% | -12.8% | -14.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.117 | ok |
| bitpack-contend-d16-control | -0.245 | moderate- |
| bitpack-contend-d32 | -0.609 | HIGH- (thermal bounce) |
| bitpack-contend-d64 | -0.063 | ok |
| bitpack-contend-packed | -0.232 | moderate- |
| bitpack-contend-packed-simd | -0.001 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 77.5ns | 558320.9ns | 0.0% |  |
| bitpack-contend-d16-control | 82.3ns | 577059.9ns | 0.0% |  |
| bitpack-contend-d32 | 216.1ns | 838491.4ns | 0.0% |  |
| bitpack-contend-d64 | 253.3ns | 1492489.7ns | 0.0% |  |
| bitpack-contend-packed | 85.0ns | 463555.5ns | 0.0% |  |
| bitpack-contend-packed-simd | 61.2ns | 466466.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 515640.1-615132.9 ns)
  515640.1 |
  520614.7 |
  525589.4 |########################################
  530564.0 |########################
  535538.6 |####
  540513.3 |####
  545487.9 |
  550462.6 |
  555437.2 |####
  560411.9 |
  565386.5 |
  570361.1 |
  575335.8 |
  580310.4 |####
  585285.1 |
  590259.7 |########
  595234.4 |############################
  600209.0 |########
  605183.7 |
  610158.3 |
  (5 below, 4 above range)

bitpack-contend-d16-control (n=40, range 535066.0-622115.2 ns)
  535066.0 |########################################
  539418.5 |########################################
  543770.9 |#############
  548123.4 |######
  552475.8 |
  556828.3 |
  561180.7 |
  565533.2 |
  569885.7 |
  574238.1 |####################
  578590.6 |######
  582943.0 |
  587295.5 |
  591648.0 |######
  596000.4 |########################################
  600352.9 |
  604705.3 |########################################
  609057.8 |####################
  613410.2 |
  617762.7 |
  (2 below, 3 above range)

bitpack-contend-d32 (n=40, range 820327.7-852124.1 ns)
  820327.7 |
  821917.5 |
  823507.3 |##########
  825097.2 |##########
  826687.0 |##########
  828276.8 |########################################
  829866.6 |##############################
  831456.4 |##############################
  833046.3 |####################
  834636.1 |
  836225.9 |########################################
  837815.7 |##########
  839405.5 |
  840995.3 |
  842585.2 |
  844175.0 |####################
  845764.8 |########################################
  847354.6 |########################################
  848944.4 |########################################
  850534.2 |##############################
  (1 below, 2 above range)

bitpack-contend-d64 (n=40, range 1459801.7-1551724.1 ns)
  1459801.7 |########################################
  1464397.8 |####################
  1468994.0 |########################################
  1473590.1 |##########################
  1478186.2 |#############
  1482782.3 |######
  1487378.4 |######
  1491974.6 |######
  1496570.7 |######
  1501166.8 |#############
  1505762.9 |
  1510359.0 |
  1514955.2 |
  1519551.3 |
  1524147.4 |
  1528743.5 |
  1533339.6 |
  1537935.8 |####################
  1542531.9 |#############
  1547128.0 |######
  (3 below, 4 above range)

bitpack-contend-packed (n=40, range 444402.7-485182.3 ns)
  444402.7 |######################
  446441.7 |
  448480.7 |
  450519.6 |
  452558.6 |
  454597.6 |#####
  456636.6 |
  458675.6 |########################################
  460714.5 |##################################
  462753.5 |#################
  464792.5 |###########
  466831.5 |##################################
  468870.5 |
  470909.4 |#####
  472948.4 |
  474987.4 |
  477026.4 |
  479065.4 |#####
  481104.3 |
  483143.3 |
  (4 below, 5 above range)

bitpack-contend-packed-simd (n=40, range 457089.7-481677.2 ns)
  457089.7 |
  458319.0 |
  459548.4 |
  460777.8 |####
  462007.2 |################
  463236.5 |########################################
  464465.9 |################
  465695.3 |########################
  466924.7 |####
  468154.0 |
  469383.4 |####
  470612.8 |
  471842.2 |########
  473071.5 |
  474300.9 |
  475530.3 |
  476759.7 |
  477989.0 |
  479218.4 |####
  480447.8 |
  (7 below, 3 above range)

```
