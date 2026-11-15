# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (216.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 9.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1241% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (9.98 us) leads bitpack-write-windowed (133.76 us) by 1241%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 95% (significant)

bitpack-write-dense is -206.70 us (95%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 21.7x slower than the field

bitpack-write-aligned (216.82 us) is 21.7x the fastest (9.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 21.7x the fastest

Fastest bitpack-write-dense (9.98 us) to slowest bitpack-write-aligned (216.82 us): 21.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 9977.0 ns median (-95.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 21.73x (fastest 9977.0 ns, slowest 216818.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 215286ns | 216917ns | 193154ns | 215271ns | 237464ns | base |
| bitpack-write-dense | 10088ns | 10057ns | 10038ns | 10067ns | 10200ns | -95.31% |
| bitpack-write-windowed | 133637ns | 133841ns | 119312ns | 133716ns | 147726ns | -37.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 215188ns | 193062ns | 237352ns | base | 0.305 |
| bitpack-write-dense | 10006ns | 9962ns | 10110ns | -95.35% | 6.550 |
| bitpack-write-windowed | 133553ns | 119241ns | 147628ns | -37.94% | 0.491 |

## Performance model

- Peak throughput: **6.579 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.302 | 4.6% |
| bitpack-write-dense | 6.569 | 99.8% |
| bitpack-write-windowed | 0.490 | 7.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 215286ns | 215286ns | base |
| bitpack-write-dense | 10088ns | 10088ns | -95.31% |
| bitpack-write-windowed | 133637ns | 133637ns | -37.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 216818ns | base | --- | [194143, 235600] | --- | --- | --- | --- |
| bitpack-write-dense | 9977ns | -206695.8ns (-95.3%) | [-225627, -184141]ns | [9970, 10010] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 133764ns | -81105.8ns (-37.4%) | [-90239, -73829]ns | [120336, 147089] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 194123ns | -94.8% | -38.1% |
| 1 | 2 | 192744ns | -94.8% | -23.7% |
| 1 | 3 | 237314ns | -95.8% | -38.0% |
| 1 | 4 | 235594ns | -95.8% | -49.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.231 | moderate+ |
| bitpack-write-dense | -0.190 | ok |
| bitpack-write-windowed | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 23.3ns | 215188.2ns | 0.0% |  |
| bitpack-write-dense | 21.4ns | 10006.2ns | 0.2% |  |
| bitpack-write-windowed | 22.5ns | 133552.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 193061.6-237352.2 ns)
  193061.6 |############################
  195276.2 |
  197490.7 |##
  199705.2 |
  201919.8 |
  204134.3 |
  206348.8 |
  208563.3 |
  210777.9 |
  212992.4 |
  215206.9 |
  217421.5 |
  219636.0 |
  221850.5 |
  224065.0 |
  226279.6 |
  228494.1 |
  230708.6 |
  232923.2 |
  235137.7 |########################################
  (6 below, 2 above range)

bitpack-write-dense (n=40, range 9961.8-10109.8 ns)
   9961.8 |########################################
   9969.2 |####################
   9976.6 |############
   9984.0 |####
   9991.4 |####
   9998.8 |####
  10006.2 |####################
  10013.6 |########
  10021.0 |
  10028.4 |####
  10035.8 |
  10043.2 |
  10050.6 |
  10058.0 |
  10065.4 |
  10072.8 |####
  10080.2 |
  10087.6 |
  10095.0 |
  10102.4 |
  (4 below, 6 above range)

bitpack-write-windowed (n=40, range 119240.9-147627.8 ns)
  119240.9 |###################################
  120660.3 |
  122079.6 |
  123499.0 |
  124918.3 |
  126337.6 |
  127757.0 |
  129176.3 |
  130595.7 |
  132015.0 |
  133434.4 |
  134853.7 |
  136273.0 |
  137692.4 |
  139111.7 |
  140531.1 |
  141950.4 |
  143369.7 |
  144789.1 |
  146208.4 |########################################
  (4 below, 2 above range)

```
