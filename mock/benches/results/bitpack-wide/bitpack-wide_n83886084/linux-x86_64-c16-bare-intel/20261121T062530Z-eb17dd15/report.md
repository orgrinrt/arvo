# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (603.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-pipe4 at 451.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-pipe4 dominates: 26% faster than the next best (bitpack-wide-d16-control)

bitpack-wide-pipe4 (451.40 us) leads bitpack-wide-d16-control (570.89 us) by 26%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 beats baseline by 26% (significant)

bitpack-wide-pipe4 is -154.22 us (26%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {bitpack-wide-pipe4} vs {bitpack-wide-d16-control, bitpack-wide-d16-padal, bitpack-wide-d16} (26% apart)

The field splits into a fast tier {bitpack-wide-pipe4} and a slow tier {bitpack-wide-d16-control, bitpack-wide-d16-padal, bitpack-wide-d16} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 451397.2 ns median (-25.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.34x (fastest 451397.2 ns, slowest 603302.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112815569 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 598282ns | 604008ns | 539903ns | 605448ns | 635164ns | base |
| bitpack-wide-d16-control | 576763ns | 571578ns | 529351ns | 573388ns | 634299ns | -3.60% |
| bitpack-wide-d16-padal | 576644ns | 585604ns | 535682ns | 573939ns | 625722ns | -3.62% |
| bitpack-wide-pipe4 | 455228ns | 451955ns | 426756ns | 451562ns | 494699ns | -23.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 597453ns | 539026ns | 634305ns | base | 14.041 |
| bitpack-wide-d16-control | 576039ns | 528669ns | 633464ns | -3.58% | 14.563 |
| bitpack-wide-d16-padal | 575882ns | 534965ns | 624870ns | -3.61% | 14.567 |
| bitpack-wide-pipe4 | 454644ns | 426289ns | 493981ns | -23.90% | 18.451 |

## Performance model

- Peak throughput: **19.678 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 13.904 | 70.7% |
| bitpack-wide-d16-control | 14.694 | 74.7% |
| bitpack-wide-d16-padal | 14.340 | 72.9% |
| bitpack-wide-pipe4 | 18.584 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 598282ns | 598282ns | base |
| bitpack-wide-d16-control | 576763ns | 576763ns | -3.60% |
| bitpack-wide-d16-padal | 576644ns | 576644ns | -3.62% |
| bitpack-wide-pipe4 | 455228ns | 455228ns | -23.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 603302ns | base | --- | [601628, 605880] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 570894ns | -3660.9ns (-0.6%) | [-57374, -1964]ns | [535363, 613021] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-d16-padal | 584994ns | -20323.5ns (-3.4%) | [-34207, -12720]ns | [546540, 595360] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-pipe4 | 451397ns | -154221.1ns (-25.6%) | [-175115, -138254]ns | [442193, 461878] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 537106ns | +3.6% | +10.7% | -14.0% |
| 1 | 2 | 618268ns | -0.6% | -3.3% | -28.6% |
| 1 | 3 | 604828ns | -12.6% | -3.3% | -25.4% |
| 1 | 4 | 602729ns | -10.7% | -11.1% | -29.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.231 | moderate- |
| bitpack-wide-d16-control | -0.270 | moderate- |
| bitpack-wide-d16-padal | 0.070 | ok |
| bitpack-wide-pipe4 | -0.292 | moderate- |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 3/4, lost 1/4
- **bitpack-wide-d16-padal**: won 3/4, lost 1/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 104.1ns | 597452.5ns | 0.0% |  |
| bitpack-wide-d16-control | 31.7ns | 576039.4ns | 0.0% |  |
| bitpack-wide-d16-padal | 48.1ns | 575882.2ns | 0.0% |  |
| bitpack-wide-pipe4 | 57.6ns | 454644.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 539026.3-634304.9 ns)
  539026.3 |
  543790.2 |
  548554.2 |###
  553318.1 |
  558082.0 |
  562845.9 |
  567609.9 |
  572373.8 |
  577137.7 |
  581901.7 |
  586665.6 |
  591429.5 |############
  596193.5 |###
  600957.4 |########################################
  605721.3 |###
  610485.3 |
  615249.2 |##############################
  620013.1 |
  624777.1 |
  629541.0 |
  (7 below, 3 above range)

bitpack-wide-d16-control (n=40, range 528668.8-633463.8 ns)
  528668.8 |##################
  533908.6 |#############################
  539148.3 |###
  544388.1 |
  549627.8 |###
  554867.6 |
  560107.3 |###
  565347.1 |
  570586.8 |
  575826.6 |###
  581066.3 |
  586306.1 |###
  591545.8 |
  596785.5 |###
  602025.3 |###
  607265.0 |###
  612504.8 |########################################
  617744.5 |###
  622984.3 |
  628224.0 |
  (4 below, 3 above range)

bitpack-wide-d16-padal (n=40, range 534965.3-624870.2 ns)
  534965.3 |########################################
  539460.6 |
  543955.8 |
  548451.1 |
  552946.3 |###
  557441.6 |#######
  561936.8 |
  566432.1 |
  570927.3 |###
  575422.5 |
  579917.8 |
  584413.0 |##################
  588908.3 |
  593403.5 |#############################
  597898.8 |##########
  602394.0 |###
  606889.3 |
  611384.5 |#######
  615879.8 |
  620375.0 |
  (3 below, 3 above range)

bitpack-wide-pipe4 (n=40, range 426289.5-493981.5 ns)
  426289.5 |#########################
  429674.1 |#########################
  433058.7 |#####
  436443.3 |
  439827.9 |
  443212.5 |
  446597.1 |
  449981.7 |########################################
  453366.3 |
  456750.9 |
  460135.5 |########################################
  463520.1 |##########
  466904.7 |
  470289.3 |
  473673.9 |##########
  477058.5 |
  480443.1 |
  483827.7 |#####
  487212.3 |#####
  490596.9 |
  (3 below, 4 above range)

```
