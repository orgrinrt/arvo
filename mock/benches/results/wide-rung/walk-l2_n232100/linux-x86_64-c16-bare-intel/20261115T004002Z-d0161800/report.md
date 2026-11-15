# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread is fastest but the noisiest (CV 6.7%)

wide-rung-ragged-overread wins on median (651.45 us) yet has the highest variance (CV 6.7%), while wide-rung-ragged is the steadiest (CV 3.5%, 688.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.56)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-ragged-overread vs stability leader wide-rung-ragged (+6% speed for 1.9x steadier)

wide-rung-ragged-overread is fastest (651.45 us, CV 6.7%); wide-rung-ragged gives up 5.8% median for 1.9x lower variance (CV 3.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 651449.9 ns median (-6.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 651449.9 ns, slowest 704970.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 705260ns | 699328ns | 657935ns | 700989ns | 765397ns | base |
| wide-rung-ragged | 695092ns | 689419ns | 663995ns | 693880ns | 729827ns | -1.44% |
| wide-rung-ragged-overread | 667998ns | 652156ns | 637441ns | 656259ns | 733770ns | -5.28% |
| wide-rung-wordround | 706527ns | 702601ns | 674291ns | 700855ns | 755779ns | +0.18% |
| wide-rung-wordround-alias | 707033ns | 705492ns | 677644ns | 703809ns | 746096ns | +0.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 704600ns | 657262ns | 764796ns | base | 0.651 |
| wide-rung-ragged | 694504ns | 663376ns | 729274ns | -1.43% | 0.661 |
| wide-rung-ragged-overread | 667459ns | 636964ns | 733215ns | -5.27% | 0.687 |
| wide-rung-wordround | 705918ns | 673594ns | 755129ns | +0.19% | 0.650 |
| wide-rung-wordround-alias | 706455ns | 677057ns | 745555ns | +0.26% | 0.649 |

## Performance model

- Peak throughput: **0.720 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.657 | 91.2% |
| wide-rung-ragged | 0.666 | 92.5% |
| wide-rung-ragged-overread | 0.704 | 97.8% |
| wide-rung-wordround | 0.653 | 90.7% |
| wide-rung-wordround-alias | 0.651 | 90.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 705260ns | 705260ns | base |
| wide-rung-ragged | 695092ns | 695092ns | -1.44% |
| wide-rung-ragged-overread | 667998ns | 667998ns | -5.28% |
| wide-rung-wordround | 706527ns | 706527ns | +0.18% |
| wide-rung-wordround-alias | 707033ns | 707033ns | +0.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 698632ns | base | --- | [696270, 709068] | --- | --- | --- | --- |
| wide-rung-ragged | 688970ns | no significant difference | [-18504, +11693]ns | [688075, 705813] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-ragged-overread | 651450ns | -30034.6ns (-4.3%) | [-46654, -18401]ns | [641929, 652473] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 702161ns | no significant difference | [-19, +15516]ns | [700771, 702540] | no | 0.1614 | 0.0807 | 0 |
| wide-rung-wordround-alias | 704970ns | no significant difference | [-18808, +11613]ns | [698903, 706858] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 724279ns | -1.6% | -11.6% | -5.9% | -2.6% |
| 1 | 2 | 661805ns | +7.4% | -3.0% | +6.1% | +5.9% |
| 1 | 3 | 700679ns | -3.0% | -0.8% | +3.8% | -0.6% |
| 1 | 4 | 697758ns | -1.3% | -6.5% | +0.4% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.559 | HIGH- (thermal bounce) |
| wide-rung-ragged | 0.157 | ok |
| wide-rung-ragged-overread | -0.258 | moderate- |
| wide-rung-wordround | -0.050 | ok |
| wide-rung-wordround-alias | -0.351 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 1/4, lost 3/4
- **wide-rung-wordround-alias**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 147.6ns | 704600.1ns | 0.0% |  |
| wide-rung-ragged | 187.2ns | 694504.0ns | 0.0% |  |
| wide-rung-ragged-overread | 200.1ns | 667459.4ns | 0.0% |  |
| wide-rung-wordround | 157.2ns | 705917.7ns | 0.0% |  |
| wide-rung-wordround-alias | 199.0ns | 706455.4ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 657261.6-764795.8 ns)
  657261.6 |#######
  662638.3 |
  668015.1 |#######
  673391.8 |##############
  678768.5 |
  684145.2 |###
  689521.9 |
  694898.6 |########################################
  700275.3 |#######
  705652.0 |
  711028.7 |###
  716405.4 |
  721782.1 |#########################
  727158.8 |
  732535.5 |###
  737912.2 |
  743288.9 |
  748665.7 |
  754042.4 |
  759419.1 |
  (4 below, 5 above range)

wide-rung-ragged (n=40, range 663376.3-729274.5 ns)
  663376.3 |
  666671.2 |
  669966.2 |
  673261.1 |#####
  676556.0 |####################
  679850.9 |####################
  683145.8 |
  686440.7 |########################################
  689735.6 |####################
  693030.5 |
  696325.4 |
  699620.3 |
  702915.2 |#####
  706210.1 |##########
  709505.0 |##############################
  712799.9 |
  716094.9 |
  719389.8 |
  722684.7 |#####
  725979.6 |
  (4 below, 5 above range)

wide-rung-ragged-overread (n=40, range 636964.3-733214.9 ns)
  636964.3 |########################################
  641776.9 |###############################
  646589.4 |####
  651401.9 |###############################
  656214.4 |
  661027.0 |
  665839.5 |
  670652.0 |
  675464.6 |
  680277.1 |####
  685089.6 |
  689902.1 |#############
  694714.7 |##########################
  699527.2 |
  704339.7 |
  709152.2 |
  713964.8 |
  718777.3 |
  723589.8 |
  728402.3 |
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 673594.4-755128.9 ns)
  673594.4 |###
  677671.1 |###
  681747.9 |#########
  685824.6 |######
  689901.3 |###
  693978.0 |###
  698054.8 |#####################
  702131.5 |########################################
  706208.2 |
  710284.9 |######
  714361.7 |
  718438.4 |
  722515.1 |
  726591.8 |
  730668.6 |###
  734745.3 |
  738822.0 |
  742898.7 |
  746975.5 |
  751052.2 |###
  (3 below, 4 above range)

wide-rung-wordround-alias (n=40, range 677056.8-745555.4 ns)
  677056.8 |
  680481.7 |
  683906.6 |##########
  687331.5 |#####
  690756.5 |#####
  694181.4 |#########################
  697606.3 |####################
  701031.3 |###############
  704456.2 |########################################
  707881.1 |########################################
  711306.1 |
  714731.0 |#####
  718155.9 |
  721580.9 |
  725005.8 |
  728430.7 |
  731855.7 |
  735280.6 |
  738705.5 |
  742130.5 |
  (3 below, 4 above range)

```
