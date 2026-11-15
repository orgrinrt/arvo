# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (195.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 54.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 72% (significant)

warm-container-minimum is -140.94 us (72%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.6x slower than the field

warm-container-headroom (195.00 us) is 3.6x the fastest (54.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.76)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-plusone, warm-container-headroom} (245% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom} with a 245% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-container-minimum (54.34 us) to slowest warm-container-headroom (195.00 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-minimum** at 54341.7 ns median (-72.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.59x (fastest 54341.7 ns, slowest 194996.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 194383ns | 195185ns | 189628ns | 195405ns | 196069ns | base |
| warm-container-minimum | 54446ns | 54487ns | 54123ns | 54490ns | 54637ns | -71.99% |
| warm-container-native | 60176ns | 55483ns | 54733ns | 56975ns | 75223ns | -69.04% |
| warm-container-plusone | 191297ns | 191323ns | 186912ns | 191209ns | 195945ns | -1.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 194232ns | 189480ns | 195916ns | base | 21.594 |
| warm-container-minimum | 54294ns | 53974ns | 54484ns | -72.05% | 77.252 |
| warm-container-native | 60004ns | 54588ns | 74985ns | -69.11% | 69.900 |
| warm-container-plusone | 191149ns | 186771ns | 195799ns | -1.59% | 21.943 |

## Performance model

- Peak throughput: **77.709 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 21.510 | 27.7% |
| warm-container-minimum | 77.184 | 99.3% |
| warm-container-native | 75.804 | 97.5% |
| warm-container-plusone | 21.942 | 28.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 194383ns | 194383ns | base |
| warm-container-minimum | 54446ns | 54446ns | -71.99% |
| warm-container-native | 60176ns | 60176ns | -69.04% |
| warm-container-plusone | 191297ns | 191297ns | -1.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 194996ns | base | --- | [194965, 195634] | --- | --- | --- | --- |
| warm-container-minimum | 54342ns | -140939.0ns (-72.3%) | [-141279, -140623]ns | [54293, 54378] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 55331ns | -139579.0ns (-71.6%) | [-140148, -133320]ns | [55322, 55397] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 191154ns | -901.6ns (-0.5%) | [-4662, -180]ns | [186960, 195096] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 195655ns | -72.2% | -71.7% | -0.2% |
| 1 | 2 | 188102ns | -71.1% | -71.0% | -0.7% |
| 1 | 3 | 194982ns | -72.2% | -61.6% | +0.3% |
| 1 | 4 | 194967ns | -72.1% | -71.6% | -4.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.466 | moderate- |
| warm-container-minimum | -0.095 | ok |
| warm-container-native | -0.423 | moderate- |
| warm-container-plusone | -0.763 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 20.6ns | 194232.5ns | 0.0% |  |
| warm-container-minimum | 20.3ns | 54293.5ns | 0.0% |  |
| warm-container-native | 25.8ns | 60004.1ns | 0.0% |  |
| warm-container-plusone | 21.0ns | 191149.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 189480.0-195916.1 ns)
  189480.0 |
  189801.8 |
  190123.6 |
  190445.4 |
  190767.2 |
  191089.0 |
  191410.8 |
  191732.6 |
  192054.4 |
  192376.2 |
  192698.0 |
  193019.8 |
  193341.6 |
  193663.4 |
  193985.2 |
  194307.0 |
  194628.8 |##############
  194950.6 |############################
  195272.4 |########
  195594.2 |########################################
  (6 below, 2 above range)

warm-container-minimum (n=40, range 53974.5-54483.6 ns)
  53974.5 |
  53999.9 |########
  54025.4 |
  54050.8 |
  54076.3 |
  54101.7 |########
  54127.2 |########
  54152.6 |########
  54178.1 |################
  54203.6 |################
  54229.0 |
  54254.5 |########################
  54279.9 |########################
  54305.4 |########################
  54330.8 |################################
  54356.3 |########################################
  54381.7 |################################
  54407.2 |########
  54432.7 |################################
  54458.1 |########
  (1 below, 3 above range)

warm-container-native (n=40, range 54587.7-74985.5 ns)
  54587.7 |########################################
  55607.6 |
  56627.5 |
  57647.4 |
  58667.3 |
  59687.2 |
  60707.1 |
  61726.9 |
  62746.8 |
  63766.7 |
  64786.6 |
  65806.5 |
  66826.4 |
  67846.3 |
  68866.2 |
  69886.0 |
  70905.9 |
  71925.8 |#
  72945.7 |
  73965.6 |########
  (5 below, 4 above range)

warm-container-plusone (n=40, range 186771.4-195798.8 ns)
  186771.4 |########################################
  187222.8 |##
  187674.1 |
  188125.5 |
  188576.9 |
  189028.3 |
  189479.6 |
  189931.0 |
  190382.4 |
  190833.7 |
  191285.1 |
  191736.5 |
  192187.8 |
  192639.2 |
  193090.6 |
  193541.9 |
  193993.3 |
  194444.7 |##########
  194896.0 |###############
  195347.4 |####################
  (3 below, 2 above range)

```
