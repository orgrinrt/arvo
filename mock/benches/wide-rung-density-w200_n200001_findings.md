# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround-alias is fastest but the noisiest (CV 31.0%)

wide-rung-wordround-alias wins on median (4.65 us) yet has the highest variance (CV 31.0%), while wide-rung-ragged is the steadiest (CV 2.0%, 4.95 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (wide-rung-wordround-alias, wide-rung-wordround) are a dead heat (<1%)

wide-rung-wordround-alias (4.65 us) and wide-rung-wordround (4.68 us) differ by 0.66%, inside the noise, even though the wider field spreads 8.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.89)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (385 ns) is smaller than the fastest variant's own run-to-run std-dev (1.44 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-ragged (+7% speed for 15.7x steadier)

wide-rung-wordround-alias is fastest (4.65 us, CV 31.0%); wide-rung-ragged gives up 6.6% median for 15.7x lower variance (CV 2.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-wordround-alias is inconsistent: worst-20% is 1.8x its best-20%

wide-rung-wordround-alias's best 20% of batches run at 4.56 us but its worst 20% at 8.01 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-wordround-alias's edge over baseline is significant but tiny (30 ns, 0.64%)

wide-rung-wordround-alias differs from baseline wide-rung-align16 by 30 ns (0.64%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 4646.0 ns median (-1.1% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.08x (fastest 4646.0 ns, slowest 5031.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5230ns | 4762ns | 4588ns | 4921ns | 6799ns | base |
| wide-rung-ragged | 5028ns | 5018ns | 4916ns | 5013ns | 5185ns | -3.86% |
| wide-rung-ragged-overread | 5134ns | 5094ns | 4936ns | 5061ns | 5550ns | -1.84% |
| wide-rung-wordround | 4993ns | 4739ns | 4612ns | 4814ns | 5911ns | -4.53% |
| wide-rung-wordround-alias | 5527ns | 4724ns | 4626ns | 4961ns | 8127ns | +5.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5155ns | 4529ns | 6706ns | base | 0.795 |
| wide-rung-ragged | 4964ns | 4856ns | 5115ns | -3.71% | 0.825 |
| wide-rung-ragged-overread | 5069ns | 4873ns | 5481ns | -1.68% | 0.808 |
| wide-rung-wordround | 4928ns | 4552ns | 5834ns | -4.42% | 0.831 |
| wide-rung-wordround-alias | 5450ns | 4565ns | 8013ns | +5.72% | 0.751 |

## Performance model

- Peak throughput: **0.904 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.872 | 96.4% |
| wide-rung-ragged | 0.827 | 91.4% |
| wide-rung-ragged-overread | 0.814 | 90.0% |
| wide-rung-wordround | 0.876 | 96.8% |
| wide-rung-wordround-alias | 0.882 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5230ns | 5230ns | base |
| wide-rung-ragged | 5028ns | 5028ns | -3.86% |
| wide-rung-ragged-overread | 5134ns | 5134ns | -1.84% |
| wide-rung-wordround | 4993ns | 4993ns | -4.53% |
| wide-rung-wordround-alias | 5527ns | 5527ns | +5.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4697ns | base | --- | [4669, 4722] | --- | --- | --- | --- |
| wide-rung-ragged | 4955ns | +202.9ns (+4.3%) | [+156, +272]ns | [4908, 4975] | YES | 0.0044 | 0.0022 | 0 |
| wide-rung-ragged-overread | 5031ns | +231.9ns (+4.9%) | [+183, +343]ns | [4949, 5043] | YES | 0.0044 | 0.0022 | 0 |
| wide-rung-wordround | 4677ns | no significant difference | [-150, +42]ns | [4621, 4741] | no | 0.6358 | 0.6358 | 0 |
| wide-rung-wordround-alias | 4646ns | +33.1ns (+0.7%) | [+14, +69]ns | [4600, 4723] | YES | 0.0221 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 6739ns | -25.2% | -3.2% | -9.4% | +26.0% |
| 2 | 6682ns | -21.9% | -10.2% | -8.4% | +27.1% |
| 3 | 6686ns | -23.7% | -19.9% | -9.0% | +26.5% |
| 4 | 6678ns | -24.1% | -22.8% | -15.3% | +25.3% |
| 5 | 6696ns | -23.1% | -24.4% | -15.7% | +12.1% |
| 6 | 6685ns | -25.7% | -18.6% | -15.0% | +14.0% |
| 7 | 6628ns | -23.9% | -23.3% | -14.0% | +14.0% |
| 8 | 6660ns | -24.0% | -21.5% | -14.9% | +14.1% |
| 9 | 6742ns | -23.5% | -26.2% | -16.8% | +11.6% |
| 10 | 6743ns | -27.9% | -24.9% | -16.5% | +11.7% |
| 11 | 4700ns | +5.3% | +7.1% | -0.3% | +0.4% |
| 12 | 4705ns | +5.4% | +7.2% | -2.4% | +0.6% |
| 13 | 4782ns | +3.3% | +5.2% | -5.6% | -2.2% |
| 14 | 4810ns | +2.9% | +4.6% | -5.7% | -4.5% |
| 15 | 4694ns | +5.5% | +7.5% | -3.2% | -2.8% |
| 16 | 4813ns | +3.0% | +4.8% | -4.9% | -5.1% |
| 17 | 4538ns | +9.4% | +11.2% | +0.9% | +1.2% |
| 18 | 4522ns | +9.8% | +11.5% | +1.5% | +1.3% |
| 19 | 4559ns | +8.8% | +8.5% | -0.1% | +0.4% |
| 20 | 4666ns | +4.3% | +4.4% | -2.3% | -2.0% |
| 21 | 4672ns | +5.4% | +7.8% | -2.5% | +0.6% |
| 22 | 4542ns | +7.1% | +11.0% | +2.0% | +0.8% |
| 23 | 4543ns | +6.7% | +10.7% | +1.4% | -0.4% |
| 24 | 4575ns | +6.1% | +10.0% | +0.9% | -0.4% |
| 25 | 4532ns | +7.3% | +8.3% | +2.2% | +1.0% |
| 26 | 4527ns | +8.0% | +9.7% | +3.0% | +2.0% |
| 27 | 4522ns | +9.3% | +7.9% | +3.6% | +1.7% |
| 28 | 4521ns | +11.5% | +7.8% | +3.4% | +2.0% |
| 29 | 4528ns | +11.2% | +7.6% | +2.1% | +1.6% |
| 30 | 4540ns | +12.9% | +8.6% | +2.4% | +1.5% |
| 31 | 4707ns | +6.7% | +7.2% | +0.8% | +0.4% |
| 32 | 4692ns | +7.1% | +6.0% | +0.8% | +1.2% |
| 33 | 4719ns | +5.6% | +4.9% | -0.9% | -0.2% |
| 34 | 4693ns | +3.3% | +4.6% | +0.3% | +0.2% |
| 35 | 4702ns | +3.3% | +3.6% | +1.3% | +0.5% |
| 36 | 4693ns | +3.6% | +3.8% | +1.0% | +0.1% |
| 37 | 4664ns | +4.4% | +4.6% | +2.8% | -1.4% |
| 38 | 4700ns | +3.5% | +3.8% | +1.4% | -1.8% |
| 39 | 4685ns | +3.6% | +4.0% | -1.6% | -1.8% |
| 40 | 4726ns | +2.8% | +3.7% | -3.2% | -2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.889 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.633 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.552 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.873 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.887 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 10/40, lost 30/40
- **wide-rung-ragged-overread**: won 10/40, lost 30/40
- **wide-rung-wordround**: won 21/40, lost 18/40
- **wide-rung-wordround-alias**: won 12/40, lost 28/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.4ns | 5155.3ns | 0.0% |  |
| wide-rung-ragged | 2.1ns | 4964.1ns | 0.0% |  |
| wide-rung-ragged-overread | 2.0ns | 5068.5ns | 0.0% |  |
| wide-rung-wordround | 2.0ns | 4927.5ns | 0.0% |  |
| wide-rung-wordround-alias | 2.2ns | 5450.5ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4528.9-6706.4 ns)
   4528.9 |##################
   4637.7 |########################################
   4746.6 |########
   4855.5 |
   4964.4 |
   5073.2 |
   5182.1 |
   5291.0 |
   5399.9 |
   5508.8 |
   5617.6 |
   5726.5 |
   5835.4 |
   5944.3 |
   6053.2 |
   6162.0 |
   6270.9 |
   6379.8 |
   6488.7 |
   6597.6 |##################
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 4856.0-5115.4 ns)
   4856.0 |########################################
   4869.0 |
   4882.0 |####
   4894.9 |
   4907.9 |
   4920.9 |####
   4933.8 |########
   4946.8 |######################
   4959.8 |#################
   4972.7 |####
   4985.7 |
   4998.7 |
   5011.6 |####
   5024.6 |########
   5037.6 |#############
   5050.6 |####
   5063.5 |####
   5076.5 |
   5089.5 |####
   5102.4 |
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 4872.8-5480.6 ns)
   4872.8 |##################
   4903.2 |#########
   4933.6 |######
   4964.0 |#########
   4994.4 |
   5024.8 |########################################
   5055.1 |#########
   5085.5 |
   5115.9 |
   5146.3 |###
   5176.7 |
   5207.1 |###
   5237.5 |
   5267.9 |
   5298.2 |
   5328.6 |###
   5359.0 |
   5389.4 |
   5419.8 |###
   5450.2 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 4552.4-5834.0 ns)
   4552.4 |########################################
   4616.5 |################################
   4680.5 |########################
   4744.6 |############
   4808.7 |
   4872.8 |
   4936.9 |
   5000.9 |
   5065.0 |
   5129.1 |
   5193.2 |
   5257.3 |
   5321.3 |
   5385.4 |
   5449.5 |
   5513.6 |
   5577.6 |########
   5641.7 |####################
   5705.8 |
   5769.9 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 4564.7-8013.4 ns)
   4564.7 |########################################
   4737.2 |#
   4909.6 |
   5082.0 |
   5254.5 |
   5426.9 |
   5599.3 |
   5771.8 |
   5944.2 |
   6116.6 |
   6289.1 |
   6461.5 |
   6633.9 |
   6806.3 |
   6978.8 |
   7151.2 |
   7323.6 |
   7496.1 |########
   7668.5 |
   7840.9 |
  (2 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: CV=26.4% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: autocorrelation=0.89 (measurement drift or warm-up artifact)
