# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d32 beats baseline by 25% (significant)

bitpack-carrier-d32 is -40.32 us (25%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d32 shows alternating (throttle bounce) (autocorr -0.65)

bitpack-carrier-d32's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-carrier-d32 vs stability leader bitpack-carrier-packed-simd (+10% speed for 6.7x steadier)

bitpack-carrier-d32 is fastest (120.00 us, CV 3.4%); bitpack-carrier-packed-simd gives up 9.8% median for 6.7x lower variance (CV 0.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-carrier-d32** at 120000.8 ns median (-26.6% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.64x (fastest 120000.8 ns, slowest 196722.0 ns)

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
| bitpack-carrier-d16 | 162812ns | 163721ns | 159734ns | 163465ns | 163932ns | base |
| bitpack-carrier-d16-control | 160459ns | 159670ns | 159579ns | 159726ns | 163541ns | -1.45% |
| bitpack-carrier-d32 | 122553ns | 120186ns | 119875ns | 121070ns | 129680ns | -24.73% |
| bitpack-carrier-d64 | 189245ns | 197141ns | 160825ns | 192610ns | 207572ns | +16.24% |
| bitpack-carrier-packed | 129271ns | 127927ns | 127834ns | 128754ns | 132258ns | -20.60% |
| bitpack-carrier-packed-simd | 131865ns | 131949ns | 131236ns | 131941ns | 132267ns | -19.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 162635ns | 159583ns | 163689ns | base | 6.447 |
| bitpack-carrier-d16-control | 160295ns | 159429ns | 163380ns | -1.44% | 6.542 |
| bitpack-carrier-d32 | 122383ns | 119715ns | 129500ns | -24.75% | 8.568 |
| bitpack-carrier-d64 | 188916ns | 160466ns | 207249ns | +16.16% | 5.550 |
| bitpack-carrier-packed | 129090ns | 127685ns | 132000ns | -20.63% | 8.123 |
| bitpack-carrier-packed-simd | 131682ns | 131079ns | 132048ns | -19.03% | 7.963 |

## Performance model

- Peak throughput: **8.759 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.411 | 73.2% |
| bitpack-carrier-d16-control | 6.574 | 75.1% |
| bitpack-carrier-d32 | 8.738 | 99.8% |
| bitpack-carrier-d64 | 5.330 | 60.9% |
| bitpack-carrier-packed | 8.207 | 93.7% |
| bitpack-carrier-packed-simd | 7.957 | 90.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 162812ns | 162812ns | base |
| bitpack-carrier-d16-control | 160459ns | 160459ns | -1.45% |
| bitpack-carrier-d32 | 122553ns | 122553ns | -24.73% |
| bitpack-carrier-d64 | 189245ns | 189245ns | +16.24% |
| bitpack-carrier-packed | 129271ns | 129271ns | -20.60% |
| bitpack-carrier-packed-simd | 131865ns | 131865ns | -19.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 163570ns | base | --- | [163557, 163579] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 159500ns | -3913.2ns (-2.4%) | [-4096, -586]ns | [159474, 159529] | YES | 0.0022 | 0.0022 | 0 |
| bitpack-carrier-d32 | 120001ns | -40315.9ns (-24.6%) | [-43650, -39518]ns | [119917, 120037] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 196722ns | +33151.0ns (+20.3%) | [+17451, +41111]ns | [178966, 204570] | YES | 0.0022 | 0.0022 | 0 |
| bitpack-carrier-packed | 127764ns | -35824.2ns (-21.9%) | [-35855, -35053]ns | [127731, 128581] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 131779ns | -31793.5ns (-19.4%) | [-31852, -31747]ns | [131735, 131818] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 163572ns | -2.5% | -26.7% | +20.2% | -21.9% | -19.6% |
| 1 | 2 | 163563ns | -2.5% | -21.3% | +25.3% | -21.9% | -19.4% |
| 1 | 3 | 159542ns | -0.0% | -24.8% | +29.0% | -17.3% | -17.4% |
| 1 | 4 | 163607ns | +0.2% | -25.1% | -1.8% | -21.9% | -19.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.421 | moderate- |
| bitpack-carrier-d16-control | -0.074 | ok |
| bitpack-carrier-d32 | -0.648 | HIGH- (thermal bounce) |
| bitpack-carrier-d64 | -0.144 | ok |
| bitpack-carrier-packed | -0.405 | moderate- |
| bitpack-carrier-packed-simd | 0.096 | ok |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 2/4, lost 1/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 1/4, lost 3/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 39.5ns | 162634.9ns | 0.0% |  |
| bitpack-carrier-d16-control | 37.1ns | 160295.3ns | 0.0% |  |
| bitpack-carrier-d32 | 40.7ns | 122382.7ns | 0.0% |  |
| bitpack-carrier-d64 | 70.3ns | 188916.4ns | 0.0% |  |
| bitpack-carrier-packed | 38.0ns | 129090.5ns | 0.0% |  |
| bitpack-carrier-packed-simd | 40.8ns | 131681.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 159582.6-163689.4 ns)
  159582.6 |
  159788.0 |
  159993.3 |#
  160198.6 |#
  160404.0 |#
  160609.3 |
  160814.7 |
  161020.0 |
  161225.3 |
  161430.7 |
  161636.0 |
  161841.4 |
  162046.7 |
  162252.1 |
  162457.4 |
  162662.7 |
  162868.1 |
  163073.4 |
  163278.8 |
  163484.1 |########################################
  (7 below, 3 above range)

bitpack-carrier-d16-control (n=40, range 159428.8-163380.1 ns)
  159428.8 |########################################
  159626.4 |###
  159823.9 |###
  160021.5 |
  160219.1 |#
  160416.6 |
  160614.2 |
  160811.8 |
  161009.3 |#
  161206.9 |
  161404.5 |
  161602.0 |
  161799.6 |
  161997.1 |
  162194.7 |
  162392.3 |
  162589.8 |
  162787.4 |
  162985.0 |
  163182.5 |
  (5 below, 6 above range)

bitpack-carrier-d32 (n=40, range 119715.4-129500.3 ns)
  119715.4 |########################################
  120204.6 |#
  120693.9 |#
  121183.1 |
  121672.4 |
  122161.6 |
  122650.9 |
  123140.1 |
  123629.3 |#
  124118.6 |
  124607.8 |
  125097.1 |
  125586.3 |
  126075.6 |
  126564.8 |
  127054.1 |
  127543.3 |#
  128032.6 |
  128521.8 |#
  129011.1 |#####
  (3 below, 5 above range)

bitpack-carrier-d64 (n=40, range 160465.5-207249.2 ns)
  160465.5 |################################
  162804.7 |
  165143.9 |
  167483.1 |####
  169822.3 |
  172161.5 |########
  174500.6 |####
  176839.8 |
  179179.0 |
  181518.2 |####
  183857.4 |
  186196.6 |
  188535.8 |
  190874.9 |
  193214.1 |####
  195553.3 |########################
  197892.5 |
  200231.7 |####
  202570.9 |####################
  204910.1 |########################################
  (2 below, 2 above range)

bitpack-carrier-packed (n=40, range 127684.5-131999.9 ns)
  127684.5 |########################################
  127900.3 |
  128116.1 |#
  128331.8 |#
  128547.6 |#
  128763.4 |
  128979.2 |
  129194.9 |
  129410.7 |
  129626.5 |#
  129842.2 |
  130058.0 |
  130273.8 |
  130489.5 |
  130705.3 |
  130921.1 |
  131136.9 |
  131352.6 |
  131568.4 |#
  131784.2 |###############
  (3 below, 3 above range)

bitpack-carrier-packed-simd (n=40, range 131078.8-132048.1 ns)
  131078.8 |
  131127.3 |
  131175.7 |
  131224.2 |
  131272.7 |
  131321.1 |
  131369.6 |
  131418.1 |
  131466.5 |########
  131515.0 |########
  131563.5 |#################
  131611.9 |########
  131660.4 |########
  131708.9 |######################
  131757.3 |###############################
  131805.8 |########################################
  131854.3 |#############
  131902.7 |
  131951.2 |
  131999.7 |
  (1 below, 3 above range)

```
