# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 550% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (28.34 us) leads bitpack-write-guarded (184.11 us) by 550%, a clear separation rather than a photo finish. CV 22.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 22.9%)

bitpack-write-dense wins on median (28.34 us) yet has the highest variance (CV 22.9%), while bitpack-write-guarded is the steadiest (CV 10.7%, 184.11 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (28.34 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.5x the fastest

Fastest bitpack-write-dense (28.34 us) to slowest bitpack-write-guarded (184.11 us): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 2.1x its best-20%

bitpack-write-dense's best 20% of batches run at 14.65 us but its worst 20% at 31.02 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 28345.0 ns median
- 1 variant significantly slower than baseline
- Spread: 6.50x (fastest 28345.0 ns, slowest 184108.0 ns)

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
| bitpack-write-dense | 25784ns | 28417ns | 14738ns | 27689ns | 31113ns | base |
| bitpack-write-guarded | 170476ns | 184229ns | 135656ns | 177022ns | 185657ns | +561.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 25696ns | 14648ns | 31021ns | base | 2.550 |
| bitpack-write-guarded | 170359ns | 135546ns | 185539ns | +562.98% | 0.385 |

## Performance model

- Peak throughput: **4.474 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 2.312 | 51.7% |
| bitpack-write-guarded | 0.356 | 8.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 25784ns | 25784ns | base |
| bitpack-write-guarded | 170476ns | 170476ns | +561.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 28345ns | base | --- | [28181, 29545] | --- | --- | --- | --- |
| bitpack-write-guarded | 184108ns | +152193.5ns (+536.9%) | [+141515, +155283]ns | [171603, 184266] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 30042ns | +471.3% |
| 1 | 2 | 14651ns | +823.9% |
| 1 | 3 | 29663ns | +523.2% |
| 1 | 4 | 28327ns | +550.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.499 | moderate- |
| bitpack-write-guarded | -0.234 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 24.0ns | 25696.1ns | 0.1% |  |
| bitpack-write-guarded | 25.3ns | 170358.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 14648.4-31021.0 ns)
  14648.4 |################
  15467.0 |
  16285.6 |
  17104.3 |
  17922.9 |
  18741.5 |
  19560.2 |
  20378.8 |
  21197.4 |
  22016.1 |
  22834.7 |
  23653.3 |
  24471.9 |
  25290.6 |
  26109.2 |
  26927.8 |######
  27746.5 |#################################
  28565.1 |##########
  29383.7 |########################################
  30202.3 |
  (5 below, 3 above range)

bitpack-write-guarded (n=40, range 135546.4-185539.3 ns)
  135546.4 |
  138046.1 |##
  140545.7 |
  143045.4 |
  145545.0 |####
  148044.7 |
  150544.3 |
  153044.0 |
  155543.6 |
  158043.2 |
  160542.9 |
  163042.5 |
  165542.2 |
  168041.8 |
  170541.5 |##############
  173041.1 |
  175540.8 |##
  178040.4 |##
  180540.0 |
  183039.7 |########################################
  (7 below, 1 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=25.2% (high variance, measurements may be unstable)
