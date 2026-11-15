# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.76)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.76, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 1.6% of the fastest

All 5 variants sit between 4.12 us and 4.18 us - a 1.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged's edge over baseline is significant but tiny (16 ns, 0.39%)

wide-rung-ragged differs from baseline wide-rung-align16 by 16 ns (0.39%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 4115.6 ns median (-1.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.02x (fastest 4115.6 ns, slowest 4180.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4291ns | 4225ns | 4196ns | 4269ns | 4451ns | base |
| wide-rung-ragged | 4253ns | 4227ns | 4222ns | 4229ns | 4355ns | -0.89% |
| wide-rung-ragged-overread | 4258ns | 4249ns | 4219ns | 4246ns | 4333ns | -0.76% |
| wide-rung-wordround | 4178ns | 4177ns | 4130ns | 4168ns | 4256ns | -2.63% |
| wide-rung-wordround-alias | 4236ns | 4198ns | 4188ns | 4211ns | 4362ns | -1.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4223ns | 4131ns | 4372ns | base | 1.455 |
| wide-rung-ragged | 4188ns | 4158ns | 4288ns | -0.83% | 1.467 |
| wide-rung-ragged-overread | 4191ns | 4153ns | 4266ns | -0.76% | 1.466 |
| wide-rung-wordround | 4113ns | 4067ns | 4187ns | -2.60% | 1.494 |
| wide-rung-wordround-alias | 4170ns | 4123ns | 4293ns | -1.24% | 1.473 |

## Performance model

- Peak throughput: **1.511 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 6144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.476 | 97.7% |
| wide-rung-ragged | 1.476 | 97.7% |
| wide-rung-ragged-overread | 1.470 | 97.3% |
| wide-rung-wordround | 1.493 | 98.8% |
| wide-rung-wordround-alias | 1.486 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4291ns | 4291ns | base |
| wide-rung-ragged | 4253ns | 4253ns | -0.89% |
| wide-rung-ragged-overread | 4258ns | 4258ns | -0.76% |
| wide-rung-wordround | 4178ns | 4178ns | -2.63% |
| wide-rung-wordround-alias | 4236ns | 4236ns | -1.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4162ns | base | --- | [4146, 4272] | --- | --- | --- | --- |
| wide-rung-ragged | 4164ns | no significant difference | [-23, +17]ns | [4160, 4166] | no | 0.1076 | 0.0807 | 0 |
| wide-rung-ragged-overread | 4180ns | no significant difference | [-63, +36]ns | [4167, 4189] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround | 4116ns | -82.3ns (-2.0%) | [-147, -69]ns | [4078, 4126] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 4135ns | -20.0ns (-0.5%) | [-58, -3]ns | [4131, 4144] | YES | 0.0190 | 0.0095 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 4279ns | -2.8% | -2.7% | -3.6% | -3.0% |
| 2 | 4273ns | -2.7% | -1.1% | +4.3% | +1.6% |
| 3 | 4270ns | -2.6% | -2.6% | -3.3% | -0.1% |
| 4 | 4582ns | -9.1% | -9.1% | -10.1% | -6.6% |
| 5 | 4250ns | -2.2% | -2.0% | -2.7% | +0.6% |
| 6 | 4367ns | -4.6% | -4.5% | -5.2% | -1.9% |
| 7 | 4280ns | -1.1% | -2.6% | -3.6% | +0.1% |
| 8 | 4135ns | +0.7% | +0.9% | +2.0% | +3.1% |
| 9 | 4136ns | +0.7% | +0.6% | -0.5% | +2.9% |
| 10 | 4217ns | -1.3% | -1.3% | -2.2% | -2.0% |
| 11 | 4297ns | +0.4% | -1.6% | -4.1% | -4.0% |
| 12 | 4300ns | +0.4% | +0.4% | -4.2% | -2.3% |
| 13 | 4291ns | +0.5% | +0.8% | -3.7% | -3.9% |
| 14 | 4294ns | +0.3% | +0.1% | -3.7% | -3.8% |
| 15 | 4295ns | +0.3% | -2.5% | -3.9% | -3.2% |
| 16 | 4434ns | -2.5% | -6.1% | -6.8% | -6.7% |
| 17 | 4251ns | -1.7% | -2.0% | -2.7% | -2.8% |
| 18 | 4309ns | -3.5% | -3.1% | -3.8% | -3.2% |
| 19 | 4294ns | -3.0% | -3.2% | -3.9% | +1.3% |
| 20 | 4394ns | -5.3% | -5.2% | -5.9% | -6.0% |
| 21 | 4147ns | +0.3% | +1.1% | -1.8% | -0.5% |
| 22 | 4159ns | +0.0% | +0.8% | -1.5% | -0.8% |
| 23 | 4145ns | +0.3% | +1.1% | -1.5% | -0.6% |
| 24 | 4146ns | +0.3% | +1.1% | -2.1% | -0.3% |
| 25 | 4145ns | +0.4% | +1.0% | -1.8% | -0.5% |
| 26 | 4149ns | +0.3% | +1.0% | -1.8% | -0.2% |
| 27 | 4150ns | +0.4% | +0.9% | -1.7% | -0.4% |
| 28 | 4150ns | +0.3% | +0.9% | -1.5% | -0.5% |
| 29 | 4148ns | +0.4% | +0.9% | -1.9% | -0.7% |
| 30 | 4142ns | +0.5% | +1.1% | -1.7% | -0.4% |
| 31 | 4269ns | -2.6% | -2.2% | -4.8% | -3.4% |
| 32 | 4126ns | +1.4% | +0.2% | -1.2% | +0.2% |
| 33 | 4124ns | +0.9% | +2.3% | -0.2% | +0.4% |
| 34 | 4131ns | +0.7% | +3.2% | -1.3% | -0.0% |
| 35 | 4138ns | +0.5% | +1.2% | -1.6% | +0.2% |
| 36 | 4132ns | +0.7% | +0.8% | -1.5% | +0.1% |
| 37 | 4140ns | +1.2% | +0.7% | -1.5% | +0.1% |
| 38 | 4164ns | +0.1% | +2.0% | -2.1% | -0.8% |
| 39 | 4130ns | +0.7% | +0.8% | -1.5% | -0.1% |
| 40 | 4132ns | +0.9% | -0.1% | -1.6% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.475 | moderate+ |
| wide-rung-ragged | 0.758 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.530 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.243 | moderate+ |
| wide-rung-wordround-alias | 0.566 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 14/40, lost 25/40
- **wide-rung-ragged-overread**: won 17/40, lost 23/40
- **wide-rung-wordround**: won 38/40, lost 2/40
- **wide-rung-wordround-alias**: won 25/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.5ns | 4222.8ns | 0.1% |  |
| wide-rung-ragged | 2.1ns | 4188.0ns | 0.1% |  |
| wide-rung-ragged-overread | 2.2ns | 4190.8ns | 0.1% |  |
| wide-rung-wordround | 2.3ns | 4113.1ns | 0.1% |  |
| wide-rung-wordround-alias | 2.1ns | 4170.3ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4130.6-4372.1 ns)
   4130.6 |########################################
   4142.7 |########################################
   4154.8 |##########
   4166.8 |
   4178.9 |
   4191.0 |
   4203.1 |
   4215.1 |#####
   4227.2 |
   4239.3 |##########
   4251.4 |
   4263.4 |###############
   4275.5 |##########
   4287.6 |#########################
   4299.7 |##########
   4311.7 |
   4323.8 |
   4335.9 |
   4348.0 |
   4360.0 |#####
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 4158.4-4287.6 ns)
   4158.4 |########################################
   4164.9 |############
   4171.3 |
   4177.8 |##
   4184.2 |####
   4190.7 |
   4197.2 |
   4203.6 |
   4210.1 |
   4216.6 |
   4223.0 |
   4229.5 |##
   4235.9 |
   4242.4 |
   4248.9 |
   4255.3 |
   4261.8 |
   4268.3 |
   4274.7 |
   4281.2 |
  (4 below, 6 above range)

wide-rung-ragged-overread (n=40, range 4153.2-4265.9 ns)
   4153.2 |#####
   4158.8 |##############################
   4164.4 |###################################
   4170.1 |###############
   4175.7 |#####
   4181.3 |###############
   4187.0 |########################################
   4192.6 |#####
   4198.3 |
   4203.9 |
   4209.5 |
   4215.2 |#####
   4220.8 |#####
   4226.4 |#####
   4232.1 |
   4237.7 |
   4243.3 |#####
   4249.0 |
   4254.6 |
   4260.3 |#####
  (2 below, 3 above range)

wide-rung-wordround (n=40, range 4066.9-4186.5 ns)
   4066.9 |###################################
   4072.9 |#########################
   4078.9 |##########
   4084.9 |#####
   4090.8 |
   4096.8 |#####
   4102.8 |
   4108.8 |
   4114.8 |##########
   4120.8 |########################################
   4126.7 |#####
   4132.7 |###################################
   4138.7 |
   4144.7 |#####
   4150.7 |
   4156.6 |
   4162.6 |
   4168.6 |
   4174.6 |
   4180.6 |
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 4123.3-4293.3 ns)
   4123.3 |########################################
   4131.8 |#################################
   4140.3 |##########
   4148.8 |######
   4157.3 |
   4165.8 |###
   4174.3 |
   4182.8 |
   4191.3 |
   4199.8 |###
   4208.3 |
   4216.8 |
   4225.3 |
   4233.8 |
   4242.3 |
   4250.8 |###
   4259.3 |###
   4267.8 |###
   4276.3 |#############
   4284.8 |
  (2 below, 2 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.57 (measurement drift or warm-up artifact)
