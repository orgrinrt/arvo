# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 2044% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (10.15 us) leads bitpack-write-unsound (217.57 us) by 2044%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 24.1x slower than the field

bitpack-write-guarded (244.18 us) is 24.1x the fastest (10.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (10.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 24.1x the fastest

Fastest bitpack-write-dense (10.15 us) to slowest bitpack-write-guarded (244.18 us): 24.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 10148.0 ns median
- 2 variants significantly slower than baseline
- Spread: 24.06x (fastest 10148.0 ns, slowest 244182.9 ns)

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
| Counter frequency | 2112761742 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 10252ns | 10235ns | 10191ns | 10236ns | 10358ns | base |
| bitpack-write-guarded | 244448ns | 244300ns | 219166ns | 244458ns | 269701ns | +2284.49% |
| bitpack-write-unsound | 215990ns | 217658ns | 194159ns | 215296ns | 239902ns | +2006.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 10163ns | 10107ns | 10269ns | base | 6.448 |
| bitpack-write-guarded | 244352ns | 219086ns | 269584ns | +2304.32% | 0.268 |
| bitpack-write-unsound | 215893ns | 194079ns | 239793ns | +2024.30% | 0.304 |

## Performance model

- Peak throughput: **6.484 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 6.458 | 99.6% |
| bitpack-write-guarded | 0.268 | 4.1% |
| bitpack-write-unsound | 0.301 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 10252ns | 10252ns | base |
| bitpack-write-guarded | 244448ns | 244448ns | +2284.49% |
| bitpack-write-unsound | 215990ns | 215990ns | +2006.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 10148ns | base | --- | [10140, 10153] | --- | --- | --- | --- |
| bitpack-write-guarded | 244183ns | +234035.5ns (+2306.2%) | [+210285, +258277]ns | [220408, 268432] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 217567ns | +207349.9ns (+2043.3%) | [+183956, +225403]ns | [194100, 235551] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|---|
| 1 | 1 | 10142ns | +2073.2% | +1813.8% |
| 1 | 2 | 10144ns | +2060.7% | +2239.1% |
| 1 | 3 | 10148ns | +2555.3% | +2221.1% |
| 1 | 4 | 10153ns | +2537.8% | +1811.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.249 | moderate+ |
| bitpack-write-guarded | 0.235 | moderate+ |
| bitpack-write-unsound | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4
- **bitpack-write-unsound**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 20.7ns | 10163.0ns | 0.2% |  |
| bitpack-write-guarded | 22.9ns | 244352.0ns | 0.0% |  |
| bitpack-write-unsound | 23.0ns | 215893.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 10107.0-10268.9 ns)
  10107.0 |#######
  10115.1 |#######
  10123.2 |###
  10131.3 |##################
  10139.4 |#####################
  10147.5 |########################################
  10155.6 |#######
  10163.6 |###
  10171.7 |
  10179.8 |
  10187.9 |
  10196.0 |
  10204.1 |
  10212.2 |
  10220.3 |
  10228.4 |###
  10236.5 |
  10244.6 |
  10252.7 |
  10260.8 |
  (4 below, 5 above range)

bitpack-write-guarded (n=40, range 219086.3-269584.0 ns)
  219086.3 |#####################################
  221611.2 |
  224136.1 |
  226661.0 |
  229185.9 |
  231710.7 |
  234235.6 |
  236760.5 |
  239285.4 |
  241810.3 |
  244335.2 |
  246860.0 |
  249384.9 |
  251909.8 |
  254434.7 |
  256959.6 |
  259484.4 |
  262009.3 |
  264534.2 |
  267059.1 |########################################
  (4 below, 3 above range)

bitpack-write-unsound (n=40, range 194078.5-239792.9 ns)
  194078.5 |########################################
  196364.2 |
  198649.9 |##
  200935.7 |
  203221.4 |
  205507.1 |
  207792.8 |
  210078.5 |
  212364.3 |
  214650.0 |
  216935.7 |
  219221.4 |
  221507.1 |
  223792.8 |
  226078.6 |
  228364.3 |
  230650.0 |
  232935.7 |
  235221.4 |########################################
  237507.1 |#####
  (3 below, 2 above range)

```
