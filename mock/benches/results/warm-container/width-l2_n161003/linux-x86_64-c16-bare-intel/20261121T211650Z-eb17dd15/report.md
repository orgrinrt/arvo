# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (194.66 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 54.35 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 72% (significant)

warm-container-minimum is -139.80 us (72%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.6x slower than the field

warm-container-headroom (194.66 us) is 3.6x the fastest (54.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (54.35 us) and warm-container-native (54.67 us) differ by 0.59%, inside the noise, even though the wider field spreads 258.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-plusone, warm-container-headroom} (256% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom} with a 256% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-container-minimum (54.35 us) to slowest warm-container-headroom (194.66 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-minimum** at 54345.4 ns median (-72.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.58x (fastest 54345.4 ns, slowest 194655.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112751190 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 192100ns | 194805ns | 187007ns | 192738ns | 195279ns | base |
| warm-container-minimum | 54821ns | 54525ns | 54384ns | 54680ns | 55681ns | -71.46% |
| warm-container-native | 54270ns | 54817ns | 51443ns | 54679ns | 55871ns | -71.75% |
| warm-container-plusone | 195734ns | 194693ns | 186787ns | 193208ns | 212261ns | +1.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 191944ns | 186856ns | 195121ns | base | 21.852 |
| warm-container-minimum | 54655ns | 54234ns | 55503ns | -71.53% | 76.741 |
| warm-container-native | 54102ns | 51247ns | 55705ns | -71.81% | 77.525 |
| warm-container-plusone | 195575ns | 186636ns | 212081ns | +1.89% | 21.446 |

## Performance model

- Peak throughput: **81.844 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 21.547 | 26.3% |
| warm-container-minimum | 77.179 | 94.3% |
| warm-container-native | 76.724 | 93.7% |
| warm-container-plusone | 21.559 | 26.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 192100ns | 192100ns | base |
| warm-container-minimum | 54821ns | 54821ns | -71.46% |
| warm-container-native | 54270ns | 54270ns | -71.75% |
| warm-container-plusone | 195734ns | 195734ns | +1.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 194656ns | base | --- | [188652, 194827] | --- | --- | --- | --- |
| warm-container-minimum | 54345ns | -139796.2ns (-71.8%) | [-139988, -134354]ns | [54322, 54448] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 54668ns | -139217.3ns (-71.5%) | [-139859, -135815]ns | [54508, 54903] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 194549ns | no significant difference | [-246, +632]ns | [191962, 195512] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 195109ns | -71.7% | -72.0% | +0.3% |
| 1 | 2 | 194656ns | -72.1% | -71.9% | +0.4% |
| 1 | 3 | 194830ns | -72.1% | -71.5% | -0.2% |
| 1 | 4 | 186904ns | -70.9% | -72.6% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.088 | ok |
| warm-container-minimum | -0.119 | ok |
| warm-container-native | -0.237 | moderate- |
| warm-container-plusone | 0.013 | ok |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 19.4ns | 191943.6ns | 0.0% |  |
| warm-container-minimum | 19.8ns | 54655.4ns | 0.0% |  |
| warm-container-native | 20.0ns | 54102.4ns | 0.0% |  |
| warm-container-plusone | 22.2ns | 195575.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 186855.9-195120.5 ns)
  186855.9 |####################################
  187269.2 |
  187682.4 |
  188095.6 |
  188508.9 |
  188922.1 |
  189335.3 |
  189748.5 |
  190161.8 |###
  190575.0 |
  190988.2 |
  191401.5 |
  191814.7 |
  192227.9 |
  192641.1 |
  193054.4 |
  193467.6 |
  193880.8 |###
  194294.1 |#######################
  194707.3 |########################################
  (3 below, 5 above range)

warm-container-minimum (n=40, range 54234.4-55502.7 ns)
  54234.4 |########################################
  54297.8 |########################################
  54361.3 |#############
  54424.7 |#############
  54488.1 |
  54551.5 |
  54614.9 |
  54678.3 |
  54741.7 |
  54805.1 |
  54868.6 |
  54932.0 |
  54995.4 |
  55058.8 |########
  55122.2 |########
  55185.6 |#################
  55249.0 |####
  55312.4 |####
  55375.9 |
  55439.3 |
  (3 below, 3 above range)

warm-container-native (n=40, range 51247.2-55704.5 ns)
  51247.2 |##########
  51470.1 |
  51693.0 |##########
  51915.8 |
  52138.7 |
  52361.6 |
  52584.4 |
  52807.3 |
  53030.2 |
  53253.0 |
  53475.9 |
  53698.8 |
  53921.6 |
  54144.5 |
  54367.3 |########################################
  54590.2 |##############################
  54813.1 |####################
  55035.9 |##########
  55258.8 |####################
  55481.7 |###############
  (6 below, 3 above range)

warm-container-plusone (n=40, range 186636.4-212080.6 ns)
  186636.4 |############################
  187908.6 |
  189180.8 |##
  190453.0 |
  191725.2 |
  192997.5 |
  194269.7 |########################################
  195541.9 |###############################
  196814.1 |
  198086.3 |
  199358.5 |
  200630.7 |
  201902.9 |
  203175.1 |
  204447.3 |
  205719.5 |
  206991.7 |
  208264.0 |
  209536.2 |
  210808.4 |
  (3 below, 1 above range)

```
