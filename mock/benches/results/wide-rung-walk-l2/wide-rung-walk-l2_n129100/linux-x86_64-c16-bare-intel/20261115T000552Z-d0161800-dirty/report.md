# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (720.24 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 421.13 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged dominates: 22% faster than the next best (wide-rung-ragged-overread)

wide-rung-ragged (421.13 us) leads wide-rung-ragged-overread (513.69 us) by 22%, a clear separation rather than a photo finish. CV 18.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 44% (significant)

wide-rung-ragged is -316.23 us (44%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-ragged is fastest but the noisiest (CV 18.1%)

wide-rung-ragged wins on median (421.13 us) yet has the highest variance (CV 18.1%), while wide-rung-ragged-overread is the steadiest (CV 1.2%, 513.69 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-ragged is inconsistent: worst-20% is 1.6x its best-20%

wide-rung-ragged's best 20% of batches run at 334.06 us but its worst 20% at 549.16 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged** at 421129.9 ns median (-41.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.71x (fastest 421129.9 ns, slowest 720238.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 734465ns | 720827ns | 707886ns | 722855ns | 795875ns | base |
| wide-rung-ragged | 430819ns | 421321ns | 334236ns | 423402ns | 549655ns | -41.34% |
| wide-rung-ragged-overread | 517455ns | 513923ns | 511900ns | 516116ns | 527030ns | -29.55% |
| wide-rung-wordround | 604624ns | 612847ns | 534300ns | 603173ns | 679300ns | -17.68% |
| wide-rung-wordround-alias | 649876ns | 644849ns | 608188ns | 646706ns | 701073ns | -11.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 733828ns | 707209ns | 795298ns | base | 0.625 |
| wide-rung-ragged | 430571ns | 334055ns | 549164ns | -41.33% | 1.065 |
| wide-rung-ragged-overread | 517229ns | 511665ns | 526747ns | -29.52% | 0.887 |
| wide-rung-wordround | 604207ns | 533906ns | 678779ns | -17.66% | 0.759 |
| wide-rung-wordround-alias | 649367ns | 607674ns | 700572ns | -11.51% | 0.706 |

## Performance model

- Peak throughput: **1.373 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.637 | 46.4% |
| wide-rung-ragged | 1.089 | 79.3% |
| wide-rung-ragged-overread | 0.893 | 65.0% |
| wide-rung-wordround | 0.749 | 54.5% |
| wide-rung-wordround-alias | 0.712 | 51.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 734465ns | 734465ns | base |
| wide-rung-ragged | 430819ns | 430819ns | -41.34% |
| wide-rung-ragged-overread | 517455ns | 517455ns | -29.55% |
| wide-rung-wordround | 604624ns | 604624ns | -17.68% |
| wide-rung-wordround-alias | 649876ns | 649876ns | -11.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 720239ns | base | --- | [716383, 723420] | --- | --- | --- | --- |
| wide-rung-ragged | 421130ns | -316226.5ns (-43.9%) | [-359423, -281331]ns | [407958, 434204] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 513694ns | -202993.1ns (-28.2%) | [-211735, -201566]ns | [512618, 518834] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 612471ns | -104505.4ns (-14.5%) | [-171054, -98333]ns | [606848, 617159] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 644460ns | -95098.9ns (-13.2%) | [-103590, -68585]ns | [616086, 682431] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 724353ns | -43.7% | -29.4% | -15.4% | -10.1% |
| 1 | 2 | 714985ns | -53.3% | -26.5% | -14.4% | -14.5% |
| 1 | 3 | 720239ns | -24.4% | -28.0% | -11.1% | -5.1% |
| 1 | 4 | 745943ns | -41.8% | -31.3% | -28.4% | -8.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.049 | ok |
| wide-rung-ragged | -0.368 | moderate- |
| wide-rung-ragged-overread | -0.321 | moderate- |
| wide-rung-wordround | -0.320 | moderate- |
| wide-rung-wordround-alias | -0.040 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 229.6ns | 733827.7ns | 0.0% |  |
| wide-rung-ragged | 61.4ns | 430571.1ns | 0.0% |  |
| wide-rung-ragged-overread | 37.3ns | 517229.2ns | 0.0% |  |
| wide-rung-wordround | 145.6ns | 604206.7ns | 0.0% |  |
| wide-rung-wordround-alias | 156.5ns | 649366.5ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 707209.2-795298.3 ns)
  707209.2 |###
  711613.7 |########################################
  716018.1 |#########################
  720422.6 |################################
  724827.0 |
  729231.5 |
  733635.9 |
  738040.4 |###
  742444.8 |
  746849.3 |
  751253.8 |
  755658.2 |
  760062.7 |###
  764467.1 |
  768871.6 |
  773276.0 |###
  777680.5 |
  782084.9 |
  786489.4 |
  790893.9 |##############
  (2 below, 3 above range)

wide-rung-ragged (n=40, range 334055.2-549164.5 ns)
  334055.2 |####################
  344810.7 |
  355566.1 |
  366321.6 |
  377077.1 |
  387832.5 |
  398588.0 |########################################
  409343.4 |
  420098.9 |
  430854.4 |########################################
  441609.8 |
  452365.3 |
  463120.8 |
  473876.2 |
  484631.7 |
  495387.2 |
  506142.6 |
  516898.1 |
  527653.5 |####################
  538409.0 |
  (5 below, 5 above range)

wide-rung-ragged-overread (n=40, range 511665.0-526747.0 ns)
  511665.0 |#########################
  512419.1 |########################################
  513173.2 |###############
  513927.3 |#####
  514681.4 |
  515435.5 |
  516189.6 |
  516943.7 |
  517697.8 |#####
  518451.9 |##############################
  519206.0 |
  519960.1 |#####
  520714.2 |
  521468.3 |
  522222.4 |
  522976.5 |
  523730.6 |
  524484.7 |####################
  525238.8 |####################
  525992.9 |#####
  (5 below, 1 above range)

wide-rung-wordround (n=40, range 533905.8-678779.2 ns)
  533905.8 |########################################
  541149.4 |
  548393.1 |
  555636.8 |####
  562880.4 |
  570124.1 |
  577367.8 |
  584611.4 |
  591855.1 |
  599098.8 |####
  606342.5 |########################################
  613586.1 |#################
  620829.8 |####
  628073.5 |####
  635317.1 |##########################
  642560.8 |
  649804.5 |
  657048.2 |####
  664291.8 |
  671535.5 |
  (3 below, 4 above range)

wide-rung-wordround-alias (n=40, range 607673.7-700571.7 ns)
  607673.7 |########################################
  612318.6 |#########################
  616963.5 |###############
  621608.4 |
  626253.3 |#####
  630898.2 |
  635543.1 |
  640188.0 |#####
  644832.9 |##########
  649477.8 |
  654122.7 |#####
  658767.6 |
  663412.5 |#####
  668057.4 |
  672702.3 |
  677347.2 |##########
  681992.1 |###################################
  686637.0 |#########################
  691281.9 |
  695926.8 |
  (2 below, 2 above range)

```
