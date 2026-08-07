# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 70% (significant)

warm-container-kernel is -5.95 us (70%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 3.5x slower than the field

warm-container-minimum (8.58 us) is 3.5x the fastest (2.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.72)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.72, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-kernel, warm-container-native} vs {warm-container-plusone, warm-container-headroom, warm-container-minimum} (238% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-kernel, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom, warm-container-minimum} with a 238% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.5x the fastest

Fastest warm-container-lanes-deferred (2.42 us) to slowest warm-container-minimum (8.58 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 2417.5 ns median (-71.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.55x (fastest 2417.5 ns, slowest 8581.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8738ns | 8575ns | 8406ns | 8614ns | 9446ns | base |
| warm-container-kernel | 2611ns | 2515ns | 2477ns | 2588ns | 2814ns | -70.12% |
| warm-container-lanes-deferred | 2517ns | 2483ns | 2476ns | 2484ns | 2654ns | -71.20% |
| warm-container-minimum | 8868ns | 8647ns | 8388ns | 8787ns | 9591ns | +1.49% |
| warm-container-native | 2625ns | 2534ns | 2489ns | 2585ns | 2883ns | -69.96% |
| warm-container-plusone | 8477ns | 8416ns | 8343ns | 8426ns | 8762ns | -2.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8639ns | 8309ns | 9341ns | base | 3.793 |
| warm-container-kernel | 2544ns | 2415ns | 2741ns | -70.55% | 12.879 |
| warm-container-lanes-deferred | 2451ns | 2413ns | 2584ns | -71.63% | 13.370 |
| warm-container-minimum | 8798ns | 8325ns | 9520ns | +1.84% | 3.724 |
| warm-container-native | 2564ns | 2427ns | 2819ns | -70.32% | 12.781 |
| warm-container-plusone | 8409ns | 8284ns | 8672ns | -2.67% | 3.897 |

## Performance model

- Peak throughput: **13.579 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.865 | 28.5% |
| warm-container-kernel | 13.377 | 98.5% |
| warm-container-lanes-deferred | 13.554 | 99.8% |
| warm-container-minimum | 3.818 | 28.1% |
| warm-container-native | 13.255 | 97.6% |
| warm-container-plusone | 3.923 | 28.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8738ns | 8738ns | base |
| warm-container-kernel | 2611ns | 2611ns | -70.12% |
| warm-container-lanes-deferred | 2517ns | 2517ns | -71.20% |
| warm-container-minimum | 8868ns | 8868ns | +1.49% |
| warm-container-native | 2625ns | 2625ns | -69.96% |
| warm-container-plusone | 8477ns | 8477ns | -2.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8477ns | base | --- | [8433, 8610] | --- | --- | --- | --- |
| warm-container-kernel | 2450ns | -5903.1ns (-69.6%) | [-6044, -5834]ns | [2418, 2704] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 2418ns | -6031.1ns (-71.1%) | [-6124, -5935]ns | [2415, 2420] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8582ns | no significant difference | [-60, +402]ns | [8432, 8860] | no | 0.2682 | 0.2682 | 0 |
| warm-container-native | 2472ns | -5949.9ns (-70.2%) | [-6085, -5882]ns | [2460, 2553] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8354ns | no significant difference | [-231, +41]ns | [8294, 8411] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8472ns | -67.7% | -71.5% | -1.4% | -67.7% | -2.3% |
| 2 | 8304ns | -67.0% | -70.9% | +3.4% | -70.4% | +0.4% |
| 3 | 8310ns | -67.0% | -71.0% | +1.8% | -70.4% | +0.6% |
| 4 | 8305ns | -67.0% | -70.9% | +1.3% | -70.0% | -0.3% |
| 5 | 8368ns | -67.2% | -71.1% | +12.2% | -69.5% | -1.0% |
| 6 | 8473ns | -67.7% | -71.5% | +10.9% | -69.7% | -2.1% |
| 7 | 8569ns | -68.0% | -71.8% | +10.5% | -62.0% | -2.4% |
| 8 | 8304ns | -67.8% | -70.9% | +13.9% | -70.8% | +0.7% |
| 9 | 8320ns | -70.6% | -71.0% | +12.9% | -70.7% | +0.3% |
| 10 | 8341ns | -71.0% | -71.1% | +6.1% | -70.9% | +0.7% |
| 11 | 8482ns | -67.7% | -71.1% | -1.5% | -71.0% | -2.3% |
| 12 | 8438ns | -70.9% | -66.7% | -0.3% | -70.8% | -1.8% |
| 13 | 8672ns | -71.9% | -70.2% | -1.1% | -71.6% | -4.4% |
| 14 | 8438ns | -70.9% | -71.2% | +3.3% | -70.4% | +1.3% |
| 15 | 8580ns | -68.0% | -71.5% | +7.3% | -71.4% | -1.6% |
| 16 | 8502ns | -67.8% | -70.9% | +0.2% | -71.1% | +2.6% |
| 17 | 8641ns | -68.3% | -71.3% | +1.2% | -71.3% | +1.7% |
| 18 | 8515ns | -67.8% | -71.6% | -1.6% | -70.0% | +2.3% |
| 19 | 8428ns | -67.5% | -71.3% | +5.3% | -70.8% | +4.6% |
| 20 | 8355ns | -67.2% | -71.1% | +14.5% | -71.0% | +2.8% |
| 21 | 9420ns | -74.3% | -74.4% | +6.7% | -70.9% | -12.0% |
| 22 | 9510ns | -74.6% | -74.6% | -8.1% | -71.1% | -11.0% |
| 23 | 9420ns | -74.4% | -74.3% | -12.0% | -70.8% | -12.1% |
| 24 | 9422ns | -74.3% | -74.3% | -12.1% | -70.1% | -12.1% |
| 25 | 9418ns | -74.4% | -74.3% | -12.0% | -70.9% | -12.0% |
| 26 | 9485ns | -74.5% | -74.5% | -1.0% | -71.0% | -12.6% |
| 27 | 8785ns | -72.5% | -67.5% | +7.2% | -68.7% | -5.7% |
| 28 | 8439ns | -71.4% | -69.5% | +11.4% | -67.5% | -1.4% |
| 29 | 8540ns | -71.0% | -71.6% | +10.2% | -67.9% | -3.0% |
| 30 | 8453ns | -70.8% | -71.4% | -0.3% | -67.5% | -1.7% |
| 31 | 8309ns | -70.9% | -70.9% | +10.8% | -70.8% | +1.2% |
| 32 | 8307ns | -70.9% | -70.9% | +4.3% | -70.8% | +1.2% |
| 33 | 8897ns | -72.8% | -72.6% | -4.4% | -72.6% | -4.8% |
| 34 | 9157ns | -73.6% | -73.6% | -7.8% | -73.6% | -7.5% |
| 35 | 8320ns | -71.0% | -71.0% | +1.2% | -70.2% | +3.9% |
| 36 | 8313ns | -70.9% | -70.9% | +1.2% | -70.3% | +2.3% |
| 37 | 8340ns | -70.0% | -71.0% | +0.4% | -70.5% | +2.6% |
| 38 | 8732ns | -72.4% | -72.3% | -5.1% | -71.9% | -4.5% |
| 39 | 8740ns | -72.3% | -72.4% | -4.4% | -71.9% | -5.2% |
| 40 | 8746ns | -72.4% | -72.3% | -3.6% | -71.6% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.710 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.717 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.311 | moderate+ |
| warm-container-minimum | 0.523 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.353 | moderate+ |
| warm-container-plusone | 0.695 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 16/40, lost 24/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 24/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.9ns | 8639.3ns | 0.0% |  |
| warm-container-kernel | 2.5ns | 2544.3ns | 0.1% |  |
| warm-container-lanes-deferred | 2.5ns | 2450.9ns | 0.1% |  |
| warm-container-minimum | 3.2ns | 8798.0ns | 0.0% |  |
| warm-container-native | 3.0ns | 2563.9ns | 0.1% |  |
| warm-container-plusone | 3.4ns | 8408.6ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8309.0-9341.2 ns)
   8309.0 |########################################
   8360.6 |#####
   8412.2 |############################
   8463.9 |############################
   8515.5 |#####
   8567.1 |###########
   8618.7 |#####
   8670.3 |#####
   8721.9 |#################
   8773.5 |#####
   8825.1 |
   8876.7 |#####
   8928.3 |
   8979.9 |
   9031.5 |
   9083.2 |
   9134.8 |#####
   9186.4 |
   9238.0 |
   9289.6 |
  (5 below, 6 above range)

warm-container-kernel (n=40, range 2414.7-2741.3 ns)
   2414.7 |########################################
   2431.0 |##
   2447.3 |########
   2463.7 |#####
   2480.0 |
   2496.3 |##
   2512.7 |
   2529.0 |
   2545.3 |
   2561.7 |
   2578.0 |
   2594.3 |
   2610.6 |
   2627.0 |
   2643.3 |
   2659.6 |##
   2676.0 |
   2692.3 |
   2708.6 |
   2725.0 |##################################
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 2413.2-2584.4 ns)
   2413.2 |########################################
   2421.7 |#
   2430.3 |###
   2438.9 |#
   2447.4 |#
   2456.0 |
   2464.5 |
   2473.1 |###
   2481.7 |
   2490.2 |
   2498.8 |
   2507.3 |
   2515.9 |
   2524.5 |
   2533.0 |
   2541.6 |
   2550.1 |
   2558.7 |
   2567.3 |
   2575.8 |###
  (4 below, 2 above range)

warm-container-minimum (n=40, range 8324.5-9520.1 ns)
   8324.5 |############################
   8384.3 |########################################
   8444.1 |###########
   8503.9 |#####
   8563.6 |###########
   8623.4 |#####
   8683.2 |###########
   8743.0 |#####
   8802.7 |#####
   8862.5 |#####
   8922.3 |
   8982.1 |
   9041.8 |
   9101.6 |
   9161.4 |###########
   9221.2 |
   9280.9 |
   9340.7 |############################
   9400.5 |#################
   9460.3 |#####
  (4 below, 2 above range)

warm-container-native (n=40, range 2427.1-2819.1 ns)
   2427.1 |##########
   2446.7 |########################################
   2466.3 |##############
   2485.9 |#######
   2505.5 |
   2525.1 |
   2544.7 |#######
   2564.3 |###
   2583.9 |
   2603.5 |
   2623.1 |
   2642.7 |
   2662.3 |
   2681.9 |
   2701.5 |
   2721.1 |###
   2740.7 |################################
   2760.3 |
   2779.9 |
   2799.5 |###
  (5 below, 1 above range)

warm-container-plusone (n=40, range 8283.8-8672.1 ns)
   8283.8 |########################################
   8303.2 |######
   8322.6 |######
   8342.0 |##########
   8361.4 |######
   8380.9 |###
   8400.3 |######
   8419.7 |
   8439.1 |###
   8458.5 |##########
   8477.9 |
   8497.4 |###
   8516.8 |
   8536.2 |###
   8555.6 |###
   8575.0 |###
   8594.5 |
   8613.9 |
   8633.3 |###
   8652.7 |
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.69 (measurement drift or warm-up artifact)
