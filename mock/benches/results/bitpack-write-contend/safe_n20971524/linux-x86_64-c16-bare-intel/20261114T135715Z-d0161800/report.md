# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (3.37 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 406.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 542% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (406.19 us) leads bitpack-write-windowed (2.61 ms) by 542%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 88% (significant)

bitpack-write-dense is -2.96 ms (88%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 8.3x slower than the field

bitpack-write-aligned (3.37 ms) is 8.3x the fastest (406.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 8.7%)

bitpack-write-dense wins on median (406.19 us) yet has the highest variance (CV 8.7%), while bitpack-write-windowed is the steadiest (CV 0.2%, 2.61 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.3x the fastest

Fastest bitpack-write-dense (406.19 us) to slowest bitpack-write-aligned (3.37 ms): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 406194.0 ns median (-87.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.29x (fastest 406194.0 ns, slowest 3367328.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 3374433ns | 3367633ns | 3366067ns | 3368837ns | 3399584ns | base |
| bitpack-write-dense | 416569ns | 406409ns | 379653ns | 408969ns | 476282ns | -87.66% |
| bitpack-write-windowed | 2610609ns | 2609618ns | 2604601ns | 2609147ns | 2621003ns | -22.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 3374120ns | 3365763ns | 3399240ns | base | 0.622 |
| bitpack-write-dense | 416241ns | 379408ns | 475719ns | -87.66% | 5.038 |
| bitpack-write-windowed | 2610235ns | 2604226ns | 2620678ns | -22.64% | 0.803 |

## Performance model

- Peak throughput: **5.527 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.623 | 11.3% |
| bitpack-write-dense | 5.163 | 93.4% |
| bitpack-write-windowed | 0.804 | 14.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 3374433ns | 3374433ns | base |
| bitpack-write-dense | 416569ns | 416569ns | -87.66% |
| bitpack-write-windowed | 2610609ns | 2610609ns | -22.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 3367328ns | base | --- | [3366639, 3368062] | --- | --- | --- | --- |
| bitpack-write-dense | 406194ns | -2961545.8ns (-87.9%) | [-2980460, -2959949]ns | [405760, 406976] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 2609312ns | -760496.3ns (-22.6%) | [-762401, -758411]ns | [2606386, 2610456] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 3367272ns | -88.7% | -22.4% |
| 1 | 2 | 3374295ns | -87.9% | -22.7% |
| 1 | 3 | 3366678ns | -87.9% | -22.6% |
| 1 | 4 | 3366857ns | -86.2% | -22.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.389 | moderate- |
| bitpack-write-dense | -0.027 | ok |
| bitpack-write-windowed | 0.298 | moderate+ |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 63.9ns | 3374119.6ns | 0.0% |  |
| bitpack-write-dense | 83.4ns | 416241.5ns | 0.0% |  |
| bitpack-write-windowed | 53.2ns | 2610235.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 3365763.1-3399240.4 ns)
  3365763.1 |########################################
  3367437.0 |##########
  3369110.9 |
  3370784.7 |####
  3372458.6 |####
  3374132.4 |
  3375806.3 |
  3377480.2 |
  3379154.0 |##
  3380827.9 |######
  3382501.8 |
  3384175.6 |
  3385849.5 |
  3387523.3 |
  3389197.2 |
  3390871.1 |
  3392544.9 |##
  3394218.8 |
  3395892.7 |
  3397566.5 |
  (3 below, 4 above range)

bitpack-write-dense (n=40, range 379408.5-475718.6 ns)
  379408.5 |#########
  384224.0 |##
  389039.5 |
  393855.0 |##
  398670.5 |##
  403486.0 |########################################
  408301.5 |##
  413117.0 |
  417932.5 |
  422748.0 |
  427563.5 |
  432379.0 |
  437194.6 |
  442010.1 |
  446825.6 |
  451641.1 |
  456456.6 |
  461272.1 |################
  466087.6 |
  470903.1 |
  (5 below, 3 above range)

bitpack-write-windowed (n=40, range 2604226.3-2620678.2 ns)
  2604226.3 |#################
  2605048.9 |############################
  2605871.5 |############################
  2606694.1 |
  2607516.7 |
  2608339.3 |###########
  2609161.9 |########################################
  2609984.5 |
  2610807.1 |###########
  2611629.7 |######################
  2612452.3 |
  2613274.9 |
  2614097.5 |
  2614920.1 |#####
  2615742.7 |
  2616565.3 |###########
  2617387.9 |#####
  2618210.5 |
  2619033.1 |###########
  2619855.7 |
  (4 below, 2 above range)

```
