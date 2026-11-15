# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 74% (significant)

warm-clamp-accfit is -6.96 us (74%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 4.0x slower than the field

warm-clamp-minimum (9.92 us) is 4.0x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit is fastest but the noisiest (CV 7.2%)

warm-clamp-accfit wins on median (2.48 us) yet has the highest variance (CV 7.2%), while warm-clamp-acc64 is the steadiest (CV 1.3%, 9.40 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-accfit, warm-clamp-head) are a dead heat (<1%)

warm-clamp-accfit (2.48 us) and warm-clamp-head (2.50 us) differ by 0.88%, inside the noise, even though the wider field spreads 300.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit-dyn shows warm-up / thermal drift (autocorr +0.84)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-min-lanes} vs {warm-clamp-acc64, warm-clamp-minimum} (159% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-min-lanes} and a slow tier {warm-clamp-acc64, warm-clamp-minimum} with a 159% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest warm-clamp-accfit (2.48 us) to slowest warm-clamp-minimum (9.92 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 2479.6 ns median (-73.6% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.00x (fastest 2479.6 ns, slowest 9920.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 9502ns | 9474ns | 9385ns | 9475ns | 9698ns | base |
| warm-clamp-accfit | 2626ns | 2542ns | 2539ns | 2548ns | 2946ns | -72.36% |
| warm-clamp-accfit-dyn | 2999ns | 2894ns | 2889ns | 2938ns | 3291ns | -68.44% |
| warm-clamp-head | 2698ns | 2591ns | 2586ns | 2630ns | 3015ns | -71.61% |
| warm-clamp-min-lanes | 3710ns | 3688ns | 3631ns | 3699ns | 3825ns | -60.95% |
| warm-clamp-minimum | 10157ns | 10013ns | 9449ns | 10118ns | 10979ns | +6.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 9433ns | 9321ns | 9626ns | base | 0.868 |
| warm-clamp-accfit | 2560ns | 2475ns | 2872ns | -72.86% | 3.200 |
| warm-clamp-accfit-dyn | 2937ns | 2833ns | 3224ns | -68.86% | 2.789 |
| warm-clamp-head | 2605ns | 2497ns | 2910ns | -72.39% | 3.145 |
| warm-clamp-min-lanes | 3645ns | 3568ns | 3754ns | -61.36% | 2.248 |
| warm-clamp-minimum | 10079ns | 9381ns | 10899ns | +6.85% | 0.813 |

## Performance model

- Peak throughput: **3.310 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 0.872 | 26.3% |
| warm-clamp-accfit | 3.304 | 99.8% |
| warm-clamp-accfit-dyn | 2.887 | 87.2% |
| warm-clamp-head | 3.275 | 98.9% |
| warm-clamp-min-lanes | 2.262 | 68.3% |
| warm-clamp-minimum | 0.826 | 24.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 9502ns | 9502ns | base |
| warm-clamp-accfit | 2626ns | 2626ns | -72.36% |
| warm-clamp-accfit-dyn | 2999ns | 2999ns | -68.44% |
| warm-clamp-head | 2698ns | 2698ns | -71.61% |
| warm-clamp-min-lanes | 3710ns | 3710ns | -60.95% |
| warm-clamp-minimum | 10157ns | 10157ns | +6.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 9398ns | base | --- | [9359, 9455] | --- | --- | --- | --- |
| warm-clamp-accfit | 2480ns | -6879.6ns (-73.2%) | [-6927, -6851]ns | [2478, 2489] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2838ns | -6498.8ns (-69.1%) | [-6540, -6481]ns | [2836, 2862] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2501ns | -6851.0ns (-72.9%) | [-6900, -6819]ns | [2500, 2521] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3622ns | -5763.5ns (-61.3%) | [-5832, -5712]ns | [3604, 3662] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 9921ns | +492.1ns (+5.2%) | [+257, +1043]ns | [9729, 10512] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 9876ns | -74.8% | -71.3% | -74.7% | -63.6% | -4.0% |
| 2 | 9745ns | -74.6% | -70.9% | -73.1% | -63.0% | -0.9% |
| 3 | 9509ns | -73.7% | -70.2% | -73.7% | -62.6% | +6.6% |
| 4 | 9591ns | -74.1% | -70.4% | -70.9% | -61.8% | +10.2% |
| 5 | 9349ns | -73.5% | -69.6% | -69.6% | -59.9% | +13.6% |
| 6 | 9332ns | -73.5% | -69.5% | -69.6% | -61.1% | +5.4% |
| 7 | 9345ns | -73.5% | -69.3% | -69.7% | -60.5% | -0.3% |
| 8 | 9360ns | -73.5% | -69.7% | -69.7% | -61.1% | +0.3% |
| 9 | 9440ns | -73.7% | -70.0% | -70.0% | -60.9% | +0.5% |
| 10 | 9441ns | -73.7% | -69.7% | -70.0% | -61.7% | +0.8% |
| 11 | 9391ns | -72.9% | -69.5% | -73.4% | -60.3% | +12.5% |
| 12 | 9314ns | -73.4% | -69.6% | -73.2% | -60.6% | +14.4% |
| 13 | 9387ns | -73.6% | -69.8% | -73.4% | -61.4% | +13.3% |
| 14 | 9319ns | -73.4% | -69.6% | -73.2% | -60.8% | +14.5% |
| 15 | 9333ns | -73.4% | -69.7% | -73.2% | -60.8% | +3.3% |
| 16 | 9395ns | -73.2% | -69.8% | -73.4% | -60.9% | +0.4% |
| 17 | 9317ns | -73.4% | -69.6% | -73.2% | -60.5% | +13.4% |
| 18 | 9358ns | -73.5% | -68.3% | -73.3% | -59.7% | +12.0% |
| 19 | 9406ns | -73.7% | -69.8% | -73.5% | -61.5% | +4.2% |
| 20 | 9366ns | -73.6% | -69.7% | -73.3% | -61.6% | +5.1% |
| 21 | 9330ns | -67.6% | -65.5% | -73.2% | -62.0% | +2.7% |
| 22 | 9542ns | -68.5% | -66.3% | -73.1% | -62.5% | +2.8% |
| 23 | 9461ns | -68.2% | -66.0% | -73.5% | -62.4% | +12.8% |
| 24 | 9468ns | -68.2% | -66.0% | -73.6% | -62.2% | +11.7% |
| 25 | 9371ns | -67.9% | -67.0% | -73.3% | -61.6% | +13.1% |
| 26 | 9462ns | -70.0% | -66.0% | -73.2% | -62.1% | +7.5% |
| 27 | 9512ns | -73.9% | -65.6% | -73.0% | -61.9% | +4.1% |
| 28 | 9585ns | -74.1% | -66.4% | -68.4% | -62.4% | +2.3% |
| 29 | 9475ns | -73.9% | -66.0% | -68.0% | -62.3% | +4.9% |
| 30 | 9448ns | -73.8% | -65.9% | -67.9% | -62.3% | +7.5% |
| 31 | 9465ns | -73.8% | -69.2% | -73.6% | -60.9% | +19.4% |
| 32 | 9401ns | -73.7% | -69.9% | -73.4% | -61.5% | +20.5% |
| 33 | 9438ns | -72.9% | -70.0% | -73.5% | -61.9% | +19.8% |
| 34 | 9318ns | -73.4% | -69.5% | -73.2% | -58.6% | +13.1% |
| 35 | 9331ns | -73.5% | -69.6% | -73.2% | -61.2% | +1.9% |
| 36 | 9341ns | -73.3% | -69.6% | -73.2% | -60.3% | +9.5% |
| 37 | 9318ns | -73.1% | -69.6% | -73.2% | -59.9% | +0.0% |
| 38 | 9320ns | -73.1% | -69.6% | -73.2% | -61.3% | +1.6% |
| 39 | 9635ns | -74.3% | -70.6% | -74.0% | -62.6% | -3.3% |
| 40 | 9520ns | -73.8% | -70.1% | -73.7% | -60.3% | -2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.486 | moderate+ |
| warm-clamp-accfit | 0.809 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.837 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.682 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.127 | ok |
| warm-clamp-minimum | 0.572 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 5/40, lost 34/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.6ns | 9432.9ns | 0.0% |  |
| warm-clamp-accfit | 2.6ns | 2560.4ns | 0.1% |  |
| warm-clamp-accfit-dyn | 3.2ns | 2937.2ns | 0.1% |  |
| warm-clamp-head | 2.8ns | 2604.8ns | 0.1% |  |
| warm-clamp-min-lanes | 2.6ns | 3644.5ns | 0.1% |  |
| warm-clamp-minimum | 2.9ns | 10078.8ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 9320.8-9625.8 ns)
   9320.8 |########################################
   9336.0 |##############################
   9351.3 |##############################
   9366.5 |##########
   9381.8 |##############################
   9397.0 |####################
   9412.3 |
   9427.6 |##############################
   9442.8 |##########
   9458.1 |########################################
   9473.3 |##########
   9488.6 |
   9503.8 |####################
   9519.1 |##########
   9534.3 |##########
   9549.6 |
   9564.8 |
   9580.1 |####################
   9595.3 |
   9610.6 |
  (6 below, 3 above range)

warm-clamp-accfit (n=40, range 2474.9-2872.3 ns)
   2474.9 |########################################
   2494.8 |######
   2514.6 |#
   2534.5 |#
   2554.4 |#
   2574.2 |
   2594.1 |
   2614.0 |
   2633.8 |
   2653.7 |
   2673.6 |
   2693.5 |
   2713.3 |
   2733.2 |
   2753.1 |
   2772.9 |
   2792.8 |
   2812.7 |
   2832.5 |#
   2852.4 |
  (3 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 2833.5-3224.5 ns)
   2833.5 |########################################
   2853.0 |#####
   2872.6 |
   2892.1 |
   2911.7 |#
   2931.2 |
   2950.8 |#
   2970.3 |
   2989.9 |
   3009.4 |
   3029.0 |
   3048.5 |
   3068.1 |
   3087.6 |#
   3107.2 |
   3126.7 |
   3146.3 |
   3165.8 |
   3185.4 |
   3204.9 |##############
  (3 below, 1 above range)

warm-clamp-head (n=40, range 2497.4-2910.1 ns)
   2497.4 |########################################
   2518.1 |#
   2538.7 |
   2559.3 |###
   2580.0 |
   2600.6 |#
   2621.2 |
   2641.9 |
   2662.5 |
   2683.1 |
   2703.8 |
   2724.4 |
   2745.0 |
   2765.7 |
   2786.3 |#
   2806.9 |
   2827.6 |##########
   2848.2 |
   2868.8 |
   2889.5 |
  (2 below, 3 above range)

warm-clamp-min-lanes (n=40, range 3567.6-3754.2 ns)
   3567.6 |################
   3576.9 |########
   3586.2 |################
   3595.6 |########################################
   3604.9 |################
   3614.2 |########################################
   3623.6 |################
   3632.9 |
   3642.2 |########
   3651.6 |################
   3660.9 |################
   3670.2 |########
   3679.6 |################
   3688.9 |########
   3698.2 |################
   3707.6 |
   3716.9 |
   3726.2 |########
   3735.6 |########
   3744.9 |########
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 9381.1-10899.0 ns)
   9381.1 |################
   9457.0 |########################################
   9532.9 |########
   9608.8 |################
   9684.7 |
   9760.6 |########################
   9836.5 |########################
   9912.4 |########
   9988.3 |
  10064.1 |########
  10140.0 |################
  10215.9 |########
  10291.8 |
  10367.7 |
  10443.6 |########
  10519.5 |########################################
  10595.4 |################################
  10671.3 |################
  10747.2 |
  10823.1 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.68 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.57 (measurement drift or warm-up artifact)
