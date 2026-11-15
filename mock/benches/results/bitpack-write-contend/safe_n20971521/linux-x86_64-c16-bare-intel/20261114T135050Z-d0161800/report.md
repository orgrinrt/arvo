# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (6.28 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 414.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 829% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (414.46 us) leads bitpack-write-windowed (3.85 ms) by 829%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 93% (significant)

bitpack-write-dense is -5.86 ms (93%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 15.1x slower than the field

bitpack-write-aligned (6.28 ms) is 15.1x the fastest (414.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-aligned shows alternating (throttle bounce) (autocorr -0.69)

bitpack-write-aligned's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 15.1x the fastest

Fastest bitpack-write-dense (414.46 us) to slowest bitpack-write-aligned (6.28 ms): 15.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 414457.6 ns median (-93.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 15.14x (fastest 414457.6 ns, slowest 6275726.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6277223ns | 6276060ns | 6270914ns | 6276760ns | 6284921ns | base |
| bitpack-write-dense | 413389ns | 414728ns | 400916ns | 411692ns | 430953ns | -93.41% |
| bitpack-write-windowed | 3850776ns | 3848664ns | 3843645ns | 3848625ns | 3864361ns | -38.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 6276920ns | 6270603ns | 6284653ns | base | 0.334 |
| bitpack-write-dense | 413134ns | 400691ns | 430673ns | -93.42% | 5.076 |
| bitpack-write-windowed | 3850496ns | 3843365ns | 3864078ns | -38.66% | 0.545 |

## Performance model

- Peak throughput: **5.234 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.334 | 6.4% |
| bitpack-write-dense | 5.060 | 96.7% |
| bitpack-write-windowed | 0.545 | 10.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 6277223ns | 6277223ns | base |
| bitpack-write-dense | 413389ns | 413389ns | -93.41% |
| bitpack-write-windowed | 3850776ns | 3850776ns | -38.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6275726ns | base | --- | [6274080, 6278938] | --- | --- | --- | --- |
| bitpack-write-dense | 414458ns | -5862374.4ns (-93.4%) | [-5871287, -5858662]ns | [402149, 417504] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 3848380ns | -2427145.5ns (-38.7%) | [-2429910, -2424197]ns | [3846486, 3849800] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 6273462ns | -93.6% | -38.7% |
| 1 | 2 | 6279635ns | -93.6% | -38.7% |
| 1 | 3 | 6275036ns | -93.3% | -38.7% |
| 1 | 4 | 6277243ns | -93.3% | -38.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.691 | HIGH- (thermal bounce) |
| bitpack-write-dense | 0.243 | moderate+ |
| bitpack-write-windowed | -0.329 | moderate- |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 62.9ns | 6276920.3ns | 0.0% |  |
| bitpack-write-dense | 86.1ns | 413134.3ns | 0.0% |  |
| bitpack-write-windowed | 76.7ns | 3850496.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 6270603.0-6284653.0 ns)
  6270603.0 |########################################
  6271305.5 |
  6272008.0 |####################
  6272710.5 |########################################
  6273413.0 |##########
  6274115.5 |##############################
  6274818.0 |####################
  6275520.5 |####################
  6276223.0 |##########
  6276925.5 |##########
  6277628.0 |####################
  6278330.5 |##########
  6279033.0 |##########
  6279735.5 |####################
  6280438.0 |##########
  6281140.5 |##############################
  6281843.0 |##########
  6282545.5 |##########
  6283248.0 |##########
  6283950.5 |##########
  (3 below, 3 above range)

bitpack-write-dense (n=40, range 400690.7-430672.6 ns)
  400690.7 |########################################
  402189.8 |#####
  403688.9 |#####
  405188.0 |
  406687.1 |
  408186.2 |
  409685.3 |
  411184.4 |#####
  412683.5 |###############
  414182.6 |#####
  415681.7 |####################
  417180.7 |###################################
  418679.8 |#####
  420178.9 |#####
  421678.0 |
  423177.1 |#####
  424676.2 |#####
  426175.3 |
  427674.4 |
  429173.5 |
  (6 below, 4 above range)

bitpack-write-windowed (n=40, range 3843365.5-3864078.0 ns)
  3843365.5 |########################
  3844401.1 |################################
  3845436.7 |################################
  3846472.3 |################
  3847508.0 |########################################
  3848543.6 |########################################
  3849579.2 |################
  3850614.9 |########
  3851650.5 |################################
  3852686.1 |########
  3853721.7 |########
  3854757.4 |########
  3855793.0 |
  3856828.6 |
  3857864.2 |########
  3858899.9 |
  3859935.5 |########
  3860971.1 |
  3862006.8 |
  3863042.4 |
  (3 below, 2 above range)

```
