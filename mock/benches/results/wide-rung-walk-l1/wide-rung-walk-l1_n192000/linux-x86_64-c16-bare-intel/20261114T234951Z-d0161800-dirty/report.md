# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (3.24 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 2.61 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged-overread dominates: 14% faster than the next best (wide-rung-wordround)

wide-rung-ragged-overread (2.61 us) leads wide-rung-wordround (2.97 us) by 14%, a clear separation rather than a photo finish. CV 45.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged-overread is fastest but the noisiest (CV 45.3%)

wide-rung-ragged-overread wins on median (2.61 us) yet has the highest variance (CV 45.3%), while wide-rung-ragged is the steadiest (CV 25.8%, 3.02 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.80)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (632 ns) is smaller than the fastest variant's own run-to-run std-dev (1.18 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-wordround is inconsistent: worst-20% is 4.3x its best-20%

wide-rung-wordround's best 20% of batches run at 855 ns but its worst 20% at 3.65 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 2605.1 ns median (-19.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.24x (fastest 2605.1 ns, slowest 3237.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3143ns | 3466ns | 1350ns | 3378ns | 4233ns | base |
| wide-rung-ragged | 3023ns | 3272ns | 1503ns | 3260ns | 3830ns | -3.83% |
| wide-rung-ragged-overread | 2678ns | 2826ns | 1068ns | 2611ns | 4489ns | -14.80% |
| wide-rung-wordround | 2844ns | 3219ns | 927ns | 3087ns | 4032ns | -9.51% |
| wide-rung-wordround-alias | 2925ns | 3301ns | 1071ns | 3200ns | 3951ns | -6.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2924ns | 1261ns | 3906ns | base | 0.700 |
| wide-rung-ragged | 2773ns | 1385ns | 3485ns | -5.14% | 0.738 |
| wide-rung-ragged-overread | 2449ns | 984ns | 4087ns | -16.25% | 0.836 |
| wide-rung-wordround | 2605ns | 855ns | 3648ns | -10.92% | 0.786 |
| wide-rung-wordround-alias | 2688ns | 986ns | 3614ns | -8.05% | 0.762 |

## Performance model

- Peak throughput: **2.396 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.633 | 26.4% |
| wide-rung-ragged | 0.679 | 28.3% |
| wide-rung-ragged-overread | 0.786 | 32.8% |
| wide-rung-wordround | 0.690 | 28.8% |
| wide-rung-wordround-alias | 0.677 | 28.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3143ns | 3143ns | base |
| wide-rung-ragged | 3023ns | 3023ns | -3.83% |
| wide-rung-ragged-overread | 2678ns | 2678ns | -14.80% |
| wide-rung-wordround | 2844ns | 2844ns | -9.51% |
| wide-rung-wordround-alias | 2925ns | 2925ns | -6.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3237ns | base | --- | [2963, 3460] | --- | --- | --- | --- |
| wide-rung-ragged | 3016ns | -203.2ns (-6.3%) | [-362, -30]ns | [2879, 3166] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-ragged-overread | 2605ns | -492.5ns (-15.2%) | [-766, -21]ns | [1868, 2950] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| wide-rung-wordround | 2968ns | -311.1ns (-9.6%) | [-370, -187]ns | [2759, 3176] | YES | 0.0014 | 0.0007 | 0 |
| wide-rung-wordround-alias | 3027ns | -209.0ns (-6.5%) | [-316, -145]ns | [2670, 3349] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 2865ns | +1.3% | -59.9% | -67.7% | -6.8% |
| 1 | 2 | 3237ns | +0.5% | -21.6% | -4.9% | -2.5% |
| 1 | 3 | 3464ns | -16.7% | -72.3% | -16.0% | +0.1% |
| 1 | 4 | 3505ns | -10.3% | -9.5% | -9.8% | -10.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.205 | moderate+ |
| wide-rung-ragged | -0.796 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.653 | HIGH- (thermal bounce) |
| wide-rung-wordround | -0.123 | ok |
| wide-rung-wordround-alias | 0.028 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 60.0ns | 2923.7ns | 2.1% |  |
| wide-rung-ragged | 68.2ns | 2773.5ns | 2.5% |  |
| wide-rung-ragged-overread | 61.2ns | 2448.6ns | 2.5% |  |
| wide-rung-wordround | 64.9ns | 2604.5ns | 2.5% |  |
| wide-rung-wordround-alias | 65.7ns | 2688.4ns | 2.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1261.2-3906.0 ns)
   1261.2 |
   1393.5 |
   1525.7 |#############
   1657.9 |
   1790.2 |
   1922.4 |######
   2054.7 |
   2186.9 |
   2319.1 |
   2451.4 |
   2583.6 |######
   2715.9 |####################
   2848.1 |######
   2980.3 |####################
   3112.6 |########################################
   3244.8 |#############
   3377.1 |##########################
   3509.3 |####################
   3641.5 |#############
   3773.8 |####################
  (6 below, 3 above range)

wide-rung-ragged (n=40, range 1385.5-3485.2 ns)
   1385.5 |#####
   1490.4 |
   1595.4 |
   1700.4 |
   1805.4 |
   1910.4 |
   2015.4 |#####
   2120.4 |
   2225.3 |
   2330.3 |
   2435.3 |#####
   2540.3 |
   2645.3 |######################
   2750.3 |#####
   2855.3 |########################################
   2960.3 |
   3065.2 |##################################
   3170.2 |############################
   3275.2 |#####
   3380.2 |##################################
  (5 below, 2 above range)

wide-rung-ragged-overread (n=40, range 984.0-4087.3 ns)
    984.0 |
   1139.2 |########################################
   1294.3 |
   1449.5 |######
   1604.7 |
   1759.8 |
   1915.0 |
   2070.2 |
   2225.3 |#############
   2380.5 |####################
   2535.7 |#############
   2690.8 |#############
   2846.0 |####################
   3001.1 |####################
   3156.3 |####################
   3311.5 |
   3466.6 |####################
   3621.8 |
   3777.0 |######
   3932.1 |#############
  (7 below, 2 above range)

wide-rung-wordround (n=40, range 854.6-3648.1 ns)
    854.6 |##########################
    994.3 |######
   1134.0 |
   1273.6 |
   1413.3 |
   1553.0 |
   1692.7 |
   1832.3 |
   1972.0 |######
   2111.7 |
   2251.4 |
   2391.0 |
   2530.7 |#############
   2670.4 |######
   2810.1 |########################################
   2949.7 |#############
   3089.4 |########################################
   3229.1 |######
   3368.8 |########################################
   3508.4 |#############
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 985.5-3614.1 ns)
    985.5 |
   1117.0 |#####
   1248.4 |
   1379.8 |
   1511.2 |#####
   1642.7 |
   1774.1 |
   1905.5 |
   2037.0 |
   2168.4 |
   2299.8 |#####
   2431.2 |#################
   2562.7 |#################
   2694.1 |
   2825.5 |######################
   2957.0 |
   3088.4 |############################
   3219.8 |#####
   3351.2 |########################################
   3482.7 |############################
  (7 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=31.9% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **wide-rung-ragged**: CV=28.0% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=48.2% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **wide-rung-wordround**: CV=40.4% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=35.7% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.7x (possible bimodal distribution)
