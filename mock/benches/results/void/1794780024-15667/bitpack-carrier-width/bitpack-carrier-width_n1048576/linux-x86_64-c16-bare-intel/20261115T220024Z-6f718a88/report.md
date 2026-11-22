# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d32 beats baseline by 20% (significant)

bitpack-carrier-d32 is -32.25 us (20%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-carrier-d32, bitpack-carrier-packed-simd) are a dead heat (<1%)

bitpack-carrier-d32 (131.44 us) and bitpack-carrier-packed-simd (132.33 us) differ by 0.68%, inside the noise, even though the wider field spreads 47.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-carrier-d16 shows alternating (throttle bounce) (autocorr -0.75)

bitpack-carrier-d16's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-carrier-d32 vs stability leader bitpack-carrier-packed-simd (+1% speed for 45.4x steadier)

bitpack-carrier-d32 is fastest (131.44 us, CV 3.8%); bitpack-carrier-packed-simd gives up 0.7% median for 45.4x lower variance (CV 0.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bitpack-carrier-d64 is inconsistent: worst-20% is 1.9x its best-20%

bitpack-carrier-d64's best 20% of batches run at 153.03 us but its worst 20% at 288.92 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-carrier-d32** at 131437.0 ns median (-17.6% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.47x (fastest 131437.0 ns, slowest 193202.5 ns)

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
| bitpack-carrier-d16 | 161430ns | 159737ns | 159616ns | 161120ns | 164172ns | base |
| bitpack-carrier-d16-control | 163708ns | 163836ns | 162629ns | 163832ns | 164412ns | +1.41% |
| bitpack-carrier-d32 | 128325ns | 131601ns | 120210ns | 129835ns | 131912ns | -20.51% |
| bitpack-carrier-d64 | 204554ns | 193592ns | 153235ns | 193404ns | 289325ns | +26.71% |
| bitpack-carrier-packed | 132880ns | 132820ns | 132673ns | 132854ns | 133162ns | -17.69% |
| bitpack-carrier-packed-simd | 132502ns | 132486ns | 132356ns | 132490ns | 132682ns | -17.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 161262ns | 159452ns | 163996ns | base | 6.502 |
| bitpack-carrier-d16-control | 163481ns | 162467ns | 164112ns | +1.38% | 6.414 |
| bitpack-carrier-d32 | 128129ns | 120033ns | 131636ns | -20.55% | 8.184 |
| bitpack-carrier-d64 | 204184ns | 153025ns | 288920ns | +26.62% | 5.135 |
| bitpack-carrier-packed | 132644ns | 132444ns | 132800ns | -17.75% | 7.905 |
| bitpack-carrier-packed-simd | 132339ns | 132200ns | 132517ns | -17.94% | 7.923 |

## Performance model

- Peak throughput: **8.736 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.572 | 75.2% |
| bitpack-carrier-d16-control | 6.409 | 73.4% |
| bitpack-carrier-d32 | 7.978 | 91.3% |
| bitpack-carrier-d64 | 5.427 | 62.1% |
| bitpack-carrier-packed | 7.904 | 90.5% |
| bitpack-carrier-packed-simd | 7.924 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 161430ns | 161430ns | base |
| bitpack-carrier-d16-control | 163708ns | 163708ns | +1.41% |
| bitpack-carrier-d32 | 128325ns | 128325ns | -20.51% |
| bitpack-carrier-d64 | 204554ns | 204554ns | +26.71% |
| bitpack-carrier-packed | 132880ns | 132880ns | -17.69% |
| bitpack-carrier-packed-simd | 132502ns | 132502ns | -17.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 159545ns | base | --- | [159504, 163674] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 163612ns | +3878.9ns (+2.4%) | [+73, +3996]ns | [163458, 163713] | YES | 0.0022 | 0.0022 | 0 |
| bitpack-carrier-d32 | 131437ns | -32253.5ns (-20.2%) | [-32892, -30196]ns | [131406, 131468] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 193202ns | +29337.9ns (+18.4%) | [+26916, +37280]ns | [190639, 197040] | YES | 0.0022 | 0.0022 | 0 |
| bitpack-carrier-packed | 132667ns | -27025.4ns (-16.9%) | [-30975, -26986]ns | [132659, 132680] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 132329ns | -27247.9ns (-17.1%) | [-31236, -27203]ns | [132290, 132359] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 159533ns | +2.7% | -17.6% | +23.3% | -16.8% | -17.1% |
| 1 | 2 | 163729ns | -0.0% | -19.7% | +17.2% | -19.0% | -19.2% |
| 1 | 3 | 159454ns | +2.5% | -24.5% | -3.8% | -16.9% | -17.0% |
| 1 | 4 | 163731ns | -0.2% | -19.7% | +49.2% | -19.0% | -19.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.754 | HIGH- (thermal bounce) |
| bitpack-carrier-d16-control | 0.288 | moderate+ |
| bitpack-carrier-d32 | -0.417 | moderate- |
| bitpack-carrier-d64 | -0.447 | moderate- |
| bitpack-carrier-packed | -0.241 | moderate- |
| bitpack-carrier-packed-simd | -0.038 | ok |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 1/4, lost 2/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 1/4, lost 3/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 37.1ns | 161261.6ns | 0.0% |  |
| bitpack-carrier-d16-control | 40.2ns | 163481.3ns | 0.0% |  |
| bitpack-carrier-d32 | 39.7ns | 128128.7ns | 0.0% |  |
| bitpack-carrier-d64 | 101.7ns | 204184.4ns | 0.0% |  |
| bitpack-carrier-packed | 38.2ns | 132643.9ns | 0.0% |  |
| bitpack-carrier-packed-simd | 41.4ns | 132339.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 159452.1-163996.0 ns)
  159452.1 |########################################
  159679.3 |##
  159906.5 |
  160133.7 |##
  160360.9 |
  160588.0 |
  160815.2 |
  161042.4 |
  161269.6 |
  161496.8 |
  161724.0 |
  161951.2 |
  162178.4 |
  162405.6 |
  162632.8 |
  162860.0 |
  163087.2 |
  163314.4 |
  163541.6 |##################
  163768.8 |##########
  (3 below, 2 above range)

bitpack-carrier-d16-control (n=40, range 162466.8-164111.6 ns)
  162466.8 |
  162549.0 |
  162631.3 |
  162713.5 |
  162795.8 |
  162878.0 |
  162960.2 |
  163042.5 |
  163124.7 |
  163207.0 |
  163289.2 |
  163371.4 |########################################
  163453.7 |#############
  163535.9 |######
  163618.2 |################
  163700.4 |##########
  163782.6 |######
  163864.9 |################
  163947.1 |
  164029.4 |###
  (2 below, 4 above range)

bitpack-carrier-d32 (n=40, range 120032.7-131636.4 ns)
  120032.7 |####
  120612.9 |###
  121193.1 |
  121773.2 |
  122353.4 |
  122933.6 |#
  123513.8 |
  124094.0 |
  124674.2 |
  125254.4 |
  125834.5 |
  126414.7 |
  126994.9 |
  127575.1 |
  128155.3 |#
  128735.5 |
  129315.7 |
  129895.8 |
  130476.0 |
  131056.2 |########################################
  (6 below, 1 above range)

bitpack-carrier-d64 (n=40, range 153025.3-288920.4 ns)
  153025.3 |##########################
  159820.0 |
  166614.8 |####
  173409.5 |####
  180204.3 |####
  186999.1 |###################################
  193793.8 |########################################
  200588.6 |
  207383.3 |
  214178.1 |
  220972.8 |
  227767.6 |
  234562.3 |
  241357.1 |##########################
  248151.8 |
  254946.6 |
  261741.3 |
  268536.1 |
  275330.8 |
  282125.6 |
  (4 below, 4 above range)

bitpack-carrier-packed (n=40, range 132443.6-132800.4 ns)
  132443.6 |##################
  132461.4 |###
  132479.3 |
  132497.1 |
  132514.9 |###
  132532.8 |
  132550.6 |
  132568.5 |
  132586.3 |
  132604.1 |
  132622.0 |###
  132639.8 |#######
  132657.6 |########################################
  132675.5 |##################
  132693.3 |###
  132711.2 |##############
  132729.0 |
  132746.8 |#######
  132764.7 |###
  132782.5 |
  (3 below, 3 above range)

bitpack-carrier-packed-simd (n=40, range 132199.9-132517.3 ns)
  132199.9 |##########
  132215.7 |##########
  132231.6 |##############################
  132247.5 |####################
  132263.3 |##########
  132279.2 |########################################
  132295.1 |##############################
  132311.0 |####################
  132326.8 |########################################
  132342.7 |####################
  132358.6 |####################
  132374.4 |####################
  132390.3 |##########
  132406.2 |##########
  132422.1 |
  132437.9 |####################
  132453.8 |##########
  132469.7 |
  132485.5 |##########
  132501.4 |
  (3 below, 4 above range)

```

## Diagnostics

- **bitpack-carrier-d64**: CV=25.4% (high variance, measurements may be unstable)
