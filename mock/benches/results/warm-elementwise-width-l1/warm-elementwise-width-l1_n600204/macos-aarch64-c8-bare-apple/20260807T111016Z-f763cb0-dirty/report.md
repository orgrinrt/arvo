# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (3.54 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 3.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.85)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (77 ns) is smaller than the fastest variant's own run-to-run std-dev (104 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader warm-container-native vs stability leader warm-container-headroom (+2% speed for 1.4x steadier)

warm-container-native is fastest (3.47 us, CV 3.0%); warm-container-headroom gives up 2.2% median for 1.4x lower variance (CV 2.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.2% of the fastest

All 5 variants sit between 3.47 us and 3.54 us - a 2.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: warm-container-native** at 3465.8 ns median (-2.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.02x (fastest 3465.8 ns, slowest 3542.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3689ns | 3641ns | 3626ns | 3663ns | 3829ns | base |
| warm-container-kernel | 3662ns | 3545ns | 3472ns | 3563ns | 4146ns | -0.73% |
| warm-container-minimum | 3704ns | 3572ns | 3519ns | 3597ns | 4207ns | +0.41% |
| warm-container-native | 3538ns | 3520ns | 3468ns | 3509ns | 3693ns | -4.09% |
| warm-container-plusone | 3653ns | 3560ns | 3469ns | 3587ns | 4036ns | -0.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3590ns | 3533ns | 3730ns | base | 11.409 |
| warm-container-kernel | 3592ns | 3409ns | 4065ns | +0.04% | 11.404 |
| warm-container-minimum | 3641ns | 3459ns | 4140ns | +1.42% | 11.249 |
| warm-container-native | 3479ns | 3413ns | 3627ns | -3.11% | 11.775 |
| warm-container-plusone | 3592ns | 3413ns | 3971ns | +0.04% | 11.405 |

## Performance model

- Peak throughput: **12.014 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 11.562 | 96.2% |
| warm-container-kernel | 11.787 | 98.1% |
| warm-container-minimum | 11.659 | 97.0% |
| warm-container-native | 11.818 | 98.4% |
| warm-container-plusone | 11.711 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3689ns | 3689ns | base |
| warm-container-kernel | 3662ns | 3662ns | -0.73% |
| warm-container-minimum | 3704ns | 3704ns | +0.41% |
| warm-container-native | 3538ns | 3538ns | -4.09% |
| warm-container-plusone | 3653ns | 3653ns | -0.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3542ns | base | --- | [3539, 3584] | --- | --- | --- | --- |
| warm-container-kernel | 3475ns | -125.4ns (-3.5%) | [-128, -76]ns | [3461, 3527] | YES | 0.0044 | 0.0022 | 0 |
| warm-container-minimum | 3513ns | no significant difference | [-87, +55]ns | [3479, 3595] | no | 0.2682 | 0.2682 | 0 |
| warm-container-native | 3466ns | -120.4ns (-3.4%) | [-125, -114]ns | [3418, 3479] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3498ns | -114.4ns (-3.2%) | [-124, -8]ns | [3419, 3600] | YES | 0.0221 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 3601ns | -3.5% | -3.6% | -2.7% | -0.1% |
| 2 | 3606ns | -3.6% | -3.7% | -3.5% | -0.3% |
| 3 | 3666ns | -5.2% | -5.2% | -5.1% | -1.9% |
| 4 | 3612ns | -3.8% | -3.6% | -3.6% | -0.2% |
| 5 | 3640ns | -3.5% | -4.6% | -4.3% | +2.3% |
| 6 | 3764ns | -7.7% | -7.5% | -7.6% | -5.3% |
| 7 | 3740ns | -7.1% | -7.0% | -6.9% | -6.0% |
| 8 | 3738ns | -3.4% | -6.9% | -6.8% | -3.5% |
| 9 | 3743ns | -3.6% | -7.3% | -7.1% | -5.6% |
| 10 | 3737ns | -3.6% | -7.0% | -6.7% | -6.9% |
| 11 | 3539ns | -2.3% | -1.8% | -1.4% | +9.4% |
| 12 | 3542ns | +9.5% | -0.2% | -1.9% | +2.8% |
| 13 | 3558ns | -1.4% | -2.8% | -3.9% | -4.1% |
| 14 | 3542ns | -3.2% | +1.6% | -3.5% | -3.5% |
| 15 | 3538ns | -3.6% | +1.6% | -3.4% | -3.1% |
| 16 | 3537ns | -3.6% | +1.5% | -3.4% | -3.5% |
| 17 | 3538ns | -3.6% | +1.6% | +2.7% | +6.8% |
| 18 | 3540ns | -3.7% | +1.6% | -3.4% | +9.5% |
| 19 | 3538ns | -3.7% | +1.7% | -2.4% | +9.4% |
| 20 | 3539ns | -3.6% | +1.6% | -3.2% | +9.4% |
| 21 | 3727ns | +11.0% | -8.4% | +2.6% | +0.9% |
| 22 | 3538ns | +16.9% | +5.6% | +10.2% | -3.5% |
| 23 | 3537ns | +16.9% | +17.1% | -3.5% | -3.5% |
| 24 | 3530ns | +18.5% | +17.3% | -3.2% | -3.3% |
| 25 | 3538ns | +16.9% | +16.9% | -3.4% | -3.4% |
| 26 | 3534ns | +17.0% | +17.3% | -3.4% | -3.3% |
| 27 | 3534ns | +7.0% | +17.1% | -3.4% | +4.5% |
| 28 | 3535ns | -2.0% | +17.0% | -3.5% | +17.2% |
| 29 | 3543ns | -3.6% | +16.8% | -3.8% | +17.0% |
| 30 | 3716ns | -7.9% | +11.4% | -8.2% | +13.5% |
| 31 | 3530ns | +0.9% | -0.2% | -3.3% | -2.7% |
| 32 | 3539ns | -2.5% | -2.2% | -3.5% | -3.4% |
| 33 | 3547ns | +0.2% | -2.3% | -2.3% | -1.9% |
| 34 | 3550ns | -1.9% | -1.8% | -2.3% | -3.7% |
| 35 | 3581ns | -2.8% | -2.6% | -2.0% | -4.7% |
| 36 | 3587ns | -4.9% | -3.4% | -3.4% | -4.8% |
| 37 | 3532ns | -3.5% | -0.2% | +4.1% | -3.2% |
| 38 | 3551ns | -1.1% | -1.3% | -2.3% | -3.8% |
| 39 | 3534ns | -3.5% | -1.9% | -3.3% | -3.4% |
| 40 | 3672ns | -3.6% | -1.7% | -7.0% | -7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.513 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.756 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.853 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.307 | moderate+ |
| warm-container-plusone | 0.616 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 30/40, lost 10/40
- **warm-container-minimum**: won 24/40, lost 16/40
- **warm-container-native**: won 36/40, lost 4/40
- **warm-container-plusone**: won 28/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.2ns | 3590.2ns | 0.1% |  |
| warm-container-kernel | 2.6ns | 3591.7ns | 0.1% |  |
| warm-container-minimum | 2.6ns | 3641.2ns | 0.1% |  |
| warm-container-native | 2.5ns | 3478.5ns | 0.1% |  |
| warm-container-plusone | 2.5ns | 3591.5ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3533.3-3729.5 ns)
   3533.3 |########################################
   3543.1 |######
   3552.9 |##
   3562.7 |
   3572.5 |##
   3582.4 |##
   3592.2 |##
   3602.0 |####
   3611.8 |
   3621.6 |
   3631.4 |##
   3641.2 |
   3651.0 |
   3660.8 |##
   3670.7 |##
   3680.5 |
   3690.3 |
   3700.1 |
   3709.9 |##
   3719.7 |##
  (3 below, 5 above range)

warm-container-kernel (n=40, range 3409.3-4065.2 ns)
   3409.3 |###################################
   3442.1 |########################################
   3474.9 |####################
   3507.7 |##########
   3540.5 |###############
   3573.3 |#####
   3606.1 |##########
   3638.9 |
   3671.7 |
   3704.5 |
   3737.3 |
   3770.1 |#####
   3802.9 |
   3835.7 |
   3868.5 |#####
   3901.2 |
   3934.0 |
   3966.8 |
   3999.6 |
   4032.4 |
  (5 below, 6 above range)

warm-container-minimum (n=40, range 3459.1-4139.8 ns)
   3459.1 |########################################
   3493.1 |######
   3527.2 |##
   3561.2 |########
   3595.2 |########
   3629.3 |
   3663.3 |
   3697.3 |
   3731.4 |##
   3765.4 |
   3799.5 |
   3833.5 |
   3867.5 |
   3901.6 |
   3935.6 |
   3969.6 |
   4003.7 |
   4037.7 |
   4071.8 |
   4105.8 |###########
  (1 below, 3 above range)

warm-container-native (n=40, range 3412.7-3627.0 ns)
   3412.7 |########################################
   3423.4 |###
   3434.1 |
   3444.9 |###
   3455.6 |###
   3466.3 |############
   3477.0 |###########################
   3487.7 |###
   3498.4 |######
   3509.2 |
   3519.9 |
   3530.6 |
   3541.3 |
   3552.0 |
   3562.7 |
   3573.4 |
   3584.2 |
   3594.9 |
   3605.6 |
   3616.3 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 3413.0-3971.4 ns)
   3413.0 |########################################
   3440.9 |
   3468.9 |#####
   3496.8 |##
   3524.7 |##
   3552.6 |##
   3580.5 |##############
   3608.5 |
   3636.4 |##
   3664.3 |##
   3692.2 |
   3720.1 |##
   3748.1 |##
   3776.0 |##
   3803.9 |
   3831.8 |
   3859.7 |###########
   3887.6 |
   3915.6 |
   3943.5 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.62 (measurement drift or warm-up artifact)
