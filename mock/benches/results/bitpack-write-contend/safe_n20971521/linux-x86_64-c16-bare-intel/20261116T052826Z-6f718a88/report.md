# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (6.28 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 428.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 800% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (428.25 us) leads bitpack-write-windowed (3.85 ms) by 800%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 93% (significant)

bitpack-write-dense is -5.86 ms (93%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 14.7x slower than the field

bitpack-write-aligned (6.28 ms) is 14.7x the fastest (428.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 8.7%)

bitpack-write-dense wins on median (428.25 us) yet has the highest variance (CV 8.7%), while bitpack-write-windowed is the steadiest (CV 1.0%, 3.85 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.65)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 14.7x the fastest

Fastest bitpack-write-dense (428.25 us) to slowest bitpack-write-aligned (6.28 ms): 14.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 428249.7 ns median (-93.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 14.67x (fastest 428249.7 ns, slowest 6282823.0 ns)

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
| bitpack-write-aligned | 6344310ns | 6283119ns | 6260980ns | 6288989ns | 6593605ns | base |
| bitpack-write-dense | 439126ns | 428572ns | 416057ns | 431233ns | 485872ns | -93.08% |
| bitpack-write-windowed | 3866083ns | 3852608ns | 3845713ns | 3853140ns | 3925279ns | -39.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 6343960ns | 6260555ns | 6593247ns | base | 0.331 |
| bitpack-write-dense | 438775ns | 415813ns | 485371ns | -93.08% | 4.780 |
| bitpack-write-windowed | 3865754ns | 3845399ns | 3925000ns | -39.06% | 0.542 |

## Performance model

- Peak throughput: **5.044 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.334 | 6.6% |
| bitpack-write-dense | 4.897 | 97.1% |
| bitpack-write-windowed | 0.544 | 10.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 6344310ns | 6344310ns | base |
| bitpack-write-dense | 439126ns | 439126ns | -93.08% |
| bitpack-write-windowed | 3866083ns | 3866083ns | -39.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6282823ns | base | --- | [6273316, 6292341] | --- | --- | --- | --- |
| bitpack-write-dense | 428250ns | -5863712.3ns (-93.3%) | [-5872759, -5834318]ns | [419632, 431943] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 3852220ns | -2428134.1ns (-38.6%) | [-2438587, -2418542]ns | [3850492, 3854517] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 6288306ns | -93.3% | -38.7% |
| 1 | 2 | 6264273ns | -92.6% | -38.5% |
| 1 | 3 | 6289168ns | -93.4% | -38.8% |
| 1 | 4 | 6341233ns | -93.2% | -39.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.045 | ok |
| bitpack-write-dense | -0.653 | HIGH- (thermal bounce) |
| bitpack-write-windowed | -0.200 | moderate- |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 76.5ns | 6343959.7ns | 0.0% |  |
| bitpack-write-dense | 72.3ns | 438774.5ns | 0.0% |  |
| bitpack-write-windowed | 74.1ns | 3865753.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 6260554.8-6593246.7 ns)
  6260554.8 |########################################
  6277189.4 |###########################
  6293824.0 |############
  6310458.6 |###
  6327093.2 |
  6343727.8 |
  6360362.4 |
  6376997.0 |
  6393631.5 |
  6410266.1 |#########
  6426900.7 |###
  6443535.3 |
  6460169.9 |
  6476804.5 |
  6493439.1 |
  6510073.7 |
  6526708.3 |
  6543342.9 |
  6559977.5 |###
  6576612.1 |
  (4 below, 4 above range)

bitpack-write-dense (n=40, range 415812.7-485371.5 ns)
  415812.7 |########################################
  419290.6 |#############
  422768.5 |##########
  426246.5 |
  429724.4 |#######################
  433202.4 |
  436680.3 |###
  440158.3 |
  443636.2 |
  447114.1 |
  450592.1 |###
  454070.0 |###
  457548.0 |######
  461025.9 |####################
  464503.8 |
  467981.8 |
  471459.7 |###
  474937.7 |
  478415.6 |
  481893.5 |
  (1 below, 1 above range)

bitpack-write-windowed (n=40, range 3845399.4-3924999.7 ns)
  3845399.4 |##############################
  3849379.4 |########################################
  3853359.4 |#######################
  3857339.4 |######
  3861319.4 |######
  3865299.5 |###
  3869279.5 |
  3873259.5 |
  3877239.5 |
  3881219.5 |
  3885199.6 |###
  3889179.6 |
  3893159.6 |
  3897139.6 |
  3901119.6 |
  3905099.6 |
  3909079.7 |
  3913059.7 |
  3917039.7 |
  3921019.7 |
  (2 below, 4 above range)

```
