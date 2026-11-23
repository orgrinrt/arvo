# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-contend-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-contend-d16 has the worst median (3.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-contend-d16-control at 1.97 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-contend-d16-control dominates: 32% faster than the next best (bitpack-contend-pipe2)

bitpack-contend-d16-control (1.97 us) leads bitpack-contend-pipe2 (2.60 us) by 32%, a clear separation rather than a photo finish. CV 71.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d16-control is fastest but the noisiest (CV 71.3%)

bitpack-contend-d16-control wins on median (1.97 us) yet has the highest variance (CV 71.3%), while bitpack-contend-d16 is the steadiest (CV 35.5%, 3.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-packed-simd shows alternating (throttle bounce) (autocorr -0.76)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-contend-pipe4 is inconsistent: worst-20% is 3.9x its best-20%

bitpack-contend-pipe4's best 20% of batches run at 973 ns but its worst 20% at 3.77 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d16-control** at 1969.5 ns median (-45.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.84x (fastest 1969.5 ns, slowest 3625.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112791945 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3043ns | 3823ns | 1354ns | 3102ns | 4553ns | base |
| bitpack-contend-d16-control | 2733ns | 2083ns | 1318ns | 2471ns | 4935ns | -10.17% |
| bitpack-contend-packed-simd | 2693ns | 3084ns | 1069ns | 2780ns | 4059ns | -11.48% |
| bitpack-contend-pipe2 | 2543ns | 2763ns | 1088ns | 2451ns | 4273ns | -16.43% |
| bitpack-contend-pipe4 | 2626ns | 3026ns | 1040ns | 2686ns | 4031ns | -13.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 2873ns | 1293ns | 4270ns | base | 5.702 |
| bitpack-contend-d16-control | 2587ns | 1261ns | 4664ns | -9.97% | 6.334 |
| bitpack-contend-packed-simd | 2524ns | 1013ns | 3798ns | -12.15% | 6.491 |
| bitpack-contend-pipe2 | 2394ns | 1034ns | 3987ns | -16.68% | 6.844 |
| bitpack-contend-pipe4 | 2455ns | 973ns | 3766ns | -14.57% | 6.675 |

## Performance model

- Peak throughput: **16.838 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 4.519 | 26.8% |
| bitpack-contend-d16-control | 8.319 | 49.4% |
| bitpack-contend-packed-simd | 5.696 | 33.8% |
| bitpack-contend-pipe2 | 6.292 | 37.4% |
| bitpack-contend-pipe4 | 5.831 | 34.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 3043ns | 3043ns | base |
| bitpack-contend-d16-control | 2733ns | 2733ns | -10.17% |
| bitpack-contend-packed-simd | 2693ns | 2693ns | -11.48% |
| bitpack-contend-pipe2 | 2543ns | 2543ns | -16.43% |
| bitpack-contend-pipe4 | 2626ns | 2626ns | -13.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3626ns | base | --- | [1337, 3748] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1970ns | no significant difference | [-80, +82]ns | [1315, 3364] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-packed-simd | 2876ns | -496.7ns (-13.7%) | [-677, -285]ns | [2701, 2955] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 2604ns | -266.0ns (-7.3%) | [-544, -249]ns | [1063, 3440] | YES | 0.0002 | 0.0002 | 0 |
| bitpack-contend-pipe4 | 2810ns | -402.0ns (-11.1%) | [-772, -303]ns | [2610, 2933] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 3349ns | -0.7% | -12.9% | -20.0% | -16.1% |
| 1 | 2 | 2657ns | -50.3% | -37.6% | +27.8% | -17.5% |
| 1 | 3 | 3705ns | -27.0% | -24.9% | -71.1% | -26.8% |
| 1 | 4 | 1337ns | -5.3% | +48.0% | +49.8% | +114.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.459 | moderate- |
| bitpack-contend-d16-control | -0.609 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.762 | HIGH- (thermal bounce) |
| bitpack-contend-pipe2 | -0.192 | ok |
| bitpack-contend-pipe4 | -0.317 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 3/4, lost 1/4
- **bitpack-contend-pipe2**: won 2/4, lost 2/4
- **bitpack-contend-pipe4**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 58.1ns | 2873.2ns | 2.0% |  |
| bitpack-contend-d16-control | 49.8ns | 2586.8ns | 1.9% |  |
| bitpack-contend-packed-simd | 59.7ns | 2524.2ns | 2.4% |  |
| bitpack-contend-pipe2 | 51.0ns | 2394.0ns | 2.1% |  |
| bitpack-contend-pipe4 | 59.9ns | 2454.5ns | 2.4% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1292.9-4269.9 ns)
   1292.9 |########################################
   1441.8 |
   1590.6 |####
   1739.5 |
   1888.3 |
   2037.2 |
   2186.0 |
   2334.9 |
   2483.7 |
   2632.6 |
   2781.4 |
   2930.3 |
   3079.1 |####
   3228.0 |
   3376.8 |####
   3525.7 |########################
   3674.5 |################
   3823.4 |
   3972.2 |########################
   4121.1 |############
  (5 below, 3 above range)

bitpack-contend-d16-control (n=40, range 1260.7-4663.5 ns)
   1260.7 |########################################
   1430.8 |
   1601.0 |
   1771.1 |
   1941.2 |
   2111.4 |
   2281.5 |
   2451.7 |##
   2621.8 |
   2792.0 |##
   2962.1 |##
   3132.2 |#####
   3302.4 |##########
   3472.5 |##
   3642.7 |
   3812.8 |
   3983.0 |##########
   4153.1 |##
   4323.3 |
   4493.4 |##
  (4 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 1013.1-3797.7 ns)
   1013.1 |#########################
   1152.4 |##########
   1291.6 |
   1430.8 |
   1570.1 |
   1709.3 |
   1848.5 |
   1987.7 |
   2127.0 |#####
   2266.2 |
   2405.4 |
   2544.7 |
   2683.9 |#########################
   2823.1 |########################################
   2962.4 |###############
   3101.6 |#####
   3240.8 |##########
   3380.0 |#####
   3519.3 |##########
   3658.5 |##########
  (5 below, 3 above range)

bitpack-contend-pipe2 (n=40, range 1034.2-3986.9 ns)
   1034.2 |########################################
   1181.8 |
   1329.4 |
   1477.1 |
   1624.7 |
   1772.3 |
   1920.0 |##
   2067.6 |
   2215.3 |
   2362.9 |##
   2510.6 |
   2658.2 |##
   2805.8 |##
   2953.5 |
   3101.1 |
   3248.8 |##
   3396.4 |####################
   3544.0 |##
   3691.7 |##############
   3839.3 |
  (4 below, 4 above range)

bitpack-contend-pipe4 (n=40, range 973.0-3765.9 ns)
    973.0 |########################################
   1112.7 |
   1252.3 |#####
   1391.9 |#####
   1531.6 |
   1671.2 |
   1810.9 |
   1950.5 |
   2090.2 |
   2229.8 |
   2369.5 |
   2509.1 |#################
   2648.7 |#################
   2788.4 |########################################
   2928.0 |#################
   3067.7 |###########
   3207.3 |#####
   3347.0 |#################
   3486.6 |
   3626.2 |#####
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=44.7% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=54.3% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=41.3% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-contend-pipe2**: CV=53.6% (high variance, measurements may be unstable)
- **bitpack-contend-pipe2**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-contend-pipe4**: CV=42.9% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: worst_20/best_20 = 3.9x (possible bimodal distribution)
