# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 is an outlier: 2.2x slower than the field

bitpack-carrier-d64 (558.74 us) is 2.2x the fastest (255.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-carrier-packed, bitpack-carrier-packed-simd, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} vs {bitpack-carrier-d64} (71% apart)

The field splits into a fast tier {bitpack-carrier-packed, bitpack-carrier-packed-simd, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} and a slow tier {bitpack-carrier-d64} with a 71% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-carrier-d32 is inconsistent: worst-20% is 1.7x its best-20%

bitpack-carrier-d32's best 20% of batches run at 245.30 us but its worst 20% at 413.73 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-carrier-packed** at 255776.9 ns median (-19.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.18x (fastest 255776.9 ns, slowest 558739.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112762950 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 324703ns | 319249ns | 319018ns | 319866ns | 344897ns | base |
| bitpack-carrier-d16-control | 325196ns | 327419ns | 319075ns | 325550ns | 330252ns | +0.15% |
| bitpack-carrier-d32 | 311917ns | 296920ns | 245680ns | 299923ns | 414135ns | -3.94% |
| bitpack-carrier-d64 | 542483ns | 559900ns | 483839ns | 539200ns | 610976ns | +67.07% |
| bitpack-carrier-packed | 259221ns | 255922ns | 255508ns | 258700ns | 264497ns | -20.17% |
| bitpack-carrier-packed-simd | 260109ns | 261862ns | 255510ns | 260097ns | 264747ns | -19.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 324520ns | 318863ns | 344670ns | base | 6.462 |
| bitpack-carrier-d16-control | 325026ns | 318916ns | 330086ns | +0.16% | 6.452 |
| bitpack-carrier-d32 | 311609ns | 245302ns | 413726ns | -3.98% | 6.730 |
| bitpack-carrier-d64 | 541532ns | 483335ns | 609884ns | +66.87% | 3.873 |
| bitpack-carrier-packed | 259037ns | 255352ns | 264249ns | -20.18% | 8.096 |
| bitpack-carrier-packed-simd | 259894ns | 255354ns | 264395ns | -19.91% | 8.069 |

## Performance model

- Peak throughput: **8.549 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.572 | 76.9% |
| bitpack-carrier-d16-control | 6.408 | 75.0% |
| bitpack-carrier-d32 | 7.068 | 82.7% |
| bitpack-carrier-d64 | 3.753 | 43.9% |
| bitpack-carrier-packed | 8.199 | 95.9% |
| bitpack-carrier-packed-simd | 8.014 | 93.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 324703ns | 324703ns | base |
| bitpack-carrier-d16-control | 325196ns | 325196ns | +0.15% |
| bitpack-carrier-d32 | 311917ns | 311917ns | -3.94% |
| bitpack-carrier-d64 | 542483ns | 542483ns | +67.07% |
| bitpack-carrier-packed | 259221ns | 259221ns | -20.17% |
| bitpack-carrier-packed-simd | 260109ns | 260109ns | -19.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 319097ns | base | --- | [319000, 319544] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 327265ns | +1848.6ns (+0.6%) | [+111, +7330]ns | [322644, 327451] | YES | 0.0080 | 0.0064 | 0 |
| bitpack-carrier-d32 | 296727ns | no significant difference | [-55184, +9699]ns | [265397, 329342] | no | 1.0000 | 1.0000 | 0 |
| bitpack-carrier-d64 | 558740ns | +233631.5ns (+73.2%) | [+183432, +241074]ns | [504116, 561602] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 255777ns | -63514.9ns (-19.9%) | [-63732, -63045]ns | [255448, 263537] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 261696ns | -63459.5ns (-19.9%) | [-63612, -60240]ns | [255445, 263943] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 321917ns | +1.7% | -23.7% | +73.9% | -18.0% | -18.0% |
| 1 | 2 | 319632ns | +2.4% | -16.9% | +78.3% | -20.1% | -17.4% |
| 1 | 3 | 319327ns | +3.0% | +18.0% | +57.9% | -20.0% | -20.0% |
| 1 | 4 | 319010ns | -0.0% | +3.0% | +51.5% | -17.4% | -19.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.034 | ok |
| bitpack-carrier-d16-control | -0.223 | moderate- |
| bitpack-carrier-d32 | 0.114 | ok |
| bitpack-carrier-d64 | 0.259 | moderate+ |
| bitpack-carrier-packed | -0.250 | moderate- |
| bitpack-carrier-packed-simd | 0.254 | moderate+ |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 0/4, lost 3/4
- **bitpack-carrier-d32**: won 2/4, lost 2/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 40.9ns | 324520.4ns | 0.0% |  |
| bitpack-carrier-d16-control | 44.6ns | 325025.9ns | 0.0% |  |
| bitpack-carrier-d32 | 103.5ns | 311608.7ns | 0.0% |  |
| bitpack-carrier-d64 | 285.9ns | 541532.3ns | 0.1% |  |
| bitpack-carrier-packed | 38.5ns | 259037.4ns | 0.0% |  |
| bitpack-carrier-packed-simd | 42.9ns | 259893.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 318862.8-344670.3 ns)
  318862.8 |########################################
  320153.1 |#
  321443.5 |
  322733.9 |
  324024.3 |####
  325314.6 |
  326605.0 |#########
  327895.4 |
  329185.8 |
  330476.2 |
  331766.5 |
  333056.9 |
  334347.3 |
  335637.7 |
  336928.0 |
  338218.4 |
  339508.8 |
  340799.2 |
  342089.6 |
  343379.9 |
  (3 below, 1 above range)

bitpack-carrier-d16-control (n=40, range 318916.0-330086.1 ns)
  318916.0 |##########################
  319474.5 |########
  320033.0 |####
  320591.5 |
  321150.0 |
  321708.5 |
  322267.0 |
  322825.6 |
  323384.1 |
  323942.6 |
  324501.1 |
  325059.6 |####
  325618.1 |
  326176.6 |
  326735.1 |########################################
  327293.6 |###############################
  327852.1 |####
  328410.6 |####
  328969.1 |########
  329527.6 |
  (5 below, 5 above range)

bitpack-carrier-d32 (n=40, range 245302.0-413725.8 ns)
  245302.0 |############################
  253723.2 |
  262144.4 |########################################
  270565.6 |
  278986.7 |
  287407.9 |
  295829.1 |
  304250.3 |
  312671.5 |
  321092.7 |############################
  329513.9 |########
  337935.1 |####
  346356.3 |
  354777.4 |
  363198.6 |####
  371619.8 |####################
  380041.0 |
  388462.2 |####
  396883.4 |
  405304.6 |
  (3 below, 3 above range)

bitpack-carrier-d64 (n=40, range 483335.0-609884.4 ns)
  483335.0 |#############
  489662.5 |
  495990.0 |#################
  502317.5 |#################
  508644.9 |########
  514972.4 |
  521299.9 |
  527627.3 |
  533954.8 |
  540282.3 |
  546609.7 |
  552937.2 |#############
  559264.7 |##########################
  565592.2 |########################################
  571919.6 |
  578247.1 |
  584574.6 |
  590902.0 |
  597229.5 |
  603557.0 |
  (5 below, 4 above range)

bitpack-carrier-packed (n=40, range 255352.3-264248.8 ns)
  255352.3 |########################################
  255797.1 |##
  256242.0 |##
  256686.8 |
  257131.6 |##
  257576.4 |
  258021.3 |
  258466.1 |
  258910.9 |
  259355.7 |
  259800.5 |
  260245.4 |##
  260690.2 |
  261135.0 |
  261579.8 |
  262024.6 |
  262469.5 |
  262914.3 |
  263359.1 |##############
  263803.9 |################
  (3 below, 3 above range)

bitpack-carrier-packed-simd (n=40, range 255354.3-264395.0 ns)
  255354.3 |########################################
  255806.3 |##
  256258.3 |
  256710.4 |
  257162.4 |
  257614.5 |
  258066.5 |
  258518.5 |
  258970.6 |
  259422.6 |##
  259874.6 |
  260326.7 |
  260778.7 |
  261230.8 |
  261682.8 |
  262134.8 |
  262586.9 |
  263038.9 |
  263490.9 |#################
  263943.0 |##################################
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-carrier-d32**: CV=21.2% (high variance, measurements may be unstable)
