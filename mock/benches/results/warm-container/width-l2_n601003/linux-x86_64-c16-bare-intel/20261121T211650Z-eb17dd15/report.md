# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (714.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 197.41 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 70% (significant)

warm-container-minimum is -501.60 us (70%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.6x slower than the field

warm-container-headroom (714.11 us) is 3.6x the fastest (197.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 40.8%)

warm-container-native wins on median (197.41 us) yet has the highest variance (CV 40.8%), while warm-container-minimum is the steadiest (CV 4.2%, 212.30 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-plusone} vs {warm-container-headroom} (147% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-plusone} and a slow tier {warm-container-headroom} with a 147% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-container-native (197.41 us) to slowest warm-container-headroom (714.11 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-native vs stability leader warm-container-minimum (+8% speed for 9.8x steadier)

warm-container-native is fastest (197.41 us, CV 40.8%); warm-container-minimum gives up 7.5% median for 9.8x lower variance (CV 4.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-native is inconsistent: worst-20% is 2.4x its best-20%

warm-container-native's best 20% of batches run at 151.84 us but its worst 20% at 365.09 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 197413.0 ns median (-72.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.62x (fastest 197413.0 ns, slowest 714105.5 ns)

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
| warm-container-headroom | 668819ns | 714775ns | 588115ns | 674167ns | 733476ns | base |
| warm-container-minimum | 209384ns | 212475ns | 194755ns | 211379ns | 218031ns | -68.69% |
| warm-container-native | 234848ns | 197605ns | 152021ns | 218926ns | 365440ns | -64.89% |
| warm-container-plusone | 271041ns | 289607ns | 197920ns | 280719ns | 315129ns | -59.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 668140ns | 587442ns | 732936ns | base | 6.278 |
| warm-container-minimum | 209213ns | 194595ns | 217858ns | -68.69% | 20.048 |
| warm-container-native | 234616ns | 151842ns | 365086ns | -64.89% | 17.877 |
| warm-container-plusone | 270788ns | 197731ns | 314806ns | -59.47% | 15.489 |

## Performance model

- Peak throughput: **27.623 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.874 | 21.3% |
| warm-container-minimum | 19.756 | 71.5% |
| warm-container-native | 21.246 | 76.9% |
| warm-container-plusone | 14.494 | 52.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 668819ns | 668819ns | base |
| warm-container-minimum | 209384ns | 209384ns | -68.69% |
| warm-container-native | 234848ns | 234848ns | -64.89% |
| warm-container-plusone | 271041ns | 271041ns | -59.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 714106ns | base | --- | [598987, 716458] | --- | --- | --- | --- |
| warm-container-minimum | 212304ns | -501596.9ns (-70.2%) | [-503306, -397357]ns | [211750, 212861] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 197413ns | -434277.1ns (-60.8%) | [-457319, -396191]ns | [191322, 251346] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 289380ns | -410459.5ns (-57.5%) | [-436961, -320109]ns | [268753, 297616] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 598406ns | -64.6% | -74.7% | -49.9% |
| 1 | 2 | 587588ns | -66.8% | -67.4% | -54.2% |
| 1 | 3 | 726728ns | -70.3% | -67.2% | -59.9% |
| 1 | 4 | 715843ns | -70.3% | -55.0% | -72.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.201 | moderate+ |
| warm-container-minimum | -0.437 | moderate- |
| warm-container-native | 0.208 | moderate+ |
| warm-container-plusone | -0.229 | moderate- |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 37.2ns | 668139.7ns | 0.0% |  |
| warm-container-minimum | 21.6ns | 209212.8ns | 0.0% |  |
| warm-container-native | 61.5ns | 234615.9ns | 0.0% |  |
| warm-container-plusone | 29.1ns | 270787.8ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 587441.6-732936.0 ns)
  587441.6 |########################################
  594716.3 |
  601991.1 |####
  609265.8 |####
  616540.5 |
  623815.2 |
  631089.9 |####
  638364.7 |
  645639.4 |
  652914.1 |
  660188.8 |
  667463.5 |
  674738.3 |
  682013.0 |
  689287.7 |
  696562.4 |
  703837.1 |
  711111.9 |########################################
  718386.6 |####################
  725661.3 |########################
  (4 below, 2 above range)

warm-container-minimum (n=40, range 194595.1-217857.8 ns)
  194595.1 |#################
  195758.2 |
  196921.3 |
  198084.5 |
  199247.6 |
  200410.7 |
  201573.9 |
  202737.0 |
  203900.1 |
  205063.3 |
  206226.4 |
  207389.5 |
  208552.7 |
  209715.8 |
  210878.9 |##############
  212042.1 |########################################
  213205.2 |
  214368.3 |###########
  215531.5 |##############
  216694.6 |##
  (4 below, 1 above range)

warm-container-native (n=40, range 151841.7-365085.7 ns)
  151841.7 |##########
  162503.9 |
  173166.1 |
  183828.3 |########################################
  194490.5 |###
  205152.7 |###
  215814.9 |##########
  226477.1 |
  237139.3 |
  247801.5 |##############
  258463.7 |###
  269125.9 |
  279788.1 |
  290450.3 |
  301112.5 |
  311774.7 |#####################
  322436.9 |
  333099.1 |###
  343761.3 |
  354423.5 |
  (6 below, 3 above range)

warm-container-plusone (n=40, range 197731.0-314806.0 ns)
  197731.0 |
  203584.8 |
  209438.5 |###########
  215292.3 |#####
  221146.0 |
  226999.8 |
  232853.5 |
  238707.3 |
  244561.0 |
  250414.8 |
  256268.5 |
  262122.3 |###########
  267976.0 |######################
  273829.8 |#####
  279683.5 |#####
  285537.3 |############################
  291391.0 |#################
  297244.8 |########################################
  303098.5 |######################
  308952.3 |
  (7 below, 3 above range)

```

## Diagnostics

- **warm-container-native**: CV=34.3% (high variance, measurements may be unstable)
