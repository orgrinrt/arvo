# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 60% (significant)

warm-container-minimum is -3.60 us (60%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.6x slower than the field

warm-container-plusone (6.23 us) is 2.6x the fastest (2.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (2.43 us) and warm-container-native (2.44 us) differ by 0.54%, inside the noise, even though the wider field spreads 156.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-headroom shows warm-up / thermal drift (autocorr +0.78)

warm-container-headroom's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-headroom, warm-container-plusone} (141% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-plusone} with a 141% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+1% speed for 2.5x steadier)

warm-container-minimum is fastest (2.43 us, CV 5.2%); warm-container-native gives up 0.5% median for 2.5x lower variance (CV 2.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: warm-container-minimum** at 2430.0 ns median (-59.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.56x (fastest 2430.0 ns, slowest 6230.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6203ns | 6129ns | 6076ns | 6186ns | 6382ns | base |
| warm-container-kernel | 2569ns | 2557ns | 2480ns | 2535ns | 2758ns | -58.59% |
| warm-container-minimum | 2534ns | 2491ns | 2438ns | 2505ns | 2718ns | -59.15% |
| warm-container-native | 2528ns | 2507ns | 2479ns | 2521ns | 2599ns | -59.24% |
| warm-container-plusone | 6320ns | 6334ns | 6114ns | 6304ns | 6578ns | +1.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6106ns | 5979ns | 6282ns | base | 4.025 |
| warm-container-kernel | 2506ns | 2422ns | 2685ns | -58.96% | 9.807 |
| warm-container-minimum | 2474ns | 2382ns | 2652ns | -59.48% | 9.934 |
| warm-container-native | 2467ns | 2422ns | 2537ns | -59.60% | 9.961 |
| warm-container-plusone | 6216ns | 6015ns | 6459ns | +1.80% | 3.953 |

## Performance model

- Peak throughput: **10.315 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.075 | 39.5% |
| warm-container-kernel | 9.830 | 95.3% |
| warm-container-minimum | 10.114 | 98.0% |
| warm-container-native | 10.059 | 97.5% |
| warm-container-plusone | 3.945 | 38.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6203ns | 6203ns | base |
| warm-container-kernel | 2569ns | 2569ns | -58.59% |
| warm-container-minimum | 2534ns | 2534ns | -59.15% |
| warm-container-native | 2528ns | 2528ns | -59.24% |
| warm-container-plusone | 6320ns | 6320ns | +1.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 6031ns | base | --- | [6025, 6236] | --- | --- | --- | --- |
| warm-container-kernel | 2500ns | -3536.0ns (-58.6%) | [-3811, -3513]ns | [2427, 2512] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2430ns | -3609.0ns (-59.8%) | [-3635, -3592]ns | [2426, 2468] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2443ns | -3581.8ns (-59.4%) | [-3682, -3530]ns | [2426, 2502] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 6230ns | +157.8ns (+2.6%) | [+11, +212]ns | [6165, 6248] | YES (adj: no) | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 6035ns | -58.4% | -54.6% | -58.3% | +3.5% |
| 2 | 6028ns | -58.2% | -57.0% | -58.4% | +3.7% |
| 3 | 6036ns | -58.3% | -58.8% | -58.2% | +3.6% |
| 4 | 6032ns | -58.3% | -58.4% | -58.0% | +3.2% |
| 5 | 6026ns | -58.2% | -58.3% | -58.1% | +3.9% |
| 6 | 6030ns | -58.3% | -58.4% | -58.3% | +3.7% |
| 7 | 6053ns | -58.5% | -59.9% | -58.4% | +3.5% |
| 8 | 6029ns | -58.3% | -59.7% | -58.2% | +3.9% |
| 9 | 6022ns | -58.3% | -59.7% | -58.3% | +3.8% |
| 10 | 6012ns | -59.6% | -59.7% | -58.1% | +4.7% |
| 11 | 6242ns | -61.1% | -59.6% | -61.2% | -3.4% |
| 12 | 6242ns | -61.2% | -50.3% | -61.1% | -3.6% |
| 13 | 6240ns | -61.3% | -58.3% | -61.2% | +12.0% |
| 14 | 6377ns | -62.1% | -61.3% | -62.0% | +1.7% |
| 15 | 6248ns | -61.2% | -59.0% | -61.2% | +3.2% |
| 16 | 6024ns | -58.6% | -57.0% | -59.8% | +3.7% |
| 17 | 6025ns | -59.8% | -58.2% | -59.8% | +0.4% |
| 18 | 6026ns | -59.4% | -59.5% | -59.8% | +2.5% |
| 19 | 6022ns | -59.8% | -59.6% | -59.7% | +3.5% |
| 20 | 6043ns | -58.5% | -59.7% | -58.0% | +2.7% |
| 21 | 6014ns | -57.3% | -60.3% | -59.6% | +3.7% |
| 22 | 6017ns | -57.6% | -60.4% | -58.6% | +6.4% |
| 23 | 5987ns | -58.1% | -60.2% | -59.5% | +3.9% |
| 24 | 5955ns | -57.9% | -59.9% | -59.0% | -0.5% |
| 25 | 6008ns | -56.3% | -60.3% | -59.3% | +1.5% |
| 26 | 5989ns | -55.2% | -60.2% | -59.1% | +1.3% |
| 27 | 6004ns | -58.2% | -60.3% | -59.6% | +3.1% |
| 28 | 5922ns | -54.5% | -59.8% | -59.1% | +10.3% |
| 29 | 5976ns | -58.0% | -60.1% | -57.9% | -0.5% |
| 30 | 5988ns | -44.6% | -60.2% | -59.6% | +2.0% |
| 31 | 6261ns | -61.3% | -61.2% | -58.0% | -0.4% |
| 32 | 6243ns | -61.1% | -60.1% | -59.7% | -0.0% |
| 33 | 6257ns | -61.2% | -60.6% | -60.3% | -0.1% |
| 34 | 6249ns | -61.2% | -61.2% | -61.1% | -2.5% |
| 35 | 6347ns | -61.4% | -61.8% | -61.9% | -3.1% |
| 36 | 6245ns | -61.2% | -61.1% | -61.2% | -0.8% |
| 37 | 6248ns | -61.2% | -61.1% | -61.2% | -3.0% |
| 38 | 6268ns | -61.3% | -61.4% | -60.5% | -3.9% |
| 39 | 6232ns | -61.1% | -60.5% | -61.1% | -2.3% |
| 40 | 6244ns | -61.1% | -61.2% | -60.6% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.782 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.077 | ok |
| warm-container-minimum | 0.403 | moderate+ |
| warm-container-native | 0.360 | moderate+ |
| warm-container-plusone | 0.178 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 12/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.5ns | 6106.1ns | 0.0% |  |
| warm-container-kernel | 3.1ns | 2505.9ns | 0.1% |  |
| warm-container-minimum | 2.6ns | 2473.9ns | 0.1% |  |
| warm-container-native | 2.3ns | 2467.1ns | 0.1% |  |
| warm-container-plusone | 2.8ns | 6216.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5978.7-6281.8 ns)
   5978.7 |############
   5993.8 |########
   6009.0 |####################
   6024.1 |########################################
   6039.3 |########
   6054.4 |
   6069.6 |
   6084.8 |
   6099.9 |
   6115.1 |
   6130.2 |
   6145.4 |
   6160.6 |
   6175.7 |
   6190.9 |
   6206.0 |
   6221.2 |####
   6236.3 |####################################
   6251.5 |########
   6266.7 |####
  (3 below, 2 above range)

warm-container-kernel (n=40, range 2421.9-2685.2 ns)
   2421.9 |########################################
   2435.1 |##
   2448.3 |##
   2461.4 |
   2474.6 |
   2487.8 |##
   2500.9 |####################
   2514.1 |####################
   2527.3 |
   2540.4 |##
   2553.6 |
   2566.8 |##
   2579.9 |
   2593.1 |
   2606.3 |
   2619.4 |##
   2632.6 |
   2645.8 |
   2658.9 |
   2672.1 |
  (3 below, 3 above range)

warm-container-minimum (n=40, range 2382.4-2652.3 ns)
   2382.4 |#########################
   2395.9 |
   2409.4 |#######
   2422.9 |########################################
   2436.4 |###
   2449.9 |###
   2463.4 |#######
   2476.9 |###
   2490.4 |###
   2503.9 |##########
   2517.4 |#######
   2530.9 |
   2544.4 |
   2557.9 |###
   2571.4 |
   2584.9 |#######
   2598.4 |###
   2611.9 |
   2625.3 |
   2638.8 |
  (3 below, 2 above range)

warm-container-native (n=40, range 2421.6-2537.3 ns)
   2421.6 |########################################
   2427.4 |#########
   2433.2 |
   2439.0 |######
   2444.7 |###
   2450.5 |
   2456.3 |###
   2462.1 |
   2467.9 |
   2473.7 |###
   2479.4 |###
   2485.2 |
   2491.0 |###
   2496.8 |
   2502.6 |
   2508.4 |###############
   2514.2 |############
   2519.9 |###
   2525.7 |######
   2531.5 |###
  (3 below, 1 above range)

warm-container-plusone (n=40, range 6015.2-6459.0 ns)
   6015.2 |##########
   6037.3 |###
   6059.5 |#######
   6081.7 |##########
   6103.9 |###
   6126.1 |###
   6148.3 |###
   6170.5 |#######
   6192.7 |#######
   6214.9 |##########
   6237.1 |########################################
   6259.3 |#######
   6281.5 |###
   6303.7 |
   6325.9 |
   6348.0 |
   6370.2 |
   6392.4 |###
   6414.6 |
   6436.8 |###
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.78 (measurement drift or warm-up artifact)
