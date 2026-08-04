# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1444% faster than the next best (quantiser-software)

quantiser-hardware (303 ns) leads quantiser-software (4.68 us) by 1444%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows warm-up / thermal drift (autocorr +0.84)

quantiser-hardware's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (303 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.4x the fastest

Fastest quantiser-hardware (303 ns) to slowest quantiser-software (4.68 us): 15.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 303.3 ns median
- 1 variant significantly slower than baseline
- Spread: 15.44x (fastest 303.3 ns, slowest 4682.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 370ns | 375ns | 352ns | 374ns | 379ns | base |
| quantiser-software | 4937ns | 4757ns | 4640ns | 4821ns | 5581ns | +1232.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 300ns | 285ns | 306ns | base | 0.854 |
| quantiser-software | 4852ns | 4571ns | 5445ns | +1518.96% | 0.053 |

## Performance model

- Peak throughput: **0.897 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.844 | 94.1% |
| quantiser-software | 0.055 | 6.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 370ns | 370ns | base |
| quantiser-software | 4937ns | 4937ns | +1232.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 303ns | base | --- | [302, 305] | --- | --- | --- | --- |
| quantiser-software | 4683ns | +4379.8ns (+1444.0%) | [+4293, +4609]ns | [4598, 4913] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 301ns | +1435.7% |
| 2 | 307ns | +1404.2% |
| 3 | 302ns | +1414.8% |
| 4 | 305ns | +1399.1% |
| 5 | 305ns | +1400.0% |
| 6 | 306ns | +1395.7% |
| 7 | 305ns | +1398.0% |
| 8 | 304ns | +1404.8% |
| 9 | 303ns | +1407.0% |
| 10 | 303ns | +1486.3% |
| 11 | 303ns | +1545.8% |
| 12 | 307ns | +1399.8% |
| 13 | 305ns | +1410.2% |
| 14 | 308ns | +1394.0% |
| 15 | 306ns | +1402.9% |
| 16 | 305ns | +1405.3% |
| 17 | 307ns | +1398.1% |
| 18 | 306ns | +1402.1% |
| 19 | 303ns | +1440.4% |
| 20 | 306ns | +1504.2% |
| 21 | 302ns | +1560.5% |
| 22 | 306ns | +1588.8% |
| 23 | 302ns | +1515.4% |
| 24 | 301ns | +1531.5% |
| 25 | 304ns | +1533.1% |
| 26 | 302ns | +1451.5% |
| 27 | 302ns | +1410.4% |
| 28 | 304ns | +1400.0% |
| 29 | 302ns | +1545.4% |
| 30 | 305ns | +1407.7% |
| 31 | 284ns | +1617.7% |
| 32 | 286ns | +1654.3% |
| 33 | 284ns | +2693.9% |
| 34 | 285ns | +1830.7% |
| 35 | 286ns | +1639.4% |
| 36 | 287ns | +1625.0% |
| 37 | 286ns | +1619.6% |
| 38 | 288ns | +1609.7% |
| 39 | 286ns | +1624.9% |
| 40 | 286ns | +1605.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.843 | HIGH+ (drift/warm-up) |
| quantiser-software | 0.325 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.5ns | 299.7ns | 0.8% |  |
| quantiser-software | 2.7ns | 4851.7ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 285.4-306.5 ns)
    285.4 |#########################
    286.4 |#####
    287.5 |#####
    288.5 |
    289.6 |
    290.7 |
    291.7 |
    292.8 |
    293.8 |
    294.9 |
    295.9 |
    297.0 |
    298.0 |
    299.1 |
    300.1 |
    301.2 |#########################
    302.2 |####################
    303.3 |##############################
    304.3 |###############
    305.4 |########################################
  (3 below, 4 above range)

quantiser-software (n=40, range 4571.0-5445.4 ns)
   4571.0 |########################################
   4614.8 |##
   4658.5 |#####
   4702.2 |
   4745.9 |
   4789.6 |##
   4833.4 |
   4877.1 |##################
   4920.8 |#####
   4964.5 |##########
   5008.2 |#####
   5052.0 |
   5095.7 |
   5139.4 |##
   5183.1 |
   5226.8 |
   5270.6 |
   5314.3 |
   5358.0 |
   5401.7 |
  (3 below, 2 above range)

```

## Diagnostics

- **quantiser-hardware**: autocorrelation=0.84 (measurement drift or warm-up artifact)
