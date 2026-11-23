# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (6.28 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 420.76 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 816% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (420.76 us) leads bitpack-write-windowed (3.86 ms) by 816%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 93% (significant)

bitpack-write-dense is -5.86 ms (93%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 14.9x slower than the field

bitpack-write-aligned (6.28 ms) is 14.9x the fastest (420.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 14.9x the fastest

Fastest bitpack-write-dense (420.76 us) to slowest bitpack-write-aligned (6.28 ms): 14.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 420761.7 ns median (-93.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 14.93x (fastest 420761.7 ns, slowest 6283659.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef1 |
| Counter frequency | 2112601419 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6314891ns | 6283896ns | 6273611ns | 6295953ns | 6412986ns | base |
| bitpack-write-dense | 421692ns | 421002ns | 413000ns | 420746ns | 433223ns | -93.32% |
| bitpack-write-windowed | 3855019ns | 3855450ns | 3845778ns | 3855087ns | 3864059ns | -38.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 6314604ns | 6273281ns | 6412719ns | base | 0.332 |
| bitpack-write-dense | 421462ns | 412826ns | 432968ns | -93.33% | 4.976 |
| bitpack-write-windowed | 3854755ns | 3845538ns | 3863774ns | -38.95% | 0.544 |

## Performance model

- Peak throughput: **5.080 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.334 | 6.6% |
| bitpack-write-dense | 4.984 | 98.1% |
| bitpack-write-windowed | 0.544 | 10.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 6314891ns | 6314891ns | base |
| bitpack-write-dense | 421692ns | 421692ns | -93.32% |
| bitpack-write-windowed | 3855019ns | 3855019ns | -38.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6283659ns | base | --- | [6278811, 6304400] | --- | --- | --- | --- |
| bitpack-write-dense | 420762ns | -5863766.2ns (-93.3%) | [-5882867, -5861874]ns | [418536, 422215] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 3855178ns | -2431888.5ns (-38.7%) | [-2446579, -2425940]ns | [3852181, 3857085] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 6374997ns | -93.4% | -39.4% |
| 1 | 2 | 6279671ns | -93.3% | -38.6% |
| 1 | 3 | 6303770ns | -93.2% | -38.9% |
| 1 | 4 | 6274918ns | -93.4% | -38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.254 | moderate- |
| bitpack-write-dense | -0.452 | moderate- |
| bitpack-write-windowed | 0.244 | moderate+ |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 64.1ns | 6314603.6ns | 0.0% |  |
| bitpack-write-dense | 100.4ns | 421461.9ns | 0.0% |  |
| bitpack-write-windowed | 54.5ns | 3854755.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 6273280.8-6412718.5 ns)
  6273280.8 |########################################
  6280252.7 |##############
  6287224.6 |#####
  6294196.4 |
  6301168.3 |########
  6308140.2 |##
  6315112.1 |##
  6322084.0 |
  6329055.9 |
  6336027.8 |##
  6342999.7 |
  6349971.5 |
  6356943.4 |##
  6363915.3 |#####
  6370887.2 |##
  6377859.1 |#####
  6384831.0 |
  6391802.9 |
  6398774.7 |
  6405746.6 |
  (3 below, 4 above range)

bitpack-write-dense (n=40, range 412826.0-432967.5 ns)
  412826.0 |########################################
  413833.1 |######
  414840.2 |
  415847.2 |######
  416854.3 |#############
  417861.4 |#############
  418868.5 |####################
  419875.5 |####################
  420882.6 |####################
  421889.7 |##########################
  422896.8 |
  423903.8 |
  424910.9 |#############
  425918.0 |####################
  426925.1 |
  427932.2 |####################
  428939.2 |
  429946.3 |
  430953.4 |######
  431960.5 |######
  (3 below, 2 above range)

bitpack-write-windowed (n=40, range 3845538.0-3863773.9 ns)
  3845538.0 |#############
  3846449.8 |####################
  3847361.6 |######
  3848273.4 |#############
  3849185.2 |####################
  3850097.0 |
  3851008.8 |######
  3851920.6 |######
  3852832.4 |#############
  3853744.2 |######
  3854656.0 |####################
  3855567.8 |#################################
  3856479.6 |
  3857391.3 |######
  3858303.1 |######
  3859214.9 |
  3860126.7 |####################
  3861038.5 |########################################
  3861950.3 |
  3862862.1 |
  (2 below, 3 above range)

```
