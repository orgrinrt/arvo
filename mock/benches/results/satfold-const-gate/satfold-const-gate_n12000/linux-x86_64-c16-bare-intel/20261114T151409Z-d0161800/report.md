# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16 beats baseline by 92% (significant)

satfold-lanes16 is -25.01 us (92%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 12.0x slower than the field

satfold-seq (27.29 us) is 12.0x the fastest (2.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-gate-false shows alternating (throttle bounce) (autocorr -0.60)

satfold-gate-false's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16, satfold-lanes16-3, satfold-gate-true} vs {satfold-gate-false, satfold-seq} (1086% apart)

The field splits into a fast tier {satfold-lanes16, satfold-lanes16-3, satfold-gate-true} and a slow tier {satfold-gate-false, satfold-seq} with a 1086% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 12.0x the fastest

Fastest satfold-lanes16 (2.27 us) to slowest satfold-seq (27.29 us): 12.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-3 is inconsistent: worst-20% is 2.2x its best-20%

satfold-lanes16-3's best 20% of batches run at 1.56 us but its worst 20% at 3.38 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-seq's edge over baseline is significant but tiny (6 ns, 0.02%)

satfold-seq differs from baseline satfold-gate-false by 6 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes16** at 2273.3 ns median (-91.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 12.00x (fastest 2273.3 ns, slowest 27286.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 27354ns | 27349ns | 27338ns | 27349ns | 27383ns | base |
| satfold-gate-true | 2682ns | 2545ns | 2307ns | 2598ns | 3310ns | -90.19% |
| satfold-lanes16 | 2627ns | 2528ns | 2286ns | 2580ns | 3109ns | -90.40% |
| satfold-lanes16-3 | 2651ns | 2553ns | 1725ns | 2590ns | 3761ns | -90.31% |
| satfold-seq | 27371ns | 27357ns | 27346ns | 27358ns | 27434ns | +0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 27281ns | 27271ns | 27286ns | base | 1.201 |
| satfold-gate-true | 2416ns | 2091ns | 2979ns | -91.14% | 13.562 |
| satfold-lanes16 | 2358ns | 2063ns | 2750ns | -91.36% | 13.897 |
| satfold-lanes16-3 | 2393ns | 1564ns | 3378ns | -91.23% | 13.696 |
| satfold-seq | 27294ns | 27275ns | 27334ns | +0.05% | 1.201 |

## Performance model

- Peak throughput: **20.958 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 1.201 | 5.7% |
| satfold-gate-true | 14.247 | 68.0% |
| satfold-lanes16 | 14.414 | 68.8% |
| satfold-lanes16-3 | 14.266 | 68.1% |
| satfold-seq | 1.201 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 27354ns | 27354ns | base |
| satfold-gate-true | 2682ns | 2682ns | -90.19% |
| satfold-lanes16 | 2627ns | 2627ns | -90.40% |
| satfold-lanes16-3 | 2651ns | 2651ns | -90.31% |
| satfold-seq | 27371ns | 27371ns | +0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 27282ns | base | --- | [27282, 27283] | --- | --- | --- | --- |
| satfold-gate-true | 2300ns | -24980.4ns (-91.6%) | [-24987, -24932]ns | [2296, 2350] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 2273ns | -25009.0ns (-91.7%) | [-25066, -24836]ns | [2211, 2445] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 2297ns | -24975.5ns (-91.5%) | [-24987, -24926]ns | [2296, 2349] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 27287ns | +6.0ns (+0.0%) | [+5, +9]ns | [27286, 27289] | YES | 0.0007 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 27282ns | -91.7% | -90.9% | -91.6% | +0.0% |
| 1 | 2 | 27281ns | -90.6% | -91.0% | -91.6% | +0.0% |
| 1 | 3 | 27283ns | -91.3% | -92.2% | -88.3% | +0.0% |
| 1 | 4 | 27282ns | -91.6% | -91.7% | -91.5% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | -0.604 | HIGH- (thermal bounce) |
| satfold-gate-true | -0.403 | moderate- |
| satfold-lanes16 | 0.063 | ok |
| satfold-lanes16-3 | -0.401 | moderate- |
| satfold-seq | 0.111 | ok |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 23.3ns | 27280.7ns | 0.1% |  |
| satfold-gate-true | 86.5ns | 2416.1ns | 3.6% |  |
| satfold-lanes16 | 90.7ns | 2357.9ns | 3.8% |  |
| satfold-lanes16-3 | 86.8ns | 2392.5ns | 3.6% |  |
| satfold-seq | 24.4ns | 27294.1ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 27271.4-27285.8 ns)
  27271.4 |####################
  27272.2 |#############
  27272.9 |
  27273.6 |
  27274.3 |
  27275.0 |
  27275.7 |
  27276.5 |
  27277.2 |
  27277.9 |
  27278.6 |
  27279.3 |######
  27280.0 |#############
  27280.8 |#############
  27281.5 |########################################
  27282.2 |########################################
  27282.9 |#################################
  27283.6 |##########################
  27284.3 |####################
  27285.1 |
  (3 below, 3 above range)

satfold-gate-true (n=40, range 2091.3-2979.2 ns)
   2091.3 |#####
   2135.7 |#####
   2180.1 |##
   2224.5 |
   2268.9 |########################################
   2313.3 |#####
   2357.7 |#####
   2402.1 |
   2446.5 |
   2490.9 |##
   2535.3 |#######
   2579.6 |
   2624.0 |
   2668.4 |
   2712.8 |
   2757.2 |##
   2801.6 |##
   2846.0 |#######
   2890.4 |
   2934.8 |
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 2062.8-2749.7 ns)
   2062.8 |
   2097.2 |
   2131.5 |
   2165.8 |##################################
   2200.2 |######################
   2234.5 |######################
   2268.9 |###########
   2303.2 |
   2337.6 |###########
   2371.9 |
   2406.3 |
   2440.6 |########################################
   2475.0 |#####
   2509.3 |#####
   2543.6 |###########
   2578.0 |
   2612.3 |
   2646.7 |#####
   2681.0 |
   2715.4 |
  (5 below, 5 above range)

satfold-lanes16-3 (n=40, range 1563.5-3378.5 ns)
   1563.5 |
   1654.3 |
   1745.0 |
   1835.8 |
   1926.5 |
   2017.3 |######
   2108.0 |####
   2198.8 |##
   2289.5 |########################################
   2380.3 |####
   2471.0 |##
   2561.8 |####
   2652.5 |
   2743.3 |##
   2834.0 |
   2924.8 |
   3015.5 |##
   3106.3 |
   3197.0 |##
   3287.8 |##
  (3 below, 3 above range)

satfold-seq (n=40, range 27275.4-27334.3 ns)
  27275.4 |#############
  27278.4 |
  27281.3 |######
  27284.2 |########################################
  27287.2 |##############################
  27290.1 |######
  27293.1 |##########
  27296.0 |###
  27299.0 |######
  27301.9 |
  27304.8 |
  27307.8 |
  27310.7 |
  27313.7 |
  27316.6 |
  27319.6 |
  27322.5 |
  27325.4 |
  27328.4 |
  27331.3 |
  (4 below, 1 above range)

```

## Diagnostics

- **satfold-lanes16-3**: CV=29.4% (high variance, measurements may be unstable)
