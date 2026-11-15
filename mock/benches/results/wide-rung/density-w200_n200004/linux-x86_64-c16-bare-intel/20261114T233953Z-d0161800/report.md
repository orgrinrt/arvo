# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.54)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-ragged is inconsistent: worst-20% is 2.0x its best-20%

wide-rung-ragged's best 20% of batches run at 7.89 us but its worst 20% at 15.65 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-wordround's edge over baseline is significant but tiny (-5 ns, 0.06%)

wide-rung-wordround differs from baseline wide-rung-align16 by -5 ns (0.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 8110.8 ns median (-9.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.11x (fastest 8110.8 ns, slowest 8997.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 9460ns | 8999ns | 7359ns | 8933ns | 13141ns | base |
| wide-rung-ragged | 10508ns | 9080ns | 7966ns | 9592ns | 15798ns | +11.08% |
| wide-rung-ragged-overread | 8322ns | 8316ns | 7448ns | 8322ns | 9194ns | -12.03% |
| wide-rung-wordround | 8185ns | 8205ns | 7355ns | 8184ns | 9016ns | -13.48% |
| wide-rung-wordround-alias | 9022ns | 9016ns | 8993ns | 9015ns | 9069ns | -4.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 9366ns | 7292ns | 12992ns | base | 1.093 |
| wide-rung-ragged | 10406ns | 7889ns | 15645ns | +11.11% | 0.984 |
| wide-rung-ragged-overread | 8241ns | 7371ns | 9109ns | -12.01% | 1.243 |
| wide-rung-wordround | 8105ns | 7287ns | 8920ns | -13.46% | 1.263 |
| wide-rung-wordround-alias | 8930ns | 8910ns | 8958ns | -4.65% | 1.147 |

## Performance model

- Peak throughput: **1.405 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 10240

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.148 | 81.7% |
| wide-rung-ragged | 1.138 | 81.0% |
| wide-rung-ragged-overread | 1.246 | 88.7% |
| wide-rung-wordround | 1.263 | 89.8% |
| wide-rung-wordround-alias | 1.147 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 9460ns | 9460ns | base |
| wide-rung-ragged | 10508ns | 10508ns | +11.08% |
| wide-rung-ragged-overread | 8322ns | 8322ns | -12.03% |
| wide-rung-wordround | 8185ns | 8185ns | -13.48% |
| wide-rung-wordround-alias | 9022ns | 9022ns | -4.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 8917ns | base | --- | [8916, 8918] | --- | --- | --- | --- |
| wide-rung-ragged | 8998ns | +90.2ns (+1.0%) | [+80, +348]ns | [8819, 9002] | YES | 0.0044 | 0.0022 | 0 |
| wide-rung-ragged-overread | 8215ns | no significant difference | [-1467, +124]ns | [7429, 9071] | no | 0.2682 | 0.2682 | 0 |
| wide-rung-wordround | 8111ns | -5.4ns (-0.1%) | [-1618, -2]ns | [7296, 8914] | YES | 0.0027 | 0.0007 | 0 |
| wide-rung-wordround-alias | 8927ns | +14.3ns (+0.2%) | [+2, +26]ns | [8920, 8937] | YES | 0.0086 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 8917ns | -1.1% | -16.6% | -18.2% | +0.1% |
| 1 | 2 | 8917ns | +0.9% | +1.7% | -0.0% | +0.2% |
| 1 | 3 | 11709ns | +33.6% | -22.2% | -23.9% | -23.7% |
| 1 | 4 | 7294ns | +15.4% | +1.5% | -0.0% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.539 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.440 | moderate- |
| wide-rung-ragged-overread | -0.250 | moderate- |
| wide-rung-wordround | -0.250 | moderate- |
| wide-rung-wordround-alias | -0.322 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 3/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 2/4, lost 0/4
- **wide-rung-wordround-alias**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 26.6ns | 9365.9ns | 0.3% |  |
| wide-rung-ragged | 29.2ns | 10406.3ns | 0.3% |  |
| wide-rung-ragged-overread | 22.7ns | 8240.9ns | 0.3% |  |
| wide-rung-wordround | 22.5ns | 8105.0ns | 0.3% |  |
| wide-rung-wordround-alias | 24.8ns | 8930.3ns | 0.3% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 7292.0-12991.6 ns)
   7292.0 |###########
   7577.0 |
   7861.9 |
   8146.9 |
   8431.9 |
   8716.9 |########################################
   9001.9 |#
   9286.8 |
   9571.8 |#
   9856.8 |
  10141.8 |
  10426.8 |
  10711.7 |
  10996.7 |
  11281.7 |
  11566.7 |
  11851.7 |
  12136.6 |
  12421.6 |
  12706.6 |
  (2 below, 5 above range)

wide-rung-ragged (n=40, range 7889.2-15645.1 ns)
   7889.2 |
   8277.0 |##############
   8664.8 |########################################
   9052.6 |####
   9440.4 |
   9828.2 |##
  10216.0 |
  10603.8 |
  10991.6 |
  11379.4 |
  11767.2 |
  12155.0 |
  12542.8 |
  12930.6 |
  13318.4 |
  13706.2 |
  14094.0 |
  14481.8 |
  14869.6 |
  15257.4 |###########
  (4 below, 5 above range)

wide-rung-ragged-overread (n=40, range 7371.1-9108.6 ns)
   7371.1 |########################################
   7458.0 |
   7544.9 |
   7631.7 |
   7718.6 |
   7805.5 |
   7892.4 |
   7979.2 |
   8066.1 |
   8153.0 |
   8239.9 |
   8326.7 |
   8413.6 |
   8500.5 |
   8587.4 |
   8674.3 |
   8761.1 |
   8848.0 |
   8934.9 |##
   9021.8 |################################
  (4 below, 6 above range)

wide-rung-wordround (n=40, range 7287.2-8919.8 ns)
   7287.2 |#####################################
   7368.8 |
   7450.4 |
   7532.1 |
   7613.7 |
   7695.3 |
   7776.9 |
   7858.6 |
   7940.2 |
   8021.8 |
   8103.5 |
   8185.1 |
   8266.7 |
   8348.3 |
   8430.0 |
   8511.6 |
   8593.2 |
   8674.9 |
   8756.5 |
   8838.1 |########################################
  (5 below, 4 above range)

wide-rung-wordround-alias (n=40, range 8910.4-8957.7 ns)
   8910.4 |
   8912.8 |###########
   8915.2 |########################################
   8917.5 |#################
   8919.9 |#################
   8922.3 |###########
   8924.6 |#####
   8927.0 |#####
   8929.3 |#####
   8931.7 |###########
   8934.1 |###########
   8936.4 |##################################
   8938.8 |#####
   8941.2 |############################
   8943.5 |#####
   8945.9 |
   8948.3 |
   8950.6 |
   8953.0 |
   8955.3 |
  (2 below, 1 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=24.9% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=29.5% (high variance, measurements may be unstable)
