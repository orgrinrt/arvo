# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (616.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 452.64 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged dominates: 31% faster than the next best (wide-rung-ragged-overread)

wide-rung-ragged (452.64 us) leads wide-rung-ragged-overread (594.01 us) by 31%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 26% (significant)

wide-rung-ragged is -158.72 us (26%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.64)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.64, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wide-rung-ragged} vs {wide-rung-ragged-overread, wide-rung-wordround, wide-rung-wordround-alias, wide-rung-align16} (31% apart)

The field splits into a fast tier {wide-rung-ragged} and a slow tier {wide-rung-ragged-overread, wide-rung-wordround, wide-rung-wordround-alias, wide-rung-align16} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: wide-rung-ragged** at 452642.1 ns median (-26.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.36x (fastest 452642.1 ns, slowest 616400.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 621174ns | 617296ns | 609011ns | 618949ns | 640013ns | base |
| wide-rung-ragged | 453354ns | 453221ns | 446563ns | 453183ns | 460659ns | -27.02% |
| wide-rung-ragged-overread | 596010ns | 594790ns | 590824ns | 595283ns | 603376ns | -4.05% |
| wide-rung-wordround | 614513ns | 613431ns | 610216ns | 613759ns | 621071ns | -1.07% |
| wide-rung-wordround-alias | 616622ns | 615531ns | 609254ns | 615856ns | 626289ns | -0.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 620351ns | 608352ns | 639066ns | base | 0.740 |
| wide-rung-ragged | 452696ns | 445750ns | 460024ns | -27.03% | 1.013 |
| wide-rung-ragged-overread | 595343ns | 590198ns | 602719ns | -4.03% | 0.771 |
| wide-rung-wordround | 613636ns | 609429ns | 620110ns | -1.08% | 0.748 |
| wide-rung-wordround-alias | 615807ns | 608569ns | 625406ns | -0.73% | 0.745 |

## Performance model

- Peak throughput: **1.029 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.744 | 72.3% |
| wide-rung-ragged | 1.013 | 98.5% |
| wide-rung-ragged-overread | 0.772 | 75.0% |
| wide-rung-wordround | 0.749 | 72.8% |
| wide-rung-wordround-alias | 0.746 | 72.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 621174ns | 621174ns | base |
| wide-rung-ragged | 453354ns | 453354ns | -27.02% |
| wide-rung-ragged-overread | 596010ns | 596010ns | -4.05% |
| wide-rung-wordround | 614513ns | 614513ns | -1.07% |
| wide-rung-wordround-alias | 616622ns | 616622ns | -0.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 616400ns | base | --- | [614214, 621249] | --- | --- | --- | --- |
| wide-rung-ragged | 452642ns | -166530.2ns (-27.0%) | [-169222, -163480]ns | [450685, 454128] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 594007ns | -22300.5ns (-3.6%) | [-25612, -19108]ns | [592978, 596204] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 612552ns | -1717.3ns (-0.3%) | [-6176, -566]ns | [611170, 614155] | YES | 0.0221 | 0.0166 | 0 |
| wide-rung-wordround-alias | 614618ns | no significant difference | [-7641, +713]ns | [613523, 616782] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 632806ns | -28.4% | -6.1% | -1.7% | -2.3% |
| 2 | 620532ns | -26.2% | -3.9% | -0.0% | -1.2% |
| 3 | 626649ns | -26.2% | -5.4% | -2.7% | -1.9% |
| 4 | 627271ns | -27.9% | -4.4% | -2.8% | -2.3% |
| 5 | 615851ns | -27.1% | -1.1% | -0.8% | -0.4% |
| 6 | 627920ns | -27.9% | -5.4% | -2.6% | -2.2% |
| 7 | 611738ns | -25.3% | -3.3% | -0.3% | +0.6% |
| 8 | 606841ns | -24.8% | -2.8% | +0.6% | +4.0% |
| 9 | 610792ns | -24.4% | -3.1% | -0.1% | +1.6% |
| 10 | 611362ns | -25.1% | -2.0% | -0.2% | +0.1% |
| 11 | 606985ns | -26.0% | -2.2% | +0.3% | +0.3% |
| 12 | 634778ns | -29.0% | -6.9% | -3.7% | -4.2% |
| 13 | 615693ns | -27.6% | -4.2% | -0.4% | -1.2% |
| 14 | 614081ns | -27.5% | -4.0% | -0.5% | -1.0% |
| 15 | 615374ns | -27.6% | -3.6% | -1.1% | -1.3% |
| 16 | 611225ns | -26.7% | -3.3% | -0.1% | +1.1% |
| 17 | 612536ns | -27.3% | -3.7% | +0.0% | -0.3% |
| 18 | 609674ns | -27.0% | -2.8% | +0.2% | +0.3% |
| 19 | 605834ns | -26.4% | -2.6% | +0.3% | +1.3% |
| 20 | 607235ns | -26.7% | -2.8% | +0.7% | +0.2% |
| 21 | 621022ns | -27.9% | -2.3% | -0.3% | -1.0% |
| 22 | 615607ns | -26.0% | -2.6% | -0.5% | -0.3% |
| 23 | 619699ns | -27.2% | -3.9% | -0.3% | +1.0% |
| 24 | 632936ns | -28.7% | -5.2% | -3.3% | -1.7% |
| 25 | 639683ns | -29.2% | -7.3% | -4.7% | -3.9% |
| 26 | 614347ns | -26.6% | -3.1% | -0.2% | -0.1% |
| 27 | 616949ns | -27.0% | -3.4% | +0.0% | +0.1% |
| 28 | 617489ns | -26.5% | -3.8% | -0.2% | -0.6% |
| 29 | 658104ns | -31.3% | -9.6% | -6.9% | -6.3% |
| 30 | 625423ns | -28.1% | -5.2% | -2.2% | +0.0% |
| 31 | 610378ns | -25.2% | -2.1% | +1.9% | +1.5% |
| 32 | 614648ns | -26.3% | -1.3% | +0.8% | +2.1% |
| 33 | 644624ns | -29.2% | -7.6% | -5.0% | -5.4% |
| 34 | 638000ns | -27.9% | -6.5% | -3.3% | -1.9% |
| 35 | 617758ns | -26.8% | -3.4% | +0.3% | +0.0% |
| 36 | 627069ns | -26.6% | -4.1% | -2.2% | -1.6% |
| 37 | 631595ns | -27.0% | -6.0% | -2.4% | -1.3% |
| 38 | 612990ns | -25.3% | -2.5% | +1.1% | +0.5% |
| 39 | 621477ns | -26.9% | -4.8% | -0.9% | -1.5% |
| 40 | 609076ns | -25.6% | -1.6% | +1.0% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.215 | moderate+ |
| wide-rung-ragged | 0.640 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.219 | moderate+ |
| wide-rung-wordround | 0.405 | moderate+ |
| wide-rung-wordround-alias | 0.271 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 40/40, lost 0/40
- **wide-rung-ragged-overread**: won 40/40, lost 0/40
- **wide-rung-wordround**: won 26/40, lost 10/40
- **wide-rung-wordround-alias**: won 22/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 15.5ns | 620351.3ns | 0.0% |  |
| wide-rung-ragged | 10.9ns | 452696.1ns | 0.0% |  |
| wide-rung-ragged-overread | 9.4ns | 595343.1ns | 0.0% |  |
| wide-rung-wordround | 16.9ns | 613636.1ns | 0.0% |  |
| wide-rung-wordround-alias | 22.0ns | 615806.9ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 608351.9-639065.9 ns)
  608351.9 |################
  609887.6 |################################
  611423.3 |################
  612959.0 |########################
  614494.7 |########################################
  616030.4 |################
  617566.1 |########
  619101.8 |################
  620637.5 |################
  622173.2 |
  623708.9 |
  625244.6 |################
  626780.3 |########################
  628316.0 |
  629851.7 |
  631387.4 |################
  632923.1 |########
  634458.8 |########
  635994.5 |
  637530.2 |########
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 445750.5-460023.8 ns)
  445750.5 |####################
  446464.1 |
  447177.8 |####################
  447891.4 |
  448605.1 |####################
  449318.8 |##########
  450032.4 |####################
  450746.1 |##############################
  451459.8 |
  452173.4 |########################################
  452887.1 |########################################
  453600.8 |##########
  454314.4 |##########
  455028.1 |
  455741.8 |##############################
  456455.4 |####################
  457169.1 |
  457882.8 |##############################
  458596.4 |
  459310.1 |
  (5 below, 5 above range)

wide-rung-ragged-overread (n=40, range 590198.5-602719.4 ns)
  590198.5 |
  590824.5 |########################
  591450.6 |################
  592076.6 |################
  592702.7 |################
  593328.7 |########################################
  593954.8 |########
  594580.8 |################
  595206.9 |########
  595832.9 |################################
  596459.0 |########
  597085.0 |################
  597711.0 |
  598337.1 |
  598963.1 |########################
  599589.2 |################
  600215.2 |
  600841.3 |
  601467.3 |########
  602093.4 |
  (6 below, 3 above range)

wide-rung-wordround (n=40, range 609429.0-620110.0 ns)
  609429.0 |##############################
  609963.0 |########################################
  610497.1 |########################################
  611031.1 |##############################
  611565.2 |####################
  612099.2 |####################
  612633.3 |########################################
  613167.3 |##########
  613701.4 |
  614235.4 |
  614769.5 |##########
  615303.5 |##########
  615837.6 |##########
  616371.6 |####################
  616905.7 |##########
  617439.7 |##########
  617973.8 |
  618507.8 |
  619041.9 |####################
  619575.9 |####################
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 608568.8-625405.8 ns)
  608568.8 |
  609410.7 |########
  610252.5 |########
  611094.4 |########
  611936.2 |################
  612778.1 |########################
  613619.9 |########################################
  614461.8 |########################################
  615303.6 |########
  616145.5 |################
  616987.3 |################
  617829.2 |################
  618671.0 |########
  619512.9 |########
  620354.7 |
  621196.6 |
  622038.4 |########
  622880.3 |########
  623722.1 |
  624564.0 |
  (6 below, 5 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.64 (measurement drift or warm-up artifact)
