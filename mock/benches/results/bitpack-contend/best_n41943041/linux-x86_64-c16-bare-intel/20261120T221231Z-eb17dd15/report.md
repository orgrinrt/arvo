# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d32-padal shows alternating (throttle bounce) (autocorr -0.78)

bitpack-contend-d32-padal's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-contend-packed-simd** at 516838.9 ns median (-22.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.37x (fastest 516838.9 ns, slowest 706928.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761890 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 679101ns | 666846ns | 641561ns | 664288ns | 761080ns | base |
| bitpack-contend-d16-padal | 682125ns | 648282ns | 638784ns | 658868ns | 795238ns | +0.45% |
| bitpack-contend-d32 | 720274ns | 701808ns | 697658ns | 707534ns | 781112ns | +6.06% |
| bitpack-contend-d32-padal | 732464ns | 707826ns | 696266ns | 730331ns | 775060ns | +7.86% |
| bitpack-contend-packed-simd | 534621ns | 517001ns | 515846ns | 522480ns | 589821ns | -21.28% |
| bitpack-contend-pipe4 | 559620ns | 529421ns | 516302ns | 549603ns | 632990ns | -17.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 678862ns | 641372ns | 760756ns | base | 6.178 |
| bitpack-contend-d16-padal | 681881ns | 638587ns | 794923ns | +0.44% | 6.151 |
| bitpack-contend-d32 | 719381ns | 696714ns | 780410ns | +5.97% | 5.830 |
| bitpack-contend-d32-padal | 731695ns | 695260ns | 774620ns | +7.78% | 5.732 |
| bitpack-contend-packed-simd | 534437ns | 515685ns | 589560ns | -21.27% | 7.848 |
| bitpack-contend-pipe4 | 559406ns | 516140ns | 632812ns | -17.60% | 7.498 |

## Performance model

- Peak throughput: **8.133 Gops/s** (bitpack-contend-packed-simd; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.293 | 77.4% |
| bitpack-contend-d16-padal | 6.472 | 79.6% |
| bitpack-contend-d32 | 5.985 | 73.6% |
| bitpack-contend-d32-padal | 5.933 | 72.9% |
| bitpack-contend-packed-simd | 8.115 | 99.8% |
| bitpack-contend-pipe4 | 7.926 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 679101ns | 679101ns | base |
| bitpack-contend-d16-padal | 682125ns | 682125ns | +0.45% |
| bitpack-contend-d32 | 720274ns | 720274ns | +6.06% |
| bitpack-contend-d32-padal | 732464ns | 732464ns | +7.86% |
| bitpack-contend-packed-simd | 534621ns | 534621ns | -21.28% |
| bitpack-contend-pipe4 | 559620ns | 559620ns | -17.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 666543ns | base | --- | [648115, 671506] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 648080ns | -5896.3ns (-0.9%) | [-8906, -1681]ns | [640041, 652740] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-contend-d32 | 700857ns | +43911.7ns (+6.6%) | [+32035, +52020]ns | [697542, 705920] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 706929ns | +45042.3ns (+6.8%) | [+28353, +117032]ns | [700906, 766249] | YES | 0.0028 | 0.0022 | 0 |
| bitpack-contend-packed-simd | 516839ns | -127650.5ns (-19.2%) | [-133891, -124057]ns | [516007, 517759] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 529188ns | -117210.6ns (-17.6%) | [-133683, -89988]ns | [517067, 579429] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 646250ns | -1.0% | +8.5% | +18.7% | -20.2% | -20.1% |
| 1 | 2 | 669429ns | -2.6% | +4.4% | +3.9% | -12.0% | -13.4% |
| 1 | 3 | 645655ns | -1.1% | +8.0% | +19.6% | -19.9% | -2.4% |
| 1 | 4 | 758382ns | +4.7% | +2.8% | -7.7% | -32.0% | -31.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.230 | moderate- |
| bitpack-contend-d16-padal | -0.139 | ok |
| bitpack-contend-d32 | -0.107 | ok |
| bitpack-contend-d32-padal | -0.779 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.409 | moderate- |
| bitpack-contend-pipe4 | -0.283 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 3/4, lost 1/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d32-padal**: won 1/4, lost 3/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 56.9ns | 678862.5ns | 0.0% |  |
| bitpack-contend-d16-padal | 26.2ns | 681881.3ns | 0.0% |  |
| bitpack-contend-d32 | 103.5ns | 719380.9ns | 0.0% |  |
| bitpack-contend-d32-padal | 35.9ns | 731694.9ns | 0.0% |  |
| bitpack-contend-packed-simd | 32.3ns | 534436.9ns | 0.0% |  |
| bitpack-contend-pipe4 | 23.9ns | 559406.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 641371.9-760756.4 ns)
  641371.9 |########################################
  647341.1 |######################
  653310.4 |####
  659279.6 |
  665248.8 |###############################
  671218.0 |#################
  677187.3 |
  683156.5 |
  689125.7 |
  695094.9 |
  701064.1 |
  707033.4 |####
  713002.6 |
  718971.8 |####
  724941.0 |
  730910.3 |
  736879.5 |
  742848.7 |
  748817.9 |
  754787.2 |##########################
  (4 below, 2 above range)

bitpack-contend-d16-padal (n=40, range 638587.3-794922.8 ns)
  638587.3 |########################################
  646404.1 |######################
  654220.8 |
  662037.6 |##
  669854.4 |
  677671.2 |
  685487.9 |
  693304.7 |
  701121.5 |
  708938.3 |
  716755.0 |
  724571.8 |
  732388.6 |
  740205.4 |
  748022.1 |
  755838.9 |
  763655.7 |
  771472.5 |
  779289.2 |
  787106.0 |#################
  (4 below, 3 above range)

bitpack-contend-d32 (n=40, range 696714.3-780409.7 ns)
  696714.3 |########################################
  700899.1 |###############
  705083.9 |#######
  709268.6 |
  713453.4 |##
  717638.2 |
  721822.9 |
  726007.7 |
  730192.5 |
  734377.2 |
  738562.0 |
  742746.8 |
  746931.6 |
  751116.3 |##
  755301.1 |
  759485.9 |
  763670.6 |
  767855.4 |##
  772040.2 |
  776224.9 |##########
  (4 below, 4 above range)

bitpack-contend-d32-padal (n=40, range 695260.2-774620.4 ns)
  695260.2 |############
  699228.2 |########################################
  703196.2 |########
  707164.2 |####
  711132.2 |
  715100.2 |
  719068.2 |
  723036.2 |
  727004.3 |
  730972.3 |####
  734940.3 |
  738908.3 |
  742876.3 |####
  746844.3 |
  750812.3 |
  754780.3 |
  758748.3 |####
  762716.3 |############
  766684.3 |####################
  770652.4 |################
  (5 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 515685.0-589560.1 ns)
  515685.0 |########################################
  519378.8 |#
  523072.5 |#
  526766.3 |
  530460.0 |
  534153.8 |
  537847.6 |
  541541.3 |
  545235.1 |
  548928.8 |
  552622.6 |
  556316.3 |
  560010.1 |
  563703.8 |
  567397.6 |
  571091.4 |
  574785.1 |#
  578478.9 |#
  582172.6 |
  585866.4 |######
  (3 below, 4 above range)

bitpack-contend-pipe4 (n=40, range 516139.8-632812.3 ns)
  516139.8 |########################################
  521973.4 |
  527807.1 |#####
  533640.7 |
  539474.3 |
  545307.9 |
  551141.6 |
  556975.2 |
  562808.8 |
  568642.4 |
  574476.0 |####################
  580309.7 |#####
  586143.3 |##
  591976.9 |
  597810.5 |
  603644.1 |
  609477.8 |
  615311.4 |
  621145.0 |
  626978.6 |####################
  (5 below, 2 above range)

```
