# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1419% faster than the next best (quantiser-software)

quantiser-hardware (286 ns) leads quantiser-software (4.34 us) by 1419%, a clear separation rather than a photo finish. CV 16.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware is fastest but the noisiest (CV 16.0%)

quantiser-hardware wins on median (286 ns) yet has the highest variance (CV 16.0%), while quantiser-software is the steadiest (CV 4.0%, 4.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-software shows warm-up / thermal drift (autocorr +0.72)

quantiser-software's per-pass series has lag-1 autocorrelation +0.72, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (286 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.2x the fastest

Fastest quantiser-hardware (286 ns) to slowest quantiser-software (4.34 us): 15.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 285.8 ns median
- 1 variant significantly slower than baseline
- Spread: 15.19x (fastest 285.8 ns, slowest 4341.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 369ns | 354ns | 350ns | 356ns | 426ns | base |
| quantiser-software | 4365ns | 4417ns | 4070ns | 4389ns | 4591ns | +1083.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 297ns | 283ns | 339ns | base | 0.863 |
| quantiser-software | 4288ns | 3999ns | 4497ns | +1345.51% | 0.060 |

## Performance model

- Peak throughput: **0.904 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.896 | 99.0% |
| quantiser-software | 0.059 | 6.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 369ns | 369ns | base |
| quantiser-software | 4365ns | 4365ns | +1083.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 286ns | base | --- | [284, 287] | --- | --- | --- | --- |
| quantiser-software | 4341ns | +4025.4ns (+1408.5%) | [+3991, +4066]ns | [4281, 4367] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 284ns | +1312.3% |
| 2 | 283ns | +1318.6% |
| 3 | 285ns | +1311.3% |
| 4 | 287ns | +1371.5% |
| 5 | 282ns | +1421.9% |
| 6 | 283ns | +1418.1% |
| 7 | 286ns | +1403.5% |
| 8 | 284ns | +1408.6% |
| 9 | 282ns | +1415.8% |
| 10 | 287ns | +1393.3% |
| 11 | 300ns | +1355.7% |
| 12 | 578ns | +655.8% |
| 13 | 307ns | +1335.2% |
| 14 | 307ns | +1321.8% |
| 15 | 303ns | +1342.3% |
| 16 | 303ns | +1341.2% |
| 17 | 305ns | +1327.5% |
| 18 | 304ns | +1337.4% |
| 19 | 306ns | +1329.4% |
| 20 | 305ns | +1352.7% |
| 21 | 284ns | +1442.0% |
| 22 | 288ns | +1425.6% |
| 23 | 287ns | +1427.1% |
| 24 | 289ns | +1496.0% |
| 25 | 286ns | +1426.6% |
| 26 | 288ns | +1416.0% |
| 27 | 286ns | +1482.7% |
| 28 | 284ns | +1516.7% |
| 29 | 285ns | +1485.1% |
| 30 | 285ns | +1486.3% |
| 31 | 285ns | +1298.0% |
| 32 | 283ns | +1306.5% |
| 33 | 286ns | +1294.5% |
| 34 | 284ns | +1326.1% |
| 35 | 284ns | +1304.8% |
| 36 | 284ns | +1307.9% |
| 37 | 283ns | +1425.8% |
| 38 | 284ns | +1398.7% |
| 39 | 285ns | +1401.2% |
| 40 | 286ns | +1390.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.097 | ok |
| quantiser-software | 0.721 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.1ns | 296.6ns | 0.7% |  |
| quantiser-software | 2.2ns | 4287.7ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 283.1-339.4 ns)
    283.1 |########################################
    285.9 |##########
    288.7 |##
    291.5 |
    294.3 |
    297.1 |
    300.0 |##
    302.8 |##########
    305.6 |######
    308.4 |
    311.2 |
    314.1 |
    316.9 |
    319.7 |
    322.5 |
    325.3 |
    328.2 |
    331.0 |
    333.8 |
    336.6 |
  (4 below, 1 above range)

quantiser-software (n=40, range 3998.8-4497.2 ns)
   3998.8 |################
   4023.7 |####
   4048.6 |
   4073.5 |
   4098.5 |
   4123.4 |
   4148.3 |
   4173.2 |
   4198.1 |####
   4223.1 |
   4248.0 |############
   4272.9 |########################
   4297.8 |####
   4322.7 |
   4347.7 |########################################
   4372.6 |############
   4397.5 |####
   4422.4 |####
   4447.3 |
   4472.3 |
  (4 below, 5 above range)

```

## Diagnostics

- **quantiser-software**: autocorrelation=0.72 (measurement drift or warm-up artifact)
