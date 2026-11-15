# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged-overread, wide-rung-wordround) are a dead heat (<1%)

wide-rung-ragged-overread (3.96 us) and wide-rung-wordround (3.97 us) differ by 0.38%, inside the noise, even though the wider field spreads 6.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.79)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (-48 ns, 1.19%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by -48 ns (1.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 3958.3 ns median (-1.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 3958.3 ns, slowest 4213.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4124ns | 4088ns | 4065ns | 4100ns | 4254ns | base |
| wide-rung-ragged | 4328ns | 4276ns | 4269ns | 4300ns | 4470ns | +4.94% |
| wide-rung-ragged-overread | 4057ns | 4027ns | 4015ns | 4040ns | 4152ns | -1.61% |
| wide-rung-wordround | 4048ns | 4040ns | 4003ns | 4046ns | 4098ns | -1.84% |
| wide-rung-wordround-alias | 4124ns | 4080ns | 4008ns | 4080ns | 4372ns | +0.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4058ns | 4001ns | 4186ns | base | 2.019 |
| wide-rung-ragged | 4263ns | 4206ns | 4402ns | +5.04% | 1.922 |
| wide-rung-ragged-overread | 3992ns | 3951ns | 4087ns | -1.64% | 2.052 |
| wide-rung-wordround | 3984ns | 3940ns | 4034ns | -1.83% | 2.056 |
| wide-rung-wordround-alias | 4056ns | 3942ns | 4300ns | -0.06% | 2.020 |

## Performance model

- Peak throughput: **2.079 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.035 | 97.9% |
| wide-rung-ragged | 1.944 | 93.5% |
| wide-rung-ragged-overread | 2.070 | 99.5% |
| wide-rung-wordround | 2.062 | 99.2% |
| wide-rung-wordround-alias | 2.040 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4124ns | 4124ns | base |
| wide-rung-ragged | 4328ns | 4328ns | +4.94% |
| wide-rung-ragged-overread | 4057ns | 4057ns | -1.61% |
| wide-rung-wordround | 4048ns | 4048ns | -1.84% |
| wide-rung-wordround-alias | 4124ns | 4124ns | +0.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4026ns | base | --- | [4009, 4066] | --- | --- | --- | --- |
| wide-rung-ragged | 4213ns | +202.2ns (+5.0%) | [+181, +256]ns | [4209, 4271] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 3958ns | -54.4ns (-1.4%) | [-65, -51]ns | [3956, 3989] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 3974ns | -66.1ns (-1.6%) | [-106, -38]ns | [3961, 4011] | YES | 0.0002 | 0.0002 | 0 |
| wide-rung-wordround-alias | 4015ns | -47.7ns (-1.2%) | [-60, -24]ns | [3966, 4048] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 4013ns | +6.3% | +0.3% | +0.4% | +29.5% |
| 2 | 4007ns | +7.1% | +0.4% | +0.4% | +4.0% |
| 3 | 4025ns | +6.2% | -0.0% | +1.6% | +0.6% |
| 4 | 4012ns | +6.4% | +0.1% | +0.5% | +0.8% |
| 5 | 4005ns | +6.7% | -0.8% | +0.2% | +0.3% |
| 6 | 3990ns | +7.1% | -0.8% | -0.7% | +0.7% |
| 7 | 4005ns | +6.7% | -1.3% | +0.4% | -0.7% |
| 8 | 4006ns | +6.6% | -1.3% | -1.3% | -1.6% |
| 9 | 4008ns | +6.5% | -1.2% | -1.0% | -1.4% |
| 10 | 4000ns | +6.8% | -1.2% | -0.8% | -1.1% |
| 11 | 4070ns | +3.5% | -1.3% | -2.2% | -0.7% |
| 12 | 4075ns | +3.3% | -1.3% | -2.4% | -3.0% |
| 13 | 4151ns | +1.4% | -1.5% | -4.6% | -4.5% |
| 14 | 4086ns | +3.2% | -2.8% | -2.9% | -2.9% |
| 15 | 4075ns | +10.4% | -2.9% | -3.0% | -0.5% |
| 16 | 4069ns | +9.0% | -2.4% | -3.1% | -2.8% |
| 17 | 4062ns | +9.1% | -2.6% | -3.3% | -3.2% |
| 18 | 4063ns | +9.1% | -1.2% | -3.2% | -1.9% |
| 19 | 4068ns | +8.9% | -2.8% | -3.0% | -2.1% |
| 20 | 4026ns | +9.9% | -1.9% | -1.9% | -1.8% |
| 21 | 4219ns | -0.3% | -1.3% | -4.7% | -0.8% |
| 22 | 4222ns | -0.3% | -1.5% | -5.0% | -0.8% |
| 23 | 4231ns | +1.1% | -1.6% | -4.8% | -1.4% |
| 24 | 4232ns | +1.0% | -4.4% | -4.9% | -1.4% |
| 25 | 4219ns | -0.3% | -6.4% | -4.6% | -1.3% |
| 26 | 4125ns | +2.0% | -2.9% | -2.8% | +0.8% |
| 27 | 4053ns | +3.8% | -2.5% | -1.5% | +2.1% |
| 28 | 4000ns | +5.2% | -1.2% | -0.1% | +2.3% |
| 29 | 4007ns | +5.0% | -1.3% | +0.3% | -1.3% |
| 30 | 4008ns | +5.0% | -1.3% | +0.1% | -1.3% |
| 31 | 4083ns | +3.1% | -3.0% | -3.3% | -1.7% |
| 32 | 4017ns | +4.7% | -1.6% | -0.1% | +1.1% |
| 33 | 4022ns | +4.6% | -1.7% | -1.5% | +0.4% |
| 34 | 4027ns | +4.5% | -1.9% | -1.7% | -0.3% |
| 35 | 4032ns | +4.3% | -1.5% | -2.0% | +1.6% |
| 36 | 4005ns | +5.1% | -1.2% | -0.5% | -1.7% |
| 37 | 3998ns | +5.3% | -1.3% | -1.7% | -1.6% |
| 38 | 4007ns | +5.0% | +0.5% | -1.6% | -1.5% |
| 39 | 4003ns | +5.3% | -1.2% | -0.8% | -0.9% |
| 40 | 4010ns | +4.9% | -1.4% | -1.5% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.791 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.714 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.566 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.581 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.208 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 3/40, lost 37/40
- **wide-rung-ragged-overread**: won 35/40, lost 4/40
- **wide-rung-wordround**: won 31/40, lost 8/40
- **wide-rung-wordround-alias**: won 28/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.3ns | 4058.4ns | 0.1% |  |
| wide-rung-ragged | 2.3ns | 4262.8ns | 0.1% |  |
| wide-rung-ragged-overread | 2.0ns | 3991.9ns | 0.1% |  |
| wide-rung-wordround | 2.4ns | 3984.0ns | 0.1% |  |
| wide-rung-wordround-alias | 2.3ns | 4056.0ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4000.6-4185.6 ns)
   4000.6 |########################################
   4009.8 |##########
   4019.1 |##############
   4028.3 |###
   4037.6 |
   4046.8 |###
   4056.1 |#######
   4065.3 |##########
   4074.6 |##########
   4083.8 |###
   4093.1 |
   4102.4 |
   4111.6 |
   4120.9 |###
   4130.1 |
   4139.4 |
   4148.6 |###
   4157.9 |
   4167.1 |
   4176.4 |
  (4 below, 5 above range)

wide-rung-ragged (n=40, range 4206.0-4402.0 ns)
   4206.0 |########################################
   4215.8 |##
   4225.6 |
   4235.4 |
   4245.2 |
   4255.0 |
   4264.8 |######################
   4274.6 |##
   4284.4 |##
   4294.2 |
   4304.0 |
   4313.8 |
   4323.6 |
   4333.4 |
   4343.2 |
   4353.0 |
   4362.8 |
   4372.6 |
   4382.4 |
   4392.2 |
  (3 below, 6 above range)

wide-rung-ragged-overread (n=40, range 3950.7-4087.4 ns)
   3950.7 |########################################
   3957.5 |####
   3964.3 |##
   3971.2 |######
   3978.0 |
   3984.8 |
   3991.7 |
   3998.5 |##
   4005.4 |
   4012.2 |######
   4019.0 |######
   4025.9 |####
   4032.7 |
   4039.5 |
   4046.4 |##
   4053.2 |
   4060.0 |
   4066.9 |
   4073.7 |
   4080.5 |
  (2 below, 4 above range)

wide-rung-wordround (n=40, range 3940.5-4034.2 ns)
   3940.5 |################
   3945.1 |################
   3949.8 |################################
   3954.5 |################
   3959.2 |########################
   3963.9 |################
   3968.6 |################
   3973.3 |########
   3978.0 |########
   3982.6 |########
   3987.3 |
   3992.0 |########
   3996.7 |########
   4001.4 |
   4006.1 |
   4010.8 |########################################
   4015.5 |########
   4020.1 |################################
   4024.8 |########################
   4029.5 |########
  (3 below, 1 above range)

wide-rung-wordround-alias (n=40, range 3942.4-4300.0 ns)
   3942.4 |########################################
   3960.3 |#############
   3978.1 |#############
   3996.0 |
   4013.9 |#################
   4031.8 |#############
   4049.7 |#############
   4067.6 |
   4085.4 |########
   4103.3 |
   4121.2 |####
   4139.1 |
   4157.0 |######################
   4174.9 |########
   4192.7 |
   4210.6 |
   4228.5 |
   4246.4 |
   4264.3 |
   4282.2 |
  (4 below, 1 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.58 (measurement drift or warm-up artifact)
