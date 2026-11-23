# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 933% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (428.13 us) leads bitpack-write-guarded (4.42 ms) by 933%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 6.3%)

bitpack-write-dense wins on median (428.13 us) yet has the highest variance (CV 6.3%), while bitpack-write-guarded is the steadiest (CV 1.3%, 4.42 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (428.13 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 10.3x the fastest

Fastest bitpack-write-dense (428.13 us) to slowest bitpack-write-guarded (4.42 ms): 10.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 428130.0 ns median
- 1 variant significantly slower than baseline
- Spread: 10.33x (fastest 428130.0 ns, slowest 4422302.4 ns)

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
| bitpack-write-dense | 418734ns | 428377ns | 372396ns | 424874ns | 446652ns | base |
| bitpack-write-guarded | 4424884ns | 4422631ns | 4346941ns | 4424853ns | 4502920ns | +956.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 418437ns | 371983ns | 446342ns | base | 5.012 |
| bitpack-write-guarded | 4424528ns | 4346548ns | 4502574ns | +957.39% | 0.474 |

## Performance model

- Peak throughput: **5.638 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.898 | 86.9% |
| bitpack-write-guarded | 0.474 | 8.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 418734ns | 418734ns | base |
| bitpack-write-guarded | 4424884ns | 4424884ns | +956.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 428130ns | base | --- | [418047, 429793] | --- | --- | --- | --- |
| bitpack-write-guarded | 4422302ns | +4018935.8ns (+938.7%) | [+3979101, +4038386]ns | [4395484, 4451429] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 366064ns | +1099.8% |
| 1 | 2 | 445564ns | +905.2% |
| 1 | 3 | 418352ns | +968.6% |
| 1 | 4 | 429206ns | +918.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.378 | moderate- |
| bitpack-write-guarded | -0.234 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 73.8ns | 418436.7ns | 0.0% |  |
| bitpack-write-guarded | 137.4ns | 4424527.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 371982.9-446341.6 ns)
  371982.9 |
  375700.9 |
  379418.8 |
  383136.7 |
  386854.7 |
  390572.6 |
  394290.5 |
  398008.4 |############
  401726.4 |####
  405444.3 |
  409162.2 |
  412880.2 |
  416598.1 |############################
  420316.0 |####
  424034.0 |########
  427751.9 |########################################
  431469.8 |
  435187.8 |
  438905.7 |
  442623.6 |############################
  (6 below, 3 above range)

bitpack-write-guarded (n=40, range 4346548.1-4502574.4 ns)
  4346548.1 |
  4354349.5 |##########################
  4362150.8 |######
  4369952.1 |
  4377753.4 |####################
  4385554.7 |####################
  4393356.0 |#############
  4401157.3 |######
  4408958.6 |
  4416760.0 |########################################
  4424561.3 |######
  4432362.6 |######
  4440163.9 |######
  4447965.2 |
  4455766.5 |######
  4463567.8 |#############
  4471369.1 |#############
  4479170.5 |####################
  4486971.8 |######
  4494773.1 |######
  (3 below, 4 above range)

```
