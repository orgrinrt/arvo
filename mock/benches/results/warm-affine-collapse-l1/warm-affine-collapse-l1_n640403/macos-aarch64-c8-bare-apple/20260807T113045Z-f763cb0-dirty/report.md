# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 60% (significant)

warm-container-lanes-deferred is -3.62 us (60%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.5x slower than the field

warm-container-plusone (6.23 us) is 2.5x the fastest (2.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (2.45 us) and warm-container-native (2.46 us) differ by 0.64%, inside the noise, even though the wider field spreads 154.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows warm-up / thermal drift (autocorr +0.58)

warm-container-native's per-pass series has lag-1 autocorrelation +0.58, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-lanes-deferred, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (139% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-lanes-deferred, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 139% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-kernel vs stability leader warm-container-native (+1% speed for 1.6x steadier)

warm-container-kernel is fastest (2.45 us, CV 3.7%); warm-container-native gives up 0.6% median for 1.6x lower variance (CV 2.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: warm-container-kernel** at 2445.6 ns median (-59.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.55x (fastest 2445.6 ns, slowest 6228.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6245ns | 6172ns | 6107ns | 6192ns | 6542ns | base |
| warm-container-kernel | 2538ns | 2510ns | 2462ns | 2513ns | 2689ns | -59.35% |
| warm-container-lanes-deferred | 2567ns | 2598ns | 2464ns | 2576ns | 2640ns | -58.90% |
| warm-container-minimum | 2648ns | 2602ns | 2468ns | 2590ns | 3001ns | -57.60% |
| warm-container-native | 2547ns | 2521ns | 2477ns | 2544ns | 2625ns | -59.22% |
| warm-container-plusone | 6310ns | 6326ns | 6042ns | 6272ns | 6690ns | +1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6145ns | 6012ns | 6434ns | base | 5.332 |
| warm-container-kernel | 2468ns | 2397ns | 2596ns | -59.84% | 13.277 |
| warm-container-lanes-deferred | 2498ns | 2400ns | 2572ns | -59.34% | 13.115 |
| warm-container-minimum | 2583ns | 2413ns | 2924ns | -57.96% | 12.684 |
| warm-container-native | 2489ns | 2423ns | 2566ns | -59.49% | 13.163 |
| warm-container-plusone | 6207ns | 5947ns | 6566ns | +1.00% | 5.279 |

## Performance model

- Peak throughput: **13.670 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.394 | 39.5% |
| warm-container-kernel | 13.399 | 98.0% |
| warm-container-lanes-deferred | 12.975 | 94.9% |
| warm-container-minimum | 12.905 | 94.4% |
| warm-container-native | 13.314 | 97.4% |
| warm-container-plusone | 5.261 | 38.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6245ns | 6245ns | base |
| warm-container-kernel | 2538ns | 2538ns | -59.35% |
| warm-container-lanes-deferred | 2567ns | 2567ns | -58.90% |
| warm-container-minimum | 2648ns | 2648ns | -57.60% |
| warm-container-native | 2547ns | 2547ns | -59.22% |
| warm-container-plusone | 6310ns | 6310ns | +1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 6074ns | base | --- | [6047, 6136] | --- | --- | --- | --- |
| warm-container-kernel | 2446ns | -3634.4ns (-59.8%) | [-3676, -3595]ns | [2439, 2458] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 2525ns | -3612.5ns (-59.5%) | [-3647, -3567]ns | [2490, 2529] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2539ns | -3581.5ns (-59.0%) | [-3632, -3500]ns | [2491, 2549] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2461ns | -3608.9ns (-59.4%) | [-3642, -3581]ns | [2457, 2530] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 6229ns | no significant difference | [-74, +122]ns | [6084, 6263] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 6869ns | -65.1% | -63.3% | -63.0% | -64.3% | -8.6% |
| 2 | 6107ns | -58.0% | -58.4% | -58.4% | -60.0% | +2.7% |
| 3 | 6728ns | -63.5% | -62.4% | -62.3% | -64.1% | -7.0% |
| 4 | 6443ns | -62.1% | -60.8% | -60.6% | -62.5% | -2.8% |
| 5 | 6034ns | -59.6% | -58.0% | -30.0% | -60.0% | +3.8% |
| 6 | 6358ns | -61.6% | -60.1% | -58.4% | -62.1% | -1.5% |
| 7 | 6265ns | -60.9% | -59.4% | -60.2% | -61.5% | +0.1% |
| 8 | 6279ns | -60.8% | -59.6% | -59.6% | -61.5% | +16.6% |
| 9 | 6190ns | -60.5% | -59.1% | -53.7% | -58.9% | +4.3% |
| 10 | 6144ns | -59.9% | -58.9% | -58.7% | -59.7% | +4.9% |
| 11 | 6054ns | -60.4% | -58.3% | -56.1% | -57.9% | -1.6% |
| 12 | 6028ns | -60.2% | -58.1% | -58.7% | -57.7% | -1.3% |
| 13 | 6054ns | -60.4% | -58.3% | -57.4% | -57.9% | -2.0% |
| 14 | 6105ns | -60.7% | -58.6% | -54.4% | -58.3% | -2.4% |
| 15 | 6040ns | -60.2% | -58.3% | -56.4% | -57.9% | -0.3% |
| 16 | 6069ns | -60.5% | -58.3% | -58.6% | -55.9% | +3.4% |
| 17 | 5999ns | -60.0% | -57.8% | -58.5% | -57.5% | +4.7% |
| 18 | 6171ns | -61.0% | -59.0% | -59.8% | -58.7% | +1.6% |
| 19 | 5929ns | -59.6% | -57.4% | -58.4% | -57.0% | +5.5% |
| 20 | 6155ns | -61.1% | -58.9% | -59.6% | -58.7% | +1.7% |
| 21 | 6262ns | -61.0% | -61.7% | -61.5% | -60.7% | -1.1% |
| 22 | 6270ns | -61.0% | -61.7% | -61.5% | -60.8% | -4.7% |
| 23 | 6117ns | -60.1% | -60.8% | -60.6% | -59.8% | -2.8% |
| 24 | 6048ns | -59.6% | -60.4% | -60.1% | -59.3% | +2.3% |
| 25 | 6128ns | -60.1% | -60.8% | -60.7% | -59.8% | -2.5% |
| 26 | 6037ns | -59.5% | -60.3% | -60.0% | -59.3% | -0.8% |
| 27 | 6049ns | -59.5% | -60.1% | -60.0% | -59.4% | -0.2% |
| 28 | 6099ns | -59.9% | -60.5% | -60.4% | -59.7% | -2.3% |
| 29 | 6030ns | -59.6% | -59.1% | -60.1% | -59.3% | -1.5% |
| 30 | 6043ns | -58.4% | -60.3% | -60.0% | -59.3% | -1.7% |
| 31 | 6183ns | -52.7% | -59.4% | -58.2% | -60.0% | +1.3% |
| 32 | 6045ns | -58.3% | -59.7% | -56.2% | -59.3% | +12.3% |
| 33 | 6058ns | -59.3% | -56.8% | -57.4% | -59.5% | +3.2% |
| 34 | 6014ns | -58.7% | -57.4% | -57.6% | -58.1% | +4.2% |
| 35 | 6046ns | -58.0% | -57.7% | -57.8% | -57.7% | +10.5% |
| 36 | 6042ns | -58.0% | -59.5% | -58.0% | -59.3% | +0.7% |
| 37 | 6028ns | -56.7% | -59.4% | -57.0% | -57.9% | +2.9% |
| 38 | 6080ns | -58.3% | -56.3% | -58.2% | -59.6% | +0.3% |
| 39 | 6163ns | -58.9% | -58.4% | -53.6% | -58.6% | -0.1% |
| 40 | 6038ns | -58.1% | -59.0% | -55.2% | -58.3% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.190 | ok |
| warm-container-kernel | 0.373 | moderate+ |
| warm-container-lanes-deferred | 0.541 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.104 | ok |
| warm-container-native | 0.581 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.358 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 18/40, lost 21/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.7ns | 6145.0ns | 0.0% |  |
| warm-container-kernel | 2.7ns | 2468.1ns | 0.1% |  |
| warm-container-lanes-deferred | 2.6ns | 2498.4ns | 0.1% |  |
| warm-container-minimum | 3.0ns | 2583.4ns | 0.1% |  |
| warm-container-native | 3.1ns | 2489.5ns | 0.1% |  |
| warm-container-plusone | 3.1ns | 6206.7ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 6012.4-6434.3 ns)
   6012.4 |#############
   6033.5 |########################################
   6054.6 |######
   6075.7 |###
   6096.8 |#############
   6117.8 |###
   6138.9 |######
   6160.0 |######
   6181.1 |######
   6202.2 |
   6223.3 |
   6244.4 |######
   6265.5 |######
   6286.6 |
   6307.7 |
   6328.8 |
   6349.9 |###
   6371.0 |
   6392.1 |
   6413.2 |
  (2 below, 3 above range)

warm-container-kernel (n=40, range 2397.1-2596.0 ns)
   2397.1 |####################
   2407.0 |
   2417.0 |
   2426.9 |####
   2436.8 |########################################
   2446.8 |################
   2456.7 |############
   2466.7 |
   2476.6 |####
   2486.6 |
   2496.5 |
   2506.5 |####
   2516.4 |####
   2526.4 |############
   2536.3 |########
   2546.3 |
   2556.2 |
   2566.1 |####
   2576.1 |
   2586.0 |
  (6 below, 2 above range)

warm-container-lanes-deferred (n=40, range 2400.3-2571.7 ns)
   2400.3 |###
   2408.8 |#######
   2417.4 |
   2426.0 |
   2434.6 |###
   2443.1 |#######
   2451.7 |
   2460.3 |###
   2468.8 |###
   2477.4 |
   2486.0 |
   2494.6 |
   2503.1 |###
   2511.7 |
   2520.3 |########################################
   2528.9 |#########################
   2537.4 |#######
   2546.0 |
   2554.6 |##########
   2563.2 |
  (6 below, 2 above range)

warm-container-minimum (n=40, range 2412.6-2923.8 ns)
   2412.6 |########################################
   2438.2 |
   2463.7 |####################
   2489.3 |##########################
   2514.8 |#################################
   2540.4 |#################################
   2566.0 |##########################
   2591.5 |
   2617.1 |######
   2642.6 |####################
   2668.2 |
   2693.8 |######
   2719.3 |
   2744.9 |
   2770.4 |######
   2796.0 |
   2821.5 |
   2847.1 |#############
   2872.7 |
   2898.2 |
  (4 below, 1 above range)

warm-container-native (n=40, range 2423.2-2566.3 ns)
   2423.2 |
   2430.4 |
   2437.5 |###
   2444.7 |
   2451.8 |########################################
   2459.0 |##############
   2466.1 |
   2473.3 |#######
   2480.5 |
   2487.6 |
   2494.8 |
   2501.9 |
   2509.1 |###
   2516.2 |###
   2523.4 |
   2530.5 |
   2537.7 |##############
   2544.8 |#########################
   2552.0 |#######
   2559.1 |
  (6 below, 1 above range)

warm-container-plusone (n=40, range 5946.9-6566.2 ns)
   5946.9 |####################
   5977.8 |###
   6008.8 |######
   6039.8 |
   6070.7 |##########
   6101.7 |
   6132.7 |###
   6163.6 |######
   6194.6 |###
   6225.6 |##########
   6256.5 |########################################
   6287.5 |
   6318.5 |
   6349.4 |
   6380.4 |
   6411.4 |
   6442.3 |######
   6473.3 |
   6504.3 |
   6535.2 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-lanes-deferred**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.58 (measurement drift or warm-up artifact)
