# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes is an outlier: 2.4x slower than the field

warm-clamp-min-lanes (6.02 us) is 2.4x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-accfit-dyn, warm-clamp-accfit) are a dead heat (<1%)

warm-clamp-accfit-dyn (2.49 us) and warm-clamp-accfit (2.50 us) differ by 0.43%, inside the noise, even though the wider field spreads 142.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.55)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-minimum, warm-clamp-acc64, warm-clamp-head} vs {warm-clamp-min-lanes} (97% apart)

The field splits into a fast tier {warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-minimum, warm-clamp-acc64, warm-clamp-head} and a slow tier {warm-clamp-min-lanes} with a 97% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes is inconsistent: worst-20% is 13.9x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.97 us but its worst 20% at 27.43 us (13.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2485.3 ns median (-7.4% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.42x (fastest 2485.3 ns, slowest 6023.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2947ns | 2938ns | 2487ns | 2917ns | 3497ns | base |
| warm-clamp-accfit | 2781ns | 2735ns | 2436ns | 2733ns | 3270ns | -5.65% |
| warm-clamp-accfit-dyn | 2715ns | 2726ns | 2468ns | 2686ns | 3049ns | -7.89% |
| warm-clamp-head | 4675ns | 3146ns | 2320ns | 4004ns | 9042ns | +58.62% |
| warm-clamp-min-lanes | 10198ns | 6239ns | 2039ns | 6879ns | 28314ns | +246.02% |
| warm-clamp-minimum | 2746ns | 2730ns | 2488ns | 2705ns | 3128ns | -6.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2682ns | 2277ns | 3167ns | base | 3.054 |
| warm-clamp-accfit | 2527ns | 2216ns | 2953ns | -5.77% | 3.241 |
| warm-clamp-accfit-dyn | 2473ns | 2255ns | 2754ns | -7.81% | 3.313 |
| warm-clamp-head | 4529ns | 2252ns | 8758ns | +68.85% | 1.809 |
| warm-clamp-min-lanes | 9861ns | 1968ns | 27434ns | +267.64% | 0.831 |
| warm-clamp-minimum | 2501ns | 2279ns | 2791ns | -6.76% | 3.275 |

## Performance model

- Peak throughput: **4.163 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.051 | 73.3% |
| warm-clamp-accfit | 3.282 | 78.8% |
| warm-clamp-accfit-dyn | 3.296 | 79.2% |
| warm-clamp-head | 2.674 | 64.2% |
| warm-clamp-min-lanes | 1.360 | 32.7% |
| warm-clamp-minimum | 3.275 | 78.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2947ns | 2947ns | base |
| warm-clamp-accfit | 2781ns | 2781ns | -5.65% |
| warm-clamp-accfit-dyn | 2715ns | 2715ns | -7.89% |
| warm-clamp-head | 4675ns | 4675ns | +58.62% |
| warm-clamp-min-lanes | 10198ns | 10198ns | +246.02% |
| warm-clamp-minimum | 2746ns | 2746ns | -6.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2685ns | base | --- | [2499, 2749] | --- | --- | --- | --- |
| warm-clamp-accfit | 2496ns | no significant difference | [-335, +11]ns | [2407, 2557] | no | 0.2682 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 2485ns | -99.1ns (-3.7%) | [-180, -35]ns | [2354, 2513] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-head | 3063ns | +519.0ns (+19.3%) | [+290, +2578]ns | [3047, 5171] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 6023ns | +2890.2ns (+107.6%) | [+128, +4037]ns | [2411, 6844] | YES | 0.0107 | 0.0064 | 0 |
| warm-clamp-minimum | 2502ns | -121.7ns (-4.5%) | [-180, -13]ns | [2373, 2572] | YES (adj: no) | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2993ns | -15.6% | -17.7% | +1.9% | +37.2% | -20.7% |
| 1 | 2 | 2324ns | +2.0% | +0.0% | +8.1% | +3.7% | +6.8% |
| 1 | 3 | 2688ns | -13.1% | -6.1% | +14.2% | +916.0% | -6.8% |
| 1 | 4 | 2594ns | +3.9% | -3.8% | +99.4% | -12.4% | -1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.553 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.244 | moderate- |
| warm-clamp-accfit-dyn | -0.319 | moderate- |
| warm-clamp-head | 0.019 | ok |
| warm-clamp-min-lanes | -0.473 | moderate- |
| warm-clamp-minimum | 0.085 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 0/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 72.9ns | 2682.3ns | 2.7% |  |
| warm-clamp-accfit | 69.0ns | 2527.5ns | 2.7% |  |
| warm-clamp-accfit-dyn | 67.3ns | 2472.7ns | 2.7% |  |
| warm-clamp-head | 41.3ns | 4529.1ns | 0.9% |  |
| warm-clamp-min-lanes | 168.0ns | 9861.0ns | 1.7% |  |
| warm-clamp-minimum | 68.0ns | 2501.0ns | 2.7% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2277.4-3167.2 ns)
   2277.4 |
   2321.9 |########################################
   2366.4 |################
   2410.9 |
   2455.3 |########################################
   2499.8 |########################
   2544.3 |########
   2588.8 |
   2633.3 |
   2677.8 |########################################
   2722.3 |########################
   2766.8 |
   2811.2 |
   2855.7 |########
   2900.2 |################
   2944.7 |########################
   2989.2 |########################
   3033.7 |########
   3078.2 |
   3122.7 |
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 2216.4-2952.9 ns)
   2216.4 |#####
   2253.2 |#####
   2290.0 |####################
   2326.9 |##########
   2363.7 |#####
   2400.5 |###############
   2437.3 |
   2474.2 |########################################
   2511.0 |#####
   2547.8 |###############
   2584.6 |#####
   2621.5 |#####
   2658.3 |#####
   2695.1 |#####
   2731.9 |#####
   2768.8 |#####
   2805.6 |#####
   2842.4 |
   2879.2 |
   2916.1 |##########
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2255.3-2753.7 ns)
   2255.3 |
   2280.2 |
   2305.1 |########################################
   2330.1 |##########
   2355.0 |###############
   2379.9 |
   2404.8 |
   2429.7 |#####
   2454.7 |#####
   2479.6 |##############################
   2504.5 |##########
   2529.4 |##########
   2554.3 |##########
   2579.2 |
   2604.2 |
   2629.1 |#####
   2654.0 |#####
   2678.9 |
   2703.8 |###############
   2728.8 |##########
  (4 below, 2 above range)

warm-clamp-head (n=40, range 2252.2-8758.2 ns)
   2252.2 |################
   2577.5 |
   2902.8 |########################################
   3228.1 |
   3553.4 |
   3878.7 |
   4204.0 |##
   4529.3 |
   4854.6 |##
   5179.9 |#####
   5505.2 |##
   5830.5 |##
   6155.8 |########
   6481.1 |
   6806.4 |
   7131.7 |
   7457.0 |##
   7782.3 |##
   8107.6 |
   8432.9 |#####
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1967.9-27434.3 ns)
   1967.9 |########################################
   3241.3 |
   4514.6 |#######
   5787.9 |#############################
   7061.2 |#######
   8334.5 |###
   9607.8 |
  10881.1 |
  12154.5 |
  13427.8 |
  14701.1 |
  15974.4 |
  17247.7 |
  18521.0 |
  19794.4 |
  21067.7 |
  22341.0 |
  23614.3 |###
  24887.6 |
  26160.9 |#####################
  (6 below, 3 above range)

warm-clamp-minimum (n=40, range 2279.0-2791.0 ns)
   2279.0 |#####
   2304.6 |########################################
   2330.2 |#####
   2355.8 |######################
   2381.4 |#####
   2407.0 |
   2432.6 |#####
   2458.2 |#####
   2483.8 |##################################
   2509.4 |
   2535.0 |
   2560.6 |######################
   2586.2 |
   2611.8 |#####
   2637.4 |
   2663.0 |
   2688.6 |###########
   2714.2 |######################
   2739.8 |#####
   2765.4 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-clamp-head**: CV=55.0% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=102.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 13.9x (possible bimodal distribution)
