# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (195.84 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 54.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native dominates: 243% faster than the next best (warm-container-minimum)

warm-container-native (54.56 us) leads warm-container-minimum (187.07 us) by 243%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 72% (significant)

warm-container-native is -141.17 us (72%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.6x slower than the field

warm-container-headroom (195.84 us) is 3.6x the fastest (54.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-native} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (243% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 243% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-container-native (54.56 us) to slowest warm-container-headroom (195.84 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 54562.1 ns median (-72.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.59x (fastest 54562.1 ns, slowest 195838.9 ns)

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
| warm-container-headroom | 195760ns | 195998ns | 193045ns | 195991ns | 197781ns | base |
| warm-container-minimum | 186778ns | 187229ns | 185307ns | 187007ns | 187565ns | -4.59% |
| warm-container-native | 54200ns | 54726ns | 51776ns | 54520ns | 55660ns | -72.31% |
| warm-container-plusone | 187103ns | 187221ns | 185808ns | 187207ns | 188088ns | -4.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 195600ns | 192881ns | 197632ns | base | 21.443 |
| warm-container-minimum | 186611ns | 185133ns | 187388ns | -4.60% | 22.476 |
| warm-container-native | 54042ns | 51609ns | 55502ns | -72.37% | 77.612 |
| warm-container-plusone | 186943ns | 185657ns | 187923ns | -4.43% | 22.436 |

## Performance model

- Peak throughput: **81.270 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 21.417 | 26.4% |
| warm-container-minimum | 22.421 | 27.6% |
| warm-container-native | 76.872 | 94.6% |
| warm-container-plusone | 22.421 | 27.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 195760ns | 195760ns | base |
| warm-container-minimum | 186778ns | 186778ns | -4.59% |
| warm-container-native | 54200ns | 54200ns | -72.31% |
| warm-container-plusone | 187103ns | 187103ns | -4.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 195839ns | base | --- | [195583, 196045] | --- | --- | --- | --- |
| warm-container-minimum | 187066ns | -9477.7ns (-4.8%) | [-9829, -8549]ns | [186585, 187121] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 54562ns | -141170.0ns (-72.1%) | [-141389, -140819]ns | [54435, 54615] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 187068ns | -8901.6ns (-4.5%) | [-9267, -8595]ns | [186824, 187290] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 195825ns | -4.7% | -71.7% | -4.3% |
| 1 | 2 | 195857ns | -5.5% | -72.1% | -5.1% |
| 1 | 3 | 195237ns | -4.1% | -72.1% | -4.4% |
| 1 | 4 | 196907ns | -4.9% | -73.8% | -4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.414 | moderate- |
| warm-container-minimum | -0.204 | moderate- |
| warm-container-native | 0.001 | ok |
| warm-container-plusone | -0.288 | moderate- |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 19.5ns | 195600.0ns | 0.0% |  |
| warm-container-minimum | 22.4ns | 186610.9ns | 0.0% |  |
| warm-container-native | 23.3ns | 54042.0ns | 0.0% |  |
| warm-container-plusone | 20.0ns | 186943.4ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 192880.9-197632.2 ns)
  192880.9 |
  193118.5 |
  193356.1 |####
  193593.6 |
  193831.2 |
  194068.8 |
  194306.3 |
  194543.9 |
  194781.5 |####
  195019.0 |######################
  195256.6 |######################
  195494.2 |####
  195731.7 |########################################
  195969.3 |######################
  196206.8 |########
  196444.4 |########
  196682.0 |####
  196919.5 |#############
  197157.1 |
  197394.7 |####
  (2 below, 2 above range)

warm-container-minimum (n=40, range 185133.1-187387.7 ns)
  185133.1 |############
  185245.9 |####
  185358.6 |
  185471.3 |
  185584.0 |####
  185696.8 |
  185809.5 |
  185922.2 |
  186035.0 |
  186147.7 |
  186260.4 |
  186373.1 |
  186485.9 |########################
  186598.6 |############
  186711.3 |####
  186824.1 |
  186936.8 |
  187049.5 |########################################
  187162.3 |############################
  187275.0 |############
  (4 below, 1 above range)

warm-container-native (n=40, range 51609.3-55502.1 ns)
  51609.3 |#####################
  51803.9 |###
  51998.5 |
  52193.2 |
  52387.8 |
  52582.5 |
  52777.1 |
  52971.8 |
  53166.4 |
  53361.0 |
  53555.7 |
  53750.3 |
  53945.0 |
  54139.6 |
  54334.3 |################################
  54528.9 |########################################
  54723.5 |
  54918.2 |
  55112.8 |##########
  55307.5 |##################
  (3 below, 2 above range)

warm-container-plusone (n=40, range 185656.7-187922.5 ns)
  185656.7 |
  185770.0 |
  185883.3 |
  185996.6 |
  186109.9 |
  186223.2 |######
  186336.4 |
  186449.7 |#############
  186563.0 |##########################
  186676.3 |#############
  186789.6 |
  186902.9 |#################################
  187016.2 |#############
  187129.5 |#############
  187242.8 |########################################
  187356.1 |#################################
  187469.4 |######
  187582.6 |######
  187695.9 |######
  187809.2 |
  (5 below, 3 above range)

```
