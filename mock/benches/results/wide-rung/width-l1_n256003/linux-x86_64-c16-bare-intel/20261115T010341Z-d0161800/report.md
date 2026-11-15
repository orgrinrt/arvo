# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread shows alternating (throttle bounce) (autocorr -0.63)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8 ns) is smaller than the fastest variant's own run-to-run std-dev (597 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-wordround vs stability leader wide-rung-wordround-alias (+0% speed for 26.7x steadier)

wide-rung-wordround is fastest (6.88 us, CV 8.7%); wide-rung-wordround-alias gives up 0.1% median for 26.7x lower variance (CV 0.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.1% of the fastest

All 5 variants sit between 6.88 us and 6.89 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-wordround's edge over baseline is significant but tiny (-10 ns, 0.15%)

wide-rung-wordround differs from baseline wide-rung-align16 by -10 ns (0.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 6883.0 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 6883.0 ns, slowest 6890.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 7015ns | 6973ns | 6959ns | 6971ns | 7205ns | base |
| wide-rung-ragged | 6789ns | 6972ns | 5700ns | 6867ns | 7643ns | -3.23% |
| wide-rung-ragged-overread | 7071ns | 6973ns | 6959ns | 6972ns | 7478ns | +0.79% |
| wide-rung-wordround | 6690ns | 6965ns | 5697ns | 6863ns | 7164ns | -4.64% |
| wide-rung-wordround-alias | 6979ns | 6973ns | 6958ns | 6971ns | 7024ns | -0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 6926ns | 6877ns | 7091ns | base | 1.183 |
| wide-rung-ragged | 6700ns | 5629ns | 7523ns | -3.27% | 1.223 |
| wide-rung-ragged-overread | 6979ns | 6876ns | 7358ns | +0.77% | 1.174 |
| wide-rung-wordround | 6605ns | 5629ns | 7056ns | -4.64% | 1.240 |
| wide-rung-wordround-alias | 6892ns | 6877ns | 6918ns | -0.49% | 1.189 |

## Performance model

- Peak throughput: **1.455 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.189 | 81.7% |
| wide-rung-ragged | 1.189 | 81.7% |
| wide-rung-ragged-overread | 1.189 | 81.7% |
| wide-rung-wordround | 1.190 | 81.8% |
| wide-rung-wordround-alias | 1.189 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 7015ns | 7015ns | base |
| wide-rung-ragged | 6789ns | 6789ns | -3.23% |
| wide-rung-ragged-overread | 7071ns | 7071ns | +0.79% |
| wide-rung-wordround | 6690ns | 6690ns | -4.64% |
| wide-rung-wordround-alias | 6979ns | 6979ns | -0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 6890ns | base | --- | [6884, 6891] | --- | --- | --- | --- |
| wide-rung-ragged | 6889ns | no significant difference | [-10, +0]ns | [6877, 6891] | no | 0.1614 | 0.0730 | 2 |
| wide-rung-ragged-overread | 6889ns | no significant difference | [-5, +3]ns | [6884, 6890] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround | 6883ns | -10.2ns (-0.1%) | [-13, -0]ns | [6879, 6888] | YES (adj: no) | 0.1614 | 0.0807 | 0 |
| wide-rung-wordround-alias | 6891ns | no significant difference | [-3, +4]ns | [6886, 6892] | no | 0.5728 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 6891ns | -0.0% | -0.0% | +0.0% | +0.0% |
| 1 | 2 | 6884ns | -18.2% | +0.1% | -0.0% | +0.1% |
| 1 | 3 | 6891ns | +0.0% | -0.1% | +0.0% | +0.0% |
| 1 | 4 | 6891ns | -0.0% | +0.0% | -18.3% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.419 | moderate- |
| wide-rung-ragged | -0.417 | moderate- |
| wide-rung-ragged-overread | -0.631 | HIGH- (thermal bounce) |
| wide-rung-wordround | -0.087 | ok |
| wide-rung-wordround-alias | -0.302 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 0/4
- **wide-rung-ragged-overread**: won 0/4, lost 0/4
- **wide-rung-wordround**: won 1/4, lost 0/4
- **wide-rung-wordround-alias**: won 0/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 24.0ns | 6926.1ns | 0.3% |  |
| wide-rung-ragged | 23.3ns | 6699.8ns | 0.3% |  |
| wide-rung-ragged-overread | 24.3ns | 6979.2ns | 0.3% |  |
| wide-rung-wordround | 23.2ns | 6605.0ns | 0.4% |  |
| wide-rung-wordround-alias | 23.9ns | 6892.0ns | 0.3% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 6877.0-7090.5 ns)
   6877.0 |##############################
   6887.7 |########################################
   6898.4 |##
   6909.0 |##
   6919.7 |
   6930.4 |
   6941.1 |
   6951.7 |
   6962.4 |
   6973.1 |
   6983.8 |
   6994.4 |
   7005.1 |
   7015.8 |
   7026.5 |
   7037.1 |
   7047.8 |
   7058.5 |
   7069.2 |
   7079.8 |
  (2 below, 1 above range)

wide-rung-ragged (n=40, range 5628.9-7523.0 ns)
   5628.9 |#########
   5723.6 |
   5818.3 |
   5913.0 |
   6007.7 |
   6102.4 |
   6197.1 |
   6291.8 |
   6386.5 |
   6481.2 |
   6575.9 |
   6670.6 |
   6765.3 |
   6860.0 |########################################
   6954.7 |
   7049.5 |
   7144.2 |
   7238.9 |
   7333.6 |
   7428.3 |
  (3 below, 1 above range)

wide-rung-ragged-overread (n=40, range 6876.5-7358.4 ns)
   6876.5 |########################################
   6900.6 |#
   6924.7 |
   6948.8 |
   6972.9 |
   6997.0 |
   7021.1 |
   7045.2 |
   7069.3 |
   7093.3 |
   7117.4 |
   7141.5 |
   7165.6 |
   7189.7 |
   7213.8 |
   7237.9 |
   7262.0 |
   7286.1 |
   7310.2 |
   7334.3 |
  (2 below, 1 above range)

wide-rung-wordround (n=40, range 5628.6-7056.2 ns)
   5628.6 |########
   5700.0 |
   5771.4 |
   5842.8 |
   5914.2 |
   5985.5 |
   6056.9 |
   6128.3 |
   6199.7 |
   6271.1 |
   6342.4 |
   6413.8 |
   6485.2 |
   6556.6 |
   6627.9 |
   6699.3 |
   6770.7 |
   6842.1 |########################################
   6913.5 |
   6984.8 |
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 6876.6-6917.5 ns)
   6876.6 |##########
   6878.7 |#############
   6880.7 |###
   6882.8 |
   6884.8 |####################
   6886.9 |
   6888.9 |#############
   6891.0 |########################################
   6893.0 |#############
   6895.0 |
   6897.1 |
   6899.1 |
   6901.2 |###
   6903.2 |
   6905.3 |
   6907.3 |
   6909.3 |
   6911.4 |
   6913.4 |
   6915.5 |
  (3 below, 2 above range)

```
