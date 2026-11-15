# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 15% faster than the next best (warm-clamp-accfit)

warm-clamp-min-lanes (331 ns) leads warm-clamp-accfit (380 ns) by 15%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 23% (significant)

warm-clamp-min-lanes is -101 ns (23%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 15.4x slower than the field

warm-clamp-minimum (5.09 us) is 15.4x the fastest (331 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.85)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} vs {warm-clamp-minimum} (959% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} and a slow tier {warm-clamp-minimum} with a 959% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 15.4x the fastest

Fastest warm-clamp-min-lanes (331 ns) to slowest warm-clamp-minimum (5.09 us): 15.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 330.6 ns median (-23.2% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 15.40x (fastest 330.6 ns, slowest 5091.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 496ns | 499ns | 442ns | 498ns | 544ns | base |
| warm-clamp-accfit | 470ns | 441ns | 438ns | 453ns | 554ns | -5.15% |
| warm-clamp-accfit-dyn | 457ns | 448ns | 444ns | 450ns | 489ns | -7.92% |
| warm-clamp-head | 578ns | 545ns | 516ns | 564ns | 683ns | +16.53% |
| warm-clamp-min-lanes | 394ns | 395ns | 389ns | 395ns | 399ns | -20.48% |
| warm-clamp-minimum | 5533ns | 5159ns | 5036ns | 5332ns | 6634ns | +1015.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 428ns | 385ns | 469ns | base | 19.152 |
| warm-clamp-accfit | 406ns | 379ns | 478ns | -5.15% | 20.192 |
| warm-clamp-accfit-dyn | 393ns | 383ns | 421ns | -8.04% | 20.828 |
| warm-clamp-head | 510ns | 455ns | 603ns | +19.27% | 16.058 |
| warm-clamp-min-lanes | 329ns | 325ns | 333ns | -23.03% | 24.884 |
| warm-clamp-minimum | 5460ns | 4971ns | 6551ns | +1176.50% | 1.500 |

## Performance model

- Peak throughput: **25.187 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 19.025 | 75.5% |
| warm-clamp-accfit | 21.535 | 85.5% |
| warm-clamp-accfit-dyn | 21.223 | 84.3% |
| warm-clamp-head | 17.031 | 67.6% |
| warm-clamp-min-lanes | 24.779 | 98.4% |
| warm-clamp-minimum | 1.609 | 6.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 496ns | 496ns | base |
| warm-clamp-accfit | 470ns | 470ns | -5.15% |
| warm-clamp-accfit-dyn | 457ns | 457ns | -7.92% |
| warm-clamp-head | 578ns | 578ns | +16.53% |
| warm-clamp-min-lanes | 394ns | 394ns | -20.48% |
| warm-clamp-minimum | 5533ns | 5533ns | +1015.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 431ns | base | --- | [399, 460] | --- | --- | --- | --- |
| warm-clamp-accfit | 380ns | -9.2ns (-2.1%) | [-79, -5]ns | [380, 382] | YES | 0.0064 | 0.0064 | 0 |
| warm-clamp-accfit-dyn | 386ns | -18.2ns (-4.2%) | [-60, -10]ns | [385, 391] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 481ns | +94.6ns (+22.0%) | [+81, +119]ns | [465, 518] | YES | 0.0028 | 0.0022 | 0 |
| warm-clamp-min-lanes | 331ns | -101.0ns (-23.5%) | [-128, -72]ns | [327, 331] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 5092ns | +4704.8ns (+1092.6%) | [+4655, +4856]ns | [5053, 5248] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 459ns | -17.0% | -16.3% | +29.1% | -27.6% | +983.1% |
| 2 | 460ns | -17.2% | -16.5% | +31.7% | -27.9% | +981.0% |
| 3 | 460ns | -17.5% | -16.4% | +31.0% | -27.8% | +993.6% |
| 4 | 462ns | -17.2% | -16.7% | +31.2% | -28.3% | +1318.8% |
| 5 | 461ns | -17.4% | -16.4% | +30.8% | -28.1% | +1321.9% |
| 6 | 460ns | -17.5% | -8.8% | +30.0% | -28.1% | +1322.2% |
| 7 | 461ns | -17.5% | -5.4% | +30.2% | -27.8% | +1320.9% |
| 8 | 460ns | -17.4% | -5.5% | +31.6% | -28.0% | +1322.7% |
| 9 | 458ns | -16.8% | -5.1% | +31.2% | -27.5% | +1333.0% |
| 10 | 459ns | -17.2% | -4.5% | +31.4% | -27.9% | +1213.2% |
| 11 | 401ns | +14.6% | -2.6% | +13.4% | -18.4% | +1164.0% |
| 12 | 403ns | +14.1% | -3.1% | +29.5% | -18.6% | +1196.3% |
| 13 | 400ns | +14.9% | -0.6% | +28.8% | -18.3% | +1167.6% |
| 14 | 401ns | +14.7% | -0.4% | +29.0% | -19.0% | +1211.4% |
| 15 | 400ns | +15.0% | -4.0% | +29.5% | -18.0% | +1299.1% |
| 16 | 395ns | +16.1% | -2.5% | +30.8% | -16.8% | +1224.3% |
| 17 | 388ns | +18.7% | -0.6% | +33.1% | -16.1% | +1246.3% |
| 18 | 399ns | +37.6% | -3.3% | +30.1% | -18.2% | +1250.6% |
| 19 | 398ns | +15.7% | -3.4% | +30.0% | -18.0% | +1168.9% |
| 20 | 400ns | +15.2% | -3.9% | +29.3% | -18.2% | +1164.6% |
| 21 | 382ns | -0.7% | +0.1% | +22.6% | -15.0% | +1271.3% |
| 22 | 387ns | -1.9% | -0.1% | +21.2% | -13.8% | +1206.6% |
| 23 | 388ns | -2.0% | -1.2% | +23.7% | -15.8% | +1203.4% |
| 24 | 383ns | -1.0% | +0.1% | +21.4% | -14.5% | +1228.2% |
| 25 | 386ns | -1.5% | -0.6% | +24.6% | -15.7% | +1186.9% |
| 26 | 385ns | -1.0% | -0.2% | +20.5% | -15.4% | +1191.1% |
| 27 | 387ns | -2.0% | -1.1% | +24.3% | -15.6% | +1189.7% |
| 28 | 385ns | -1.8% | -0.3% | +20.8% | -15.7% | +1196.1% |
| 29 | 390ns | -2.7% | -1.5% | +22.8% | -16.3% | +1185.3% |
| 30 | 387ns | -1.8% | -1.0% | +20.0% | -16.1% | +1216.9% |
| 31 | 462ns | +11.6% | -14.7% | -1.5% | -28.4% | +976.6% |
| 32 | 521ns | -26.0% | -25.8% | -12.2% | -36.4% | +855.2% |
| 33 | 460ns | -16.4% | -15.7% | -0.8% | -28.1% | +1248.1% |
| 34 | 460ns | -17.5% | -14.9% | -1.2% | -28.1% | +1322.9% |
| 35 | 459ns | -17.2% | -16.1% | -0.3% | -27.6% | +1326.7% |
| 36 | 458ns | -17.3% | -15.7% | -0.5% | -27.7% | +1220.7% |
| 37 | 462ns | -17.9% | -13.9% | -1.4% | -27.9% | +976.4% |
| 38 | 460ns | -17.4% | -12.1% | -1.0% | -27.4% | +981.1% |
| 39 | 461ns | -17.6% | -14.0% | -1.3% | -28.2% | +996.5% |
| 40 | 462ns | -18.5% | -15.0% | -1.4% | -28.1% | +1017.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.827 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.583 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.770 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.846 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.592 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.756 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 29/40, lost 11/40
- **warm-clamp-accfit-dyn**: won 38/40, lost 2/40
- **warm-clamp-head**: won 10/40, lost 30/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.9ns | 427.7ns | 0.7% |  |
| warm-clamp-accfit | 2.9ns | 405.7ns | 0.7% |  |
| warm-clamp-accfit-dyn | 2.9ns | 393.3ns | 0.7% |  |
| warm-clamp-head | 2.8ns | 510.2ns | 0.6% |  |
| warm-clamp-min-lanes | 2.4ns | 329.2ns | 0.7% |  |
| warm-clamp-minimum | 3.0ns | 5460.1ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 385.2-468.9 ns)
    385.2 |####################
    389.4 |###
    393.6 |###
    397.8 |#######################
    401.9 |###
    406.1 |
    410.3 |
    414.5 |
    418.7 |
    422.9 |
    427.1 |
    431.2 |
    435.4 |
    439.6 |
    443.8 |
    448.0 |
    452.2 |
    456.3 |########################################
    460.5 |#######################
    464.7 |
  (4 below, 1 above range)

warm-clamp-accfit (n=40, range 378.7-478.3 ns)
    378.7 |########################################
    383.7 |###
    388.6 |
    393.6 |
    398.6 |
    403.6 |
    408.6 |
    413.5 |
    418.5 |
    423.5 |
    428.5 |
    433.5 |
    438.4 |
    443.4 |
    448.4 |
    453.4 |#
    458.4 |############
    463.3 |
    468.3 |
    473.3 |
  (2 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 383.2-420.7 ns)
    383.2 |########################################
    385.1 |############
    386.9 |##
    388.8 |
    390.7 |#######
    392.6 |#####
    394.4 |##
    396.3 |#####
    398.2 |##
    400.1 |
    401.9 |
    403.8 |##
    405.7 |
    407.6 |
    409.4 |
    411.3 |
    413.2 |
    415.1 |
    416.9 |
    418.8 |##
  (3 below, 4 above range)

warm-clamp-head (n=40, range 455.3-603.4 ns)
    455.3 |##############################
    462.7 |##############################
    470.1 |
    477.5 |####################
    484.9 |
    492.3 |
    499.7 |
    507.1 |
    514.5 |########################################
    521.9 |#####
    529.3 |
    536.7 |
    544.1 |
    551.6 |
    559.0 |
    566.4 |
    573.8 |
    581.2 |
    588.6 |#####
    596.0 |##############################
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 325.2-332.8 ns)
    325.2 |######
    325.6 |#############
    326.0 |####################
    326.4 |##########################
    326.8 |
    327.1 |####################
    327.5 |
    327.9 |
    328.3 |#############
    328.7 |
    329.0 |
    329.4 |
    329.8 |
    330.2 |######
    330.6 |####################
    330.9 |########################################
    331.3 |
    331.7 |#############
    332.1 |#############
    332.4 |##########################
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 4971.3-6550.6 ns)
   4971.3 |##########################
   5050.3 |########################################
   5129.2 |####
   5208.2 |######################
   5287.2 |
   5366.1 |####
   5445.1 |
   5524.1 |####
   5603.0 |
   5682.0 |
   5761.0 |
   5839.9 |
   5918.9 |
   5997.9 |########
   6076.8 |
   6155.8 |####
   6234.8 |
   6313.7 |
   6392.7 |
   6471.7 |######################
  (6 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.76 (measurement drift or warm-up artifact)
