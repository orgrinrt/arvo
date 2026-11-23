# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread beats baseline by 23% (significant)

wide-rung-ragged-overread is -160.07 us (23%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-ragged-overread is fastest but the noisiest (CV 10.3%)

wide-rung-ragged-overread wins on median (532.40 us) yet has the highest variance (CV 10.3%), while wide-rung-align16 is the steadiest (CV 5.4%, 702.72 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.73)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 532399.8 ns median (-24.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.38x (fastest 532399.8 ns, slowest 737139.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112737653 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 703173ns | 703315ns | 645049ns | 706592ns | 751039ns | base |
| wide-rung-ragged | 567754ns | 575402ns | 504361ns | 568245ns | 629673ns | -19.26% |
| wide-rung-ragged-overread | 553376ns | 532764ns | 499548ns | 541282ns | 643484ns | -21.30% |
| wide-rung-wordround | 718592ns | 737725ns | 641948ns | 721125ns | 787635ns | +2.19% |
| wide-rung-wordround-alias | 689228ns | 669758ns | 612710ns | 692591ns | 755657ns | -1.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 702520ns | 644384ns | 750338ns | base | 0.653 |
| wide-rung-ragged | 567333ns | 504006ns | 629174ns | -19.24% | 0.809 |
| wide-rung-ragged-overread | 552993ns | 499219ns | 643015ns | -21.28% | 0.830 |
| wide-rung-wordround | 717931ns | 641241ns | 787024ns | +2.19% | 0.639 |
| wide-rung-wordround-alias | 688558ns | 611890ns | 755090ns | -1.99% | 0.666 |

## Performance model

- Peak throughput: **0.919 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.653 | 71.0% |
| wide-rung-ragged | 0.798 | 86.8% |
| wide-rung-ragged-overread | 0.862 | 93.8% |
| wide-rung-wordround | 0.622 | 67.7% |
| wide-rung-wordround-alias | 0.686 | 74.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 703173ns | 703173ns | base |
| wide-rung-ragged | 567754ns | 567754ns | -19.26% |
| wide-rung-ragged-overread | 553376ns | 553376ns | -21.30% |
| wide-rung-wordround | 718592ns | 718592ns | +2.19% |
| wide-rung-wordround-alias | 689228ns | 689228ns | -1.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 702715ns | base | --- | [695220, 716518] | --- | --- | --- | --- |
| wide-rung-ragged | 574959ns | -139717.5ns (-19.9%) | [-173040, -120253]ns | [535255, 601569] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 532400ns | -160069.4ns (-22.8%) | [-189808, -129525]ns | [525509, 553191] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 737139ns | no significant difference | [-26987, +43212]ns | [683626, 744289] | no | 0.2051 | 0.1539 | 0 |
| wide-rung-wordround-alias | 669130ns | no significant difference | [-54337, +11332]ns | [651402, 745538] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 694451ns | -27.6% | -23.1% | +13.2% | -10.8% |
| 1 | 2 | 734134ns | -17.6% | -28.1% | -6.9% | +1.5% |
| 1 | 3 | 695666ns | -13.6% | -14.7% | +6.3% | +8.4% |
| 1 | 4 | 712665ns | -24.9% | -29.2% | -8.6% | -8.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.727 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.241 | moderate- |
| wide-rung-ragged-overread | -0.575 | HIGH- (thermal bounce) |
| wide-rung-wordround | -0.425 | moderate- |
| wide-rung-wordround-alias | -0.231 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 157.0ns | 702519.7ns | 0.0% |  |
| wide-rung-ragged | 139.0ns | 567332.6ns | 0.0% |  |
| wide-rung-ragged-overread | 111.5ns | 552993.3ns | 0.0% |  |
| wide-rung-wordround | 174.1ns | 717931.0ns | 0.0% |  |
| wide-rung-wordround-alias | 163.5ns | 688558.2ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 644383.8-750338.5 ns)
  644383.8 |
  649681.5 |#####
  654979.2 |
  660277.0 |#####
  665574.7 |
  670872.4 |#####
  676170.2 |#####
  681467.9 |###########
  686765.6 |#####
  692063.4 |########################################
  697361.1 |###########
  702658.8 |#####
  707956.6 |###########
  713254.3 |##################################
  718552.1 |
  723849.8 |
  729147.5 |######################
  734445.3 |###########
  739743.0 |
  745040.7 |
  (4 below, 5 above range)

wide-rung-ragged (n=40, range 504005.6-629174.3 ns)
  504005.6 |
  510264.0 |
  516522.5 |########################
  522780.9 |
  529039.3 |########################################
  535297.8 |################
  541556.2 |########
  547814.6 |################
  554073.1 |
  560331.5 |
  566589.9 |
  572848.4 |################
  579106.8 |########
  585365.2 |########################
  591623.7 |
  597882.1 |########################################
  604140.5 |########################
  610399.0 |########################
  616657.4 |################
  622915.8 |
  (6 below, 2 above range)

wide-rung-ragged-overread (n=40, range 499218.8-643015.2 ns)
  499218.8 |##############################
  506408.6 |#####
  513598.5 |#####
  520788.3 |########################################
  527978.1 |########################################
  535167.9 |
  542357.7 |
  549547.5 |
  556737.4 |
  563927.2 |
  571117.0 |###############
  578306.8 |#####
  585496.6 |#####
  592686.5 |##########
  599876.3 |##########
  607066.1 |
  614255.9 |
  621445.7 |#####
  628635.5 |
  635825.4 |#####
  (2 below, 3 above range)

wide-rung-wordround (n=40, range 641240.9-787023.8 ns)
  641240.9 |#################
  648530.0 |############################
  655819.2 |###########
  663108.3 |
  670397.5 |
  677686.6 |#####
  684975.7 |###########
  692264.9 |
  699554.0 |
  706843.2 |
  714132.3 |
  721421.5 |
  728710.6 |###########
  735999.8 |##################################
  743288.9 |############################
  750578.1 |#####
  757867.2 |
  765156.4 |
  772445.5 |
  779734.7 |########################################
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 611889.6-755090.3 ns)
  611889.6 |#################
  619049.7 |#################
  626209.7 |###########
  633369.7 |
  640529.8 |#################
  647689.8 |##################################
  654849.8 |#####
  662009.9 |
  669169.9 |
  676329.9 |#####
  683490.0 |#####
  690650.0 |
  697810.0 |
  704970.1 |
  712130.1 |
  719290.1 |
  726450.2 |
  733610.2 |
  740770.2 |########################################
  747930.3 |########################################
  (2 below, 4 above range)

```
