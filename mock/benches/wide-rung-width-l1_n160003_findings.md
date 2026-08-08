# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged-overread, wide-rung-align16) are a dead heat (<1%)

wide-rung-ragged-overread (3.99 us) and wide-rung-align16 (4.02 us) differ by 0.82%, inside the noise, even though the wider field spreads 3.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.80)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.7% of the fastest

All 5 variants sit between 3.99 us and 4.13 us - a 3.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-wordround's edge over baseline is significant but tiny (-25 ns, 0.63%)

wide-rung-wordround differs from baseline wide-rung-align16 by -25 ns (0.63%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 3988.8 ns median (-0.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 3988.8 ns, slowest 4134.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4109ns | 4084ns | 3983ns | 4107ns | 4242ns | base |
| wide-rung-ragged | 4217ns | 4197ns | 4117ns | 4191ns | 4392ns | +2.62% |
| wide-rung-ragged-overread | 4025ns | 4053ns | 3918ns | 4025ns | 4129ns | -2.05% |
| wide-rung-wordround | 4129ns | 4190ns | 3971ns | 4158ns | 4198ns | +0.49% |
| wide-rung-wordround-alias | 4074ns | 4102ns | 3909ns | 4085ns | 4209ns | -0.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4044ns | 3921ns | 4174ns | base | 2.026 |
| wide-rung-ragged | 4153ns | 4057ns | 4324ns | +2.70% | 1.972 |
| wide-rung-ragged-overread | 3960ns | 3858ns | 4061ns | -2.08% | 2.069 |
| wide-rung-wordround | 4064ns | 3910ns | 4133ns | +0.49% | 2.016 |
| wide-rung-wordround-alias | 4008ns | 3847ns | 4138ns | -0.89% | 2.044 |

## Performance model

- Peak throughput: **2.129 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.037 | 95.7% |
| wide-rung-ragged | 1.981 | 93.0% |
| wide-rung-ragged-overread | 2.054 | 96.4% |
| wide-rung-wordround | 1.986 | 93.3% |
| wide-rung-wordround-alias | 2.031 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4109ns | 4109ns | base |
| wide-rung-ragged | 4217ns | 4217ns | +2.62% |
| wide-rung-ragged-overread | 4025ns | 4025ns | -2.05% |
| wide-rung-wordround | 4129ns | 4129ns | +0.49% |
| wide-rung-wordround-alias | 4074ns | 4074ns | -0.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4021ns | base | --- | [4013, 4060] | --- | --- | --- | --- |
| wide-rung-ragged | 4135ns | +96.5ns (+2.4%) | [+46, +177]ns | [4060, 4199] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-ragged-overread | 3989ns | -102.2ns (-2.5%) | [-150, -56]ns | [3918, 3994] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 4124ns | no significant difference | [-42, +114]ns | [4121, 4128] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround-alias | 4034ns | no significant difference | [-162, +35]ns | [3988, 4090] | no | 0.2051 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 4018ns | +0.9% | -4.0% | -1.2% | -4.2% |
| 2 | 4008ns | +1.3% | -3.8% | +2.9% | -3.9% |
| 3 | 4018ns | +1.0% | -1.1% | +2.8% | -4.0% |
| 4 | 4025ns | +0.8% | -4.1% | +2.6% | -4.4% |
| 5 | 4013ns | +1.1% | -2.5% | +2.9% | -4.2% |
| 6 | 4004ns | +1.3% | -2.2% | +3.0% | -3.9% |
| 7 | 3991ns | +1.7% | -1.7% | +3.4% | -3.6% |
| 8 | 4013ns | +1.2% | -3.7% | +2.9% | -4.1% |
| 9 | 4008ns | +4.3% | -3.7% | +3.0% | -4.0% |
| 10 | 4012ns | +1.2% | -3.8% | +2.7% | -4.1% |
| 11 | 4015ns | +1.1% | -0.6% | -2.2% | +1.2% |
| 12 | 4084ns | -0.7% | -2.1% | -3.9% | -1.4% |
| 13 | 4056ns | +0.0% | +0.2% | -3.2% | -1.7% |
| 14 | 4063ns | -0.2% | -3.6% | -3.3% | -0.2% |
| 15 | 4032ns | +0.7% | -4.2% | -2.7% | +1.0% |
| 16 | 4003ns | +1.4% | -3.5% | -2.0% | +1.1% |
| 17 | 4259ns | -4.7% | -8.0% | -7.9% | -5.1% |
| 18 | 4045ns | +3.1% | -4.6% | -3.9% | +1.5% |
| 19 | 4029ns | +5.4% | -4.1% | -2.5% | -4.3% |
| 20 | 4076ns | +3.3% | -5.4% | -5.7% | -5.6% |
| 21 | 4157ns | +6.9% | -3.9% | -0.7% | -3.5% |
| 22 | 4157ns | +4.7% | -1.5% | -0.8% | -4.0% |
| 23 | 4182ns | +4.0% | -1.8% | -1.2% | -4.6% |
| 24 | 4162ns | +4.5% | -3.5% | -0.9% | -3.9% |
| 25 | 4165ns | +4.4% | -3.9% | -0.8% | -4.2% |
| 26 | 4151ns | +3.4% | -3.7% | -0.7% | -4.2% |
| 27 | 4135ns | -0.0% | -2.5% | -0.2% | -0.6% |
| 28 | 4142ns | -0.0% | -1.3% | -0.5% | -0.2% |
| 29 | 4156ns | -0.6% | -2.6% | -3.1% | -2.0% |
| 30 | 4156ns | -0.5% | -3.7% | -4.2% | +0.7% |
| 31 | 4018ns | +4.5% | -0.6% | +2.8% | +2.7% |
| 32 | 4008ns | +4.8% | -0.4% | +3.2% | +3.1% |
| 33 | 4016ns | +4.5% | -0.8% | +2.9% | +2.8% |
| 34 | 4028ns | +4.3% | -1.1% | +2.5% | +2.6% |
| 35 | 3915ns | +7.3% | +1.9% | +5.4% | +5.4% |
| 36 | 3874ns | +8.4% | +3.0% | +6.5% | +6.7% |
| 37 | 3918ns | +7.4% | +1.9% | +5.4% | +5.1% |
| 38 | 3917ns | +7.3% | +3.1% | +5.3% | +5.3% |
| 39 | 3872ns | +4.8% | +3.3% | +6.9% | +6.8% |
| 40 | 3874ns | +4.9% | +3.1% | +6.6% | +6.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.701 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.801 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.675 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.725 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.788 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 5/40, lost 32/40
- **wide-rung-ragged-overread**: won 33/40, lost 7/40
- **wide-rung-wordround**: won 21/40, lost 19/40
- **wide-rung-wordround-alias**: won 25/40, lost 15/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.2ns | 4044.3ns | 0.1% |  |
| wide-rung-ragged | 2.0ns | 4153.4ns | 0.0% |  |
| wide-rung-ragged-overread | 2.0ns | 3960.2ns | 0.1% |  |
| wide-rung-wordround | 2.2ns | 4063.9ns | 0.1% |  |
| wide-rung-wordround-alias | 2.3ns | 4008.2ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3920.5-4174.3 ns)
   3920.5 |
   3933.2 |
   3945.9 |
   3958.6 |
   3971.3 |
   3984.0 |#####
   3996.6 |#########################
   4009.3 |########################################
   4022.0 |####################
   4034.7 |#####
   4047.4 |#####
   4060.1 |#####
   4072.8 |##########
   4085.5 |
   4098.1 |
   4110.8 |
   4123.5 |#####
   4136.2 |#####
   4148.9 |#########################
   4161.6 |##########
  (6 below, 2 above range)

wide-rung-ragged (n=40, range 4056.6-4323.9 ns)
   4056.6 |########################################
   4070.0 |
   4083.4 |
   4096.7 |
   4110.1 |
   4123.5 |#######
   4136.8 |##
   4150.2 |
   4163.6 |##
   4176.9 |##
   4190.3 |#################
   4203.7 |#####
   4217.0 |
   4230.4 |
   4243.8 |##
   4257.1 |
   4270.5 |
   4283.9 |##
   4297.2 |
   4310.6 |
  (2 below, 5 above range)

wide-rung-ragged-overread (n=40, range 3858.2-4061.1 ns)
   3858.2 |###################################
   3868.4 |
   3878.5 |
   3888.6 |
   3898.8 |
   3908.9 |###############
   3919.1 |##########
   3929.2 |
   3939.4 |
   3949.5 |
   3959.7 |
   3969.8 |#####
   3979.9 |####################
   3990.1 |########################################
   4000.2 |###############
   4010.4 |#####
   4020.5 |
   4030.7 |##########
   4040.8 |#####
   4050.9 |
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 3909.5-4132.6 ns)
   3909.5 |
   3920.7 |#############
   3931.8 |
   3943.0 |
   3954.1 |
   3965.3 |#
   3976.5 |#
   3987.6 |
   3998.8 |
   4009.9 |
   4021.1 |#
   4032.2 |
   4043.4 |
   4054.5 |
   4065.7 |
   4076.9 |
   4088.0 |
   4099.2 |
   4110.3 |###
   4121.5 |########################################
  (2 below, 2 above range)

wide-rung-wordround-alias (n=40, range 3847.0-4137.9 ns)
   3847.0 |###################################
   3861.5 |
   3876.1 |
   3890.6 |
   3905.2 |
   3919.7 |
   3934.3 |
   3948.8 |
   3963.3 |####
   3977.9 |#################
   3992.4 |####
   4007.0 |####
   4021.5 |####
   4036.1 |########
   4050.6 |########
   4065.2 |########
   4079.7 |
   4094.3 |####
   4108.8 |#############
   4123.3 |########################################
  (4 below, 1 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.79 (measurement drift or warm-up artifact)
