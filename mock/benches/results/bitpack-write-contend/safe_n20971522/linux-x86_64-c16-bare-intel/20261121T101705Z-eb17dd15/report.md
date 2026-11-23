# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (3.94 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 436.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 528% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (436.25 us) leads bitpack-write-windowed (2.74 ms) by 528%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 89% (significant)

bitpack-write-dense is -3.50 ms (89%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 9.0x slower than the field

bitpack-write-aligned (3.94 ms) is 9.0x the fastest (436.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-aligned shows alternating (throttle bounce) (autocorr -0.66)

bitpack-write-aligned's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.0x the fastest

Fastest bitpack-write-dense (436.25 us) to slowest bitpack-write-aligned (3.94 ms): 9.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 436249.2 ns median (-88.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.04x (fastest 436249.2 ns, slowest 3942320.8 ns)

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
| bitpack-write-aligned | 3960186ns | 3942666ns | 3868095ns | 3957681ns | 4059791ns | base |
| bitpack-write-dense | 440033ns | 436460ns | 423882ns | 437645ns | 463346ns | -88.89% |
| bitpack-write-windowed | 2752049ns | 2738472ns | 2706940ns | 2748902ns | 2806597ns | -30.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 3959818ns | 3867670ns | 4059459ns | base | 0.530 |
| bitpack-write-dense | 439754ns | 423612ns | 462973ns | -88.89% | 4.769 |
| bitpack-write-windowed | 2751593ns | 2706569ns | 2806137ns | -30.51% | 0.762 |

## Performance model

- Peak throughput: **4.951 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.532 | 10.7% |
| bitpack-write-dense | 4.807 | 97.1% |
| bitpack-write-windowed | 0.766 | 15.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 3960186ns | 3960186ns | base |
| bitpack-write-dense | 440033ns | 440033ns | -88.89% |
| bitpack-write-windowed | 2752049ns | 2752049ns | -30.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 3942321ns | base | --- | [3914950, 4006431] | --- | --- | --- | --- |
| bitpack-write-dense | 436249ns | -3499184.8ns (-88.8%) | [-3554733, -3490338]ns | [434180, 437584] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 2737998ns | -1173438.7ns (-29.8%) | [-1243521, -1156991]ns | [2731666, 2772860] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 3868957ns | -88.7% | -30.0% |
| 1 | 2 | 4038637ns | -89.4% | -32.2% |
| 1 | 3 | 3920716ns | -89.2% | -28.9% |
| 1 | 4 | 4011991ns | -88.6% | -31.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.657 | HIGH- (thermal bounce) |
| bitpack-write-dense | -0.187 | ok |
| bitpack-write-windowed | -0.082 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 60.2ns | 3959818.4ns | 0.0% |  |
| bitpack-write-dense | 66.0ns | 439754.0ns | 0.0% |  |
| bitpack-write-windowed | 76.0ns | 2751593.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 3867670.1-4059458.6 ns)
  3867670.1 |########################################
  3877259.6 |########
  3886849.0 |########
  3896438.4 |
  3906027.9 |################################
  3915617.3 |################
  3925206.7 |################################
  3934796.1 |
  3944385.5 |########
  3953975.0 |
  3963564.4 |################
  3973153.8 |
  3982743.2 |
  3992332.7 |################
  4001922.1 |################
  4011511.5 |########################
  4021100.9 |################
  4030690.4 |################
  4040279.8 |################
  4049869.2 |
  (3 below, 4 above range)

bitpack-write-dense (n=40, range 423612.2-462972.6 ns)
  423612.2 |#################
  425580.2 |#################
  427548.2 |
  429516.2 |
  431484.3 |####
  433452.3 |#################
  435420.3 |########################################
  437388.3 |####
  439356.4 |####
  441324.4 |
  443292.4 |####
  445260.4 |
  447228.4 |
  449196.5 |
  451164.5 |
  453132.5 |
  455100.5 |
  457068.6 |###############################
  459036.6 |####
  461004.6 |
  (4 below, 3 above range)

bitpack-write-windowed (n=40, range 2706568.5-2806136.5 ns)
  2706568.5 |############################
  2711546.9 |#####
  2716525.3 |#####
  2721503.7 |#####
  2726482.1 |#################
  2731460.5 |############################
  2736438.9 |###########
  2741417.3 |#####
  2746395.7 |###########
  2751374.1 |#####
  2756352.5 |
  2761330.9 |#####
  2766309.3 |
  2771287.7 |
  2776266.1 |
  2781244.5 |#################
  2786222.9 |########################################
  2791201.3 |
  2796179.7 |
  2801158.1 |#####
  (3 below, 3 above range)

```
