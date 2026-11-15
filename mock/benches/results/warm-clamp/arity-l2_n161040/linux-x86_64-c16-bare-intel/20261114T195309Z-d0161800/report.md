# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 44% (significant)

warm-clamp-accfit is -54.31 us (44%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 3.8x slower than the field

warm-clamp-min-lanes (243.87 us) is 3.8x the fastest (64.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.8x the fastest

Fastest warm-clamp-accfit (64.51 us) to slowest warm-clamp-min-lanes (243.87 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 64506.9 ns median (-47.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.78x (fastest 64506.9 ns, slowest 243874.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 122614ns | 123118ns | 120132ns | 123158ns | 123463ns | base |
| warm-clamp-accfit | 67192ns | 64684ns | 64554ns | 66685ns | 71351ns | -45.20% |
| warm-clamp-accfit-dyn | 70223ns | 70814ns | 67499ns | 70817ns | 71163ns | -42.73% |
| warm-clamp-head | 70502ns | 73317ns | 64463ns | 71524ns | 73478ns | -42.50% |
| warm-clamp-min-lanes | 244315ns | 244046ns | 243910ns | 244050ns | 245518ns | +99.26% |
| warm-clamp-minimum | 242544ns | 242505ns | 242382ns | 242513ns | 242800ns | +97.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 122463ns | 119990ns | 123315ns | base | 8.562 |
| warm-clamp-accfit | 67035ns | 64409ns | 71170ns | -45.26% | 15.642 |
| warm-clamp-accfit-dyn | 70044ns | 67351ns | 70983ns | -42.80% | 14.970 |
| warm-clamp-head | 70355ns | 64324ns | 73313ns | -42.55% | 14.904 |
| warm-clamp-min-lanes | 244139ns | 243733ns | 245348ns | +99.36% | 4.295 |
| warm-clamp-minimum | 242372ns | 242214ns | 242620ns | +97.91% | 4.326 |

## Performance model

- Peak throughput: **16.302 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.527 | 52.3% |
| warm-clamp-accfit | 16.255 | 99.7% |
| warm-clamp-accfit-dyn | 14.850 | 91.1% |
| warm-clamp-head | 14.330 | 87.9% |
| warm-clamp-min-lanes | 4.300 | 26.4% |
| warm-clamp-minimum | 4.327 | 26.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 122614ns | 122614ns | base |
| warm-clamp-accfit | 67192ns | 67192ns | -45.20% |
| warm-clamp-accfit-dyn | 70223ns | 70223ns | -42.73% |
| warm-clamp-head | 70502ns | 70502ns | -42.50% |
| warm-clamp-min-lanes | 244315ns | 244315ns | +99.26% |
| warm-clamp-minimum | 242544ns | 242544ns | +97.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 122971ns | base | --- | [122964, 123002] | --- | --- | --- | --- |
| warm-clamp-accfit | 64507ns | -54307.0ns (-44.2%) | [-58534, -52388]ns | [64486, 70577] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 70609ns | -52365.4ns (-42.6%) | [-52536, -52282]ns | [70587, 70671] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 73172ns | -49900.9ns (-40.6%) | [-54161, -49762]ns | [69061, 73215] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 243875ns | +120863.3ns (+98.3%) | [+120806, +120925]ns | [243827, 243902] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 242336ns | +119350.4ns (+97.1%) | [+119268, +119410]ns | [242308, 242378] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 122970ns | -42.3% | -42.6% | -40.4% | +98.3% | +97.1% |
| 1 | 2 | 119638ns | -46.1% | -41.0% | -46.2% | +103.9% | +102.6% |
| 1 | 3 | 123233ns | -47.7% | -42.7% | -44.0% | +97.9% | +96.6% |
| 1 | 4 | 122965ns | -42.6% | -42.9% | -40.4% | +98.3% | +97.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.434 | moderate- |
| warm-clamp-accfit | -0.250 | moderate- |
| warm-clamp-accfit-dyn | 0.058 | ok |
| warm-clamp-head | -0.302 | moderate- |
| warm-clamp-min-lanes | -0.251 | moderate- |
| warm-clamp-minimum | 0.205 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 30.3ns | 122463.5ns | 0.0% |  |
| warm-clamp-accfit | 31.6ns | 67034.9ns | 0.0% |  |
| warm-clamp-accfit-dyn | 30.7ns | 70044.4ns | 0.0% |  |
| warm-clamp-head | 32.1ns | 70355.2ns | 0.0% |  |
| warm-clamp-min-lanes | 33.3ns | 244139.1ns | 0.0% |  |
| warm-clamp-minimum | 33.2ns | 242372.1ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 119989.9-123315.0 ns)
  119989.9 |
  120156.1 |
  120322.4 |
  120488.6 |##
  120654.9 |
  120821.1 |
  120987.4 |
  121153.7 |
  121319.9 |
  121486.2 |
  121652.4 |
  121818.7 |
  121984.9 |
  122151.2 |
  122317.4 |
  122483.7 |
  122650.0 |
  122816.2 |########################################
  122982.5 |###########
  123148.7 |#################
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 64408.6-71170.5 ns)
  64408.6 |########################################
  64746.7 |
  65084.8 |
  65422.9 |
  65761.0 |
  66099.1 |
  66437.2 |
  66775.3 |
  67113.4 |
  67451.5 |
  67789.5 |
  68127.6 |
  68465.7 |
  68803.8 |
  69141.9 |
  69480.0 |
  69818.1 |
  70156.2 |
  70494.3 |####################
  70832.4 |##########
  (4 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 67351.1-70983.2 ns)
  67351.1 |
  67532.7 |
  67714.3 |
  67895.9 |
  68077.5 |
  68259.1 |
  68440.7 |
  68622.3 |
  68803.9 |
  68985.5 |
  69167.1 |
  69348.7 |##
  69530.3 |
  69711.9 |
  69893.5 |
  70075.1 |
  70256.7 |
  70438.3 |########################################
  70620.0 |##############################
  70801.6 |#####
  (4 below, 5 above range)

warm-clamp-head (n=40, range 64323.5-73313.4 ns)
  64323.5 |######
  64773.0 |
  65222.5 |
  65672.0 |
  66121.5 |
  66571.0 |
  67020.5 |
  67470.0 |
  67919.5 |
  68368.9 |
  68818.4 |##################
  69267.9 |##
  69717.4 |
  70166.9 |##
  70616.4 |
  71065.9 |
  71515.4 |
  71964.9 |
  72414.4 |
  72863.9 |########################################
  (5 below, 2 above range)

warm-clamp-min-lanes (n=40, range 243733.0-245348.2 ns)
  243733.0 |################
  243813.7 |########################################
  243894.5 |#####################
  243975.3 |########
  244056.0 |##
  244136.8 |
  244217.5 |
  244298.3 |
  244379.1 |
  244459.8 |
  244540.6 |
  244621.4 |##
  244702.1 |
  244782.9 |
  244863.7 |
  244944.4 |
  245025.2 |
  245106.0 |##
  245186.7 |
  245267.5 |
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 242214.0-242620.0 ns)
  242214.0 |########
  242234.3 |################################
  242254.6 |########
  242274.9 |################################
  242295.2 |################
  242315.5 |########################################
  242335.8 |########################
  242356.1 |################
  242376.4 |################################
  242396.7 |########
  242417.0 |################
  242437.3 |########
  242457.6 |################
  242477.9 |
  242498.2 |########
  242518.5 |########
  242538.8 |########
  242559.1 |
  242579.4 |
  242599.7 |
  (3 below, 2 above range)

```
