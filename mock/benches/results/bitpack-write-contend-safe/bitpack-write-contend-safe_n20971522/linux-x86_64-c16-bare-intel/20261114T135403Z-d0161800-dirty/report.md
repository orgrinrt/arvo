# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (3.93 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 434.04 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 525% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (434.04 us) leads bitpack-write-windowed (2.71 ms) by 525%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 89% (significant)

bitpack-write-dense is -3.50 ms (89%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 9.1x slower than the field

bitpack-write-aligned (3.93 ms) is 9.1x the fastest (434.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-windowed shows alternating (throttle bounce) (autocorr -0.58)

bitpack-write-windowed's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.1x the fastest

Fastest bitpack-write-dense (434.04 us) to slowest bitpack-write-aligned (3.93 ms): 9.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 434043.9 ns median (-89.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.06x (fastest 434043.9 ns, slowest 3932378.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 3946185ns | 3932715ns | 3900917ns | 3934168ns | 4027505ns | base |
| bitpack-write-dense | 432535ns | 434240ns | 421744ns | 432772ns | 442614ns | -89.04% |
| bitpack-write-windowed | 2709245ns | 2711263ns | 2668081ns | 2708883ns | 2751493ns | -31.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 3945897ns | 3900656ns | 4027208ns | base | 0.531 |
| bitpack-write-dense | 432258ns | 421398ns | 442318ns | -89.05% | 4.852 |
| bitpack-write-windowed | 2708955ns | 2667827ns | 2751203ns | -31.35% | 0.774 |

## Performance model

- Peak throughput: **4.977 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.533 | 10.7% |
| bitpack-write-dense | 4.832 | 97.1% |
| bitpack-write-windowed | 0.774 | 15.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 3946185ns | 3946185ns | base |
| bitpack-write-dense | 432535ns | 432535ns | -89.04% |
| bitpack-write-windowed | 2709245ns | 2709245ns | -31.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 3932379ns | base | --- | [3924016, 3939744] | --- | --- | --- | --- |
| bitpack-write-dense | 434044ns | -3499464.1ns (-89.0%) | [-3508700, -3489403]ns | [433184, 435107] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 2710990ns | -1225621.2ns (-31.2%) | [-1259253, -1188845]ns | [2708292, 2714996] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 4012486ns | -89.2% | -32.4% |
| 1 | 2 | 3902562ns | -88.9% | -30.4% |
| 1 | 3 | 3935886ns | -89.3% | -32.2% |
| 1 | 4 | 3931839ns | -88.8% | -30.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.351 | moderate- |
| bitpack-write-dense | -0.503 | HIGH- (thermal bounce) |
| bitpack-write-windowed | -0.576 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 67.1ns | 3945897.1ns | 0.0% |  |
| bitpack-write-dense | 55.6ns | 432257.8ns | 0.0% |  |
| bitpack-write-windowed | 54.0ns | 2708955.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 3900655.7-4027207.8 ns)
  3900655.7 |########################################
  3906983.3 |######
  3913310.9 |######
  3919638.5 |####################
  3925966.1 |####################
  3932293.8 |########################################
  3938621.4 |#############
  3944949.0 |#############
  3951276.6 |
  3957604.2 |######
  3963931.8 |
  3970259.4 |
  3976587.0 |#############
  3982914.6 |######
  3989242.2 |
  3995569.8 |#############
  4001897.4 |
  4008225.0 |
  4014552.6 |
  4020880.2 |######
  (5 below, 4 above range)

bitpack-write-dense (n=40, range 421397.6-442317.5 ns)
  421397.6 |##########################
  422443.6 |
  423489.6 |
  424535.6 |
  425581.6 |
  426627.6 |#############
  427673.6 |
  428719.6 |
  429765.6 |
  430811.6 |
  431857.6 |
  432903.6 |##########################
  433949.6 |######################
  434995.6 |########################################
  436041.6 |
  437087.5 |
  438133.5 |####
  439179.5 |
  440225.5 |
  441271.5 |####
  (4 below, 5 above range)

bitpack-write-windowed (n=40, range 2667827.1-2751203.4 ns)
  2667827.1 |########
  2671995.9 |#############
  2676164.8 |
  2680333.6 |
  2684502.4 |
  2688671.2 |
  2692840.0 |
  2697008.8 |
  2701177.6 |#############
  2705346.4 |########
  2709515.2 |########################################
  2713684.1 |###################################
  2717852.9 |####
  2722021.7 |####
  2726190.5 |
  2730359.3 |
  2734528.1 |####
  2738696.9 |
  2742865.7 |
  2747034.5 |
  (5 below, 5 above range)

```
