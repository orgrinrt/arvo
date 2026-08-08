# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (4.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 4.00 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-wordround-alias, wide-rung-wordround) are a dead heat (<1%)

wide-rung-wordround-alias (4.00 us) and wide-rung-wordround (4.01 us) differ by 0.02%, inside the noise, even though the wider field spreads 5.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged-overread shows warm-up / thermal drift (autocorr +0.84)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 5.0% of the fastest

All 5 variants sit between 4.00 us and 4.20 us - a 5.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged's edge over baseline is significant but tiny (-24 ns, 0.57%)

wide-rung-ragged differs from baseline wide-rung-align16 by -24 ns (0.57%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 4005.0 ns median (-4.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.05x (fastest 4005.0 ns, slowest 4203.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4281ns | 4268ns | 4144ns | 4292ns | 4386ns | base |
| wide-rung-ragged | 4151ns | 4095ns | 4081ns | 4146ns | 4238ns | -3.03% |
| wide-rung-ragged-overread | 4249ns | 4254ns | 4085ns | 4258ns | 4389ns | -0.74% |
| wide-rung-wordround | 4209ns | 4071ns | 4054ns | 4125ns | 4615ns | -1.69% |
| wide-rung-wordround-alias | 4131ns | 4068ns | 4051ns | 4102ns | 4296ns | -3.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4213ns | 4078ns | 4316ns | base | 1.458 |
| wide-rung-ragged | 4088ns | 4019ns | 4171ns | -2.98% | 1.503 |
| wide-rung-ragged-overread | 4183ns | 4022ns | 4321ns | -0.71% | 1.469 |
| wide-rung-wordround | 4145ns | 3993ns | 4546ns | -1.62% | 1.482 |
| wide-rung-wordround-alias | 4069ns | 3991ns | 4232ns | -3.42% | 1.510 |

## Performance model

- Peak throughput: **1.539 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 6144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.462 | 95.0% |
| wide-rung-ragged | 1.523 | 99.0% |
| wide-rung-ragged-overread | 1.468 | 95.4% |
| wide-rung-wordround | 1.534 | 99.6% |
| wide-rung-wordround-alias | 1.534 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4281ns | 4281ns | base |
| wide-rung-ragged | 4151ns | 4151ns | -3.03% |
| wide-rung-ragged-overread | 4249ns | 4249ns | -0.74% |
| wide-rung-wordround | 4209ns | 4209ns | -1.69% |
| wide-rung-wordround-alias | 4131ns | 4131ns | -3.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4203ns | base | --- | [4172, 4290] | --- | --- | --- | --- |
| wide-rung-ragged | 4033ns | -132.1ns (-3.1%) | [-149, -121]ns | [4024, 4158] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4185ns | no significant difference | [-98, +9]ns | [4070, 4314] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround | 4006ns | -132.7ns (-3.2%) | [-174, -39]ns | [4002, 4126] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-wordround-alias | 4005ns | -172.9ns (-4.1%) | [-186, -144]ns | [4001, 4106] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 4177ns | -3.8% | -3.7% | -1.2% | -4.5% |
| 2 | 4172ns | -3.6% | -3.5% | -0.8% | -4.2% |
| 3 | 4167ns | -3.5% | -3.6% | -1.0% | -4.0% |
| 4 | 4172ns | -1.3% | -3.5% | -3.5% | -4.5% |
| 5 | 4178ns | -0.1% | -3.6% | -4.4% | -4.5% |
| 6 | 4179ns | +0.2% | -3.6% | -4.3% | -4.0% |
| 7 | 4184ns | -3.8% | -3.8% | -4.4% | -4.4% |
| 8 | 4067ns | -1.1% | -0.5% | -1.5% | -1.6% |
| 9 | 4030ns | -0.2% | +3.3% | +0.9% | -0.8% |
| 10 | 4035ns | -0.4% | +0.7% | +3.0% | -0.8% |
| 11 | 4136ns | -2.7% | +1.4% | -3.6% | +3.2% |
| 12 | 4139ns | -2.8% | +0.9% | -3.3% | -0.1% |
| 13 | 4175ns | -3.7% | -3.7% | -4.4% | -4.1% |
| 14 | 4241ns | -5.1% | -5.3% | -5.6% | -5.7% |
| 15 | 4144ns | -2.9% | -1.0% | -3.5% | -3.5% |
| 16 | 4222ns | -4.6% | -4.5% | -5.2% | -5.4% |
| 17 | 4170ns | -3.5% | -0.4% | -4.0% | -4.0% |
| 18 | 4263ns | -5.6% | -4.4% | -6.2% | -6.1% |
| 19 | 4138ns | -2.9% | -2.8% | -3.4% | -3.4% |
| 20 | 4133ns | -0.7% | -2.6% | -3.2% | -3.1% |
| 21 | 4304ns | -3.1% | -3.2% | +7.1% | -0.5% |
| 22 | 4305ns | -3.1% | -0.4% | +7.3% | -0.4% |
| 23 | 4307ns | -6.7% | +0.2% | +7.2% | -0.6% |
| 24 | 4325ns | -7.0% | -0.1% | +6.8% | -3.9% |
| 25 | 4311ns | -6.7% | +0.1% | +7.1% | -7.3% |
| 26 | 4304ns | -6.6% | +0.2% | +7.2% | -7.4% |
| 27 | 4329ns | -6.7% | -0.1% | +1.3% | -7.3% |
| 28 | 4322ns | -3.8% | -0.1% | -0.9% | -7.3% |
| 29 | 4306ns | -3.4% | +0.2% | -0.6% | -7.1% |
| 30 | 4307ns | -3.4% | +0.3% | -0.5% | -7.2% |
| 31 | 4288ns | -3.0% | -2.1% | -6.7% | -4.2% |
| 32 | 4322ns | -3.6% | -0.0% | -4.5% | -5.0% |
| 33 | 4285ns | -3.0% | +1.0% | -6.8% | -4.2% |
| 34 | 4292ns | -3.0% | +0.6% | -6.7% | -4.3% |
| 35 | 4299ns | -3.3% | +0.3% | -7.0% | -4.4% |
| 36 | 4276ns | -2.7% | +0.9% | -6.3% | -4.1% |
| 37 | 4296ns | -3.0% | +0.5% | -6.2% | -3.0% |
| 38 | 4069ns | +2.3% | +6.0% | +0.6% | +3.1% |
| 39 | 4072ns | +2.1% | +6.0% | -1.9% | +3.4% |
| 40 | 4078ns | +2.1% | +6.0% | -2.1% | +2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.733 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.739 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.845 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.835 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.650 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 35/40, lost 4/40
- **wide-rung-ragged-overread**: won 22/40, lost 17/40
- **wide-rung-wordround**: won 30/40, lost 10/40
- **wide-rung-wordround-alias**: won 36/40, lost 4/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.0ns | 4213.0ns | 0.0% |  |
| wide-rung-ragged | 2.2ns | 4087.6ns | 0.1% |  |
| wide-rung-ragged-overread | 2.4ns | 4183.1ns | 0.1% |  |
| wide-rung-wordround | 2.4ns | 4144.7ns | 0.1% |  |
| wide-rung-wordround-alias | 2.2ns | 4069.0ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4077.5-4316.3 ns)
   4077.5 |
   4089.5 |
   4101.4 |
   4113.4 |
   4125.3 |################
   4137.2 |########################
   4149.2 |
   4161.1 |################################
   4173.0 |########################################
   4185.0 |
   4196.9 |
   4208.8 |
   4220.8 |########
   4232.7 |########
   4244.6 |
   4256.6 |########
   4268.5 |########
   4280.4 |########################
   4292.4 |################################
   4304.3 |########################################
  (6 below, 4 above range)

wide-rung-ragged (n=40, range 4019.2-4171.1 ns)
   4019.2 |########################################
   4026.8 |
   4034.4 |##
   4042.0 |
   4049.5 |
   4057.1 |
   4064.7 |
   4072.3 |
   4079.9 |
   4087.5 |
   4095.1 |
   4102.7 |##
   4110.3 |
   4117.9 |##
   4125.5 |
   4133.1 |
   4140.7 |
   4148.3 |##
   4155.9 |##################
   4163.5 |#########
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 4022.3-4321.0 ns)
   4022.3 |###########################
   4037.2 |###
   4052.2 |###
   4067.1 |###
   4082.0 |
   4097.0 |###
   4111.9 |
   4126.8 |
   4141.8 |###
   4156.7 |######
   4171.7 |###
   4186.6 |######
   4201.5 |
   4216.5 |
   4231.4 |
   4246.3 |
   4261.3 |
   4276.2 |###
   4291.2 |
   4306.1 |########################################
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 3992.9-4546.1 ns)
   3992.9 |########################################
   4020.5 |####
   4048.2 |##
   4075.8 |##
   4103.5 |######
   4131.2 |####
   4158.8 |
   4186.5 |
   4214.2 |
   4241.8 |
   4269.5 |######
   4297.1 |
   4324.8 |
   4352.5 |
   4380.1 |##
   4407.8 |
   4435.4 |
   4463.1 |
   4490.8 |
   4518.4 |
  (3 below, 6 above range)

wide-rung-wordround-alias (n=40, range 3991.5-4232.1 ns)
   3991.5 |########################################
   4003.5 |#################
   4015.5 |
   4027.6 |
   4039.6 |
   4051.6 |
   4063.7 |
   4075.7 |
   4087.7 |
   4099.8 |#################
   4111.8 |
   4123.8 |##
   4135.8 |
   4147.9 |##
   4159.9 |#####
   4171.9 |
   4184.0 |
   4196.0 |##
   4208.0 |##
   4220.0 |
  (4 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.65 (measurement drift or warm-up artifact)
