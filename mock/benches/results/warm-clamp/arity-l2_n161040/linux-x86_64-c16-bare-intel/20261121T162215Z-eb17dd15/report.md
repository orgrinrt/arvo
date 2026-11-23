# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 46% (significant)

warm-clamp-accfit is -54.09 us (46%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 3.8x slower than the field

warm-clamp-min-lanes (243.73 us) is 3.8x the fastest (64.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.8x the fastest

Fastest warm-clamp-accfit (64.73 us) to slowest warm-clamp-min-lanes (243.73 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 64730.1 ns median (-45.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.77x (fastest 64730.1 ns, slowest 243726.9 ns)

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
| warm-clamp-acc64 | 120154ns | 118946ns | 118850ns | 119524ns | 123348ns | base |
| warm-clamp-accfit | 66365ns | 64906ns | 64572ns | 65355ns | 71188ns | -44.77% |
| warm-clamp-accfit-dyn | 67933ns | 67966ns | 64584ns | 67851ns | 71529ns | -43.46% |
| warm-clamp-head | 67179ns | 67177ns | 64539ns | 67096ns | 70066ns | -44.09% |
| warm-clamp-min-lanes | 247805ns | 243912ns | 243833ns | 243942ns | 263363ns | +106.24% |
| warm-clamp-minimum | 242942ns | 242332ns | 242153ns | 242341ns | 245536ns | +102.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 119990ns | 118703ns | 123190ns | base | 8.739 |
| warm-clamp-accfit | 66199ns | 64411ns | 71035ns | -44.83% | 15.840 |
| warm-clamp-accfit-dyn | 67775ns | 64431ns | 71359ns | -43.52% | 15.471 |
| warm-clamp-head | 67017ns | 64377ns | 69904ns | -44.15% | 15.646 |
| warm-clamp-min-lanes | 247604ns | 243632ns | 263137ns | +106.35% | 4.235 |
| warm-clamp-minimum | 242760ns | 241968ns | 245351ns | +102.32% | 4.319 |

## Performance model

- Peak throughput: **16.288 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.829 | 54.2% |
| warm-clamp-accfit | 16.199 | 99.5% |
| warm-clamp-accfit-dyn | 15.462 | 94.9% |
| warm-clamp-head | 15.644 | 96.0% |
| warm-clamp-min-lanes | 4.302 | 26.4% |
| warm-clamp-minimum | 4.330 | 26.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 120154ns | 120154ns | base |
| warm-clamp-accfit | 66365ns | 66365ns | -44.77% |
| warm-clamp-accfit-dyn | 67933ns | 67933ns | -43.46% |
| warm-clamp-head | 67179ns | 67179ns | -44.09% |
| warm-clamp-min-lanes | 247805ns | 247805ns | +106.24% |
| warm-clamp-minimum | 242942ns | 242942ns | +102.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 118765ns | base | --- | [118719, 118998] | --- | --- | --- | --- |
| warm-clamp-accfit | 64730ns | -54094.0ns (-45.5%) | [-54314, -53871]ns | [64456, 64844] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 67817ns | -53574.4ns (-45.1%) | [-54261, -51634]ns | [64462, 70768] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 67026ns | -53949.6ns (-45.4%) | [-54189, -53776]ns | [64545, 69221] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 243727ns | +124916.7ns (+105.2%) | [+124846, +124989]ns | [243689, 243808] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 242153ns | +123300.6ns (+103.8%) | [+123176, +123441]ns | [242114, 242204] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 118805ns | -45.5% | -45.8% | -45.8% | +105.2% | +103.7% |
| 1 | 2 | 118712ns | -40.2% | -40.4% | -41.4% | +105.3% | +104.0% |
| 1 | 3 | 122975ns | -47.6% | -42.0% | -43.7% | +98.6% | +97.0% |
| 1 | 4 | 118713ns | -45.7% | -45.7% | -45.7% | +105.2% | +104.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.434 | moderate- |
| warm-clamp-accfit | -0.388 | moderate- |
| warm-clamp-accfit-dyn | -0.251 | moderate- |
| warm-clamp-head | -0.252 | moderate- |
| warm-clamp-min-lanes | -0.500 | moderate- |
| warm-clamp-minimum | -0.003 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 30.8ns | 119989.6ns | 0.0% |  |
| warm-clamp-accfit | 31.3ns | 66199.2ns | 0.0% |  |
| warm-clamp-accfit-dyn | 32.3ns | 67774.7ns | 0.0% |  |
| warm-clamp-head | 30.8ns | 67017.1ns | 0.0% |  |
| warm-clamp-min-lanes | 32.0ns | 247603.6ns | 0.0% |  |
| warm-clamp-minimum | 31.3ns | 242760.4ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 118703.1-123190.1 ns)
  118703.1 |########################################
  118927.5 |#
  119151.8 |#
  119376.2 |#
  119600.5 |
  119824.9 |
  120049.2 |
  120273.6 |
  120497.9 |
  120722.2 |
  120946.6 |
  121170.9 |
  121395.3 |
  121619.6 |
  121844.0 |
  122068.3 |
  122292.7 |
  122517.0 |
  122741.4 |#####
  122965.7 |##########
  (3 below, 2 above range)

warm-clamp-accfit (n=40, range 64411.1-71034.9 ns)
  64411.1 |########################################
  64742.3 |###############
  65073.5 |
  65404.7 |
  65735.9 |
  66067.1 |
  66398.2 |
  66729.4 |
  67060.6 |
  67391.8 |
  67723.0 |
  68054.2 |
  68385.4 |
  68716.6 |
  69047.8 |
  69378.9 |
  69710.1 |
  70041.3 |
  70372.5 |
  70703.7 |###########
  (5 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 64430.7-71359.0 ns)
  64430.7 |########################################
  64777.1 |#####
  65123.5 |
  65469.9 |
  65816.3 |
  66162.7 |
  66509.2 |
  66855.6 |
  67202.0 |
  67548.4 |
  67894.8 |
  68241.2 |
  68587.7 |
  68934.1 |
  69280.5 |
  69626.9 |
  69973.3 |
  70319.7 |#####
  70666.1 |####################
  71012.6 |######################
  (4 below, 3 above range)

warm-clamp-head (n=40, range 64376.6-69903.8 ns)
  64376.6 |########################################
  64652.9 |#####
  64929.3 |
  65205.7 |
  65482.0 |
  65758.4 |
  66034.8 |
  66311.1 |
  66587.5 |
  66863.8 |
  67140.2 |
  67416.6 |
  67692.9 |
  67969.3 |
  68245.6 |
  68522.0 |
  68798.4 |
  69074.7 |######################
  69351.1 |#################
  69627.4 |########
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 243632.0-263137.3 ns)
  243632.0 |########################################
  244607.3 |###
  245582.5 |
  246557.8 |#
  247533.1 |
  248508.3 |
  249483.6 |#
  250458.9 |
  251434.1 |
  252409.4 |
  253384.6 |
  254359.9 |
  255335.2 |
  256310.4 |
  257285.7 |
  258261.0 |
  259236.2 |
  260211.5 |
  261186.8 |
  262162.0 |
  (3 below, 1 above range)

warm-clamp-minimum (n=40, range 241968.0-245350.8 ns)
  241968.0 |########################
  242137.1 |########################################
  242306.3 |########
  242475.4 |
  242644.6 |
  242813.7 |
  242982.8 |
  243152.0 |##
  243321.1 |
  243490.2 |
  243659.4 |
  243828.5 |
  243997.7 |
  244166.8 |
  244335.9 |
  244505.1 |
  244674.2 |
  244843.3 |
  245012.5 |
  245181.6 |
  (4 below, 2 above range)

```
