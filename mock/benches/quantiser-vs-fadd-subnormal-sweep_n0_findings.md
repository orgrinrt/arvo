# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1550% faster than the next best (quantiser-software)

quantiser-hardware (305 ns) leads quantiser-software (5.03 us) by 1550%, a clear separation rather than a photo finish. CV 21.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware is fastest but the noisiest (CV 21.2%)

quantiser-hardware wins on median (305 ns) yet has the highest variance (CV 21.2%), while quantiser-software is the steadiest (CV 5.8%, 5.03 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-software shows warm-up / thermal drift (autocorr +0.79)

quantiser-software's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (305 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.5x the fastest

Fastest quantiser-hardware (305 ns) to slowest quantiser-software (5.03 us): 16.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 304.6 ns median
- 1 variant significantly slower than baseline
- Spread: 16.50x (fastest 304.6 ns, slowest 5026.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 407ns | 377ns | 351ns | 373ns | 567ns | base |
| quantiser-software | 4953ns | 5100ns | 4451ns | 5023ns | 5243ns | +1116.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 322ns | 285ns | 423ns | base | 0.795 |
| quantiser-software | 4870ns | 4384ns | 5151ns | +1412.15% | 0.053 |

## Performance model

- Peak throughput: **0.898 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.840 | 93.6% |
| quantiser-software | 0.051 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 407ns | 407ns | base |
| quantiser-software | 4953ns | 4953ns | +1116.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 305ns | base | --- | [295, 306] | --- | --- | --- | --- |
| quantiser-software | 5026ns | +4737.3ns (+1555.3%) | [+4435, +4760]ns | [4727, 5057] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 285ns | +1672.2% |
| 2 | 285ns | +1673.1% |
| 3 | 285ns | +1672.2% |
| 4 | 285ns | +1721.3% |
| 5 | 286ns | +1658.1% |
| 6 | 285ns | +1660.1% |
| 7 | 286ns | +1657.8% |
| 8 | 287ns | +1690.3% |
| 9 | 286ns | +1659.2% |
| 10 | 286ns | +1657.3% |
| 11 | 308ns | +1615.1% |
| 12 | 306ns | +1571.8% |
| 13 | 305ns | +1579.0% |
| 14 | 305ns | +1578.4% |
| 15 | 302ns | +1591.5% |
| 16 | 308ns | +1564.1% |
| 17 | 306ns | +1574.4% |
| 18 | 308ns | +1565.3% |
| 19 | 304ns | +1585.2% |
| 20 | 306ns | +1571.2% |
| 21 | 505ns | +904.3% |
| 22 | 508ns | +893.8% |
| 23 | 503ns | +898.9% |
| 24 | 506ns | +832.3% |
| 25 | 412ns | +1051.2% |
| 26 | 288ns | +1541.6% |
| 27 | 338ns | +1301.1% |
| 28 | 288ns | +1538.6% |
| 29 | 285ns | +1555.3% |
| 30 | 285ns | +1553.5% |
| 31 | 304ns | +1358.1% |
| 32 | 304ns | +1334.5% |
| 33 | 305ns | +1334.7% |
| 34 | 305ns | +1381.2% |
| 35 | 307ns | +1550.9% |
| 36 | 305ns | +1341.5% |
| 37 | 305ns | +1336.6% |
| 38 | 306ns | +1330.4% |
| 39 | 306ns | +1331.5% |
| 40 | 307ns | +1325.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.787 | HIGH+ (drift/warm-up) |
| quantiser-software | 0.795 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 2.9ns | 322.1ns | 0.9% |  |
| quantiser-software | 4.4ns | 4870.1ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 285.1-423.4 ns)
    285.1 |###########################
    292.0 |
    298.9 |########################################
    305.8 |#####################
    312.8 |
    319.7 |
    326.6 |
    333.5 |###
    340.4 |
    347.3 |
    354.2 |
    361.2 |
    368.1 |
    375.0 |
    381.9 |
    388.8 |
    395.7 |
    402.6 |
    409.5 |###
    416.5 |
  (5 below, 4 above range)

quantiser-software (n=40, range 4383.8-5150.7 ns)
   4383.8 |####
   4422.1 |####
   4460.4 |
   4498.8 |####
   4537.1 |
   4575.5 |
   4613.8 |
   4652.2 |
   4690.5 |########################
   4728.9 |####
   4767.2 |
   4805.6 |
   4843.9 |
   4882.2 |
   4920.6 |
   4958.9 |
   4997.3 |########################
   5035.6 |########################
   5074.0 |
   5112.3 |########################################
  (6 below, 2 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=20.1% (high variance, measurements may be unstable)
- **quantiser-hardware**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **quantiser-software**: autocorrelation=0.79 (measurement drift or warm-up artifact)
