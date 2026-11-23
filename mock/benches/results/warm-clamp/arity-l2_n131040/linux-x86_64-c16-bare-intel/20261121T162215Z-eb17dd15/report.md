# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 44% (significant)

warm-clamp-accfit is -53.58 us (44%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 2.5x slower than the field

warm-clamp-min-lanes (169.66 us) is 2.5x the fastest (68.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.73)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: warm-clamp-accfit** at 68076.8 ns median (-43.9% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.49x (fastest 68076.8 ns, slowest 169662.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112705153 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 121295ns | 121498ns | 118847ns | 121179ns | 124090ns | base |
| warm-clamp-accfit | 67968ns | 68335ns | 64646ns | 67972ns | 71276ns | -43.96% |
| warm-clamp-accfit-dyn | 69414ns | 70795ns | 64704ns | 70370ns | 71255ns | -42.77% |
| warm-clamp-head | 71681ns | 72621ns | 69215ns | 71659ns | 74212ns | -40.90% |
| warm-clamp-min-lanes | 169513ns | 169821ns | 168358ns | 169649ns | 170260ns | +39.75% |
| warm-clamp-minimum | 166701ns | 167063ns | 165207ns | 166723ns | 168131ns | +37.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 121124ns | 118683ns | 123887ns | base | 8.657 |
| warm-clamp-accfit | 67801ns | 64493ns | 71121ns | -44.02% | 15.466 |
| warm-clamp-accfit-dyn | 69255ns | 64542ns | 71101ns | -42.82% | 15.141 |
| warm-clamp-head | 71515ns | 69057ns | 74022ns | -40.96% | 14.662 |
| warm-clamp-min-lanes | 169342ns | 168186ns | 170068ns | +39.81% | 6.192 |
| warm-clamp-minimum | 166517ns | 164998ns | 167951ns | +37.48% | 6.297 |

## Performance model

- Peak throughput: **16.259 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.643 | 53.2% |
| warm-clamp-accfit | 15.403 | 94.7% |
| warm-clamp-accfit-dyn | 14.846 | 91.3% |
| warm-clamp-head | 14.471 | 89.0% |
| warm-clamp-min-lanes | 6.180 | 38.0% |
| warm-clamp-minimum | 6.283 | 38.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 121295ns | 121295ns | base |
| warm-clamp-accfit | 67968ns | 67968ns | -43.96% |
| warm-clamp-accfit-dyn | 69414ns | 69414ns | -42.77% |
| warm-clamp-head | 71681ns | 71681ns | -40.90% |
| warm-clamp-min-lanes | 169513ns | 169513ns | +39.75% |
| warm-clamp-minimum | 166701ns | 166701ns | +37.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 121316ns | base | --- | [118706, 123216] | --- | --- | --- | --- |
| warm-clamp-accfit | 68077ns | -53581.9ns (-44.2%) | [-54183, -52241]ns | [64675, 70639] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 70632ns | -50642.4ns (-41.7%) | [-52593, -48053]ns | [70622, 70701] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 72459ns | -49786.4ns (-41.0%) | [-50328, -49239]ns | [69340, 73314] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 169662ns | +49028.2ns (+40.4%) | [+45520, +50959]ns | [169645, 169676] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 166900ns | +47602.0ns (+39.2%) | [+42517, +48198]ns | [166436, 166919] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 123202ns | -47.6% | -42.7% | -43.9% | +36.6% | +33.9% |
| 1 | 2 | 118693ns | -45.6% | -40.1% | -38.2% | +42.9% | +40.6% |
| 1 | 3 | 118695ns | -40.5% | -40.5% | -41.6% | +43.0% | +40.6% |
| 1 | 4 | 123298ns | -42.4% | -47.6% | -40.5% | +37.8% | +35.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.250 | moderate- |
| warm-clamp-accfit | 0.266 | moderate+ |
| warm-clamp-accfit-dyn | -0.049 | ok |
| warm-clamp-head | -0.730 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.045 | ok |
| warm-clamp-minimum | -0.069 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 30.1ns | 121123.5ns | 0.0% |  |
| warm-clamp-accfit | 33.0ns | 67800.5ns | 0.0% |  |
| warm-clamp-accfit-dyn | 28.8ns | 69255.2ns | 0.0% |  |
| warm-clamp-head | 29.9ns | 71515.5ns | 0.0% |  |
| warm-clamp-min-lanes | 21.5ns | 169341.7ns | 0.0% |  |
| warm-clamp-minimum | 22.3ns | 166516.7ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 118683.1-123887.4 ns)
  118683.1 |#####################################
  118943.3 |##
  119203.5 |##
  119463.7 |
  119723.9 |
  119984.1 |
  120244.4 |
  120504.6 |
  120764.8 |
  121025.0 |
  121285.2 |
  121545.4 |
  121805.7 |
  122065.9 |
  122326.1 |
  122586.3 |
  122846.5 |
  123106.8 |########################################
  123367.0 |
  123627.2 |##
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 64492.8-71121.3 ns)
  64492.8 |########################################
  64824.3 |###
  65155.7 |###
  65487.1 |######
  65818.5 |
  66150.0 |
  66481.4 |
  66812.8 |
  67144.2 |
  67475.6 |
  67807.1 |
  68138.5 |
  68469.9 |
  68801.3 |
  69132.8 |
  69464.2 |
  69795.6 |
  70127.0 |
  70458.4 |##########################
  70789.9 |####################################
  (4 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 64541.9-71100.9 ns)
  64541.9 |###########
  64869.9 |##
  65197.8 |
  65525.8 |
  65853.7 |
  66181.6 |
  66509.6 |
  66837.5 |
  67165.5 |
  67493.4 |
  67821.4 |
  68149.3 |
  68477.3 |
  68805.2 |
  69133.2 |
  69461.1 |
  69789.1 |
  70117.0 |
  70445.0 |########################################
  70772.9 |######################
  (4 below, 2 above range)

warm-clamp-head (n=40, range 69056.7-74022.0 ns)
  69056.7 |#########################
  69305.0 |########
  69553.2 |
  69801.5 |
  70049.8 |##
  70298.0 |
  70546.3 |
  70794.6 |##
  71042.8 |
  71291.1 |
  71539.4 |##
  71787.6 |
  72035.9 |
  72284.2 |
  72532.4 |
  72780.7 |
  73029.0 |########
  73277.2 |########################################
  73525.5 |##
  73773.8 |
  (5 below, 2 above range)

warm-clamp-min-lanes (n=40, range 168186.4-170068.2 ns)
  168186.4 |####
  168280.5 |##
  168374.6 |####
  168468.7 |##
  168562.8 |
  168656.9 |####
  168750.9 |
  168845.0 |
  168939.1 |
  169033.2 |
  169127.3 |
  169221.4 |
  169315.5 |
  169409.5 |
  169503.6 |##
  169597.7 |########################################
  169691.8 |##
  169785.9 |####
  169880.0 |######
  169974.1 |##
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 164998.0-167950.6 ns)
  164998.0 |##############
  165145.6 |##########
  165293.3 |###
  165440.9 |
  165588.5 |
  165736.2 |###
  165883.8 |###
  166031.4 |
  166179.1 |
  166326.7 |
  166474.3 |
  166622.0 |
  166769.6 |########################################
  166917.2 |#########################
  167064.8 |##############
  167212.5 |
  167360.1 |
  167507.7 |###
  167655.4 |
  167803.0 |###
  (4 below, 2 above range)

```
