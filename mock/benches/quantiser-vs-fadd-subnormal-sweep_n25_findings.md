# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1455% faster than the next best (quantiser-software)

quantiser-hardware (304 ns) leads quantiser-software (4.72 us) by 1455%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-software shows warm-up / thermal drift (autocorr +0.79)

quantiser-software's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (304 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.6x the fastest

Fastest quantiser-hardware (304 ns) to slowest quantiser-software (4.72 us): 15.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 303.6 ns median
- 1 variant significantly slower than baseline
- Spread: 15.55x (fastest 303.6 ns, slowest 4720.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 376ns | 376ns | 372ns | 376ns | 379ns | base |
| quantiser-software | 4819ns | 4793ns | 4478ns | 4818ns | 5162ns | +1182.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 304ns | 302ns | 306ns | base | 0.842 |
| quantiser-software | 4741ns | 4406ns | 5076ns | +1460.30% | 0.054 |

## Performance model

- Peak throughput: **0.847 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.843 | 99.5% |
| quantiser-software | 0.054 | 6.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 376ns | 376ns | base |
| quantiser-software | 4819ns | 4819ns | +1182.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 304ns | base | --- | [303, 304] | --- | --- | --- | --- |
| quantiser-software | 4721ns | +4416.2ns (+1454.9%) | [+4399, +4437]ns | [4702, 4741] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 304ns | +1421.8% |
| 2 | 303ns | +1560.2% |
| 3 | 303ns | +1564.8% |
| 4 | 306ns | +1543.1% |
| 5 | 306ns | +1539.6% |
| 6 | 304ns | +1645.0% |
| 7 | 303ns | +1555.7% |
| 8 | 304ns | +1548.8% |
| 9 | 302ns | +1559.8% |
| 10 | 303ns | +1555.3% |
| 11 | 302ns | +1465.4% |
| 12 | 304ns | +1477.9% |
| 13 | 306ns | +1580.2% |
| 14 | 306ns | +1445.7% |
| 15 | 308ns | +1439.8% |
| 16 | 304ns | +1460.9% |
| 17 | 305ns | +1455.6% |
| 18 | 304ns | +1458.2% |
| 19 | 303ns | +1462.0% |
| 20 | 306ns | +1448.3% |
| 21 | 302ns | +1460.3% |
| 22 | 305ns | +1445.7% |
| 23 | 303ns | +1451.9% |
| 24 | 302ns | +1456.7% |
| 25 | 303ns | +1451.5% |
| 26 | 303ns | +1452.2% |
| 27 | 307ns | +1434.2% |
| 28 | 302ns | +1488.3% |
| 29 | 304ns | +1446.0% |
| 30 | 301ns | +1461.1% |
| 31 | 305ns | +1324.0% |
| 32 | 302ns | +1337.7% |
| 33 | 302ns | +1335.0% |
| 34 | 305ns | +1325.1% |
| 35 | 303ns | +1331.5% |
| 36 | 303ns | +1332.4% |
| 37 | 305ns | +1403.2% |
| 38 | 305ns | +1419.4% |
| 39 | 303ns | +1432.6% |
| 40 | 302ns | +1434.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.029 | ok |
| quantiser-software | 0.786 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.7ns | 303.9ns | 0.9% |  |
| quantiser-software | 2.4ns | 4741.3ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 302.1-306.2 ns)
    302.1 |
    302.3 |#################################
    302.5 |
    302.7 |########################################
    302.9 |
    303.1 |#################################
    303.3 |
    303.6 |
    303.8 |####################
    304.0 |
    304.2 |##########################
    304.4 |
    304.6 |####################
    304.8 |
    305.0 |#############
    305.2 |
    305.4 |######
    305.6 |
    305.8 |####################
    306.0 |
  (4 below, 4 above range)

quantiser-software (n=40, range 4406.4-5075.8 ns)
   4406.4 |
   4439.8 |
   4473.3 |
   4506.8 |
   4540.2 |
   4573.7 |#####
   4607.2 |##########
   4640.7 |##########
   4674.1 |########################################
   4707.6 |###################################
   4741.1 |##########
   4774.6 |##########
   4808.0 |
   4841.5 |
   4875.0 |
   4908.5 |
   4941.9 |
   4975.4 |
   5008.9 |###################################
   5042.4 |#####
  (6 below, 2 above range)

```

## Diagnostics

- **quantiser-software**: autocorrelation=0.79 (measurement drift or warm-up artifact)
