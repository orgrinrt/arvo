# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround-alias is fastest but the noisiest (CV 17.8%)

wide-rung-wordround-alias wins on median (6.88 us) yet has the highest variance (CV 17.8%), while wide-rung-wordround is the steadiest (CV 0.1%, 6.89 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (142 ns) is smaller than the fastest variant's own run-to-run std-dev (1.23 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-wordround (+0% speed for 210.0x steadier)

wide-rung-wordround-alias is fastest (6.88 us, CV 17.8%); wide-rung-wordround gives up 0.0% median for 210.0x lower variance (CV 0.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.1% of the fastest

All 5 variants sit between 6.88 us and 7.03 us - a 2.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 6884.9 ns median (-0.0% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.02x (fastest 6884.9 ns, slowest 7027.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 7020ns | 6969ns | 6956ns | 6968ns | 7241ns | base |
| wide-rung-ragged | 7139ns | 7037ns | 7025ns | 7036ns | 7561ns | +1.69% |
| wide-rung-ragged-overread | 6807ns | 7108ns | 5814ns | 7000ns | 7221ns | -3.04% |
| wide-rung-wordround | 6972ns | 6969ns | 6957ns | 6968ns | 6999ns | -0.68% |
| wide-rung-wordround-alias | 7169ns | 6967ns | 6948ns | 6965ns | 8002ns | +2.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 6930ns | 6874ns | 7123ns | base | 1.182 |
| wide-rung-ragged | 7049ns | 6942ns | 7440ns | +1.71% | 1.162 |
| wide-rung-ragged-overread | 6724ns | 5748ns | 7119ns | -2.97% | 1.218 |
| wide-rung-wordround | 6885ns | 6875ns | 6890ns | -0.66% | 1.190 |
| wide-rung-wordround-alias | 7079ns | 6865ns | 7880ns | +2.15% | 1.157 |

## Performance model

- Peak throughput: **1.425 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.189 | 83.5% |
| wide-rung-ragged | 1.178 | 82.6% |
| wide-rung-ragged-overread | 1.166 | 81.8% |
| wide-rung-wordround | 1.189 | 83.5% |
| wide-rung-wordround-alias | 1.190 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 7020ns | 7020ns | base |
| wide-rung-ragged | 7139ns | 7139ns | +1.69% |
| wide-rung-ragged-overread | 6807ns | 6807ns | -3.04% |
| wide-rung-wordround | 6972ns | 6972ns | -0.68% |
| wide-rung-wordround-alias | 7169ns | 7169ns | +2.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 6887ns | base | --- | [6885, 6888] | --- | --- | --- | --- |
| wide-rung-ragged | 6955ns | +69.0ns (+1.0%) | [+66, +71]ns | [6954, 6956] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 7027ns | +140.1ns (+2.0%) | [+128, +143]ns | [7016, 7029] | YES | 0.0129 | 0.0064 | 0 |
| wide-rung-wordround | 6887ns | no significant difference | [-2, +2]ns | [6887, 6888] | no | 0.6965 | 0.5224 | 1 |
| wide-rung-wordround-alias | 6885ns | no significant difference | [-6, +3]ns | [6877, 6888] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 6888ns | +1.0% | +2.0% | -0.0% | -0.0% |
| 1 | 2 | 6885ns | +1.0% | +2.1% | +0.0% | -0.2% |
| 1 | 3 | 6886ns | +1.0% | -16.5% | +0.0% | -0.0% |
| 1 | 4 | 6888ns | +1.0% | +2.1% | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.244 | moderate- |
| wide-rung-ragged | -0.242 | moderate- |
| wide-rung-ragged-overread | -0.418 | moderate- |
| wide-rung-wordround | -0.379 | moderate- |
| wide-rung-wordround-alias | -0.366 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 0/4, lost 4/4
- **wide-rung-ragged-overread**: won 1/4, lost 3/4
- **wide-rung-wordround**: won 0/4, lost 0/4
- **wide-rung-wordround-alias**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 24.0ns | 6930.2ns | 0.3% |  |
| wide-rung-ragged | 24.3ns | 7048.8ns | 0.3% |  |
| wide-rung-ragged-overread | 22.9ns | 6724.4ns | 0.3% |  |
| wide-rung-wordround | 23.8ns | 6884.7ns | 0.3% |  |
| wide-rung-wordround-alias | 24.7ns | 7079.0ns | 0.3% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 6873.6-7122.7 ns)
   6873.6 |##########################
   6886.1 |########################################
   6898.5 |
   6911.0 |#
   6923.4 |
   6935.9 |
   6948.3 |
   6960.8 |
   6973.3 |
   6985.7 |
   6998.2 |
   7010.6 |
   7023.1 |
   7035.5 |
   7048.0 |
   7060.5 |
   7072.9 |
   7085.4 |
   7097.8 |
   7110.3 |
  (3 below, 1 above range)

wide-rung-ragged (n=40, range 6942.2-7440.1 ns)
   6942.2 |########################################
   6967.1 |##
   6992.0 |
   7016.9 |
   7041.8 |
   7066.7 |
   7091.6 |
   7116.5 |
   7141.4 |
   7166.3 |
   7191.2 |
   7216.1 |
   7241.0 |
   7265.8 |
   7290.7 |
   7315.6 |
   7340.5 |
   7365.4 |
   7390.3 |
   7415.2 |
  (3 below, 1 above range)

wide-rung-ragged-overread (n=40, range 5747.7-7118.8 ns)
   5747.7 |########
   5816.3 |
   5884.8 |
   5953.4 |
   6021.9 |
   6090.5 |
   6159.0 |
   6227.6 |
   6296.2 |
   6364.7 |
   6433.3 |
   6501.8 |
   6570.4 |
   6638.9 |
   6707.5 |
   6776.1 |
   6844.6 |
   6913.2 |
   6981.7 |########################################
   7050.3 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 6875.4-6890.5 ns)
   6875.4 |######################
   6876.1 |###########
   6876.9 |
   6877.7 |
   6878.4 |
   6879.2 |
   6879.9 |
   6880.7 |
   6881.4 |
   6882.2 |
   6882.9 |
   6883.7 |###########
   6884.4 |
   6885.2 |
   6886.0 |###########
   6886.7 |##################################
   6887.5 |########################################
   6888.2 |############################
   6889.0 |#################
   6889.7 |#####
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 6865.3-7880.0 ns)
   6865.3 |########################################
   6916.1 |#
   6966.8 |
   7017.5 |
   7068.3 |
   7119.0 |
   7169.7 |
   7220.5 |
   7271.2 |
   7321.9 |
   7372.6 |
   7423.4 |
   7474.1 |
   7524.8 |
   7575.6 |
   7626.3 |
   7677.0 |
   7727.8 |
   7778.5 |
   7829.2 |
  (4 below, 1 above range)

```
