# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 beats baseline by 92% (significant)

satfold-neon8 is -13.90 us (92%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 17.3x slower than the field

satfold-seq (15.13 us) is 17.3x the fastest (873 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon8 is fastest but the noisiest (CV 8.0%)

satfold-neon8 wins on median (873 ns) yet has the highest variance (CV 8.0%), while satfold-lanes64 is the steadiest (CV 1.8%, 14.64 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes16-constl shows warm-up / thermal drift (autocorr +0.82)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon} vs {satfold-lanes4-idx, satfold-lanes16-constl, satfold-lanes16, satfold-nolaw, satfold-lanes64, satfold-iterfold, satfold-seq} (894% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon} and a slow tier {satfold-lanes4-idx, satfold-lanes16-constl, satfold-lanes16, satfold-nolaw, satfold-lanes64, satfold-iterfold, satfold-seq} with a 894% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 17.3x the fastest

Fastest satfold-neon8 (873 ns) to slowest satfold-seq (15.13 us): 17.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-nolaw's edge over baseline is significant but tiny (31 ns, 0.21%)

satfold-nolaw differs from baseline satfold-iterfold by 31 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon8** at 873.3 ns median (-94.2% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 17.32x (fastest 873.3 ns, slowest 15127.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 15325ns | 15195ns | 14644ns | 15204ns | 16369ns | base |
| satfold-lanes16 | 9321ns | 9171ns | 9006ns | 9201ns | 9996ns | -39.18% |
| satfold-lanes16-constl | 9435ns | 9102ns | 9027ns | 9241ns | 10426ns | -38.43% |
| satfold-lanes4-idx | 9027ns | 8885ns | 8592ns | 8944ns | 9711ns | -41.10% |
| satfold-lanes64 | 14721ns | 14706ns | 14403ns | 14686ns | 15143ns | -3.95% |
| satfold-neon | 970ns | 951ns | 932ns | 956ns | 1052ns | -93.67% |
| satfold-neon8 | 997ns | 936ns | 928ns | 982ns | 1114ns | -93.49% |
| satfold-nolaw | 15045ns | 14700ns | 14552ns | 14790ns | 16304ns | -1.83% |
| satfold-seq | 15236ns | 15190ns | 14673ns | 15172ns | 15987ns | -0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 15257ns | 14581ns | 16296ns | base | 2.148 |
| satfold-lanes16 | 9253ns | 8946ns | 9919ns | -39.35% | 3.541 |
| satfold-lanes16-constl | 9371ns | 8967ns | 10356ns | -38.58% | 3.497 |
| satfold-lanes4-idx | 8956ns | 8526ns | 9635ns | -41.30% | 3.659 |
| satfold-lanes64 | 14655ns | 14341ns | 15069ns | -3.94% | 2.236 |
| satfold-neon | 906ns | 870ns | 984ns | -94.06% | 36.154 |
| satfold-neon8 | 931ns | 867ns | 1041ns | -93.90% | 35.181 |
| satfold-nolaw | 14981ns | 14494ns | 16236ns | -1.80% | 2.187 |
| satfold-seq | 15166ns | 14606ns | 15897ns | -0.60% | 2.161 |

## Performance model

- Peak throughput: **37.791 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.166 | 5.7% |
| satfold-lanes16 | 3.600 | 9.5% |
| satfold-lanes16-constl | 3.626 | 9.6% |
| satfold-lanes4-idx | 3.717 | 9.8% |
| satfold-lanes64 | 2.238 | 5.9% |
| satfold-neon | 36.965 | 97.8% |
| satfold-neon8 | 37.522 | 99.3% |
| satfold-nolaw | 2.239 | 5.9% |
| satfold-seq | 2.166 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 15325ns | 15325ns | base |
| satfold-lanes16 | 9321ns | 9321ns | -39.18% |
| satfold-lanes16-constl | 9435ns | 9435ns | -38.43% |
| satfold-lanes4-idx | 9027ns | 9027ns | -41.10% |
| satfold-lanes64 | 14721ns | 14721ns | -3.95% |
| satfold-neon | 970ns | 970ns | -93.67% |
| satfold-neon8 | 997ns | 997ns | -93.49% |
| satfold-nolaw | 15045ns | 15045ns | -1.83% |
| satfold-seq | 15236ns | 15236ns | -0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 15125ns | base | --- | [15069, 15164] | --- | --- | --- | --- |
| satfold-lanes16 | 9103ns | -5828.6ns (-38.5%) | [-6162, -5672]ns | [8982, 9287] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 9037ns | -6006.4ns (-39.7%) | [-6117, -5681]ns | [8993, 9307] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 8815ns | -6273.9ns (-41.5%) | [-6603, -6136]ns | [8582, 8892] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 14639ns | -489.4ns (-3.2%) | [-799, -408]ns | [14576, 14687] | YES | 0.0025 | 0.0022 | 0 |
| satfold-neon | 886ns | -14225.6ns (-94.1%) | [-14262, -14172]ns | [874, 903] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 873ns | -14173.3ns (-93.7%) | [-14294, -14065]ns | [870, 967] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 14636ns | -190.4ns (-1.3%) | [-483, -68]ns | [14596, 14790] | YES | 0.0009 | 0.0007 | 0 |
| satfold-seq | 15128ns | no significant difference | [-506, +166]ns | [15067, 15170] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 15191ns | -41.0% | -40.9% | -43.8% | -2.2% | -94.0% | -94.3% | -3.9% | -0.5% |
| 2 | 15143ns | -40.6% | -40.8% | -43.8% | -3.0% | -94.1% | -94.3% | -3.4% | +5.3% |
| 3 | 15183ns | -40.9% | -40.9% | -43.9% | -4.8% | -94.1% | -94.3% | -3.5% | +0.3% |
| 4 | 15169ns | -40.9% | -40.8% | -43.9% | -5.2% | -94.0% | -94.3% | -3.2% | -0.3% |
| 5 | 15151ns | -40.8% | -40.6% | -43.7% | -4.6% | -94.0% | -94.3% | -3.7% | -3.0% |
| 6 | 15572ns | -42.5% | -41.9% | -45.1% | -7.4% | -94.2% | -94.4% | -6.1% | -6.2% |
| 7 | 15670ns | -42.6% | -42.5% | -45.2% | -6.2% | -94.2% | -94.4% | -6.9% | -6.6% |
| 8 | 15139ns | -40.8% | -40.5% | -43.6% | -5.5% | -94.1% | -94.3% | -3.6% | -3.5% |
| 9 | 15156ns | -38.7% | -40.7% | -43.8% | -5.2% | -94.0% | -94.3% | -3.2% | -3.3% |
| 10 | 15202ns | -40.1% | -40.9% | -43.5% | -5.4% | -94.1% | -94.3% | -3.3% | -3.7% |
| 11 | 16342ns | -35.8% | -34.7% | -37.8% | -9.3% | -94.1% | -94.1% | +4.3% | -7.2% |
| 12 | 16426ns | -44.7% | -35.1% | -39.9% | -10.6% | -94.1% | -94.1% | -1.1% | -8.3% |
| 13 | 16372ns | -44.4% | -31.9% | -42.0% | -10.4% | -94.1% | -94.1% | +1.8% | -7.7% |
| 14 | 16316ns | -43.4% | -36.3% | -41.7% | -10.0% | -94.1% | -94.1% | -0.2% | -7.7% |
| 15 | 16318ns | -44.8% | -38.5% | -41.8% | -10.7% | -94.1% | -94.1% | -0.5% | -7.7% |
| 16 | 16399ns | -45.2% | -39.2% | -41.9% | -12.5% | -94.1% | -94.1% | -1.0% | -6.1% |
| 17 | 16017ns | -44.9% | -37.8% | -40.7% | -7.6% | -94.0% | -94.0% | -1.6% | -5.1% |
| 18 | 15160ns | -36.7% | -34.3% | -37.4% | -5.6% | -92.8% | -93.6% | +1.6% | -3.6% |
| 19 | 15121ns | -33.2% | -34.1% | -37.2% | -5.3% | -93.6% | -93.6% | +0.5% | -3.9% |
| 20 | 15069ns | -33.3% | -33.6% | -36.9% | -5.0% | -93.6% | -93.6% | -0.0% | -3.4% |
| 21 | 16176ns | -42.6% | -42.5% | -41.3% | -10.1% | -94.6% | -93.5% | -11.5% | +0.4% |
| 22 | 15064ns | -38.3% | -38.8% | -36.9% | -3.2% | -94.2% | -93.1% | -4.9% | +7.8% |
| 23 | 15091ns | -38.4% | -40.4% | -37.0% | -3.3% | -94.2% | -93.1% | -3.9% | +7.8% |
| 24 | 15129ns | -28.8% | -40.1% | -41.0% | -2.9% | -94.2% | -93.2% | -1.3% | +7.3% |
| 25 | 15083ns | -36.8% | -39.6% | -41.6% | -2.8% | -94.2% | -93.1% | -3.3% | +0.4% |
| 26 | 15069ns | -37.2% | -40.3% | -41.5% | -3.1% | -94.2% | -93.1% | -1.7% | -0.0% |
| 27 | 15066ns | -38.3% | -40.2% | -41.3% | -3.0% | -94.2% | -93.1% | +0.1% | -0.0% |
| 28 | 15090ns | -38.4% | -40.5% | -41.6% | -2.6% | -94.2% | -93.3% | -3.2% | -0.2% |
| 29 | 14945ns | -37.9% | -39.6% | -40.9% | +0.4% | -94.2% | -94.2% | -2.0% | +0.8% |
| 30 | 14548ns | -36.2% | -38.2% | -39.4% | +1.0% | -94.0% | -94.0% | -0.1% | +3.7% |
| 31 | 14590ns | -36.5% | -38.5% | -39.4% | +3.1% | -94.0% | -94.0% | -0.3% | +3.8% |
| 32 | 14619ns | -38.7% | -38.8% | -39.5% | +6.2% | -94.1% | -94.1% | -0.5% | +3.8% |
| 33 | 14547ns | -35.6% | -38.3% | -39.8% | +4.2% | -94.0% | -92.7% | +1.5% | +4.3% |
| 34 | 15105ns | -40.6% | -40.6% | -43.2% | +0.2% | -94.2% | -94.2% | -0.7% | +0.2% |
| 35 | 14654ns | -38.8% | -38.8% | -41.6% | +0.2% | -94.0% | -94.1% | -0.5% | +6.3% |
| 36 | 14723ns | -39.1% | -36.8% | -41.8% | -1.0% | -94.0% | -94.1% | -0.8% | +2.8% |
| 37 | 14571ns | -36.2% | -35.9% | -41.2% | +2.9% | -94.0% | -94.0% | +0.1% | +4.8% |
| 38 | 14562ns | -38.4% | -35.9% | -41.3% | +0.8% | -94.0% | -94.0% | +0.2% | +4.1% |
| 39 | 14965ns | -40.1% | -37.8% | -42.8% | -2.3% | -94.2% | -94.2% | -2.5% | +1.4% |
| 40 | 14560ns | -38.4% | -36.1% | -41.0% | +0.2% | -94.0% | -94.0% | +0.2% | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.753 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.281 | moderate+ |
| satfold-lanes16-constl | 0.824 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.802 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.570 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.733 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.709 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.767 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.600 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 30/40, lost 10/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 29/40, lost 8/40
- **satfold-seq**: won 19/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.7ns | 15256.8ns | 0.0% |  |
| satfold-lanes16 | 3.0ns | 9252.9ns | 0.0% |  |
| satfold-lanes16-constl | 2.5ns | 9371.5ns | 0.0% |  |
| satfold-lanes4-idx | 3.0ns | 8955.9ns | 0.0% |  |
| satfold-lanes64 | 2.9ns | 14655.3ns | 0.0% |  |
| satfold-neon | 2.2ns | 906.3ns | 0.2% |  |
| satfold-neon8 | 2.3ns | 931.4ns | 0.3% |  |
| satfold-nolaw | 2.1ns | 14981.5ns | 0.0% |  |
| satfold-seq | 2.6ns | 15165.5ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14581.4-16295.8 ns)
  14581.4 |#############
  14667.1 |####
  14752.8 |
  14838.5 |
  14924.2 |########
  15010.0 |###############################
  15095.7 |########################################
  15181.4 |#############
  15267.1 |
  15352.9 |
  15438.6 |
  15524.3 |####
  15610.0 |####
  15695.8 |
  15781.5 |
  15867.2 |
  15952.9 |####
  16038.7 |
  16124.4 |####
  16210.1 |
  (5 below, 6 above range)

satfold-lanes16 (n=40, range 8946.1-9919.3 ns)
   8946.1 |########################################
   8994.8 |##
   9043.4 |##
   9092.1 |#####
   9140.7 |
   9189.4 |##
   9238.1 |##########
   9286.7 |###############
   9335.4 |##
   9384.1 |
   9432.7 |##
   9481.4 |##
   9530.0 |
   9578.7 |##
   9627.4 |
   9676.0 |
   9724.7 |
   9773.3 |
   9822.0 |
   9870.7 |
  (1 below, 4 above range)

satfold-lanes16-constl (n=40, range 8967.1-10356.3 ns)
   8967.1 |########################################
   9036.6 |#####
   9106.0 |##
   9175.5 |##
   9244.9 |##########
   9314.4 |#####
   9383.9 |
   9453.3 |
   9522.8 |
   9592.2 |
   9661.7 |
   9731.2 |
   9800.6 |
   9870.1 |
   9939.5 |############
  10009.0 |##
  10078.5 |
  10147.9 |
  10217.4 |
  10286.8 |
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 8525.5-9634.8 ns)
   8525.5 |################################
   8581.0 |################
   8636.5 |
   8691.9 |
   8747.4 |####
   8802.8 |################################
   8858.3 |
   8913.8 |####
   8969.2 |
   9024.7 |
   9080.2 |
   9135.6 |
   9191.1 |
   9246.5 |
   9302.0 |
   9357.5 |
   9412.9 |
   9468.4 |########################################
   9523.9 |####
   9579.3 |
  (5 below, 2 above range)

satfold-lanes64 (n=40, range 14340.8-15068.8 ns)
  14340.8 |################
  14377.2 |
  14413.6 |####
  14450.0 |########
  14486.4 |
  14522.8 |####
  14559.2 |################
  14595.6 |################
  14632.0 |####
  14668.4 |########################################
  14704.8 |
  14741.2 |
  14777.6 |####
  14814.0 |####
  14850.4 |####
  14886.8 |
  14923.2 |
  14959.6 |
  14996.0 |########
  15032.4 |####
  (4 below, 3 above range)

satfold-neon (n=40, range 870.1-983.6 ns)
    870.1 |########################################
    875.8 |##
    881.5 |
    887.2 |
    892.8 |##########
    898.5 |##########
    904.2 |#####
    909.9 |
    915.5 |
    921.2 |
    926.9 |
    932.5 |
    938.2 |
    943.9 |
    949.6 |
    955.2 |
    960.9 |##########
    966.6 |############
    972.2 |
    977.9 |
  (3 below, 1 above range)

satfold-neon8 (n=40, range 867.1-1040.7 ns)
    867.1 |########################################
    875.8 |
    884.4 |
    893.1 |
    901.8 |
    910.5 |
    919.2 |
    927.8 |
    936.5 |
    945.2 |
    953.9 |
    962.6 |######################
    971.2 |
    979.9 |
    988.6 |
    997.3 |
   1006.0 |##
   1014.6 |
   1023.3 |
   1032.0 |###########
  (3 below, 3 above range)

satfold-nolaw (n=40, range 14493.7-16236.2 ns)
  14493.7 |#########
  14580.8 |########################################
  14667.9 |####
  14755.1 |####
  14842.2 |
  14929.3 |####
  15016.4 |####
  15103.6 |
  15190.7 |##
  15277.8 |
  15364.9 |##
  15452.1 |
  15539.2 |
  15626.3 |
  15713.4 |##
  15800.6 |
  15887.7 |
  15974.8 |
  16061.9 |
  16149.1 |
  (2 below, 6 above range)

satfold-seq (n=40, range 14606.4-15897.2 ns)
  14606.4 |##################
  14670.9 |###
  14735.5 |
  14800.0 |
  14864.6 |
  14929.1 |
  14993.6 |
  15058.2 |########################################
  15122.7 |####################################
  15187.3 |#######
  15251.8 |###
  15316.3 |
  15380.9 |###
  15445.4 |
  15510.0 |###
  15574.5 |
  15639.0 |
  15703.6 |
  15768.1 |
  15832.7 |
  (3 below, 5 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.60 (measurement drift or warm-up artifact)
