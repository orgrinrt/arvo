# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (2.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 1.86 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged dominates: 23% faster than the next best (wide-rung-ragged-overread)

wide-rung-ragged (1.86 us) leads wide-rung-ragged-overread (2.29 us) by 23%, a clear separation rather than a photo finish. CV 101.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged is fastest but the noisiest (CV 101.5%)

wide-rung-ragged wins on median (1.86 us) yet has the highest variance (CV 101.5%), while wide-rung-wordround is the steadiest (CV 85.5%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.74)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (440 ns) is smaller than the fastest variant's own run-to-run std-dev (1.88 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-wordround's comparison is tie-heavy (12% tied pairs)

12% of paired samples for wide-rung-wordround are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### wide-rung-align16 is inconsistent: worst-20% is 4.0x its best-20%

wide-rung-align16's best 20% of batches run at 1.87 us but its worst 20% at 7.59 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged** at 1857.8 ns median (-19.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.24x (fastest 1857.8 ns, slowest 2297.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4092ns | 2379ns | 1942ns | 3539ns | 7900ns | base |
| wide-rung-ragged | 3578ns | 1923ns | 1904ns | 3178ns | 6448ns | -12.56% |
| wide-rung-ragged-overread | 3565ns | 2352ns | 2087ns | 2749ns | 7493ns | -12.87% |
| wide-rung-wordround | 3532ns | 2374ns | 2051ns | 2805ns | 7194ns | -13.68% |
| wide-rung-wordround-alias | 3610ns | 2376ns | 1943ns | 2955ns | 7243ns | -11.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3939ns | 1875ns | 7587ns | base | 0.520 |
| wide-rung-ragged | 3444ns | 1839ns | 6170ns | -12.58% | 0.595 |
| wide-rung-ragged-overread | 3459ns | 2030ns | 7252ns | -12.18% | 0.592 |
| wide-rung-wordround | 3398ns | 1980ns | 6887ns | -13.74% | 0.603 |
| wide-rung-wordround-alias | 3474ns | 1875ns | 6944ns | -11.80% | 0.589 |

## Performance model

- Peak throughput: **1.113 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.892 | 80.1% |
| wide-rung-ragged | 1.102 | 99.0% |
| wide-rung-ragged-overread | 0.895 | 80.3% |
| wide-rung-wordround | 0.894 | 80.3% |
| wide-rung-wordround-alias | 0.894 | 80.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4092ns | 4092ns | base |
| wide-rung-ragged | 3578ns | 3578ns | -12.56% |
| wide-rung-ragged-overread | 3565ns | 3565ns | -12.87% |
| wide-rung-wordround | 3532ns | 3532ns | -13.68% |
| wide-rung-wordround-alias | 3610ns | 3610ns | -11.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2297ns | base | --- | [2291, 5152] | --- | --- | --- | --- |
| wide-rung-ragged | 1858ns | -445.5ns (-19.4%) | [-465, -195]ns | [1850, 4916] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 2289ns | no significant difference | [-2, +138]ns | [2289, 2306] | no | 0.7493 | 0.6358 | 0 |
| wide-rung-wordround | 2292ns | no significant difference | [-102, +0]ns | [2291, 2305] | no | 0.0819 | 0.0410 | **5** (12%, HIGH) |
| wide-rung-wordround-alias | 2292ns | no significant difference | [-12, +0]ns | [2291, 2759] | no | 0.7493 | 0.7493 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 2089ns | +119.3% | +9.6% | -10.0% | -10.2% |
| 1 | 2 | 5152ns | -63.9% | -32.0% | -55.4% | -54.9% |
| 1 | 3 | 2298ns | +94.9% | +16.0% | -0.3% | +0.0% |
| 1 | 4 | 2297ns | -19.6% | -0.3% | -0.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.453 | moderate- |
| wide-rung-ragged | -0.739 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.339 | moderate- |
| wide-rung-wordround | -0.095 | ok |
| wide-rung-wordround-alias | -0.127 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 2/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 40.8ns | 3939.4ns | 1.0% |  |
| wide-rung-ragged | 46.2ns | 3443.7ns | 1.3% |  |
| wide-rung-ragged-overread | 35.8ns | 3459.5ns | 1.0% |  |
| wide-rung-wordround | 35.3ns | 3398.1ns | 1.0% |  |
| wide-rung-wordround-alias | 36.1ns | 3474.4ns | 1.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1874.8-7586.9 ns)
   1874.8 |###########
   2160.4 |########################################
   2446.0 |
   2731.6 |##
   3017.2 |
   3302.8 |
   3588.4 |
   3874.0 |
   4159.6 |
   4445.3 |##
   4730.9 |
   5016.5 |#####
   5302.1 |##
   5587.7 |##
   5873.3 |##
   6158.9 |##
   6444.5 |##
   6730.1 |#####
   7015.7 |##
   7301.3 |##
  (5 below, 4 above range)

wide-rung-ragged (n=40, range 1839.3-6170.4 ns)
   1839.3 |########################################
   2055.9 |
   2272.4 |
   2489.0 |
   2705.5 |##
   2922.1 |
   3138.7 |
   3355.2 |
   3571.8 |
   3788.3 |##
   4004.9 |##
   4221.4 |
   4438.0 |
   4654.5 |##
   4871.1 |
   5087.6 |####
   5304.2 |####
   5520.7 |########
   5737.3 |
   5953.8 |######
  (2 below, 3 above range)

wide-rung-ragged-overread (n=40, range 2030.3-7251.8 ns)
   2030.3 |########################################
   2291.4 |########
   2552.4 |
   2813.5 |##
   3074.6 |
   3335.7 |
   3596.8 |
   3857.8 |
   4118.9 |
   4380.0 |##
   4641.1 |
   4902.1 |##
   5163.2 |
   5424.3 |##
   5685.4 |
   5946.4 |##
   6207.5 |####
   6468.6 |
   6729.7 |
   6990.8 |##
  (5 below, 4 above range)

wide-rung-wordround (n=40, range 1980.2-6886.9 ns)
   1980.2 |
   2225.5 |########################################
   2470.8 |
   2716.2 |#
   2961.5 |
   3206.8 |
   3452.2 |
   3697.5 |
   3942.9 |
   4188.2 |
   4433.5 |
   4678.9 |
   4924.2 |#
   5169.5 |
   5414.9 |###
   5660.2 |#
   5905.6 |
   6150.9 |###
   6396.2 |
   6641.6 |
  (6 below, 5 above range)

wide-rung-wordround-alias (n=40, range 1874.9-6943.7 ns)
   1874.9 |###########################
   2128.4 |########################################
   2381.8 |
   2635.2 |###
   2888.7 |
   3142.1 |
   3395.5 |
   3649.0 |
   3902.4 |
   4155.9 |
   4409.3 |###
   4662.7 |######
   4916.2 |
   5169.6 |
   5423.1 |
   5676.5 |
   5929.9 |######
   6183.4 |###
   6436.8 |###
   6690.3 |#########
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=58.5% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **wide-rung-ragged**: CV=54.7% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=60.3% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **wide-rung-wordround**: CV=57.7% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=59.0% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.7x (possible bimodal distribution)
