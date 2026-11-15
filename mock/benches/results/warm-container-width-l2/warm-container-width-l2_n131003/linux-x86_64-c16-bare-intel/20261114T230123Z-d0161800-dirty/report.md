# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (186.91 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 54.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native dominates: 240% faster than the next best (warm-container-plusone)

warm-container-native (54.47 us) leads warm-container-plusone (184.96 us) by 240%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 71% (significant)

warm-container-native is -132.43 us (71%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.4x slower than the field

warm-container-headroom (186.91 us) is 3.4x the fastest (54.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-native} vs {warm-container-plusone, warm-container-minimum, warm-container-headroom} (240% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-plusone, warm-container-minimum, warm-container-headroom} with a 240% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.4x the fastest

Fastest warm-container-native (54.47 us) to slowest warm-container-headroom (186.91 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 54469.6 ns median (-70.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.43x (fastest 54469.6 ns, slowest 186906.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 188690ns | 187054ns | 186801ns | 187408ns | 194426ns | base |
| warm-container-minimum | 186342ns | 186629ns | 185144ns | 186371ns | 187455ns | -1.24% |
| warm-container-native | 54045ns | 54614ns | 51350ns | 54459ns | 55496ns | -71.36% |
| warm-container-plusone | 186399ns | 185119ns | 185029ns | 185517ns | 190415ns | -1.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 188535ns | 186651ns | 194261ns | base | 22.247 |
| warm-container-minimum | 186195ns | 185001ns | 187301ns | -1.24% | 22.526 |
| warm-container-native | 53892ns | 51194ns | 55338ns | -71.42% | 77.829 |
| warm-container-plusone | 186238ns | 184888ns | 190253ns | -1.22% | 22.521 |

## Performance model

- Peak throughput: **81.929 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 22.441 | 27.4% |
| warm-container-minimum | 22.492 | 27.5% |
| warm-container-native | 77.003 | 94.0% |
| warm-container-plusone | 22.677 | 27.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 188690ns | 188690ns | base |
| warm-container-minimum | 186342ns | 186342ns | -1.24% |
| warm-container-native | 54045ns | 54045ns | -71.36% |
| warm-container-plusone | 186399ns | 186399ns | -1.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 186906ns | base | --- | [186800, 187150] | --- | --- | --- | --- |
| warm-container-minimum | 186480ns | -1552.8ns (-0.8%) | [-1951, -800]ns | [185674, 186720] | YES | 0.0007 | 0.0007 | 0 |
| warm-container-native | 54470ns | -132428.1ns (-70.9%) | [-134921, -132206]ns | [54261, 54640] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 184958ns | -1771.9ns (-0.9%) | [-1982, -1489]ns | [184935, 185771] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 186906ns | -0.9% | -70.9% | -1.0% |
| 1 | 2 | 188145ns | -1.0% | -72.8% | -1.7% |
| 1 | 3 | 189494ns | -1.3% | -71.3% | -2.4% |
| 1 | 4 | 186873ns | -0.2% | -70.4% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.304 | moderate- |
| warm-container-minimum | 0.087 | ok |
| warm-container-native | -0.252 | moderate- |
| warm-container-plusone | -0.078 | ok |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 20.2ns | 188534.8ns | 0.0% |  |
| warm-container-minimum | 20.0ns | 186194.7ns | 0.0% |  |
| warm-container-native | 20.6ns | 53891.5ns | 0.0% |  |
| warm-container-plusone | 20.3ns | 186238.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 186651.0-194260.9 ns)
  186651.0 |########################################
  187031.5 |#####
  187412.0 |###
  187792.5 |
  188173.0 |
  188553.5 |
  188934.0 |
  189314.5 |#
  189695.0 |
  190075.5 |
  190456.0 |
  190836.5 |
  191217.0 |
  191597.4 |#
  191977.9 |###
  192358.4 |
  192738.9 |
  193119.4 |
  193499.9 |
  193880.4 |
  (4 below, 6 above range)

warm-container-minimum (n=40, range 185001.3-187300.5 ns)
  185001.3 |
  185116.3 |##############################
  185231.2 |####################
  185346.2 |##############################
  185461.1 |
  185576.1 |
  185691.1 |
  185806.0 |##############################
  185921.0 |
  186035.9 |
  186150.9 |##########
  186265.9 |
  186380.8 |########################################
  186495.8 |####################
  186610.7 |####################
  186725.7 |####################
  186840.7 |########################################
  186955.6 |##########
  187070.6 |##############################
  187185.6 |####################
  (6 below, 2 above range)

warm-container-native (n=40, range 51194.3-55337.6 ns)
  51194.3 |
  51401.5 |
  51608.6 |######
  51815.8 |#############
  52023.0 |
  52230.1 |
  52437.3 |
  52644.5 |
  52851.6 |
  53058.8 |
  53265.9 |
  53473.1 |
  53680.3 |
  53887.4 |
  54094.6 |########################################
  54301.8 |########################################
  54508.9 |########################################
  54716.1 |######
  54923.2 |#############
  55130.4 |#################################
  (7 below, 4 above range)

warm-container-plusone (n=40, range 184887.8-190253.1 ns)
  184887.8 |########################################
  185156.1 |
  185424.3 |#
  185692.6 |
  185960.9 |#
  186229.1 |#####
  186497.4 |#
  186765.7 |############
  187033.9 |
  187302.2 |
  187570.4 |
  187838.7 |
  188107.0 |
  188375.2 |
  188643.5 |
  188911.8 |
  189180.0 |
  189448.3 |
  189716.6 |
  189984.8 |
  (3 below, 2 above range)

```
