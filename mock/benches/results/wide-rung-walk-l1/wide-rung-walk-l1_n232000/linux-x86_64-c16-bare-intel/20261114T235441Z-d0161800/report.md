# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround-alias is fastest but the noisiest (CV 105.8%)

wide-rung-wordround-alias wins on median (1.86 us) yet has the highest variance (CV 105.8%), while wide-rung-ragged is the steadiest (CV 93.2%, 2.22 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (wide-rung-wordround-alias, wide-rung-wordround) are a dead heat (<1%)

wide-rung-wordround-alias (1.86 us) and wide-rung-wordround (1.86 us) differ by 0.13%, inside the noise, even though the wider field spreads 19.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (358 ns) is smaller than the fastest variant's own run-to-run std-dev (1.97 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-wordround is inconsistent: worst-20% is 3.5x its best-20%

wide-rung-wordround's best 20% of batches run at 1.76 us but its worst 20% at 6.26 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1862.1 ns median (-0.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.19x (fastest 1862.1 ns, slowest 2220.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3553ns | 1954ns | 1941ns | 3179ns | 6287ns | base |
| wide-rung-ragged | 3761ns | 2304ns | 2297ns | 3010ns | 7477ns | +5.85% |
| wide-rung-ragged-overread | 3686ns | 2006ns | 1935ns | 3265ns | 6699ns | +3.74% |
| wide-rung-wordround | 3523ns | 1950ns | 1849ns | 3049ns | 6620ns | -0.83% |
| wide-rung-wordround-alias | 3417ns | 1945ns | 1939ns | 2749ns | 6899ns | -3.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3387ns | 1857ns | 5953ns | base | 0.605 |
| wide-rung-ragged | 3609ns | 2213ns | 7132ns | +6.56% | 0.567 |
| wide-rung-ragged-overread | 3518ns | 1853ns | 6361ns | +3.89% | 0.582 |
| wide-rung-wordround | 3352ns | 1764ns | 6258ns | -1.01% | 0.611 |
| wide-rung-wordround-alias | 3262ns | 1857ns | 6556ns | -3.70% | 0.628 |

## Performance model

- Peak throughput: **1.161 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.095 | 94.4% |
| wide-rung-ragged | 0.922 | 79.5% |
| wide-rung-ragged-overread | 1.065 | 91.7% |
| wide-rung-wordround | 1.098 | 94.6% |
| wide-rung-wordround-alias | 1.100 | 94.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3553ns | 3553ns | base |
| wide-rung-ragged | 3761ns | 3761ns | +5.85% |
| wide-rung-ragged-overread | 3686ns | 3686ns | +3.74% |
| wide-rung-wordround | 3523ns | 3523ns | -0.83% |
| wide-rung-wordround-alias | 3417ns | 3417ns | -3.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1870ns | base | --- | [1859, 5004] | --- | --- | --- | --- |
| wide-rung-ragged | 2220ns | +355.4ns (+19.0%) | [+354, +368]ns | [2214, 2495] | YES | 0.0027 | 0.0007 | 0 |
| wide-rung-ragged-overread | 1924ns | +59.8ns (+3.2%) | [+48, +61]ns | [1918, 5272] | YES | 0.0332 | 0.0166 | 0 |
| wide-rung-wordround | 1864ns | no significant difference | [-9, +48]ns | [1859, 4244] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround-alias | 1862ns | no significant difference | [-2, +12]ns | [1859, 3251] | no | 0.2051 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1862ns | +18.8% | +183.0% | +1.9% | +54.4% |
| 1 | 2 | 1858ns | +19.9% | +3.2% | -6.4% | +27.4% |
| 1 | 3 | 4900ns | -49.1% | -50.1% | -48.2% | -62.1% |
| 1 | 4 | 4363ns | -49.2% | -56.0% | -57.2% | -57.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.188 | ok |
| wide-rung-ragged | -0.402 | moderate- |
| wide-rung-ragged-overread | -0.188 | ok |
| wide-rung-wordround | -0.490 | moderate- |
| wide-rung-wordround-alias | 0.250 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 45.2ns | 3386.8ns | 1.3% |  |
| wide-rung-ragged | 40.3ns | 3609.0ns | 1.1% |  |
| wide-rung-ragged-overread | 46.1ns | 3518.4ns | 1.3% |  |
| wide-rung-wordround | 46.2ns | 3352.4ns | 1.4% |  |
| wide-rung-wordround-alias | 42.8ns | 3261.7ns | 1.3% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1856.8-5953.1 ns)
   1856.8 |########################################
   2061.6 |
   2266.5 |
   2471.3 |
   2676.1 |
   2880.9 |
   3085.7 |
   3290.5 |
   3495.3 |#
   3700.2 |
   3905.0 |
   4109.8 |
   4314.6 |
   4519.4 |#
   4724.2 |
   4929.0 |###
   5133.9 |###
   5338.7 |###
   5543.5 |#######
   5748.3 |#
  (1 below, 4 above range)

wide-rung-ragged (n=40, range 2212.8-7132.3 ns)
   2212.8 |########################################
   2458.8 |
   2704.8 |#
   2950.8 |
   3196.7 |
   3442.7 |
   3688.7 |
   3934.6 |#
   4180.6 |
   4426.6 |
   4672.6 |
   4918.5 |#
   5164.5 |
   5410.5 |#
   5656.4 |
   5902.4 |
   6148.4 |###
   6394.4 |
   6640.3 |#####
   6886.3 |#
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 1852.7-6360.9 ns)
   1852.7 |########################################
   2078.1 |
   2303.6 |##
   2529.0 |
   2754.4 |##
   2979.8 |
   3205.2 |
   3430.6 |
   3656.0 |
   3881.4 |##
   4106.8 |
   4332.2 |
   4557.7 |
   4783.1 |
   5008.5 |
   5233.9 |######
   5459.3 |####
   5684.7 |##
   5910.1 |######
   6135.5 |########
  (3 below, 2 above range)

wide-rung-wordround (n=40, range 1764.3-6258.3 ns)
   1764.3 |########################################
   1989.0 |
   2213.7 |
   2438.4 |
   2663.1 |
   2887.8 |
   3112.5 |##
   3337.2 |
   3561.9 |
   3786.6 |
   4011.3 |##
   4236.0 |##
   4460.7 |
   4685.4 |
   4910.1 |
   5134.8 |
   5359.5 |####
   5584.2 |####
   5808.9 |######
   6033.6 |########
  (6 below, 2 above range)

wide-rung-wordround-alias (n=40, range 1857.0-6556.4 ns)
   1857.0 |########################################
   2091.9 |
   2326.9 |
   2561.9 |
   2796.8 |#
   3031.8 |
   3266.8 |
   3501.8 |#
   3736.7 |#
   3971.7 |#
   4206.7 |
   4441.6 |
   4676.6 |
   4911.6 |
   5146.6 |
   5381.5 |#
   5616.5 |###
   5851.5 |
   6086.4 |###
   6321.4 |#
  (2 below, 5 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=53.9% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **wide-rung-ragged**: CV=57.3% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=55.9% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **wide-rung-wordround**: CV=58.1% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=60.4% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.5x (possible bimodal distribution)
