# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-acc64 dominates: 10% faster than the next best (warm-clamp-minimum)

warm-clamp-acc64 (129.68 us) leads warm-clamp-minimum (142.69 us) by 10%, a clear separation rather than a photo finish. CV 26.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is an outlier: 4.7x slower than the field

warm-clamp-min-lanes (612.71 us) is 4.7x the fastest (129.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (129.68 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.7x the fastest

Fastest warm-clamp-acc64 (129.68 us) to slowest warm-clamp-min-lanes (612.71 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 1.8x its best-20%

warm-clamp-minimum's best 20% of batches run at 121.86 us but its worst 20% at 224.64 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 129684.6 ns median
- 3 variants significantly slower than baseline
- Spread: 4.72x (fastest 129684.6 ns, slowest 612707.2 ns)

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
| warm-clamp-acc64 | 144483ns | 129833ns | 118161ns | 132304ns | 207339ns | base |
| warm-clamp-accfit | 248999ns | 257705ns | 199475ns | 247048ns | 304376ns | +72.34% |
| warm-clamp-accfit-dyn | 158948ns | 144217ns | 128745ns | 151129ns | 212609ns | +10.01% |
| warm-clamp-head | 551185ns | 561412ns | 435780ns | 551321ns | 666179ns | +281.49% |
| warm-clamp-min-lanes | 614814ns | 612900ns | 535710ns | 614419ns | 695102ns | +325.53% |
| warm-clamp-minimum | 158642ns | 142988ns | 122055ns | 148715ns | 225008ns | +9.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 144273ns | 118001ns | 206988ns | base | 7.268 |
| warm-clamp-accfit | 248613ns | 199127ns | 303884ns | +72.32% | 4.218 |
| warm-clamp-accfit-dyn | 158738ns | 128551ns | 212307ns | +10.03% | 6.606 |
| warm-clamp-head | 550513ns | 435147ns | 665576ns | +281.58% | 1.905 |
| warm-clamp-min-lanes | 614589ns | 535528ns | 694783ns | +325.99% | 1.706 |
| warm-clamp-minimum | 158370ns | 121862ns | 224644ns | +9.77% | 6.621 |

## Performance model

- Peak throughput: **8.886 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.086 | 91.0% |
| warm-clamp-accfit | 4.075 | 45.9% |
| warm-clamp-accfit-dyn | 7.279 | 81.9% |
| warm-clamp-head | 1.870 | 21.0% |
| warm-clamp-min-lanes | 1.711 | 19.3% |
| warm-clamp-minimum | 7.349 | 82.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 144483ns | 144483ns | base |
| warm-clamp-accfit | 248999ns | 248999ns | +72.34% |
| warm-clamp-accfit-dyn | 158948ns | 158948ns | +10.01% |
| warm-clamp-head | 551185ns | 551185ns | +281.49% |
| warm-clamp-min-lanes | 614814ns | 614814ns | +325.53% |
| warm-clamp-minimum | 158642ns | 158642ns | +9.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 129685ns | base | --- | [128618, 129870] | --- | --- | --- | --- |
| warm-clamp-accfit | 257324ns | +86804.9ns (+66.9%) | [+83030, +134039]ns | [212484, 276413] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 144046ns | no significant difference | [-117, +44198]ns | [129686, 174824] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-head | 560593ns | +430464.1ns (+331.9%) | [+332336, +482927]ns | [467417, 611591] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 612707ns | +483508.2ns (+372.8%) | [+442007, +527876]ns | [570777, 657560] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 142685ns | no significant difference | [-116, +36679]ns | [129326, 166462] | no | 0.3352 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 128685ns | +107.1% | -0.1% | +375.1% | +343.8% | -0.0% |
| 1 | 2 | 216382ns | +31.0% | -40.1% | +100.3% | +147.8% | -41.5% |
| 1 | 3 | 129760ns | +86.5% | +62.1% | +257.8% | +406.0% | +28.1% |
| 1 | 4 | 117993ns | +80.1% | +45.7% | +459.4% | +488.7% | +94.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.323 | moderate- |
| warm-clamp-accfit | 0.195 | ok |
| warm-clamp-accfit-dyn | 0.006 | ok |
| warm-clamp-head | -0.225 | moderate- |
| warm-clamp-min-lanes | 0.215 | moderate+ |
| warm-clamp-minimum | 0.192 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/4, lost 4/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 2/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 30.2ns | 144273.0ns | 0.0% |  |
| warm-clamp-accfit | 116.2ns | 248613.4ns | 0.0% |  |
| warm-clamp-accfit-dyn | 45.0ns | 158737.7ns | 0.0% |  |
| warm-clamp-head | 150.1ns | 550513.2ns | 0.0% |  |
| warm-clamp-min-lanes | 31.8ns | 614588.9ns | 0.0% |  |
| warm-clamp-minimum | 98.2ns | 158370.3ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 118001.0-206987.8 ns)
  118001.0 |##########
  122450.3 |
  126899.7 |########################################
  131349.0 |##
  135798.3 |
  140247.7 |
  144697.0 |
  149146.4 |
  153595.7 |
  158045.0 |
  162494.4 |
  166943.7 |
  171393.0 |######
  175842.4 |##
  180291.7 |
  184741.1 |
  189190.4 |
  193639.7 |
  198089.1 |
  202538.4 |
  (5 below, 6 above range)

warm-clamp-accfit (n=40, range 199127.0-303883.7 ns)
  199127.0 |
  204364.9 |##################
  209602.7 |########################################
  214840.5 |
  220078.4 |
  225316.2 |
  230554.0 |
  235791.9 |
  241029.7 |
  246267.5 |###
  251505.4 |
  256743.2 |
  261981.0 |###
  267218.9 |
  272456.7 |#####################
  277694.5 |##############
  282932.4 |###
  288170.2 |
  293408.0 |
  298645.9 |##############
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 128551.2-212306.9 ns)
  128551.2 |########################################
  132739.0 |
  136926.7 |
  141114.5 |
  145302.3 |
  149490.1 |
  153677.9 |
  157865.7 |#############
  162053.4 |
  166241.2 |
  170429.0 |##
  174616.8 |########
  178804.6 |
  182992.4 |##
  187180.1 |
  191367.9 |##########
  195555.7 |
  199743.5 |##
  203931.3 |
  208119.1 |
  (5 below, 5 above range)

warm-clamp-head (n=40, range 435146.5-665575.5 ns)
  435146.5 |
  446668.0 |###############
  458189.4 |####################
  469710.9 |###############
  481232.3 |
  492753.8 |#####
  504275.2 |##########
  515796.7 |
  527318.1 |
  538839.6 |
  550361.0 |
  561882.5 |
  573403.9 |
  584925.4 |
  596446.8 |
  607968.3 |########################################
  619489.7 |
  631011.2 |#####
  642532.6 |
  654054.1 |########################################
  (7 below, 3 above range)

warm-clamp-min-lanes (n=40, range 535528.4-694783.4 ns)
  535528.4 |####################
  543491.2 |####
  551453.9 |
  559416.7 |
  567379.4 |########################################
  575342.2 |
  583304.9 |
  591267.7 |
  599230.4 |
  607193.2 |
  615155.9 |
  623118.7 |
  631081.4 |
  639044.2 |
  647006.9 |############
  654969.7 |############################
  662932.4 |
  670895.2 |
  678857.9 |
  686820.7 |########################
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 121861.8-224643.5 ns)
  121861.8 |##
  127000.9 |########################################
  132140.0 |
  137279.1 |
  142418.2 |
  147557.3 |
  152696.3 |##
  157835.4 |
  162974.5 |########################
  168113.6 |
  173252.7 |
  178391.8 |##########
  183530.9 |
  188669.9 |
  193809.0 |
  198948.1 |
  204087.2 |
  209226.3 |
  214365.4 |##
  219504.5 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=23.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=21.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=25.2% (high variance, measurements may be unstable)
