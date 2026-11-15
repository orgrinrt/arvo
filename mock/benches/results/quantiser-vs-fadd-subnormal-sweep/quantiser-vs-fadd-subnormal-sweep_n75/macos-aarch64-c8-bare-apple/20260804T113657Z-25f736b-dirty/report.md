# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1213% faster than the next best (quantiser-software)

quantiser-hardware (314 ns) leads quantiser-software (4.12 us) by 1213%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows warm-up / thermal drift (autocorr +0.92)

quantiser-hardware's per-pass series has lag-1 autocorrelation +0.92, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (314 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 13.1x the fastest

Fastest quantiser-hardware (314 ns) to slowest quantiser-software (4.12 us): 13.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 313.8 ns median
- 1 variant significantly slower than baseline
- Spread: 13.13x (fastest 313.8 ns, slowest 4118.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 388ns | 388ns | 370ns | 388ns | 406ns | base |
| quantiser-software | 4279ns | 4199ns | 4055ns | 4252ns | 4587ns | +1003.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 314ns | 299ns | 328ns | base | 0.816 |
| quantiser-software | 4199ns | 3981ns | 4493ns | +1238.21% | 0.061 |

## Performance model

- Peak throughput: **0.855 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.816 | 95.4% |
| quantiser-software | 0.062 | 7.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 388ns | 388ns | base |
| quantiser-software | 4279ns | 4279ns | +1003.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 314ns | base | --- | [301, 326] | --- | --- | --- | --- |
| quantiser-software | 4118ns | +3803.9ns (+1212.4%) | [+3771, +3952]ns | [4072, 4278] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 300ns | +1233.4% |
| 2 | 299ns | +1236.3% |
| 3 | 301ns | +1226.8% |
| 4 | 299ns | +1237.4% |
| 5 | 300ns | +1231.7% |
| 6 | 299ns | +1235.2% |
| 7 | 299ns | +1235.2% |
| 8 | 300ns | +1230.2% |
| 9 | 301ns | +1226.2% |
| 10 | 301ns | +1317.2% |
| 11 | 301ns | +1357.9% |
| 12 | 302ns | +1266.9% |
| 13 | 299ns | +1257.9% |
| 14 | 300ns | +1255.6% |
| 15 | 301ns | +1252.4% |
| 16 | 300ns | +1257.2% |
| 17 | 300ns | +1260.3% |
| 18 | 302ns | +1249.6% |
| 19 | 302ns | +1247.8% |
| 20 | 301ns | +1254.6% |
| 21 | 326ns | +1363.9% |
| 22 | 328ns | +1228.1% |
| 23 | 329ns | +1227.6% |
| 24 | 329ns | +1224.7% |
| 25 | 325ns | +1240.7% |
| 26 | 325ns | +1240.4% |
| 27 | 328ns | +1229.1% |
| 28 | 326ns | +1236.2% |
| 29 | 327ns | +1235.5% |
| 30 | 329ns | +1229.7% |
| 31 | 328ns | +1100.4% |
| 32 | 325ns | +1111.2% |
| 33 | 327ns | +1157.4% |
| 34 | 325ns | +1214.8% |
| 35 | 327ns | +1209.6% |
| 36 | 326ns | +1212.8% |
| 37 | 328ns | +1203.2% |
| 38 | 328ns | +1204.8% |
| 39 | 327ns | +1206.9% |
| 40 | 328ns | +1406.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.921 | HIGH+ (drift/warm-up) |
| quantiser-software | 0.431 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.5ns | 313.8ns | 0.8% |  |
| quantiser-software | 3.0ns | 4198.8ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 299.2-328.5 ns)
    299.2 |##########################
    300.7 |########################################
    302.2 |
    303.6 |
    305.1 |
    306.6 |
    308.0 |
    309.5 |
    310.9 |
    312.4 |
    313.9 |
    315.3 |
    316.8 |
    318.2 |
    319.7 |
    321.2 |
    322.6 |
    324.1 |#################
    325.6 |#################
    327.0 |########################################
  (5 below, 3 above range)

quantiser-software (n=40, range 3980.9-4492.7 ns)
   3980.9 |########################################
   4006.5 |
   4032.1 |####
   4057.6 |###############################
   4083.2 |
   4108.8 |########
   4134.4 |
   4160.0 |
   4185.6 |
   4211.2 |
   4236.8 |
   4262.4 |###############################
   4288.0 |
   4313.6 |
   4339.2 |###############################
   4364.7 |########
   4390.3 |####
   4415.9 |
   4441.5 |
   4467.1 |
  (2 below, 2 above range)

```

## Diagnostics

- **quantiser-hardware**: autocorrelation=0.92 (measurement drift or warm-up artifact)
