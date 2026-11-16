# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (186.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-windowed at 124.91 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-windowed beats baseline by 33% (significant)

bitpack-mac-windowed is -60.51 us (33%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {bitpack-mac-windowed, bitpack-mac-narrow, bitpack-mac-simd, bitpack-mac-native} vs {bitpack-mac-naive} (35% apart)

The field splits into a fast tier {bitpack-mac-windowed, bitpack-mac-narrow, bitpack-mac-simd, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-windowed** at 124913.4 ns median (-32.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.49x (fastest 124913.4 ns, slowest 186106.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked decode against per-element consumer work |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112768841 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 188126ns | 186189ns | 185092ns | 188002ns | 191535ns | base |
| bitpack-mac-narrow | 128183ns | 129220ns | 124975ns | 128864ns | 129350ns | -31.86% |
| bitpack-mac-native | 138235ns | 137761ns | 137608ns | 138009ns | 139537ns | -26.52% |
| bitpack-mac-simd | 127909ns | 129294ns | 124977ns | 128220ns | 129906ns | -32.01% |
| bitpack-mac-windowed | 127177ns | 125000ns | 124954ns | 126462ns | 131545ns | -32.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 188043ns | 185013ns | 191445ns | base | 1.394 |
| bitpack-mac-narrow | 128097ns | 124892ns | 129258ns | -31.88% | 2.046 |
| bitpack-mac-native | 138147ns | 137517ns | 139450ns | -26.53% | 1.898 |
| bitpack-mac-simd | 127824ns | 124897ns | 129817ns | -32.02% | 2.051 |
| bitpack-mac-windowed | 127091ns | 124872ns | 131454ns | -32.41% | 2.063 |

## Performance model

- Peak throughput: **2.099 Gops/s** (bitpack-mac-windowed; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.409 | 67.1% |
| bitpack-mac-narrow | 2.030 | 96.7% |
| bitpack-mac-native | 1.904 | 90.7% |
| bitpack-mac-simd | 2.029 | 96.6% |
| bitpack-mac-windowed | 2.099 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 188126ns | 188126ns | base |
| bitpack-mac-narrow | 128183ns | 128183ns | -31.86% |
| bitpack-mac-native | 138235ns | 138235ns | -26.52% |
| bitpack-mac-simd | 127909ns | 127909ns | -32.01% |
| bitpack-mac-windowed | 127177ns | 127177ns | -32.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 186106ns | base | --- | [185203, 191342] | --- | --- | --- | --- |
| bitpack-mac-narrow | 129129ns | -60356.7ns (-32.4%) | [-62198, -60047]ns | [129070, 129181] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 137676ns | -47875.0ns (-25.7%) | [-53691, -47459]ns | [137609, 137856] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 129214ns | -60432.3ns (-32.5%) | [-62120, -57630]ns | [126969, 129249] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 124913ns | -60509.6ns (-32.5%) | [-62166, -60198]ns | [124900, 129150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 185123ns | -32.5% | -24.7% | -30.2% | -30.2% |
| 1 | 2 | 185201ns | -30.2% | -25.7% | -30.3% | -32.6% |
| 1 | 3 | 191374ns | -32.5% | -28.1% | -32.5% | -34.7% |
| 1 | 4 | 191418ns | -32.5% | -28.1% | -34.7% | -32.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | 0.255 | moderate+ |
| bitpack-mac-narrow | -0.094 | ok |
| bitpack-mac-native | -0.082 | ok |
| bitpack-mac-simd | -0.104 | ok |
| bitpack-mac-windowed | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 20.5ns | 188042.7ns | 0.0% |  |
| bitpack-mac-narrow | 19.6ns | 128096.9ns | 0.0% |  |
| bitpack-mac-native | 19.3ns | 138147.4ns | 0.0% |  |
| bitpack-mac-simd | 19.7ns | 127824.2ns | 0.0% |  |
| bitpack-mac-windowed | 19.7ns | 127091.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 185012.6-191444.8 ns)
  185012.6 |#####################################
  185334.2 |########
  185655.8 |
  185977.4 |
  186299.0 |
  186620.6 |##
  186942.2 |##
  187263.8 |
  187585.5 |
  187907.1 |
  188228.7 |
  188550.3 |
  188871.9 |
  189193.5 |
  189515.1 |
  189836.7 |
  190158.4 |
  190480.0 |
  190801.6 |##
  191123.2 |########################################
  (4 below, 3 above range)

bitpack-mac-narrow (n=40, range 124892.2-129257.6 ns)
  124892.2 |###########
  125110.5 |
  125328.7 |
  125547.0 |
  125765.3 |
  125983.5 |
  126201.8 |
  126420.1 |
  126638.3 |
  126856.6 |
  127074.9 |
  127293.1 |
  127511.4 |
  127729.7 |
  127947.9 |
  128166.2 |
  128384.5 |
  128602.7 |
  128821.0 |###
  129039.3 |########################################
  (3 below, 3 above range)

bitpack-mac-native (n=40, range 137517.4-139450.2 ns)
  137517.4 |########################################
  137614.0 |#########################
  137710.7 |##########
  137807.3 |#######
  137904.0 |
  138000.6 |
  138097.2 |
  138193.9 |###
  138290.5 |###
  138387.2 |
  138483.8 |
  138580.5 |
  138677.1 |
  138773.7 |
  138870.4 |
  138967.0 |
  139063.7 |
  139160.3 |
  139257.0 |##############
  139353.6 |##############
  (4 below, 3 above range)

bitpack-mac-simd (n=40, range 124896.9-129817.0 ns)
  124896.9 |#################
  125143.0 |
  125389.0 |
  125635.0 |
  125881.0 |
  126127.0 |
  126373.0 |
  126619.0 |
  126865.0 |
  127111.0 |
  127357.0 |
  127603.0 |
  127849.0 |
  128095.0 |
  128341.0 |
  128587.0 |
  128833.0 |#
  129079.0 |########################################
  129325.0 |
  129571.0 |
  (4 below, 2 above range)

bitpack-mac-windowed (n=40, range 124872.0-131453.5 ns)
  124872.0 |########################################
  125201.1 |
  125530.1 |
  125859.2 |##
  126188.3 |
  126517.4 |
  126846.4 |
  127175.5 |
  127504.6 |
  127833.7 |
  128162.8 |
  128491.8 |
  128820.9 |####
  129150.0 |################
  129479.1 |
  129808.1 |
  130137.2 |##
  130466.3 |
  130795.4 |
  131124.4 |
  (3 below, 5 above range)

```
