# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 1.61 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 40% faster than the next best (warm-clamp-accfit)

warm-clamp-head (1.61 us) leads warm-clamp-accfit (2.25 us) by 40%, a clear separation rather than a photo finish. CV 26.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 48% (significant)

warm-clamp-head is -1.89 us (48%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.5x slower than the field

warm-clamp-acc64 (3.98 us) is 2.5x the fastest (1.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum is inconsistent: worst-20% is 3.7x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.95 us but its worst 20% at 7.29 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 1605.2 ns median (-59.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.48x (fastest 1605.2 ns, slowest 3977.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3650ns | 4195ns | 1367ns | 4011ns | 4850ns | base |
| warm-clamp-accfit | 2357ns | 2485ns | 1898ns | 2398ns | 2693ns | -35.43% |
| warm-clamp-accfit-dyn | 2591ns | 2531ns | 2019ns | 2582ns | 3190ns | -29.00% |
| warm-clamp-head | 2170ns | 1858ns | 1767ns | 1973ns | 3162ns | -40.55% |
| warm-clamp-min-lanes | 4319ns | 2410ns | 2235ns | 3688ns | 8296ns | +18.32% |
| warm-clamp-minimum | 4112ns | 2390ns | 2006ns | 3668ns | 7550ns | +12.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3444ns | 1296ns | 4544ns | base | 2.379 |
| warm-clamp-accfit | 2119ns | 1712ns | 2397ns | -38.47% | 3.865 |
| warm-clamp-accfit-dyn | 2334ns | 1828ns | 2840ns | -32.22% | 3.509 |
| warm-clamp-head | 1859ns | 1543ns | 2621ns | -46.03% | 4.407 |
| warm-clamp-min-lanes | 4193ns | 2176ns | 8030ns | +21.75% | 1.954 |
| warm-clamp-minimum | 3991ns | 1953ns | 7292ns | +15.88% | 2.053 |

## Performance model

- Peak throughput: **6.322 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.059 | 32.6% |
| warm-clamp-accfit | 3.642 | 57.6% |
| warm-clamp-accfit-dyn | 3.577 | 56.6% |
| warm-clamp-head | 5.103 | 80.7% |
| warm-clamp-min-lanes | 3.490 | 55.2% |
| warm-clamp-minimum | 3.521 | 55.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3650ns | 3650ns | base |
| warm-clamp-accfit | 2357ns | 2357ns | -35.43% |
| warm-clamp-accfit-dyn | 2591ns | 2591ns | -29.00% |
| warm-clamp-head | 2170ns | 2170ns | -40.55% |
| warm-clamp-min-lanes | 4319ns | 4319ns | +18.32% |
| warm-clamp-minimum | 4112ns | 4112ns | +12.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3978ns | base | --- | [3516, 4115] | --- | --- | --- | --- |
| warm-clamp-accfit | 2249ns | -1828.9ns (-46.0%) | [-1880, -1650]ns | [2048, 2250] | YES | 0.0003 | 0.0002 | 0 |
| warm-clamp-accfit-dyn | 2290ns | -1253.9ns (-31.5%) | [-1619, -1144]ns | [2256, 2461] | YES | 0.0002 | 0.0000 | 0 |
| warm-clamp-head | 1605ns | -1892.0ns (-47.6%) | [-2145, -1716]ns | [1604, 1714] | YES | 0.0003 | 0.0002 | 0 |
| warm-clamp-min-lanes | 2347ns | no significant difference | [-441, +1181]ns | [2336, 5302] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-minimum | 2327ns | no significant difference | [-712, +1523]ns | [2320, 5819] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3516ns | -50.8% | -31.9% | -54.6% | -33.4% | -33.8% |
| 1 | 2 | 4118ns | -44.7% | -44.8% | -60.4% | -43.1% | +41.3% |
| 1 | 3 | 4159ns | -45.9% | -48.0% | -40.4% | +27.1% | -54.2% |
| 1 | 4 | 3742ns | -42.9% | -30.0% | -57.1% | -37.4% | -37.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.212 | moderate- |
| warm-clamp-accfit | -0.171 | ok |
| warm-clamp-accfit-dyn | -0.319 | moderate- |
| warm-clamp-head | -0.405 | moderate- |
| warm-clamp-min-lanes | -0.416 | moderate- |
| warm-clamp-minimum | -0.443 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 72.7ns | 3444.1ns | 2.1% |  |
| warm-clamp-accfit | 81.3ns | 2119.3ns | 3.8% |  |
| warm-clamp-accfit-dyn | 89.1ns | 2334.3ns | 3.8% |  |
| warm-clamp-head | 111.1ns | 1858.8ns | 6.0% | HIGH |
| warm-clamp-min-lanes | 42.6ns | 4193.2ns | 1.0% |  |
| warm-clamp-minimum | 40.6ns | 3991.0ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1295.9-4543.6 ns)
   1295.9 |
   1458.3 |
   1620.7 |
   1783.0 |
   1945.4 |
   2107.8 |###
   2270.2 |
   2432.6 |
   2595.0 |###
   2757.3 |###
   2919.7 |
   3082.1 |
   3244.5 |###
   3406.9 |#########
   3569.3 |###
   3731.6 |###############
   3894.0 |
   4056.4 |########################################
   4218.8 |###
   4381.2 |###
  (7 below, 5 above range)

warm-clamp-accfit (n=40, range 1712.1-2397.3 ns)
   1712.1 |#########
   1746.3 |###
   1780.6 |
   1814.9 |
   1849.1 |############
   1883.4 |
   1917.7 |
   1951.9 |
   1986.2 |
   2020.4 |###############
   2054.7 |
   2089.0 |
   2123.2 |
   2157.5 |
   2191.8 |###
   2226.0 |########################################
   2260.3 |
   2294.5 |###############
   2328.8 |###
   2363.1 |###
  (3 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1827.8-2840.5 ns)
   1827.8 |####
   1878.4 |
   1929.1 |########
   1979.7 |
   2030.3 |########
   2081.0 |####
   2131.6 |
   2182.2 |####
   2232.9 |########################################
   2283.5 |#################
   2334.1 |########
   2384.8 |####
   2435.4 |
   2486.0 |######################
   2536.7 |########
   2587.3 |
   2637.9 |
   2688.6 |####
   2739.2 |
   2789.8 |#############
  (3 below, 3 above range)

warm-clamp-head (n=40, range 1543.1-2621.4 ns)
   1543.1 |#####################
   1597.0 |########################################
   1651.0 |#########
   1704.9 |
   1758.8 |#########
   1812.7 |
   1866.6 |
   1920.5 |
   1974.4 |
   2028.4 |
   2082.3 |
   2136.2 |
   2190.1 |#########
   2244.0 |
   2297.9 |
   2351.9 |######
   2405.8 |###
   2459.7 |
   2513.6 |###
   2567.5 |
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2176.4-8029.9 ns)
   2176.4 |########################################
   2469.1 |
   2761.8 |
   3054.4 |
   3347.1 |##
   3639.8 |
   3932.5 |##
   4225.1 |
   4517.8 |
   4810.5 |
   5103.2 |######
   5395.8 |##
   5688.5 |##
   5981.2 |####
   6273.9 |##
   6566.5 |
   6859.2 |
   7151.9 |
   7444.6 |######
   7737.2 |##
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 1952.7-7292.4 ns)
   1952.7 |
   2219.6 |########################################
   2486.6 |##
   2753.6 |
   3020.6 |
   3287.6 |
   3554.6 |
   3821.6 |##
   4088.5 |
   4355.5 |
   4622.5 |
   4889.5 |
   5156.5 |
   5423.5 |##
   5690.4 |
   5957.4 |#####
   6224.4 |#####
   6491.4 |
   6758.4 |##########
   7025.4 |#######
  (7 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=34.0% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-head**: CV=23.2% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=55.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=56.7% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
