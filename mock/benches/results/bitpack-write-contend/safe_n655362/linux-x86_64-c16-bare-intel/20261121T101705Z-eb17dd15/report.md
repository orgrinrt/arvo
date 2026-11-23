# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (156.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 14.71 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 631% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (14.71 us) leads bitpack-write-windowed (107.54 us) by 631%, a clear separation rather than a photo finish. CV 21.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 91% (significant)

bitpack-write-dense is -141.35 us (91%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 10.6x slower than the field

bitpack-write-aligned (156.09 us) is 10.6x the fastest (14.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 21.7%)

bitpack-write-dense wins on median (14.71 us) yet has the highest variance (CV 21.7%), while bitpack-write-aligned is the steadiest (CV 5.1%, 156.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 10.6x the fastest

Fastest bitpack-write-dense (14.71 us) to slowest bitpack-write-aligned (156.09 us): 10.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 1.6x its best-20%

bitpack-write-dense's best 20% of batches run at 13.05 us but its worst 20% at 21.41 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 14708.5 ns median (-90.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.61x (fastest 14708.5 ns, slowest 156093.5 ns)

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
| bitpack-write-aligned | 157964ns | 156198ns | 149348ns | 156613ns | 170633ns | base |
| bitpack-write-dense | 16115ns | 14782ns | 13126ns | 15317ns | 21495ns | -89.80% |
| bitpack-write-windowed | 104865ns | 107614ns | 84258ns | 105191ns | 124495ns | -33.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 157871ns | 149250ns | 170538ns | base | 0.415 |
| bitpack-write-dense | 16036ns | 13050ns | 21415ns | -89.84% | 4.087 |
| bitpack-write-windowed | 104778ns | 84177ns | 124398ns | -33.63% | 0.625 |

## Performance model

- Peak throughput: **5.022 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.420 | 8.4% |
| bitpack-write-dense | 4.456 | 88.7% |
| bitpack-write-windowed | 0.609 | 12.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 157964ns | 157964ns | base |
| bitpack-write-dense | 16115ns | 16115ns | -89.80% |
| bitpack-write-windowed | 104865ns | 104865ns | -33.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 156094ns | base | --- | [152684, 158876] | --- | --- | --- | --- |
| bitpack-write-dense | 14709ns | -141347.0ns (-90.6%) | [-144142, -138032]ns | [14656, 14759] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 107541ns | -50782.0ns (-32.5%) | [-52063, -50271]ns | [101772, 108009] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 169668ns | -87.4% | -50.3% |
| 1 | 2 | 152744ns | -90.4% | -33.3% |
| 1 | 3 | 149409ns | -90.9% | -17.3% |
| 1 | 4 | 158492ns | -90.7% | -32.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.111 | ok |
| bitpack-write-dense | -0.019 | ok |
| bitpack-write-windowed | 0.086 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 22.6ns | 157870.6ns | 0.0% |  |
| bitpack-write-dense | 20.8ns | 16036.2ns | 0.1% |  |
| bitpack-write-windowed | 22.2ns | 104777.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 149250.2-170537.7 ns)
  149250.2 |########################################
  150314.6 |
  151379.0 |#############
  152443.3 |#################################
  153507.7 |####################
  154572.1 |
  155636.4 |
  156700.8 |
  157765.2 |########################################
  158829.6 |####################
  159893.9 |
  160958.3 |######
  162022.7 |
  163087.1 |
  164151.4 |
  165215.8 |
  166280.2 |
  167344.5 |
  168408.9 |#################################
  169473.3 |####################
  (4 below, 2 above range)

bitpack-write-dense (n=40, range 13049.9-21414.9 ns)
  13049.9 |
  13468.1 |
  13886.4 |
  14304.6 |########################################
  14722.9 |##################
  15141.1 |
  15559.4 |
  15977.6 |
  16395.9 |
  16814.1 |
  17232.4 |
  17650.6 |
  18068.9 |
  18487.1 |
  18905.4 |
  19323.6 |
  19741.9 |
  20160.1 |
  20578.4 |
  20996.6 |################
  (5 below, 3 above range)

bitpack-write-windowed (n=40, range 84176.7-124398.4 ns)
  84176.7 |##########################
  86187.8 |
  88198.8 |
  90209.9 |
  92221.0 |
  94232.1 |
  96243.2 |
  98254.3 |
  100265.3 |########################################
  102276.4 |
  104287.5 |
  106298.6 |########################################
  108309.7 |####
  110320.8 |####
  112331.9 |
  114342.9 |
  116354.0 |
  118365.1 |
  120376.2 |
  122387.3 |########################################
  (4 below, 1 above range)

```
