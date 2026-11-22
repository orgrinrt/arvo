# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 796% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (20.85 us) leads bitpack-write-guarded (186.75 us) by 796%, a clear separation rather than a photo finish. CV 74.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 74.6%)

bitpack-write-dense wins on median (20.85 us) yet has the highest variance (CV 74.6%), while bitpack-write-guarded is the steadiest (CV 9.7%, 186.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-guarded shows alternating (throttle bounce) (autocorr -0.61)

bitpack-write-guarded's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (20.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.0x the fastest

Fastest bitpack-write-dense (20.85 us) to slowest bitpack-write-guarded (186.75 us): 9.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 1.8x its best-20%

bitpack-write-dense's best 20% of batches run at 20.39 us but its worst 20% at 37.25 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 20851.0 ns median
- 1 variant significantly slower than baseline
- Spread: 8.96x (fastest 20851.0 ns, slowest 186750.7 ns)

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
| bitpack-write-dense | 24326ns | 20945ns | 20470ns | 21253ns | 37401ns | base |
| bitpack-write-guarded | 181377ns | 186861ns | 152685ns | 185372ns | 198083ns | +645.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 24228ns | 20385ns | 37246ns | base | 2.705 |
| bitpack-write-guarded | 181277ns | 152601ns | 197972ns | +648.21% | 0.362 |

## Performance model

- Peak throughput: **3.215 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 3.143 | 97.8% |
| bitpack-write-guarded | 0.351 | 10.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 24326ns | 24326ns | base |
| bitpack-write-guarded | 181377ns | 181377ns | +645.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 20851ns | base | --- | [20800, 20982] | --- | --- | --- | --- |
| bitpack-write-guarded | 186751ns | +165913.4ns (+795.7%) | [+161459, +176464]ns | [182132, 197311] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 20819ns | +796.8% |
| 1 | 2 | 20444ns | +867.6% |
| 1 | 3 | 25316ns | +503.5% |
| 1 | 4 | 20949ns | +842.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.417 | moderate- |
| bitpack-write-guarded | -0.605 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 25.1ns | 24228.0ns | 0.1% |  |
| bitpack-write-guarded | 22.4ns | 181276.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 20385.0-37245.7 ns)
  20385.0 |########################################
  21228.1 |
  22071.1 |
  22914.1 |
  23757.2 |#
  24600.2 |###########
  25443.2 |
  26286.3 |
  27129.3 |
  27972.3 |
  28815.4 |
  29658.4 |
  30501.5 |
  31344.5 |
  32187.5 |
  33030.6 |
  33873.6 |
  34716.6 |
  35559.7 |
  36402.7 |
  (3 below, 1 above range)

bitpack-write-guarded (n=40, range 152601.4-197972.1 ns)
  152601.4 |####################
  154870.0 |
  157138.5 |
  159407.1 |
  161675.6 |###
  163944.1 |
  166212.7 |
  168481.2 |
  170749.7 |
  173018.3 |
  175286.8 |###
  177555.3 |######
  179823.9 |
  182092.4 |
  184360.9 |#############
  186629.5 |####################
  188898.0 |
  191166.5 |
  193435.1 |
  195703.6 |########################################
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=64.2% (high variance, measurements may be unstable)
