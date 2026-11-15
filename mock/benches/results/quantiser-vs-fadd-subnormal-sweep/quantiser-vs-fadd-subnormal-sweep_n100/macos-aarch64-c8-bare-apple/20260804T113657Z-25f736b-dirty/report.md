# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1179% faster than the next best (quantiser-software)

quantiser-hardware (319 ns) leads quantiser-software (4.09 us) by 1179%, a clear separation rather than a photo finish. CV 22.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware is fastest but the noisiest (CV 22.3%)

quantiser-hardware wins on median (319 ns) yet has the highest variance (CV 22.3%), while quantiser-software is the steadiest (CV 3.4%, 4.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (319 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 12.8x the fastest

Fastest quantiser-hardware (319 ns) to slowest quantiser-software (4.09 us): 12.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 319.4 ns median
- 1 variant significantly slower than baseline
- Spread: 12.79x (fastest 319.4 ns, slowest 4086.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 410ns | 394ns | 375ns | 394ns | 491ns | base |
| quantiser-software | 4227ns | 4166ns | 4132ns | 4173ns | 4486ns | +932.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 333ns | 304ns | 405ns | base | 0.769 |
| quantiser-software | 4140ns | 4051ns | 4378ns | +1143.67% | 0.062 |

## Performance model

- Peak throughput: **0.843 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.802 | 95.1% |
| quantiser-software | 0.063 | 7.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 410ns | 410ns | base |
| quantiser-software | 4227ns | 4227ns | +932.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 319ns | base | --- | [307, 330] | --- | --- | --- | --- |
| quantiser-software | 4087ns | +3768.6ns (+1179.9%) | [+3758, +3783]ns | [4065, 4108] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-hardware | quantiser-software |
|---|---|---|
| 1 | 305ns | +1225.0% |
| 2 | 303ns | +1231.3% |
| 3 | 303ns | +1241.5% |
| 4 | 307ns | +1224.4% |
| 5 | 304ns | +1238.6% |
| 6 | 308ns | +1221.4% |
| 7 | 307ns | +1223.9% |
| 8 | 305ns | +1233.3% |
| 9 | 304ns | +1238.1% |
| 10 | 305ns | +1232.2% |
| 11 | 302ns | +1255.4% |
| 12 | 304ns | +1260.5% |
| 13 | 308ns | +1232.7% |
| 14 | 305ns | +1239.2% |
| 15 | 305ns | +1267.3% |
| 16 | 305ns | +1378.3% |
| 17 | 307ns | +1321.6% |
| 18 | 308ns | +1229.8% |
| 19 | 310ns | +1220.6% |
| 20 | 308ns | +1229.0% |
| 21 | 329ns | +1162.4% |
| 22 | 722ns | +476.0% |
| 23 | 330ns | +1287.0% |
| 24 | 330ns | +1150.5% |
| 25 | 333ns | +1134.8% |
| 26 | 331ns | +1141.1% |
| 27 | 333ns | +1133.0% |
| 28 | 523ns | +763.0% |
| 29 | 331ns | +1160.5% |
| 30 | 331ns | +1197.2% |
| 31 | 331ns | +1126.1% |
| 32 | 332ns | +1123.9% |
| 33 | 330ns | +1130.0% |
| 34 | 332ns | +1120.9% |
| 35 | 331ns | +1120.0% |
| 36 | 332ns | +1123.9% |
| 37 | 331ns | +1125.2% |
| 38 | 330ns | +1242.5% |
| 39 | 330ns | +1128.6% |
| 40 | 330ns | +1126.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.056 | ok |
| quantiser-software | 0.181 | ok |

**Consistency summary:**

- **quantiser-software**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 3.0ns | 332.9ns | 0.9% |  |
| quantiser-software | 3.1ns | 4139.8ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 303.8-404.8 ns)
    303.8 |###################################
    308.8 |##
    313.9 |
    318.9 |
    324.0 |
    329.0 |########################################
    334.1 |
    339.1 |
    344.2 |
    349.2 |
    354.3 |
    359.3 |
    364.4 |
    369.4 |
    374.5 |
    379.5 |
    384.6 |
    389.6 |
    394.7 |
    399.7 |
  (3 below, 2 above range)

quantiser-software (n=40, range 4050.6-4378.1 ns)
   4050.6 |########################################
   4067.0 |#####
   4083.3 |##############
   4099.7 |###########
   4116.1 |##
   4132.5 |##
   4148.8 |#####
   4165.2 |#####
   4181.6 |
   4198.0 |
   4214.3 |
   4230.7 |
   4247.1 |
   4263.5 |
   4279.9 |##
   4296.2 |
   4312.6 |
   4329.0 |
   4345.4 |##
   4361.7 |
  (3 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=21.4% (high variance, measurements may be unstable)
