# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1830% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (10.06 us) leads bitpack-write-unsound (194.09 us) by 1830%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 24.3x slower than the field

bitpack-write-guarded (244.46 us) is 24.3x the fastest (10.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (10.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 24.3x the fastest

Fastest bitpack-write-dense (10.06 us) to slowest bitpack-write-guarded (244.46 us): 24.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 10057.8 ns median
- 2 variants significantly slower than baseline
- Spread: 24.31x (fastest 10057.8 ns, slowest 244463.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef2 |
| Counter frequency | 2112756716 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 10155ns | 10131ns | 10119ns | 10137ns | 10246ns | base |
| bitpack-write-guarded | 244754ns | 244551ns | 219279ns | 244413ns | 271251ns | +2310.11% |
| bitpack-write-unsound | 204968ns | 194173ns | 192874ns | 198182ns | 237418ns | +1918.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 10081ns | 10050ns | 10170ns | base | 6.501 |
| bitpack-write-guarded | 244661ns | 219198ns | 271141ns | +2327.04% | 0.268 |
| bitpack-write-unsound | 204879ns | 192795ns | 237316ns | +1932.40% | 0.320 |

## Performance model

- Peak throughput: **6.521 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 6.516 | 99.9% |
| bitpack-write-guarded | 0.268 | 4.1% |
| bitpack-write-unsound | 0.338 | 5.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 10155ns | 10155ns | base |
| bitpack-write-guarded | 244754ns | 244754ns | +2310.11% |
| bitpack-write-unsound | 204968ns | 204968ns | +1918.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 10058ns | base | --- | [10056, 10065] | --- | --- | --- | --- |
| bitpack-write-guarded | 244464ns | +234381.0ns (+2330.3%) | [+210374, +257926]ns | [220435, 268016] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 194088ns | +184027.0ns (+1829.7%) | [+183118, +184864]ns | [193177, 194922] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|---|
| 1 | 1 | 10071ns | +2561.7% | +1827.3% |
| 1 | 2 | 10056ns | +2579.7% | +1818.0% |
| 1 | 3 | 10060ns | +2091.3% | +1816.7% |
| 1 | 4 | 10054ns | +2080.5% | +2259.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.235 | moderate- |
| bitpack-write-guarded | 0.249 | moderate+ |
| bitpack-write-unsound | -0.091 | ok |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4
- **bitpack-write-unsound**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 20.2ns | 10080.6ns | 0.2% |  |
| bitpack-write-guarded | 22.9ns | 244660.9ns | 0.0% |  |
| bitpack-write-unsound | 21.1ns | 204879.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 10049.6-10170.3 ns)
  10049.6 |########################################
  10055.7 |########################################
  10061.7 |################
  10067.7 |############
  10073.8 |########
  10079.8 |####
  10085.8 |####
  10091.9 |
  10097.9 |####
  10103.9 |
  10110.0 |
  10116.0 |
  10122.0 |
  10128.1 |
  10134.1 |
  10140.1 |
  10146.2 |
  10152.2 |####
  10158.2 |
  10164.3 |
  (3 below, 4 above range)

bitpack-write-guarded (n=40, range 219198.5-271141.2 ns)
  219198.5 |########################################
  221795.6 |
  224392.7 |
  226989.9 |
  229587.0 |
  232184.2 |
  234781.3 |
  237378.4 |
  239975.6 |
  242572.7 |
  245169.9 |
  247767.0 |
  250364.1 |
  252961.3 |
  255558.4 |
  258155.6 |
  260752.7 |
  263349.8 |
  265947.0 |#######################
  268544.1 |##################
  (3 below, 2 above range)

bitpack-write-unsound (n=40, range 192794.9-237315.6 ns)
  192794.9 |########################################
  195021.0 |#
  197247.0 |#
  199473.0 |###
  201699.1 |
  203925.1 |
  206151.1 |
  208377.2 |
  210603.2 |
  212829.2 |
  215055.3 |
  217281.3 |
  219507.3 |
  221733.4 |
  223959.4 |
  226185.4 |
  228411.4 |
  230637.5 |
  232863.5 |
  235089.5 |##############
  (4 below, 2 above range)

```
