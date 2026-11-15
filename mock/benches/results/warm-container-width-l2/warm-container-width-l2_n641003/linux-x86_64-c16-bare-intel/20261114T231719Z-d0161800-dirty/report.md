# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 34% faster than the next best (warm-container-native)

warm-container-minimum (245.33 us) leads warm-container-native (328.63 us) by 34%, a clear separation rather than a photo finish. CV 17.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum beats baseline by 64% (significant)

warm-container-minimum is -458.73 us (64%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.2x slower than the field

warm-container-plusone (792.04 us) is 3.2x the fastest (245.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (120% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 120% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest warm-container-minimum (245.33 us) to slowest warm-container-plusone (792.04 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 1.6x its best-20%

warm-container-native's best 20% of batches run at 256.81 us but its worst 20% at 407.03 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 245325.4 ns median (-66.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.23x (fastest 245325.4 ns, slowest 792037.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 738246ns | 722090ns | 669403ns | 724384ns | 848675ns | base |
| warm-container-minimum | 248816ns | 245668ns | 204564ns | 244402ns | 306312ns | -66.30% |
| warm-container-native | 329090ns | 328991ns | 257019ns | 326967ns | 407533ns | -55.42% |
| warm-container-plusone | 752647ns | 792768ns | 665664ns | 757210ns | 825941ns | +1.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 737572ns | 668830ns | 848085ns | base | 5.687 |
| warm-container-minimum | 248552ns | 204385ns | 306003ns | -66.30% | 16.875 |
| warm-container-native | 328720ns | 256806ns | 407031ns | -55.43% | 12.759 |
| warm-container-plusone | 751837ns | 664534ns | 825039ns | +1.93% | 5.579 |

## Performance model

- Peak throughput: **20.522 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.814 | 28.3% |
| warm-container-minimum | 17.097 | 83.3% |
| warm-container-native | 12.763 | 62.2% |
| warm-container-plusone | 5.296 | 25.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 738246ns | 738246ns | base |
| warm-container-minimum | 248816ns | 248816ns | -66.30% |
| warm-container-native | 329090ns | 329090ns | -55.42% |
| warm-container-plusone | 752647ns | 752647ns | +1.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 721410ns | base | --- | [672565, 737655] | --- | --- | --- | --- |
| warm-container-minimum | 245325ns | -458725.5ns (-63.6%) | [-468129, -446792]ns | [208369, 277175] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 328631ns | -404152.3ns (-56.0%) | [-426224, -390026]ns | [314221, 346987] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 792038ns | no significant difference | [-364, +70833]ns | [673439, 801600] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 836922ns | -75.5% | -62.4% | -20.6% |
| 1 | 2 | 672394ns | -69.4% | -48.1% | +22.7% |
| 1 | 3 | 737286ns | -59.9% | -52.9% | +8.5% |
| 1 | 4 | 669275ns | -58.7% | -61.9% | +7.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.383 | moderate- |
| warm-container-minimum | 0.178 | ok |
| warm-container-native | -0.169 | ok |
| warm-container-plusone | -0.273 | moderate- |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 70.8ns | 737571.9ns | 0.0% |  |
| warm-container-minimum | 26.4ns | 248552.4ns | 0.0% |  |
| warm-container-native | 37.6ns | 328720.3ns | 0.0% |  |
| warm-container-plusone | 69.4ns | 751837.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 668830.2-848084.6 ns)
  668830.2 |########################################
  677793.0 |##
  686755.7 |
  695718.4 |
  704681.1 |##
  713643.8 |
  722606.5 |
  731569.3 |####################
  740532.0 |
  749494.7 |##
  758457.4 |
  767420.1 |
  776382.8 |
  785345.6 |
  794308.3 |
  803271.0 |
  812233.7 |
  821196.4 |
  830159.1 |############################
  839121.9 |
  (4 below, 2 above range)

warm-container-minimum (n=40, range 204385.1-306002.8 ns)
  204385.1 |########################################
  209466.0 |#######
  214546.9 |##############
  219627.7 |
  224708.6 |
  229789.5 |
  234870.4 |
  239951.3 |
  245032.2 |
  250113.0 |
  255193.9 |
  260274.8 |
  265355.7 |
  270436.6 |#######
  275517.5 |#############################
  280598.3 |#######
  285679.2 |###
  290760.1 |#######
  295841.0 |#######
  300921.9 |
  (3 below, 3 above range)

warm-container-native (n=40, range 256806.3-407030.7 ns)
  256806.3 |####
  264317.5 |
  271828.7 |#################
  279339.9 |
  286851.2 |
  294362.4 |
  301873.6 |
  309384.8 |########################################
  316896.1 |####
  324407.3 |
  331918.5 |
  339429.7 |##########################
  346940.9 |########################################
  354452.2 |
  361963.4 |
  369474.6 |####
  376985.8 |
  384497.0 |
  392008.3 |
  399519.5 |####
  (5 below, 3 above range)

warm-container-plusone (n=40, range 664534.0-825039.0 ns)
  664534.0 |########################################
  672559.3 |####
  680584.5 |
  688609.8 |####
  696635.0 |
  704660.3 |
  712685.5 |
  720710.8 |
  728736.0 |
  736761.3 |
  744786.5 |####
  752811.8 |####
  760837.0 |
  768862.2 |
  776887.5 |####
  784912.7 |########
  792938.0 |########
  800963.2 |######################
  808988.5 |
  817013.7 |########################################
  (5 below, 3 above range)

```
