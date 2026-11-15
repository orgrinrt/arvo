# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (745.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 697.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-ragged-overread, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-ragged-overread (697.87 us) and wide-rung-wordround-alias (702.70 us) differ by 0.69%, inside the noise, even though the wider field spreads 6.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 697867.5 ns median (-6.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.07x (fastest 697867.5 ns, slowest 745429.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 737016ns | 745882ns | 679389ns | 733258ns | 805918ns | base |
| wide-rung-ragged | 699795ns | 704221ns | 626971ns | 700177ns | 771476ns | -5.05% |
| wide-rung-ragged-overread | 697384ns | 698528ns | 669167ns | 699403ns | 719542ns | -5.38% |
| wide-rung-wordround | 719738ns | 705597ns | 689468ns | 710380ns | 778082ns | -2.34% |
| wide-rung-wordround-alias | 710021ns | 703383ns | 682472ns | 707489ns | 745165ns | -3.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 736525ns | 678832ns | 805477ns | base | 0.623 |
| wide-rung-ragged | 699203ns | 626259ns | 770907ns | -5.07% | 0.656 |
| wide-rung-ragged-overread | 696716ns | 668553ns | 718852ns | -5.40% | 0.658 |
| wide-rung-wordround | 719223ns | 688910ns | 777710ns | -2.35% | 0.638 |
| wide-rung-wordround-alias | 709376ns | 681840ns | 744494ns | -3.69% | 0.647 |

## Performance model

- Peak throughput: **0.733 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.615 | 84.0% |
| wide-rung-ragged | 0.652 | 89.0% |
| wide-rung-ragged-overread | 0.657 | 89.7% |
| wide-rung-wordround | 0.651 | 88.8% |
| wide-rung-wordround-alias | 0.653 | 89.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 737016ns | 737016ns | base |
| wide-rung-ragged | 699795ns | 699795ns | -5.05% |
| wide-rung-ragged-overread | 697384ns | 697384ns | -5.38% |
| wide-rung-wordround | 719738ns | 719738ns | -2.34% |
| wide-rung-wordround-alias | 710021ns | 710021ns | -3.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 745429ns | base | --- | [703071, 747983] | --- | --- | --- | --- |
| wide-rung-ragged | 703578ns | -39486.5ns (-5.3%) | [-49413, -10869]ns | [698338, 707793] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-ragged-overread | 697868ns | -39370.2ns (-5.3%) | [-58096, -10688]ns | [695384, 705611] | YES | 0.0257 | 0.0064 | 0 |
| wide-rung-wordround | 705113ns | no significant difference | [-38614, +18734]ns | [704084, 711634] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround-alias | 702704ns | -14007.3ns (-1.9%) | [-26661, -541]ns | [702381, 706498] | YES | 0.0332 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 689009ns | -7.2% | +1.5% | +2.1% | +2.3% |
| 1 | 2 | 746393ns | -6.1% | -6.1% | -5.3% | -2.4% |
| 1 | 3 | 703071ns | +0.3% | +0.5% | +9.1% | -0.1% |
| 1 | 4 | 804388ns | -11.8% | -13.5% | -12.5% | -12.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.382 | moderate- |
| wide-rung-ragged | -0.017 | ok |
| wide-rung-ragged-overread | -0.462 | moderate- |
| wide-rung-wordround | -0.401 | moderate- |
| wide-rung-wordround-alias | -0.360 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 191.1ns | 736524.9ns | 0.0% |  |
| wide-rung-ragged | 209.2ns | 699203.1ns | 0.0% |  |
| wide-rung-ragged-overread | 250.1ns | 696716.4ns | 0.0% |  |
| wide-rung-wordround | 176.8ns | 719222.6ns | 0.0% |  |
| wide-rung-wordround-alias | 212.3ns | 709375.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 678832.0-805477.4 ns)
  678832.0 |####
  685164.3 |########
  691496.6 |#############
  697828.8 |##########################
  704161.1 |####
  710493.4 |
  716825.6 |
  723157.9 |####
  729490.2 |####
  735822.4 |
  742154.7 |########################################
  748487.0 |#################
  754819.2 |
  761151.5 |
  767483.8 |
  773816.0 |
  780148.3 |
  786480.6 |
  792812.8 |
  799145.1 |##########################
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 626258.7-770906.8 ns)
  626258.7 |
  633491.1 |###
  640723.5 |###
  647955.9 |
  655188.3 |#######
  662420.7 |#######
  669653.1 |
  676885.5 |
  684117.9 |##########
  691350.4 |###
  698582.8 |########################################
  705815.2 |#############################
  713047.6 |
  720280.0 |
  727512.4 |###
  734744.8 |
  741977.2 |
  749209.6 |##########
  756442.0 |
  763674.4 |
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 668552.6-718852.1 ns)
  668552.6 |
  671067.6 |
  673582.5 |
  676097.5 |
  678612.5 |
  681127.5 |#####
  683642.4 |######################
  686157.4 |
  688672.4 |######################
  691187.4 |#####
  693702.4 |######################
  696217.3 |#################
  698732.3 |#################
  701247.3 |###########
  703762.3 |#################
  706277.2 |########################################
  708792.2 |
  711307.2 |
  713822.2 |#####
  716337.1 |#####
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 688909.6-777709.7 ns)
  688909.6 |##
  693349.6 |########
  697789.6 |##########
  702229.6 |########################################
  706669.6 |##
  711109.6 |
  715549.6 |########
  719989.6 |
  724429.6 |
  728869.6 |##
  733309.7 |##
  737749.7 |##########
  742189.7 |
  746629.7 |
  751069.7 |
  755509.7 |
  759949.7 |
  764389.7 |
  768829.7 |
  773269.7 |
  (2 below, 5 above range)

wide-rung-wordround-alias (n=40, range 681840.3-744493.8 ns)
  681840.3 |
  684973.0 |#####
  688105.7 |#####
  691238.3 |##
  694371.0 |
  697503.7 |
  700636.4 |########################################
  703769.0 |#################
  706901.7 |
  710034.4 |
  713167.1 |
  716299.7 |
  719432.4 |##
  722565.1 |#####
  725697.8 |##
  728830.4 |##
  731963.1 |###########
  735095.8 |
  738228.5 |
  741361.1 |
  (3 below, 3 above range)

```
