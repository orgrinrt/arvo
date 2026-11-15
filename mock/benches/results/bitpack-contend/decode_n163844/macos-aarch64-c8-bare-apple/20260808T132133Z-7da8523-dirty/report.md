# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd shows warm-up / thermal drift (autocorr +0.90)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (211 ns) is smaller than the fastest variant's own run-to-run std-dev (213 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-contend-pipe4 is inconsistent: worst-20% is 3.0x its best-20%

bitpack-contend-pipe4's best 20% of batches run at 445 ns but its worst 20% at 1.33 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### bitpack-contend-d16-control's edge over baseline is significant but tiny (-3 ns, 0.57%)

bitpack-contend-d16-control differs from baseline bitpack-contend-d16 by -3 ns (0.57%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 466.9 ns median (-14.6% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.45x (fastest 466.9 ns, slowest 677.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 635ns | 622ns | 576ns | 626ns | 720ns | base |
| bitpack-contend-d16-control | 623ns | 596ns | 575ns | 607ns | 722ns | -1.79% |
| bitpack-contend-packed-simd | 793ns | 747ns | 742ns | 762ns | 934ns | +24.83% |
| bitpack-contend-pipe2 | 623ns | 533ns | 519ns | 548ns | 954ns | -1.80% |
| bitpack-contend-pipe4 | 780ns | 564ns | 519ns | 635ns | 1477ns | +22.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 560ns | 508ns | 635ns | base | 29.281 |
| bitpack-contend-d16-control | 549ns | 507ns | 636ns | -1.81% | 29.820 |
| bitpack-contend-packed-simd | 719ns | 670ns | 849ns | +28.44% | 22.797 |
| bitpack-contend-pipe2 | 542ns | 448ns | 826ns | -3.15% | 30.234 |
| bitpack-contend-pipe4 | 689ns | 445ns | 1332ns | +23.15% | 23.777 |

## Performance model

- Peak throughput: **36.827 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 29.983 | 81.4% |
| bitpack-contend-d16-control | 31.520 | 85.6% |
| bitpack-contend-packed-simd | 24.176 | 65.6% |
| bitpack-contend-pipe2 | 35.091 | 95.3% |
| bitpack-contend-pipe4 | 33.581 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 635ns | 635ns | base |
| bitpack-contend-d16-control | 623ns | 623ns | -1.79% |
| bitpack-contend-packed-simd | 793ns | 793ns | +24.83% |
| bitpack-contend-pipe2 | 623ns | 623ns | -1.80% |
| bitpack-contend-pipe4 | 780ns | 780ns | +22.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 546ns | base | --- | [512, 587] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 520ns | no significant difference | [-5, +1]ns | [510, 548] | no | 0.6358 | 0.6358 | 0 |
| bitpack-contend-packed-simd | 678ns | +166.9ns (+30.5%) | [+163, +172]ns | [676, 681] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 467ns | -67.3ns (-12.3%) | [-96, -56]ns | [459, 471] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 488ns | -53.6ns (-9.8%) | [-59, -51]ns | [458, 526] | YES | 0.0030 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|
| 1 | 517ns | -1.9% | +31.2% | -14.5% | -13.9% |
| 2 | 509ns | +0.2% | +33.2% | -10.5% | -15.0% |
| 3 | 508ns | +0.2% | +34.4% | -8.9% | -12.2% |
| 4 | 510ns | -0.7% | +34.3% | -12.2% | -10.3% |
| 5 | 510ns | -0.7% | +32.7% | -13.6% | -13.6% |
| 6 | 513ns | -0.2% | +32.5% | -12.1% | -10.6% |
| 7 | 505ns | +0.2% | +34.2% | -8.8% | -9.9% |
| 8 | 520ns | -2.4% | +30.1% | -12.2% | -11.3% |
| 9 | 516ns | -1.4% | +31.5% | -10.3% | -11.8% |
| 10 | 510ns | -0.1% | +32.2% | -10.8% | -11.8% |
| 11 | 630ns | -14.0% | +7.0% | -7.1% | +111.7% |
| 12 | 634ns | -14.0% | +6.5% | -7.3% | +107.9% |
| 13 | 636ns | -13.4% | +6.1% | -15.8% | +109.1% |
| 14 | 636ns | -13.8% | +6.2% | -15.0% | +109.2% |
| 15 | 635ns | -13.5% | +7.3% | -15.4% | +109.8% |
| 16 | 639ns | -14.3% | +4.6% | -15.8% | +107.9% |
| 17 | 636ns | -15.0% | +5.2% | -16.0% | +108.9% |
| 18 | 631ns | -13.1% | +5.1% | -14.8% | +114.1% |
| 19 | 626ns | -15.7% | +6.3% | -14.0% | +110.5% |
| 20 | 613ns | -11.1% | +10.3% | -12.4% | +112.6% |
| 21 | 514ns | -1.1% | +30.7% | +163.9% | -14.0% |
| 22 | 507ns | +0.9% | +33.9% | +153.0% | -10.1% |
| 23 | 509ns | +0.2% | +33.2% | +132.6% | -10.0% |
| 24 | 511ns | -0.2% | +32.2% | -12.7% | -10.3% |
| 25 | 508ns | +0.1% | +34.3% | -11.6% | -9.9% |
| 26 | 510ns | +0.2% | +32.0% | -10.9% | -10.3% |
| 27 | 508ns | +0.4% | +33.4% | -9.4% | -11.7% |
| 28 | 508ns | -0.2% | +33.4% | -10.0% | -10.3% |
| 29 | 509ns | -0.3% | +32.1% | -10.8% | -10.7% |
| 30 | 513ns | -0.2% | +32.0% | -11.2% | -10.4% |
| 31 | 573ns | +11.4% | +47.3% | -18.6% | -8.0% |
| 32 | 585ns | +8.0% | +45.6% | -20.1% | -11.2% |
| 33 | 582ns | +9.4% | +46.6% | -19.8% | -9.2% |
| 34 | 587ns | +8.1% | +44.7% | -20.0% | -10.0% |
| 35 | 584ns | +8.8% | +44.9% | -19.4% | -10.8% |
| 36 | 588ns | +8.2% | +43.3% | -20.4% | -10.3% |
| 37 | 593ns | +7.2% | +44.3% | -20.6% | -11.4% |
| 38 | 584ns | +9.1% | +45.5% | -21.4% | -10.1% |
| 39 | 588ns | +8.3% | +43.1% | -20.6% | -12.5% |
| 40 | 588ns | +7.6% | +42.9% | -20.4% | -11.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.853 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.868 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.897 | HIGH+ (drift/warm-up) |
| bitpack-contend-pipe2 | 0.651 | HIGH+ (drift/warm-up) |
| bitpack-contend-pipe4 | 0.852 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 21/40, lost 17/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40
- **bitpack-contend-pipe2**: won 37/40, lost 3/40
- **bitpack-contend-pipe4**: won 30/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 2.6ns | 559.6ns | 0.5% |  |
| bitpack-contend-d16-control | 2.9ns | 549.4ns | 0.5% |  |
| bitpack-contend-packed-simd | 2.2ns | 718.7ns | 0.3% |  |
| bitpack-contend-pipe2 | 2.3ns | 541.9ns | 0.4% |  |
| bitpack-contend-pipe4 | 2.3ns | 689.1ns | 0.3% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 507.5-634.7 ns)
    507.5 |########################################
    513.9 |#########
    520.2 |
    526.6 |
    533.0 |
    539.3 |
    545.7 |
    552.0 |
    558.4 |
    564.8 |
    571.1 |###
    577.5 |######
    583.8 |##################
    590.2 |###
    596.6 |
    602.9 |
    609.3 |###
    615.6 |
    622.0 |###
    628.4 |#########
  (4 below, 5 above range)

bitpack-contend-d16-control (n=40, range 506.9-636.3 ns)
    506.9 |########################################
    513.3 |
    519.8 |
    526.3 |##
    532.8 |
    539.2 |##########
    545.7 |############
    552.2 |
    558.6 |
    565.1 |
    571.6 |
    578.1 |
    584.5 |
    591.0 |
    597.5 |
    603.9 |
    610.4 |
    616.9 |
    623.4 |
    629.8 |#################
  (4 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 669.7-849.0 ns)
    669.7 |########################################
    678.7 |############
    687.6 |
    696.6 |
    705.6 |
    714.5 |
    723.5 |
    732.4 |
    741.4 |
    750.4 |
    759.3 |
    768.3 |
    777.3 |
    786.2 |
    795.2 |
    804.2 |
    813.1 |
    822.1 |
    831.0 |##
    840.0 |########
  (4 below, 5 above range)

bitpack-contend-pipe2 (n=40, range 448.1-826.4 ns)
    448.1 |########################################
    467.0 |#################
    485.9 |
    504.8 |
    523.7 |####################
    542.6 |
    561.6 |
    580.5 |#####
    599.4 |
    618.3 |
    637.2 |
    656.1 |
    675.1 |
    694.0 |
    712.9 |
    731.8 |
    750.7 |
    769.6 |
    788.6 |
    807.5 |
  (4 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 444.9-1331.8 ns)
    444.9 |########################################
    489.2 |#######################
    533.6 |
    577.9 |
    622.3 |
    666.6 |
    711.0 |
    755.3 |
    799.6 |
    844.0 |
    888.3 |
    932.7 |
    977.0 |
   1021.4 |
   1065.7 |
   1110.0 |
   1154.4 |
   1198.7 |
   1243.1 |
   1287.4 |################
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe2**: CV=39.3% (high variance, measurements may be unstable)
- **bitpack-contend-pipe2**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe4**: CV=53.7% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: autocorrelation=0.85 (measurement drift or warm-up artifact)
