# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.72)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader warm-clamp-accfit-dyn vs stability leader warm-clamp-accfit (+5% speed for 1.6x steadier)

warm-clamp-accfit-dyn is fastest (2.39 us, CV 19.1%); warm-clamp-accfit gives up 4.6% median for 1.6x lower variance (CV 11.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-minimum's best 20% of batches run at 2.11 us but its worst 20% at 8.14 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2389.9 ns median (-4.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.70x (fastest 2389.9 ns, slowest 4072.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2699ns | 2677ns | 2260ns | 2684ns | 3185ns | base |
| warm-clamp-accfit | 2785ns | 2695ns | 2444ns | 2721ns | 3321ns | +3.19% |
| warm-clamp-accfit-dyn | 2694ns | 2580ns | 2031ns | 2639ns | 3522ns | -0.18% |
| warm-clamp-head | 2901ns | 3012ns | 1947ns | 2983ns | 3610ns | +7.49% |
| warm-clamp-min-lanes | 3736ns | 4268ns | 1411ns | 3943ns | 5441ns | +38.42% |
| warm-clamp-minimum | 4497ns | 3602ns | 2170ns | 3950ns | 8464ns | +66.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2503ns | 2100ns | 2942ns | base | 3.273 |
| warm-clamp-accfit | 2573ns | 2268ns | 3043ns | +2.79% | 3.184 |
| warm-clamp-accfit-dyn | 2445ns | 1868ns | 3077ns | -2.30% | 3.350 |
| warm-clamp-head | 2607ns | 1757ns | 3221ns | +4.15% | 3.142 |
| warm-clamp-min-lanes | 3549ns | 1345ns | 5118ns | +41.80% | 2.308 |
| warm-clamp-minimum | 4345ns | 2111ns | 8144ns | +73.59% | 1.885 |

## Performance model

- Peak throughput: **6.089 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.283 | 53.9% |
| warm-clamp-accfit | 3.278 | 53.8% |
| warm-clamp-accfit-dyn | 3.428 | 56.3% |
| warm-clamp-head | 3.016 | 49.5% |
| warm-clamp-min-lanes | 2.012 | 33.0% |
| warm-clamp-minimum | 2.369 | 38.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2699ns | 2699ns | base |
| warm-clamp-accfit | 2785ns | 2785ns | +3.19% |
| warm-clamp-accfit-dyn | 2694ns | 2694ns | -0.18% |
| warm-clamp-head | 2901ns | 2901ns | +7.49% |
| warm-clamp-min-lanes | 3736ns | 3736ns | +38.42% |
| warm-clamp-minimum | 4497ns | 4497ns | +66.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2495ns | base | --- | [2360, 2572] | --- | --- | --- | --- |
| warm-clamp-accfit | 2499ns | no significant difference | [-127, +133]ns | [2412, 2622] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-accfit-dyn | 2390ns | no significant difference | [-100, +7]ns | [2308, 2505] | no | 0.4210 | 0.3368 | 1 |
| warm-clamp-head | 2716ns | +103.1ns (+4.1%) | [+33, +274]ns | [2577, 2850] | YES | 0.0017 | 0.0007 | 0 |
| warm-clamp-min-lanes | 4072ns | +1599.7ns (+64.1%) | [+1226, +1859]ns | [3720, 4376] | YES | 0.0107 | 0.0064 | 0 |
| warm-clamp-minimum | 3458ns | +729.3ns (+29.2%) | [+199, +2237]ns | [2336, 4575] | YES | 0.0017 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2308ns | +22.1% | +0.9% | +23.5% | +80.5% | +1.5% |
| 1 | 2 | 2316ns | +1.9% | -0.4% | +2.5% | +81.1% | +62.0% |
| 1 | 3 | 2641ns | -7.0% | +11.9% | +8.8% | +52.2% | +1.9% |
| 1 | 4 | 2746ns | -13.0% | -10.2% | -3.5% | -50.8% | +26.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.293 | moderate+ |
| warm-clamp-accfit | -0.242 | moderate- |
| warm-clamp-accfit-dyn | -0.272 | moderate- |
| warm-clamp-head | -0.723 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.037 | ok |
| warm-clamp-minimum | -0.696 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 68.1ns | 2503.1ns | 2.7% |  |
| warm-clamp-accfit | 78.2ns | 2572.9ns | 3.0% |  |
| warm-clamp-accfit-dyn | 77.1ns | 2445.5ns | 3.2% |  |
| warm-clamp-head | 77.3ns | 2607.0ns | 3.0% |  |
| warm-clamp-min-lanes | 62.8ns | 3549.5ns | 1.8% |  |
| warm-clamp-minimum | 45.7ns | 4345.1ns | 1.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2099.9-2941.8 ns)
   2099.9 |
   2142.0 |########################################
   2184.1 |
   2226.2 |######
   2268.3 |####################
   2310.3 |#############
   2352.4 |####################
   2394.5 |######
   2436.6 |
   2478.7 |########################################
   2520.8 |#############
   2562.9 |#############
   2605.0 |
   2647.1 |
   2689.2 |##########################
   2731.3 |######
   2773.4 |#############
   2815.5 |######
   2857.6 |
   2899.7 |
  (1 below, 5 above range)

warm-clamp-accfit (n=40, range 2267.8-3043.2 ns)
   2267.8 |################
   2306.5 |########################################
   2345.3 |################
   2384.1 |########################
   2422.8 |################
   2461.6 |################################
   2500.4 |########
   2539.1 |################
   2577.9 |################
   2616.7 |################
   2655.5 |########
   2694.2 |########
   2733.0 |########
   2771.8 |########
   2810.5 |################
   2849.3 |################
   2888.1 |
   2926.8 |
   2965.6 |########
   3004.4 |
  (3 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1867.8-3077.4 ns)
   1867.8 |
   1928.3 |
   1988.8 |
   2049.2 |###
   2109.7 |################
   2170.2 |
   2230.7 |
   2291.2 |########################################
   2351.6 |
   2412.1 |######
   2472.6 |################
   2533.1 |######
   2593.6 |######
   2654.0 |
   2714.5 |######
   2775.0 |
   2835.5 |
   2896.0 |######
   2956.5 |###
   3016.9 |
  (2 below, 4 above range)

warm-clamp-head (n=40, range 1757.4-3221.0 ns)
   1757.4 |
   1830.6 |
   1903.8 |#####
   1977.0 |
   2050.1 |
   2123.3 |##########
   2196.5 |#####
   2269.7 |#####
   2342.9 |####################
   2416.0 |#####
   2489.2 |
   2562.4 |##############################
   2635.6 |
   2708.8 |##########
   2781.9 |########################################
   2855.1 |###############
   2928.3 |#####
   3001.5 |#####
   3074.7 |
   3147.9 |###############
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1345.4-5117.8 ns)
   1345.4 |##########################
   1534.0 |
   1722.6 |
   1911.3 |#############
   2099.9 |
   2288.5 |
   2477.1 |
   2665.7 |######
   2854.4 |######
   3043.0 |
   3231.6 |
   3420.2 |
   3608.8 |##########################
   3797.4 |######
   3986.1 |########################################
   4174.7 |#############
   4363.3 |##########################
   4551.9 |#############
   4740.5 |##########################
   4929.2 |#############
  (5 below, 2 above range)

warm-clamp-minimum (n=40, range 2110.5-8144.2 ns)
   2110.5 |########################################
   2412.2 |
   2713.9 |
   3015.6 |###
   3317.3 |#############################
   3618.9 |###
   3920.6 |
   4222.3 |
   4524.0 |
   4825.7 |
   5127.4 |
   5429.0 |###
   5730.7 |
   6032.4 |###
   6334.1 |###
   6635.8 |#######
   6937.4 |##########
   7239.1 |
   7540.8 |##########
   7842.5 |###
  (5 below, 2 above range)

```

## Diagnostics

- **warm-clamp-head**: CV=21.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=39.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=55.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
