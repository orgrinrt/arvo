# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (3.94 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 425.31 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 537% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (425.31 us) leads bitpack-write-windowed (2.71 ms) by 537%, a clear separation rather than a photo finish. CV 26.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 89% (significant)

bitpack-write-dense is -3.52 ms (89%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 9.3x slower than the field

bitpack-write-aligned (3.94 ms) is 9.3x the fastest (425.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 26.0%)

bitpack-write-dense wins on median (425.31 us) yet has the highest variance (CV 26.0%), while bitpack-write-aligned is the steadiest (CV 3.1%, 3.94 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 9.3x the fastest

Fastest bitpack-write-dense (425.31 us) to slowest bitpack-write-aligned (3.94 ms): 9.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 1.5x its best-20%

bitpack-write-dense's best 20% of batches run at 351.65 us but its worst 20% at 543.06 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 425311.2 ns median (-89.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.26x (fastest 425311.2 ns, slowest 3938887.3 ns)

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
| bitpack-write-aligned | 3968486ns | 3939267ns | 3879153ns | 3933261ns | 4163496ns | base |
| bitpack-write-dense | 433566ns | 425650ns | 351927ns | 424132ns | 543507ns | -89.07% |
| bitpack-write-windowed | 2739979ns | 2710178ns | 2669589ns | 2714783ns | 2885957ns | -30.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 3968129ns | 3878808ns | 4163127ns | base | 0.528 |
| bitpack-write-dense | 433213ns | 351653ns | 543064ns | -89.08% | 4.841 |
| bitpack-write-windowed | 2739703ns | 2669345ns | 2885662ns | -30.96% | 0.765 |

## Performance model

- Peak throughput: **5.964 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.532 | 8.9% |
| bitpack-write-dense | 4.931 | 82.7% |
| bitpack-write-windowed | 0.774 | 13.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 3968486ns | 3968486ns | base |
| bitpack-write-dense | 433566ns | 433566ns | -89.07% |
| bitpack-write-windowed | 2739979ns | 2739979ns | -30.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 3938887ns | base | --- | [3916290, 3948074] | --- | --- | --- | --- |
| bitpack-write-dense | 425311ns | -3524884.9ns (-89.5%) | [-3528555, -3508568]ns | [423626, 434700] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 2709893ns | -1215332.6ns (-30.9%) | [-1233314, -1207764]ns | [2674683, 2741450] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 3879736ns | -90.9% | -31.1% |
| 1 | 2 | 3924329ns | -88.9% | -32.0% |
| 1 | 3 | 3961578ns | -89.3% | -28.6% |
| 1 | 4 | 3952161ns | -89.2% | -30.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.203 | moderate+ |
| bitpack-write-dense | -0.185 | ok |
| bitpack-write-windowed | -0.075 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 60.4ns | 3968129.5ns | 0.0% |  |
| bitpack-write-dense | 63.1ns | 433212.7ns | 0.0% |  |
| bitpack-write-windowed | 64.9ns | 2739702.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 3878807.7-4163126.7 ns)
  3878807.7 |########################
  3893023.7 |############
  3907239.6 |####
  3921455.6 |############
  3935671.5 |########################################
  3949887.5 |############
  3964103.4 |############
  3978319.4 |####
  3992535.3 |
  4006751.3 |
  4020967.2 |
  4035183.2 |
  4049399.1 |
  4063615.1 |
  4077831.0 |####
  4092047.0 |####
  4106262.9 |
  4120478.9 |
  4134694.8 |
  4148910.8 |
  (4 below, 4 above range)

bitpack-write-dense (n=40, range 351652.8-543064.0 ns)
  351652.8 |##################
  361223.4 |
  370794.0 |
  380364.5 |
  389935.1 |
  399505.6 |
  409076.2 |
  418646.7 |########################################
  428217.3 |#############################
  437787.9 |#############################
  447358.4 |
  456929.0 |
  466499.5 |
  476070.1 |###
  485640.6 |
  495211.2 |
  504781.8 |
  514352.3 |
  523922.9 |
  533493.4 |
  (5 below, 2 above range)

bitpack-write-windowed (n=40, range 2669345.0-2885662.1 ns)
  2669345.0 |########################################
  2680160.9 |######
  2690976.8 |
  2701792.6 |###
  2712608.5 |######
  2723424.3 |######
  2734240.2 |#########
  2745056.0 |#########
  2755871.9 |
  2766687.7 |
  2777503.6 |
  2788319.4 |
  2799135.3 |
  2809951.1 |###
  2820767.0 |######
  2831582.8 |#########
  2842398.7 |###
  2853214.5 |
  2864030.4 |###
  2874846.2 |
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=25.5% (high variance, measurements may be unstable)
