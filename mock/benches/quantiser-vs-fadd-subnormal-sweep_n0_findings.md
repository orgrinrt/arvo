# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1549% faster than the next best (quantiser-software)

quantiser-hardware (333 ns) leads quantiser-software (5.49 us) by 1549%, a clear separation rather than a photo finish. CV 19.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware is fastest but the noisiest (CV 19.4%)

quantiser-hardware wins on median (333 ns) yet has the highest variance (CV 19.4%), while quantiser-software is the steadiest (CV 2.6%, 5.49 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (333 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.5x the fastest

Fastest quantiser-hardware (333 ns) to slowest quantiser-software (5.49 us): 16.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 332.7 ns median
- 1 variant significantly slower than baseline
- Spread: 16.49x (fastest 332.7 ns, slowest 5486.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 442ns | 413ns | 407ns | 413ns | 566ns | base |
| quantiser-software | 5564ns | 5566ns | 5368ns | 5573ns | 5732ns | +1157.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 354ns | 331ns | 442ns | base | 0.723 |
| quantiser-software | 5482ns | 5291ns | 5639ns | +1447.52% | 0.047 |

## Performance model

- Peak throughput: **0.774 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.769 | 99.4% |
| quantiser-software | 0.047 | 6.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 442ns | 442ns | base |
| quantiser-software | 5564ns | 5564ns | +1157.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 333ns | base | --- | [332, 333] | --- | --- | --- | --- |
| quantiser-software | 5487ns | +5140.1ns (+1545.0%) | [+5119, +5161]ns | [5460, 5541] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 333ns | +1538.1% |
| 2 | 332ns | +1542.2% |
| 3 | 332ns | +1542.8% |
| 4 | 335ns | +1533.1% |
| 5 | 333ns | +1536.1% |
| 6 | 336ns | +1523.0% |
| 7 | 332ns | +1542.1% |
| 8 | 332ns | +1543.8% |
| 9 | 332ns | +1538.6% |
| 10 | 331ns | +1547.0% |
| 11 | 330ns | +1586.7% |
| 12 | 540ns | +927.1% |
| 13 | 333ns | +1564.5% |
| 14 | 332ns | +1568.9% |
| 15 | 332ns | +1566.8% |
| 16 | 332ns | +1571.1% |
| 17 | 335ns | +1558.1% |
| 18 | 333ns | +1564.9% |
| 19 | 336ns | +1550.3% |
| 20 | 461ns | +1102.9% |
| 21 | 332ns | +1553.6% |
| 22 | 333ns | +1547.8% |
| 23 | 330ns | +1563.9% |
| 24 | 332ns | +1556.2% |
| 25 | 331ns | +1561.6% |
| 26 | 668ns | +763.5% |
| 27 | 333ns | +1417.6% |
| 28 | 330ns | +1457.6% |
| 29 | 332ns | +1423.8% |
| 30 | 330ns | +1498.1% |
| 31 | 333ns | +1541.5% |
| 32 | 334ns | +1587.3% |
| 33 | 335ns | +1536.7% |
| 34 | 400ns | +1269.6% |
| 35 | 332ns | +1614.2% |
| 36 | 334ns | +1541.2% |
| 37 | 332ns | +1551.5% |
| 38 | 332ns | +1568.4% |
| 39 | 418ns | +1291.0% |
| 40 | 378ns | +1345.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.089 | ok |
| quantiser-software | 0.369 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.7ns | 354.2ns | 0.8% |  |
| quantiser-software | 2.4ns | 5481.9ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 330.7-441.9 ns)
    330.7 |########################################
    336.3 |
    341.8 |
    347.4 |
    353.0 |
    358.5 |
    364.1 |
    369.6 |
    375.2 |#
    380.8 |
    386.3 |
    391.9 |
    397.4 |#
    403.0 |
    408.6 |
    414.1 |#
    419.7 |
    425.2 |
    430.8 |
    436.4 |
  (4 below, 3 above range)

quantiser-software (n=40, range 5291.4-5639.3 ns)
   5291.4 |
   5308.8 |
   5326.2 |
   5343.6 |
   5361.0 |
   5378.4 |
   5395.8 |
   5413.2 |
   5430.6 |####
   5448.0 |########################################
   5465.4 |################
   5482.8 |########################
   5500.1 |
   5517.5 |
   5534.9 |########################################
   5552.3 |####
   5569.7 |
   5587.1 |
   5604.5 |
   5621.9 |####
  (4 below, 3 above range)

```
