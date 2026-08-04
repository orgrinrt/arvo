# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1269% faster than the next best (quantiser-software)

quantiser-hardware (292 ns) leads quantiser-software (4.00 us) by 1269%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows warm-up / thermal drift (autocorr +0.79)

quantiser-hardware's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (292 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 13.7x the fastest

Fastest quantiser-hardware (292 ns) to slowest quantiser-software (4.00 us): 13.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 292.1 ns median
- 1 variant significantly slower than baseline
- Spread: 13.69x (fastest 292.1 ns, slowest 4000.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 361ns | 362ns | 345ns | 360ns | 377ns | base |
| quantiser-software | 4152ns | 4079ns | 3712ns | 4091ns | 4776ns | +1050.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 290ns | 277ns | 301ns | base | 0.882 |
| quantiser-software | 4070ns | 3644ns | 4670ns | +1301.60% | 0.063 |

## Performance model

- Peak throughput: **0.924 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.876 | 94.8% |
| quantiser-software | 0.064 | 6.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 361ns | 361ns | base |
| quantiser-software | 4152ns | 4152ns | +1050.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 292ns | base | --- | [283, 299] | --- | --- | --- | --- |
| quantiser-software | 4000ns | +3705.4ns (+1268.6%) | [+3701, +3714]ns | [3988, 4012] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 298ns | +1242.2% |
| 2 | 301ns | +2267.2% |
| 3 | 302ns | +1226.6% |
| 4 | 303ns | +1222.7% |
| 5 | 300ns | +1244.4% |
| 6 | 300ns | +1236.1% |
| 7 | 297ns | +1247.9% |
| 8 | 298ns | +1242.7% |
| 9 | 299ns | +1238.8% |
| 10 | 301ns | +1229.8% |
| 11 | 276ns | +1175.5% |
| 12 | 281ns | +1154.0% |
| 13 | 282ns | +1147.8% |
| 14 | 279ns | +1184.6% |
| 15 | 281ns | +1178.9% |
| 16 | 282ns | +1151.0% |
| 17 | 275ns | +1370.6% |
| 18 | 274ns | +1387.4% |
| 19 | 281ns | +1370.6% |
| 20 | 286ns | +1324.4% |
| 21 | 298ns | +1438.3% |
| 22 | 298ns | +1302.6% |
| 23 | 302ns | +1224.8% |
| 24 | 300ns | +1234.7% |
| 25 | 299ns | +1387.0% |
| 26 | 301ns | +1296.4% |
| 27 | 300ns | +1332.2% |
| 28 | 300ns | +1280.2% |
| 29 | 300ns | +1223.9% |
| 30 | 300ns | +1200.8% |
| 31 | 282ns | +1310.6% |
| 32 | 285ns | +1300.9% |
| 33 | 278ns | +1335.0% |
| 34 | 272ns | +1366.5% |
| 35 | 281ns | +1317.9% |
| 36 | 287ns | +1289.2% |
| 37 | 285ns | +1296.4% |
| 38 | 283ns | +1308.2% |
| 39 | 283ns | +1448.3% |
| 40 | 282ns | +1308.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.792 | HIGH+ (drift/warm-up) |
| quantiser-software | 0.100 | ok |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.7ns | 290.3ns | 0.9% |  |
| quantiser-software | 2.9ns | 4069.5ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 277.0-301.4 ns)
    277.0 |######
    278.2 |######
    279.5 |
    280.7 |#################################
    281.9 |##########################
    283.1 |######
    284.3 |#############
    285.6 |######
    286.8 |######
    288.0 |
    289.2 |
    290.5 |
    291.7 |
    292.9 |
    294.1 |
    295.3 |
    296.6 |######
    297.8 |########################################
    299.0 |########################################
    300.2 |##########################
  (4 below, 3 above range)

quantiser-software (n=40, range 3643.9-4669.9 ns)
   3643.9 |
   3695.2 |
   3746.5 |
   3797.8 |
   3849.1 |
   3900.4 |##
   3951.7 |########################################
   4003.0 |######################
   4054.3 |#####
   4105.6 |#####
   4156.9 |#####
   4208.2 |
   4259.5 |##
   4310.8 |
   4362.1 |##
   4413.4 |##
   4464.7 |
   4516.0 |
   4567.3 |##
   4618.6 |
  (6 below, 1 above range)

```

## Diagnostics

- **quantiser-hardware**: autocorrelation=0.79 (measurement drift or warm-up artifact)
