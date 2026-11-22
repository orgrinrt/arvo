# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn beats baseline by 43% (significant)

warm-clamp-accfit-dyn is -53.29 us (43%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 3.6x slower than the field

warm-clamp-min-lanes (243.79 us) is 3.6x the fastest (67.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-clamp-accfit-dyn (67.84 us) to slowest warm-clamp-min-lanes (243.79 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 67844.0 ns median (-44.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.59x (fastest 67844.0 ns, slowest 243787.7 ns)

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
| Counter frequency | 2112826587 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 122135ns | 123050ns | 118842ns | 122568ns | 124126ns | base |
| warm-clamp-accfit | 69419ns | 70962ns | 64559ns | 70433ns | 71235ns | -43.16% |
| warm-clamp-accfit-dyn | 67990ns | 68001ns | 64620ns | 67984ns | 71379ns | -44.33% |
| warm-clamp-head | 70443ns | 69568ns | 69198ns | 69850ns | 73466ns | -42.32% |
| warm-clamp-min-lanes | 244014ns | 243976ns | 243910ns | 243981ns | 244214ns | +99.79% |
| warm-clamp-minimum | 242456ns | 242419ns | 242215ns | 242406ns | 242849ns | +98.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 121970ns | 118682ns | 123959ns | base | 8.597 |
| warm-clamp-accfit | 69259ns | 64405ns | 71067ns | -43.22% | 15.140 |
| warm-clamp-accfit-dyn | 67817ns | 64458ns | 71200ns | -44.40% | 15.462 |
| warm-clamp-head | 70284ns | 69046ns | 73298ns | -42.38% | 14.919 |
| warm-clamp-min-lanes | 243832ns | 243728ns | 244040ns | +99.91% | 4.300 |
| warm-clamp-minimum | 242277ns | 242047ns | 242669ns | +98.64% | 4.328 |

## Performance model

- Peak throughput: **16.281 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.533 | 52.4% |
| warm-clamp-accfit | 14.809 | 91.0% |
| warm-clamp-accfit-dyn | 15.456 | 94.9% |
| warm-clamp-head | 15.111 | 92.8% |
| warm-clamp-min-lanes | 4.301 | 26.4% |
| warm-clamp-minimum | 4.329 | 26.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 122135ns | 122135ns | base |
| warm-clamp-accfit | 69419ns | 69419ns | -43.16% |
| warm-clamp-accfit-dyn | 67990ns | 67990ns | -44.33% |
| warm-clamp-head | 70443ns | 70443ns | -42.32% |
| warm-clamp-min-lanes | 244014ns | 244014ns | +99.79% |
| warm-clamp-minimum | 242456ns | 242456ns | +98.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 122884ns | base | --- | [122853, 122904] | --- | --- | --- | --- |
| warm-clamp-accfit | 70809ns | -51877.8ns (-42.2%) | [-51902, -51831]ns | [70591, 70986] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 67844ns | -53292.2ns (-43.4%) | [-58344, -52123]ns | [64532, 71070] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 69393ns | -50200.4ns (-40.9%) | [-53683, -49937]ns | [69176, 69580] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 243788ns | +120904.9ns (+98.4%) | [+120868, +120938]ns | [243773, 243822] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 242234ns | +119382.7ns (+97.2%) | [+119215, +119558]ns | [242174, 242260] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 122853ns | -42.2% | -47.5% | -43.8% | +98.4% | +97.2% |
| 1 | 2 | 122889ns | -42.2% | -47.5% | -43.7% | +98.4% | +97.0% |
| 1 | 3 | 123230ns | -47.7% | -42.3% | -40.5% | +97.8% | +96.5% |
| 1 | 4 | 118683ns | -40.5% | -40.1% | -41.4% | +105.5% | +104.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.147 | ok |
| warm-clamp-accfit | -0.382 | moderate- |
| warm-clamp-accfit-dyn | 0.250 | moderate+ |
| warm-clamp-head | -0.329 | moderate- |
| warm-clamp-min-lanes | -0.082 | ok |
| warm-clamp-minimum | -0.423 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 28.6ns | 121969.5ns | 0.0% |  |
| warm-clamp-accfit | 29.9ns | 69259.5ns | 0.0% |  |
| warm-clamp-accfit-dyn | 32.1ns | 67817.4ns | 0.0% |  |
| warm-clamp-head | 29.6ns | 70284.4ns | 0.0% |  |
| warm-clamp-min-lanes | 31.6ns | 243832.4ns | 0.0% |  |
| warm-clamp-minimum | 32.2ns | 242277.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 118682.2-123959.3 ns)
  118682.2 |#####################
  118946.1 |
  119209.9 |
  119473.8 |
  119737.6 |
  120001.5 |
  120265.4 |
  120529.2 |
  120793.1 |
  121056.9 |
  121320.8 |
  121584.6 |
  121848.5 |
  122112.3 |
  122376.2 |
  122640.0 |########################################
  122903.9 |########
  123167.7 |########################
  123431.6 |##
  123695.4 |
  (3 below, 1 above range)

warm-clamp-accfit (n=40, range 64404.9-71067.0 ns)
  64404.9 |#############
  64738.0 |
  65071.1 |
  65404.2 |
  65737.3 |
  66070.4 |
  66403.5 |
  66736.6 |
  67069.7 |
  67402.8 |
  67735.9 |
  68069.1 |
  68402.2 |
  68735.3 |
  69068.4 |
  69401.5 |
  69734.6 |
  70067.7 |
  70400.8 |######################
  70733.9 |########################################
  (4 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 64457.6-71200.4 ns)
  64457.6 |########################################
  64794.8 |
  65131.9 |
  65469.0 |
  65806.2 |
  66143.3 |
  66480.5 |
  66817.6 |
  67154.7 |
  67491.9 |
  67829.0 |
  68166.2 |
  68503.3 |
  68840.4 |
  69177.6 |
  69514.7 |
  69851.9 |
  70189.0 |
  70526.1 |
  70863.3 |########################################
  (3 below, 3 above range)

warm-clamp-head (n=40, range 69046.3-73297.6 ns)
  69046.3 |########################################
  69258.9 |
  69471.4 |##########################
  69684.0 |
  69896.6 |
  70109.1 |
  70321.7 |
  70534.2 |
  70746.8 |
  70959.4 |
  71171.9 |
  71384.5 |
  71597.1 |
  71809.6 |
  72022.2 |
  72234.7 |
  72447.3 |
  72659.9 |
  72872.4 |
  73085.0 |################
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 243728.3-244040.5 ns)
  243728.3 |#############
  243743.9 |##########################
  243759.5 |########################################
  243775.1 |########################################
  243790.7 |######
  243806.4 |####################
  243822.0 |####################
  243837.6 |####################
  243853.2 |
  243868.8 |
  243884.4 |
  243900.0 |#############
  243915.6 |
  243931.2 |####################
  243946.8 |######
  243962.4 |
  243978.1 |
  243993.7 |
  244009.3 |
  244024.9 |
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 242047.1-242669.3 ns)
  242047.1 |########
  242078.2 |
  242109.4 |#################
  242140.5 |#################
  242171.6 |########
  242202.7 |#############
  242233.8 |########################################
  242264.9 |####
  242296.0 |#############
  242327.1 |####
  242358.2 |########
  242389.3 |
  242420.4 |
  242451.6 |####
  242482.7 |
  242513.8 |
  242544.9 |####
  242576.0 |
  242607.1 |
  242638.2 |
  (4 below, 3 above range)

```
