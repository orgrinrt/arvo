# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 63% (significant)

warm-container-lanes-deferred is -3.79 us (63%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.7x slower than the field

warm-container-plusone (6.07 us) is 2.7x the fastest (2.24 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-lanes-deferred, warm-container-kernel) are a dead heat (<1%)

warm-container-lanes-deferred (2.24 us) and warm-container-kernel (2.26 us) differ by 0.65%, inside the noise, even though the wider field spreads 171.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-headroom shows warm-up / thermal drift (autocorr +0.57)

warm-container-headroom's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (145% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 145% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 2240.8 ns median (-62.7% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.71x (fastest 2240.8 ns, slowest 6073.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6162ns | 6118ns | 6032ns | 6120ns | 6418ns | base |
| warm-container-kernel | 2346ns | 2323ns | 2296ns | 2327ns | 2452ns | -61.93% |
| warm-container-lanes-deferred | 2312ns | 2306ns | 2265ns | 2300ns | 2397ns | -62.47% |
| warm-container-minimum | 2538ns | 2516ns | 2477ns | 2520ns | 2656ns | -58.81% |
| warm-container-native | 2521ns | 2504ns | 2469ns | 2500ns | 2634ns | -59.10% |
| warm-container-plusone | 6200ns | 6174ns | 6054ns | 6178ns | 6414ns | +0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6056ns | 5934ns | 6294ns | base | 5.411 |
| warm-container-kernel | 2277ns | 2229ns | 2381ns | -62.40% | 14.392 |
| warm-container-lanes-deferred | 2247ns | 2203ns | 2331ns | -62.89% | 14.581 |
| warm-container-minimum | 2478ns | 2417ns | 2590ns | -59.08% | 13.222 |
| warm-container-native | 2458ns | 2413ns | 2550ns | -59.41% | 13.333 |
| warm-container-plusone | 6099ns | 5957ns | 6307ns | +0.71% | 5.373 |

## Performance model

- Peak throughput: **14.878 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.448 | 36.6% |
| warm-container-kernel | 14.529 | 97.7% |
| warm-container-lanes-deferred | 14.623 | 98.3% |
| warm-container-minimum | 13.327 | 89.6% |
| warm-container-native | 13.395 | 90.0% |
| warm-container-plusone | 5.395 | 36.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6162ns | 6162ns | base |
| warm-container-kernel | 2346ns | 2346ns | -61.93% |
| warm-container-lanes-deferred | 2312ns | 2312ns | -62.47% |
| warm-container-minimum | 2538ns | 2538ns | -58.81% |
| warm-container-native | 2521ns | 2521ns | -59.10% |
| warm-container-plusone | 6200ns | 6200ns | +0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 6015ns | base | --- | [5956, 6076] | --- | --- | --- | --- |
| warm-container-kernel | 2255ns | -3750.8ns (-62.4%) | [-3832, -3696]ns | [2243, 2266] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 2241ns | -3766.7ns (-62.6%) | [-3824, -3734]ns | [2215, 2245] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2459ns | -3515.6ns (-58.5%) | [-3583, -3485]ns | [2454, 2463] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2446ns | -3541.4ns (-58.9%) | [-3616, -3528]ns | [2418, 2458] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 6073ns | +69.6ns (+1.2%) | [+20, +123]ns | [6057, 6088] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5935ns | -62.3% | -61.9% | -59.3% | -59.3% | +7.2% |
| 2 | 5948ns | -62.4% | -62.3% | -57.4% | -59.4% | +3.3% |
| 3 | 5945ns | -61.9% | -62.3% | -56.7% | -59.4% | +5.6% |
| 4 | 6098ns | -63.1% | -63.6% | -60.2% | -60.4% | +0.4% |
| 5 | 6255ns | -63.7% | -63.8% | -60.0% | -61.4% | -4.5% |
| 6 | 6250ns | -63.9% | -62.8% | -61.3% | -61.4% | -3.0% |
| 7 | 6255ns | -64.1% | -62.8% | -61.1% | -61.1% | -4.3% |
| 8 | 6256ns | -64.2% | -64.0% | -61.5% | -60.6% | -4.2% |
| 9 | 6658ns | -66.1% | -66.4% | -63.8% | -63.2% | -10.3% |
| 10 | 6301ns | -64.5% | -63.6% | -61.7% | -60.5% | -5.4% |
| 11 | 5978ns | -58.9% | -63.1% | -58.6% | -57.6% | -0.8% |
| 12 | 6037ns | -60.0% | -63.5% | -59.4% | -56.0% | +0.5% |
| 13 | 6037ns | -62.4% | -60.1% | -59.5% | -59.8% | +0.8% |
| 14 | 6079ns | -62.9% | -61.0% | -57.0% | -58.1% | +1.7% |
| 15 | 6058ns | -62.1% | -61.1% | -56.8% | -59.5% | -1.4% |
| 16 | 6136ns | -63.6% | -64.1% | -60.2% | -60.0% | -0.8% |
| 17 | 6082ns | -63.1% | -63.5% | -58.5% | -59.7% | +1.6% |
| 18 | 6217ns | -63.9% | -63.4% | -60.1% | -60.5% | +0.8% |
| 19 | 5937ns | -62.0% | -63.0% | -58.7% | -59.1% | +2.4% |
| 20 | 6020ns | -62.2% | -63.4% | -58.8% | -59.9% | +0.0% |
| 21 | 6073ns | -63.2% | -62.3% | -57.5% | -59.8% | -2.4% |
| 22 | 5965ns | -60.5% | -63.0% | -58.9% | -59.5% | +3.4% |
| 23 | 5939ns | -62.5% | -62.9% | -58.7% | -59.4% | +2.2% |
| 24 | 5978ns | -62.7% | -63.0% | -56.2% | -59.6% | +7.4% |
| 25 | 5948ns | -60.7% | -63.0% | -58.4% | -59.4% | +9.3% |
| 26 | 5938ns | -59.2% | -62.9% | -55.3% | -59.3% | +1.6% |
| 27 | 5930ns | -61.9% | -62.7% | -58.5% | -59.2% | +2.2% |
| 28 | 5939ns | -62.0% | -62.9% | -58.6% | -59.3% | +0.3% |
| 29 | 5933ns | -62.5% | -62.8% | -58.7% | -59.4% | +0.5% |
| 30 | 5997ns | -63.2% | -63.2% | -59.8% | -59.6% | +1.5% |
| 31 | 6017ns | -62.7% | -62.8% | -59.2% | -58.6% | +2.8% |
| 32 | 6084ns | -63.2% | -63.1% | -59.6% | -59.5% | +3.0% |
| 33 | 6114ns | -63.4% | -63.3% | -59.8% | -59.6% | -0.6% |
| 34 | 6012ns | -62.4% | -62.7% | -59.1% | -58.8% | +0.9% |
| 35 | 5934ns | -62.3% | -62.1% | -58.5% | -58.5% | +2.6% |
| 36 | 5966ns | -61.7% | -62.4% | -58.7% | -58.4% | +1.5% |
| 37 | 5936ns | -60.7% | -62.1% | -58.5% | -55.0% | +2.4% |
| 38 | 6163ns | -62.3% | -63.6% | -60.1% | -59.5% | -1.8% |
| 39 | 5931ns | -59.4% | -62.2% | -58.5% | -58.6% | +3.2% |
| 40 | 5945ns | -61.0% | -62.3% | -58.6% | -57.6% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.572 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.354 | moderate+ |
| warm-container-lanes-deferred | 0.407 | moderate+ |
| warm-container-minimum | 0.101 | ok |
| warm-container-native | 0.347 | moderate+ |
| warm-container-plusone | 0.356 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 12/40, lost 27/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.4ns | 6055.5ns | 0.0% |  |
| warm-container-kernel | 2.5ns | 2276.8ns | 0.1% |  |
| warm-container-lanes-deferred | 2.3ns | 2247.3ns | 0.1% |  |
| warm-container-minimum | 2.5ns | 2478.2ns | 0.1% |  |
| warm-container-native | 2.3ns | 2457.7ns | 0.1% |  |
| warm-container-plusone | 2.7ns | 6098.8ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5934.2-6294.4 ns)
   5934.2 |########################################
   5952.3 |########
   5970.3 |########
   5988.3 |####
   6006.3 |############
   6024.3 |########
   6042.3 |####
   6060.3 |####
   6078.3 |############
   6096.3 |########
   6114.3 |
   6132.3 |####
   6150.3 |####
   6168.3 |
   6186.3 |
   6204.3 |####
   6222.3 |
   6240.3 |################
   6258.4 |
   6276.4 |
  (4 below, 2 above range)

warm-container-kernel (n=40, range 2228.6-2380.9 ns)
   2228.6 |##############################
   2236.2 |########################################
   2243.9 |##########
   2251.5 |#########################
   2259.1 |###############
   2266.7 |###############
   2274.3 |
   2281.9 |#####
   2289.5 |#####
   2297.1 |
   2304.8 |
   2312.4 |#####
   2320.0 |#####
   2327.6 |#####
   2335.2 |#####
   2342.8 |
   2350.4 |#####
   2358.0 |
   2365.7 |
   2373.3 |
  (2 below, 4 above range)

warm-container-lanes-deferred (n=40, range 2202.5-2331.2 ns)
   2202.5 |########################################
   2208.9 |########
   2215.4 |####
   2221.8 |####
   2228.2 |
   2234.7 |########################
   2241.1 |########################
   2247.5 |########
   2254.0 |
   2260.4 |########
   2266.8 |
   2273.3 |####
   2279.7 |
   2286.1 |####
   2292.6 |####
   2299.0 |
   2305.4 |
   2311.9 |
   2318.3 |########
   2324.7 |
  (2 below, 3 above range)

warm-container-minimum (n=40, range 2417.4-2589.6 ns)
   2417.4 |
   2426.1 |########
   2434.7 |####
   2443.3 |############
   2451.9 |########################################
   2460.5 |####################
   2469.1 |############
   2477.7 |####
   2486.3 |
   2494.9 |####
   2503.5 |
   2512.1 |
   2520.7 |####
   2529.3 |####
   2537.9 |
   2546.6 |
   2555.2 |
   2563.8 |
   2572.4 |########
   2581.0 |
  (6 below, 4 above range)

warm-container-native (n=40, range 2412.9-2550.0 ns)
   2412.9 |########################################
   2419.7 |###
   2426.6 |##########
   2433.4 |
   2440.3 |###
   2447.2 |##############
   2454.0 |#######
   2460.9 |##########
   2467.7 |###
   2474.6 |#######
   2481.5 |###
   2488.3 |###
   2495.2 |###
   2502.0 |
   2508.9 |
   2515.8 |
   2522.6 |###
   2529.5 |###
   2536.3 |
   2543.2 |###
  (4 below, 2 above range)

warm-container-plusone (n=40, range 5957.4-6306.7 ns)
   5957.4 |####################
   5974.8 |####################
   5992.3 |######
   6009.8 |######
   6027.2 |######
   6044.7 |##########################
   6062.2 |########################################
   6079.6 |#################################
   6097.1 |
   6114.6 |#############
   6132.0 |######
   6149.5 |######
   6167.0 |####################
   6184.4 |
   6201.9 |
   6219.4 |
   6236.8 |
   6254.3 |#############
   6271.7 |######
   6289.2 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.57 (measurement drift or warm-up artifact)
