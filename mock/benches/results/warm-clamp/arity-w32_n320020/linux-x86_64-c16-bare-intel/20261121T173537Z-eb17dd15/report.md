# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.84 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-min-lanes at 2.45 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-min-lanes dominates: 18% faster than the next best (warm-clamp-minimum)

warm-clamp-min-lanes (2.45 us) leads warm-clamp-minimum (2.90 us) by 18%, a clear separation rather than a photo finish. CV 83.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is fastest but the noisiest (CV 83.3%)

warm-clamp-min-lanes wins on median (2.45 us) yet has the highest variance (CV 83.3%), while warm-clamp-head is the steadiest (CV 32.0%, 3.36 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.52)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.39 us) is smaller than the fastest variant's own run-to-run std-dev (2.04 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.5x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.11 us but its worst 20% at 4.97 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2447.1 ns median (-36.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.57x (fastest 2447.1 ns, slowest 3838.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112820894 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3461ns | 4014ns | 1269ns | 3660ns | 5059ns | base |
| warm-clamp-accfit | 3408ns | 3984ns | 1199ns | 3563ns | 5152ns | -1.54% |
| warm-clamp-accfit-dyn | 3358ns | 3710ns | 1166ns | 3454ns | 5260ns | -3.00% |
| warm-clamp-head | 3272ns | 3585ns | 1128ns | 3597ns | 4439ns | -5.49% |
| warm-clamp-min-lanes | 3617ns | 2538ns | 1506ns | 3301ns | 6677ns | +4.50% |
| warm-clamp-minimum | 4012ns | 2987ns | 1918ns | 3772ns | 6825ns | +15.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3299ns | 1213ns | 4796ns | base | 2.483 |
| warm-clamp-accfit | 3246ns | 1146ns | 4874ns | -1.62% | 2.524 |
| warm-clamp-accfit-dyn | 3195ns | 1113ns | 4973ns | -3.17% | 2.564 |
| warm-clamp-head | 3054ns | 1056ns | 4115ns | -7.43% | 2.682 |
| warm-clamp-min-lanes | 3475ns | 1451ns | 6395ns | +5.32% | 2.358 |
| warm-clamp-minimum | 3881ns | 1860ns | 6568ns | +17.63% | 2.111 |

## Performance model

- Peak throughput: **7.758 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.134 | 27.5% |
| warm-clamp-accfit | 2.150 | 27.7% |
| warm-clamp-accfit-dyn | 2.309 | 29.8% |
| warm-clamp-head | 2.435 | 31.4% |
| warm-clamp-min-lanes | 3.348 | 43.2% |
| warm-clamp-minimum | 2.828 | 36.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3461ns | 3461ns | base |
| warm-clamp-accfit | 3408ns | 3408ns | -1.54% |
| warm-clamp-accfit-dyn | 3358ns | 3358ns | -3.00% |
| warm-clamp-head | 3272ns | 3272ns | -5.49% |
| warm-clamp-min-lanes | 3617ns | 3617ns | +4.50% |
| warm-clamp-minimum | 4012ns | 4012ns | +15.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3839ns | base | --- | [3495, 4145] | --- | --- | --- | --- |
| warm-clamp-accfit | 3810ns | no significant difference | [-19, +51]ns | [2567, 4201] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 3548ns | no significant difference | [-627, +16]ns | [3297, 3839] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-head | 3364ns | -193.8ns (-5.0%) | [-570, -164]ns | [3209, 3600] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2447ns | +194.7ns (+5.1%) | [+98, +396]ns | [1505, 4861] | YES | 0.0276 | 0.0166 | 0 |
| warm-clamp-minimum | 2897ns | +736.7ns (+19.2%) | [+499, +1393]ns | [1921, 5534] | YES | 0.0056 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3834ns | -36.4% | +0.7% | -3.7% | -62.2% | +4.2% |
| 1 | 2 | 4109ns | -4.7% | -12.1% | -20.4% | -14.9% | -41.8% |
| 1 | 3 | 4322ns | +0.1% | -17.9% | -19.5% | -1.0% | -23.0% |
| 1 | 4 | 3566ns | +6.8% | -68.2% | -5.0% | +19.0% | +7.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.328 | moderate- |
| warm-clamp-accfit | -0.004 | ok |
| warm-clamp-accfit-dyn | -0.042 | ok |
| warm-clamp-head | -0.521 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | 0.126 | ok |
| warm-clamp-minimum | -0.361 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 57.7ns | 3299.3ns | 1.8% |  |
| warm-clamp-accfit | 57.4ns | 3245.9ns | 1.8% |  |
| warm-clamp-accfit-dyn | 55.4ns | 3194.7ns | 1.7% |  |
| warm-clamp-head | 62.7ns | 3054.1ns | 2.1% |  |
| warm-clamp-min-lanes | 46.3ns | 3474.7ns | 1.3% |  |
| warm-clamp-minimum | 40.2ns | 3880.8ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1213.3-4796.3 ns)
   1213.3 |##############################
   1392.5 |
   1571.6 |
   1750.8 |
   1929.9 |#####
   2109.1 |
   2288.2 |#####
   2467.4 |
   2646.5 |
   2825.7 |
   3004.8 |
   3184.0 |
   3363.1 |##########
   3542.3 |###############
   3721.4 |###############
   3900.6 |
   4079.7 |########################################
   4258.9 |#####
   4438.0 |####################
   4617.2 |#####
  (5 below, 5 above range)

warm-clamp-accfit (n=40, range 1146.5-4873.9 ns)
   1146.5 |
   1332.8 |########################################
   1519.2 |
   1705.6 |
   1892.0 |
   2078.3 |######
   2264.7 |
   2451.1 |
   2637.4 |
   2823.8 |
   3010.2 |######
   3196.6 |
   3382.9 |
   3569.3 |######
   3755.7 |#################################
   3942.0 |####################
   4128.4 |####################
   4314.8 |####################
   4501.2 |#################################
   4687.5 |
  (7 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 1113.4-4973.2 ns)
   1113.4 |########################################
   1306.4 |
   1499.4 |
   1692.3 |
   1885.3 |######
   2078.3 |
   2271.3 |
   2464.3 |
   2657.3 |
   2850.3 |
   3043.3 |######
   3236.3 |####################
   3429.2 |#################################
   3622.2 |####################
   3815.2 |##########################
   4008.2 |##########################
   4201.2 |#############
   4394.2 |######
   4587.2 |
   4780.2 |#############
  (5 below, 3 above range)

warm-clamp-head (n=40, range 1055.9-4115.2 ns)
   1055.9 |########
   1208.9 |########
   1361.8 |
   1514.8 |
   1667.8 |
   1820.7 |
   1973.7 |
   2126.7 |
   2279.6 |
   2432.6 |########
   2585.6 |########
   2738.5 |
   2891.5 |
   3044.4 |################################
   3197.4 |########################################
   3350.4 |########################################
   3503.3 |########################################
   3656.3 |########
   3809.3 |########################################
   3962.2 |################
  (6 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1450.5-6394.8 ns)
   1450.5 |########################################
   1697.7 |#################
   1944.9 |
   2192.2 |####
   2439.4 |####
   2686.6 |
   2933.8 |
   3181.0 |
   3428.2 |
   3675.4 |
   3922.7 |####
   4169.9 |####
   4417.1 |####
   4664.3 |########
   4911.5 |#############
   5158.7 |########
   5405.9 |####
   5653.1 |########
   5900.4 |####
   6147.6 |
  (6 below, 5 above range)

warm-clamp-minimum (n=40, range 1860.5-6568.2 ns)
   1860.5 |########################################
   2095.8 |
   2331.2 |##
   2566.6 |
   2802.0 |#####
   3037.4 |
   3272.8 |
   3508.2 |
   3743.5 |
   3978.9 |
   4214.3 |
   4449.7 |
   4685.1 |##
   4920.5 |##
   5155.9 |#####
   5391.2 |#####
   5626.6 |########
   5862.0 |#####
   6097.4 |#####
   6332.8 |##########
  (3 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=42.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=46.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=45.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-clamp-head**: CV=35.2% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=58.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=52.3% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.5x (possible bimodal distribution)
