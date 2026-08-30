# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1503% faster than the next best (quantiser-software)

quantiser-hardware (329 ns) leads quantiser-software (5.27 us) by 1503%, a clear separation rather than a photo finish. CV 12.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware is fastest but the noisiest (CV 12.5%)

quantiser-hardware wins on median (329 ns) yet has the highest variance (CV 12.5%), while quantiser-software is the steadiest (CV 5.9%, 5.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-software shows warm-up / thermal drift (autocorr +0.67)

quantiser-software's per-pass series has lag-1 autocorrelation +0.67, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (329 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.0x the fastest

Fastest quantiser-hardware (329 ns) to slowest quantiser-software (5.27 us): 16.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 328.8 ns median
- 1 variant significantly slower than baseline
- Spread: 16.03x (fastest 328.8 ns, slowest 5268.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 400ns | 404ns | 374ns | 395ns | 442ns | base |
| quantiser-software | 5296ns | 5350ns | 4971ns | 5255ns | 5741ns | +1223.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 324ns | 303ns | 362ns | base | 0.789 |
| quantiser-software | 5213ns | 4894ns | 5648ns | +1506.42% | 0.049 |

## Performance model

- Peak throughput: **0.845 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.779 | 92.1% |
| quantiser-software | 0.049 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 400ns | 400ns | base |
| quantiser-software | 5296ns | 5296ns | +1223.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 329ns | base | --- | [306, 330] | --- | --- | --- | --- |
| quantiser-software | 5269ns | +4937.2ns (+1501.8%) | [+4673, +4980]ns | [4976, 5310] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 330ns | +1506.6% |
| 2 | 330ns | +1521.7% |
| 3 | 331ns | +1505.8% |
| 4 | 330ns | +1569.8% |
| 5 | 330ns | +1506.3% |
| 6 | 330ns | +1509.6% |
| 7 | 328ns | +1520.3% |
| 8 | 330ns | +1518.7% |
| 9 | 330ns | +1521.4% |
| 10 | 330ns | +1511.8% |
| 11 | 304ns | +1514.8% |
| 12 | 305ns | +1508.6% |
| 13 | 302ns | +1523.3% |
| 14 | 302ns | +1560.4% |
| 15 | 304ns | +1519.5% |
| 16 | 302ns | +1525.9% |
| 17 | 302ns | +1682.9% |
| 18 | 307ns | +1503.2% |
| 19 | 305ns | +1516.7% |
| 20 | 303ns | +1529.3% |
| 21 | 568ns | +763.0% |
| 22 | 308ns | +1493.5% |
| 23 | 305ns | +1505.8% |
| 24 | 305ns | +1608.2% |
| 25 | 304ns | +1649.4% |
| 26 | 304ns | +1656.1% |
| 27 | 308ns | +1660.5% |
| 28 | 311ns | +1808.0% |
| 29 | 303ns | +1838.8% |
| 30 | 308ns | +1957.1% |
| 31 | 331ns | +1492.1% |
| 32 | 332ns | +1486.2% |
| 33 | 333ns | +1483.6% |
| 34 | 332ns | +1488.8% |
| 35 | 332ns | +1488.2% |
| 36 | 331ns | +1448.1% |
| 37 | 330ns | +1381.3% |
| 38 | 332ns | +1367.3% |
| 39 | 334ns | +1358.8% |
| 40 | 333ns | +1427.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.033 | ok |
| quantiser-software | 0.673 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 3.1ns | 324.5ns | 1.0% |  |
| quantiser-software | 2.6ns | 5212.5ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 302.9-361.9 ns)
    302.9 |######################
    305.9 |##########
    308.8 |##
    311.8 |
    314.7 |
    317.7 |
    320.6 |
    323.6 |
    326.5 |##
    329.5 |########################################
    332.4 |#######
    335.4 |
    338.3 |
    341.3 |
    344.2 |
    347.2 |
    350.1 |
    353.1 |
    356.0 |
    359.0 |
  (5 below, 1 above range)

quantiser-software (n=40, range 4894.2-5648.0 ns)
   4894.2 |########################################
   4931.9 |####
   4969.6 |
   5007.3 |####
   5045.0 |
   5082.7 |####
   5120.3 |####
   5158.0 |
   5195.7 |####
   5233.4 |############
   5271.1 |####################
   5308.8 |########################
   5346.5 |########
   5384.2 |####
   5421.9 |####
   5459.6 |
   5497.2 |####
   5534.9 |
   5572.6 |
   5610.3 |
  (3 below, 3 above range)

```

## Diagnostics

- **quantiser-software**: autocorrelation=0.67 (measurement drift or warm-up artifact)
