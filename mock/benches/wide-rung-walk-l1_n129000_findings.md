# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged dominates: 26% faster than the next best (wide-rung-wordround-alias)

wide-rung-ragged (2.06 us) leads wide-rung-wordround-alias (2.60 us) by 26%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 22% (significant)

wide-rung-ragged is -595 ns (22%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-ragged is fastest but the noisiest (CV 8.9%)

wide-rung-ragged wins on median (2.06 us) yet has the highest variance (CV 8.9%), while wide-rung-ragged-overread is the steadiest (CV 5.7%, 2.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.89)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wide-rung-ragged} vs {wide-rung-wordround-alias, wide-rung-ragged-overread, wide-rung-align16, wide-rung-wordround} (26% apart)

The field splits into a fast tier {wide-rung-ragged} and a slow tier {wide-rung-wordround-alias, wide-rung-ragged-overread, wide-rung-align16, wide-rung-wordround} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (0 ns, 0.01%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by 0 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 2058.1 ns median (-24.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.35x (fastest 2058.1 ns, slowest 2784.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2819ns | 2800ns | 2610ns | 2822ns | 3020ns | base |
| wide-rung-ragged | 2194ns | 2122ns | 1978ns | 2170ns | 2483ns | -22.17% |
| wide-rung-ragged-overread | 2773ns | 2770ns | 2608ns | 2748ns | 3016ns | -1.62% |
| wide-rung-wordround | 2861ns | 2856ns | 2610ns | 2886ns | 3035ns | +1.47% |
| wide-rung-wordround-alias | 2756ns | 2664ns | 2611ns | 2705ns | 3055ns | -2.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2753ns | 2548ns | 2948ns | base | 0.744 |
| wide-rung-ragged | 2126ns | 1918ns | 2406ns | -22.76% | 0.963 |
| wide-rung-ragged-overread | 2708ns | 2547ns | 2946ns | -1.60% | 0.756 |
| wide-rung-wordround | 2792ns | 2549ns | 2957ns | +1.43% | 0.734 |
| wide-rung-wordround-alias | 2691ns | 2549ns | 2983ns | -2.25% | 0.761 |

## Performance model

- Peak throughput: **1.068 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.749 | 70.1% |
| wide-rung-ragged | 0.995 | 93.2% |
| wide-rung-ragged-overread | 0.756 | 70.8% |
| wide-rung-wordround | 0.735 | 68.9% |
| wide-rung-wordround-alias | 0.787 | 73.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2819ns | 2819ns | base |
| wide-rung-ragged | 2194ns | 2194ns | -22.17% |
| wide-rung-ragged-overread | 2773ns | 2773ns | -1.62% |
| wide-rung-wordround | 2861ns | 2861ns | +1.47% |
| wide-rung-wordround-alias | 2756ns | 2756ns | -2.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2735ns | base | --- | [2643, 2897] | --- | --- | --- | --- |
| wide-rung-ragged | 2058ns | -608.4ns (-22.2%) | [-674, -576]ns | [2054, 2178] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 2708ns | no significant difference | [-211, +21]ns | [2599, 2732] | no | 0.5224 | 0.4296 | 0 |
| wide-rung-wordround | 2785ns | no significant difference | [-2, +123]ns | [2734, 2946] | no | 0.5224 | 0.5224 | 1 |
| wide-rung-wordround-alias | 2601ns | no significant difference | [-113, +0]ns | [2595, 2649] | no | 0.2163 | 0.1081 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 2598ns | -14.8% | +1.5% | -1.9% | -1.9% |
| 2 | 2548ns | -13.0% | +3.6% | +0.1% | +0.1% |
| 3 | 2550ns | -13.0% | +3.3% | +0.0% | +0.2% |
| 4 | 2548ns | -12.8% | +3.6% | +0.1% | +0.1% |
| 5 | 2641ns | -18.9% | -1.4% | -3.5% | -3.5% |
| 6 | 2644ns | -22.2% | -3.6% | -3.6% | -3.6% |
| 7 | 2549ns | -19.2% | -0.1% | +0.1% | +0.1% |
| 8 | 2546ns | -19.2% | +0.1% | +0.1% | +4.8% |
| 9 | 2549ns | -19.2% | -0.1% | -0.1% | +1.7% |
| 10 | 2549ns | -19.4% | +0.0% | +0.2% | +0.0% |
| 11 | 2949ns | -19.6% | -7.4% | -7.3% | +0.0% |
| 12 | 2949ns | -19.6% | -7.4% | -7.4% | -0.0% |
| 13 | 2948ns | -10.3% | -7.3% | -7.3% | +0.0% |
| 14 | 2948ns | -19.5% | -7.4% | -7.2% | -0.1% |
| 15 | 2947ns | -19.6% | -7.2% | -7.3% | -0.1% |
| 16 | 2948ns | -19.5% | -7.3% | -7.2% | -0.1% |
| 17 | 2948ns | -19.6% | -7.3% | -7.2% | -0.1% |
| 18 | 2948ns | -19.5% | -7.2% | -7.2% | -0.0% |
| 19 | 2945ns | -19.6% | -7.1% | -7.0% | +2.7% |
| 20 | 2949ns | -19.5% | -7.4% | -9.2% | +6.9% |
| 21 | 2948ns | -35.0% | -13.6% | -0.1% | -10.5% |
| 22 | 2947ns | -34.1% | -8.9% | -0.1% | -10.5% |
| 23 | 2947ns | -34.9% | -12.0% | -0.0% | -9.3% |
| 24 | 2946ns | -34.9% | -13.4% | +0.0% | -10.5% |
| 25 | 2850ns | -32.6% | -10.5% | +3.4% | -7.4% |
| 26 | 2549ns | -24.8% | +0.0% | +15.6% | +4.5% |
| 27 | 2551ns | -24.8% | -0.1% | +15.6% | +3.0% |
| 28 | 2549ns | -24.8% | -0.1% | +15.6% | +0.1% |
| 29 | 2550ns | -24.8% | -0.1% | +18.3% | -0.0% |
| 30 | 2550ns | -24.7% | -0.2% | +11.0% | +0.0% |
| 31 | 2805ns | -26.7% | +4.9% | +5.0% | -7.5% |
| 32 | 2735ns | -24.9% | +7.7% | +7.8% | -5.0% |
| 33 | 2734ns | -24.9% | +7.7% | +7.8% | -4.9% |
| 34 | 2735ns | -24.9% | +7.7% | +7.8% | -5.1% |
| 35 | 2735ns | -24.8% | +7.8% | +7.7% | -5.0% |
| 36 | 2732ns | -24.8% | +7.8% | +7.8% | -2.7% |
| 37 | 2844ns | -27.6% | +3.5% | +3.7% | -8.6% |
| 38 | 2731ns | -24.8% | +7.9% | +7.8% | -4.8% |
| 39 | 2735ns | -24.7% | +7.6% | +7.8% | -4.9% |
| 40 | 2730ns | -24.6% | +7.7% | +8.0% | -5.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.819 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.812 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.831 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.890 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.779 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 40/40, lost 0/40
- **wide-rung-ragged-overread**: won 19/40, lost 15/40
- **wide-rung-wordround**: won 13/40, lost 19/40
- **wide-rung-wordround-alias**: won 18/40, lost 9/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.2ns | 2752.6ns | 0.1% |  |
| wide-rung-ragged | 2.6ns | 2126.2ns | 0.1% |  |
| wide-rung-ragged-overread | 2.3ns | 2708.5ns | 0.1% |  |
| wide-rung-wordround | 2.0ns | 2792.0ns | 0.1% |  |
| wide-rung-wordround-alias | 2.0ns | 2690.6ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2548.4-2948.4 ns)
   2548.4 |################################
   2568.4 |
   2588.4 |###
   2608.4 |
   2628.4 |#######
   2648.4 |
   2668.4 |
   2688.4 |
   2708.4 |
   2728.4 |#############################
   2748.4 |
   2768.4 |
   2788.4 |###
   2808.4 |
   2828.4 |###
   2848.4 |###
   2868.4 |
   2888.4 |
   2908.4 |
   2928.4 |########################################
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 1917.6-2405.9 ns)
   1917.6 |#############
   1942.0 |
   1966.4 |
   1990.9 |
   2015.3 |
   2039.7 |########################################
   2064.1 |
   2088.5 |
   2112.9 |
   2137.3 |##
   2161.8 |
   2186.2 |
   2210.6 |##########
   2235.0 |
   2259.4 |
   2283.8 |
   2308.2 |
   2332.6 |
   2357.1 |########################
   2381.5 |
  (5 below, 1 above range)

wide-rung-ragged-overread (n=40, range 2547.3-2945.5 ns)
   2547.3 |################################
   2567.2 |
   2587.1 |########
   2607.0 |
   2627.0 |################
   2646.9 |
   2666.8 |####
   2686.7 |
   2706.6 |
   2726.5 |########################################
   2746.4 |
   2766.3 |
   2786.2 |
   2806.1 |
   2826.1 |
   2846.0 |
   2865.9 |
   2885.8 |
   2905.7 |
   2925.6 |########################
  (5 below, 4 above range)

wide-rung-wordround (n=40, range 2549.4-2956.7 ns)
   2549.4 |###############
   2569.8 |
   2590.1 |
   2610.5 |
   2630.8 |
   2651.2 |
   2671.6 |##
   2691.9 |
   2712.3 |####
   2732.7 |###############
   2753.0 |
   2773.4 |
   2793.8 |
   2814.1 |##
   2834.5 |
   2854.9 |
   2875.2 |
   2895.6 |
   2916.0 |
   2936.3 |########################################
  (3 below, 1 above range)

wide-rung-wordround-alias (n=40, range 2549.4-2982.8 ns)
   2549.4 |########################
   2571.1 |
   2592.8 |########################################
   2614.4 |####
   2636.1 |################
   2657.8 |################
   2679.4 |
   2701.1 |
   2722.8 |
   2744.4 |
   2766.1 |
   2787.8 |
   2809.5 |
   2831.1 |
   2852.8 |
   2874.5 |
   2896.1 |
   2917.8 |
   2939.5 |################################
   2961.1 |
  (5 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.78 (measurement drift or warm-up artifact)
