# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (4.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 467.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 460% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (467.68 us) leads bitpack-write-windowed (2.62 ms) by 460%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 89% (significant)

bitpack-write-dense is -3.75 ms (89%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 9.0x slower than the field

bitpack-write-aligned (4.21 ms) is 9.0x the fastest (467.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-aligned shows alternating (throttle bounce) (autocorr -0.78)

bitpack-write-aligned's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.0x the fastest

Fastest bitpack-write-dense (467.68 us) to slowest bitpack-write-aligned (4.21 ms): 9.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-aligned is inconsistent: worst-20% is 2.3x its best-20%

bitpack-write-aligned's best 20% of batches run at 4.07 ms but its worst 20% at 9.43 ms (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 467679.6 ns median (-88.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.01x (fastest 467679.6 ns, slowest 4213103.5 ns)

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
| Counter frequency | 2112697486 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 5249795ns | 4213491ns | 4071022ns | 4250142ns | 9427527ns | base |
| bitpack-write-dense | 476224ns | 468074ns | 460298ns | 473792ns | 499445ns | -90.93% |
| bitpack-write-windowed | 2865514ns | 2618820ns | 2606066ns | 2744789ns | 3487135ns | -45.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 5249391ns | 4070623ns | 9427062ns | base | 0.400 |
| bitpack-write-dense | 475783ns | 459774ns | 499000ns | -90.94% | 4.408 |
| bitpack-write-windowed | 2865072ns | 2605673ns | 3486559ns | -45.42% | 0.732 |

## Performance model

- Peak throughput: **4.561 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.498 | 10.9% |
| bitpack-write-dense | 4.484 | 98.3% |
| bitpack-write-windowed | 0.801 | 17.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 5249795ns | 5249795ns | base |
| bitpack-write-dense | 476224ns | 476224ns | -90.93% |
| bitpack-write-windowed | 2865514ns | 2865514ns | -45.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 4213104ns | base | --- | [4191257, 4317887] | --- | --- | --- | --- |
| bitpack-write-dense | 467680ns | -3753334.8ns (-89.1%) | [-3848967, -3723483]ns | [465324, 484647] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 2618336ns | -1577756.4ns (-37.4%) | [-1594874, -1459215]ns | [2606835, 2825094] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 4231951ns | -88.5% | -22.4% |
| 1 | 2 | 4259295ns | -89.0% | -38.8% |
| 1 | 3 | 4213104ns | -89.1% | -34.7% |
| 1 | 4 | 4257312ns | -88.6% | -38.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.779 | HIGH- (thermal bounce) |
| bitpack-write-dense | -0.267 | moderate- |
| bitpack-write-windowed | -0.233 | moderate- |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 65.4ns | 5249391.3ns | 0.0% |  |
| bitpack-write-dense | 127.4ns | 475783.4ns | 0.0% |  |
| bitpack-write-windowed | 73.9ns | 2865072.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 4070623.3-9427062.0 ns)
  4070623.3 |########################################
  4338445.2 |###
  4606267.1 |
  4874089.1 |#
  5141911.0 |
  5409733.0 |#
  5677554.9 |
  5945376.8 |
  6213198.8 |
  6481020.7 |
  6748842.7 |
  7016664.6 |
  7284486.5 |
  7552308.5 |
  7820130.4 |
  8087952.3 |
  8355774.3 |
  8623596.2 |
  8891418.2 |
  9159240.1 |
  (5 below, 5 above range)

bitpack-write-dense (n=40, range 459774.3-498999.5 ns)
  459774.3 |################################
  461735.6 |
  463696.8 |################
  465658.1 |########################
  467619.3 |####
  469580.6 |####
  471541.9 |
  473503.1 |
  475464.4 |
  477425.6 |
  479386.9 |
  481348.2 |####
  483309.4 |########################################
  485270.7 |####
  487231.9 |
  489193.2 |
  491154.5 |
  493115.7 |####
  495077.0 |
  497038.2 |####
  (2 below, 4 above range)

bitpack-write-windowed (n=40, range 2605673.2-3486558.8 ns)
  2605673.2 |########################################
  2649717.5 |##
  2693761.8 |
  2737806.0 |########
  2781850.3 |
  2825894.6 |
  2869938.9 |##
  2913983.2 |##
  2958027.4 |
  3002071.7 |##
  3046116.0 |
  3090160.3 |##
  3134204.6 |####
  3178248.8 |
  3222293.1 |##
  3266337.4 |####
  3310381.7 |##
  3354425.9 |
  3398470.2 |
  3442514.5 |
  (3 below, 4 above range)

```

## Diagnostics

- **bitpack-write-aligned**: CV=50.2% (high variance, measurements may be unstable)
