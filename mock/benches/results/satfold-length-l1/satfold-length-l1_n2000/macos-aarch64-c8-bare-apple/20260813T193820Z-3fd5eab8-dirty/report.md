# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx dominates: 26% faster than the next best (satfold-neon)

satfold-lanes4-idx (8.02 us) leads satfold-neon (10.12 us) by 26%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes4-idx beats baseline by 21% (significant)

satfold-lanes4-idx is -2.12 us (21%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-nolaw shows warm-up / thermal drift (autocorr +0.75)

satfold-nolaw's per-pass series has lag-1 autocorrelation +0.75, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes4-idx} vs {satfold-neon, satfold-iterfold, satfold-nolaw, satfold-seq, satfold-lanes64, satfold-lanes16, satfold-lanes16-constl, satfold-neon8} (26% apart)

The field splits into a fast tier {satfold-lanes4-idx} and a slow tier {satfold-neon, satfold-iterfold, satfold-nolaw, satfold-seq, satfold-lanes64, satfold-lanes16, satfold-lanes16-constl, satfold-neon8} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-lanes16's edge over baseline is significant but tiny (28 ns, 0.28%)

satfold-lanes16 differs from baseline satfold-iterfold by 28 ns (0.28%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes4-idx** at 8023.5 ns median (-21.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.31x (fastest 8023.5 ns, slowest 10492.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 10286ns | 10218ns | 10042ns | 10270ns | 10578ns | base |
| satfold-lanes16 | 10379ns | 10366ns | 10139ns | 10348ns | 10711ns | +0.90% |
| satfold-lanes16-constl | 10420ns | 10419ns | 10064ns | 10348ns | 10995ns | +1.30% |
| satfold-lanes4-idx | 8152ns | 8097ns | 7777ns | 8070ns | 8775ns | -20.74% |
| satfold-lanes64 | 10334ns | 10321ns | 10064ns | 10316ns | 10659ns | +0.47% |
| satfold-neon | 10232ns | 10190ns | 10063ns | 10194ns | 10516ns | -0.52% |
| satfold-neon8 | 10655ns | 10559ns | 10191ns | 10593ns | 11307ns | +3.59% |
| satfold-nolaw | 10351ns | 10220ns | 10108ns | 10319ns | 10689ns | +0.63% |
| satfold-seq | 10378ns | 10303ns | 10004ns | 10347ns | 10847ns | +0.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 10221ns | 9983ns | 10510ns | base | 3.206 |
| satfold-lanes16 | 10313ns | 10078ns | 10633ns | +0.90% | 3.177 |
| satfold-lanes16-constl | 10352ns | 10004ns | 10906ns | +1.29% | 3.165 |
| satfold-lanes4-idx | 8077ns | 7711ns | 8689ns | -20.98% | 4.057 |
| satfold-lanes64 | 10270ns | 10005ns | 10592ns | +0.49% | 3.191 |
| satfold-neon | 10169ns | 10003ns | 10451ns | -0.51% | 3.222 |
| satfold-neon8 | 10586ns | 10129ns | 11229ns | +3.58% | 3.095 |
| satfold-nolaw | 10288ns | 10049ns | 10618ns | +0.66% | 3.185 |
| satfold-seq | 10316ns | 9941ns | 10778ns | +0.93% | 3.177 |

## Performance model

- Peak throughput: **4.249 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 3.228 | 76.0% |
| satfold-lanes16 | 3.180 | 74.8% |
| satfold-lanes16-constl | 3.163 | 74.4% |
| satfold-lanes4-idx | 4.084 | 96.1% |
| satfold-lanes64 | 3.195 | 75.2% |
| satfold-neon | 3.237 | 76.2% |
| satfold-neon8 | 3.123 | 73.5% |
| satfold-nolaw | 3.226 | 75.9% |
| satfold-seq | 3.200 | 75.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 10286ns | 10286ns | base |
| satfold-lanes16 | 10379ns | 10379ns | +0.90% |
| satfold-lanes16-constl | 10420ns | 10420ns | +1.30% |
| satfold-lanes4-idx | 8152ns | 8152ns | -20.74% |
| satfold-lanes64 | 10334ns | 10334ns | +0.47% |
| satfold-neon | 10232ns | 10232ns | -0.52% |
| satfold-neon8 | 10655ns | 10655ns | +3.59% |
| satfold-nolaw | 10351ns | 10351ns | +0.63% |
| satfold-seq | 10378ns | 10378ns | +0.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 10152ns | base | --- | [10109, 10300] | --- | --- | --- | --- |
| satfold-lanes16 | 10304ns | no significant difference | [-23, +175]ns | [10147, 10415] | no | 0.5728 | 0.4296 | 0 |
| satfold-lanes16-constl | 10360ns | no significant difference | [-77, +226]ns | [10134, 10392] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes4-idx | 8024ns | -2204.6ns (-21.7%) | [-2315, -2105]ns | [7963, 8039] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 10255ns | no significant difference | [-22, +111]ns | [10098, 10373] | no | 0.8746 | 0.8746 | 0 |
| satfold-neon | 10123ns | no significant difference | [-42, +7]ns | [10102, 10138] | no | 0.5728 | 0.4296 | 0 |
| satfold-neon8 | 10492ns | +293.4ns (+2.9%) | [+135, +369]ns | [10464, 10563] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 10159ns | +65.3ns (+0.6%) | [+22, +178]ns | [10131, 10439] | YES | 0.0171 | 0.0064 | 0 |
| satfold-seq | 10241ns | no significant difference | [-16, +99]ns | [10123, 10455] | no | 0.3077 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 10448ns | -1.3% | -3.8% | -25.9% | -3.5% | +0.1% | +1.5% | +0.3% | +0.1% |
| 2 | 10450ns | -0.3% | -3.9% | -26.0% | -3.3% | -0.0% | +1.9% | -1.0% | -0.2% |
| 3 | 10303ns | +1.1% | -2.6% | -22.2% | -2.1% | +1.4% | +0.6% | +0.6% | +1.3% |
| 4 | 10082ns | +3.3% | +0.9% | -20.5% | +0.2% | +5.2% | +4.0% | +3.9% | +0.7% |
| 5 | 10190ns | +4.1% | -1.4% | -21.2% | -1.0% | +3.7% | +5.1% | +0.1% | -1.0% |
| 6 | 10179ns | +2.7% | -0.9% | -21.0% | -0.9% | +1.1% | +3.1% | -0.7% | -0.3% |
| 7 | 10112ns | +0.6% | +1.3% | -20.6% | -0.5% | -0.1% | +3.6% | +0.3% | +0.3% |
| 8 | 10428ns | -2.9% | -0.1% | -22.4% | -0.2% | -2.8% | +1.1% | -4.3% | -1.6% |
| 9 | 10449ns | -3.5% | -0.3% | -23.2% | -0.1% | -3.5% | +0.1% | -3.9% | +0.1% |
| 10 | 10136ns | +0.2% | +15.0% | -20.9% | +3.0% | -0.5% | +0.9% | -2.4% | +3.2% |
| 11 | 10083ns | +2.8% | +9.9% | -23.8% | -0.3% | -0.4% | +22.4% | +0.7% | -1.8% |
| 12 | 10092ns | +1.2% | -0.9% | -23.5% | -0.2% | +0.4% | +14.0% | +0.7% | -1.9% |
| 13 | 10892ns | -6.7% | -9.4% | -30.1% | -7.4% | -6.3% | -9.0% | -7.2% | -9.1% |
| 14 | 10105ns | +0.4% | -0.7% | -13.1% | +2.5% | +0.2% | +6.4% | +0.5% | -2.0% |
| 15 | 9897ns | +4.0% | +3.9% | -6.6% | +3.4% | +1.0% | +11.2% | +2.3% | +8.7% |
| 16 | 9960ns | +4.8% | +4.3% | -10.8% | +0.1% | +3.3% | +0.3% | +1.8% | +11.4% |
| 17 | 9906ns | +5.0% | +10.1% | -10.0% | +0.5% | +0.6% | +0.2% | +2.2% | +0.1% |
| 18 | 9959ns | +6.2% | +4.0% | -20.7% | +3.6% | +1.9% | +11.4% | +1.9% | +0.6% |
| 19 | 9896ns | +1.3% | +9.8% | -22.6% | +0.7% | +0.0% | +14.2% | +2.2% | +2.9% |
| 20 | 10341ns | -1.2% | +3.7% | -24.7% | -4.3% | -4.3% | -0.8% | -2.1% | -3.9% |
| 21 | 10096ns | +0.1% | +2.9% | -23.0% | +3.4% | -0.1% | +0.6% | -0.0% | +0.3% |
| 22 | 10297ns | -1.9% | +1.2% | -24.5% | +1.5% | -1.6% | +1.6% | -2.0% | +0.8% |
| 23 | 10120ns | -0.1% | +5.5% | -22.8% | +12.3% | +0.0% | +3.4% | -0.4% | -0.1% |
| 24 | 10095ns | +0.4% | +1.6% | -23.2% | +2.6% | -0.1% | +3.6% | +0.4% | +0.2% |
| 25 | 10093ns | -0.1% | -0.3% | -22.3% | +2.0% | +0.2% | +3.6% | +0.0% | -0.4% |
| 26 | 10225ns | -1.2% | -1.9% | -21.4% | +1.6% | -1.0% | +2.5% | -1.1% | -1.1% |
| 27 | 10095ns | -0.2% | -0.2% | -20.5% | +4.7% | +0.3% | +3.7% | +1.7% | -0.3% |
| 28 | 10168ns | -0.4% | -1.3% | -20.9% | +0.9% | -0.1% | +2.6% | +2.6% | -0.4% |
| 29 | 10113ns | -0.4% | -1.1% | -18.8% | -0.4% | +0.1% | +0.8% | +3.3% | +1.0% |
| 30 | 10414ns | -3.3% | -3.7% | -23.9% | -2.1% | -2.6% | +0.4% | +5.8% | -1.2% |
| 31 | 10118ns | +2.5% | +3.0% | -20.7% | +1.3% | -0.2% | +5.2% | +3.6% | +5.5% |
| 32 | 10120ns | +5.9% | +3.2% | -20.6% | +0.4% | -0.3% | +4.5% | +4.1% | +8.2% |
| 33 | 10275ns | +4.8% | +1.1% | -21.1% | -1.8% | +2.2% | +2.6% | +2.2% | +3.5% |
| 34 | 10454ns | +2.1% | +1.3% | -21.9% | -0.2% | -2.4% | +0.9% | +0.2% | +2.0% |
| 35 | 10441ns | +2.4% | +2.9% | -16.3% | -0.1% | -3.2% | +4.8% | +0.4% | +3.4% |
| 36 | 10481ns | -0.2% | -1.2% | -18.9% | +0.8% | -3.3% | +3.9% | +2.7% | +1.5% |
| 37 | 10448ns | -0.3% | -0.7% | -22.4% | -0.0% | -2.1% | +0.7% | +1.5% | +0.9% |
| 38 | 10459ns | -0.4% | -0.9% | -21.2% | -0.1% | -3.3% | +0.1% | +0.9% | +0.2% |
| 39 | 10319ns | +1.2% | +0.5% | -22.5% | +1.2% | -2.5% | +3.4% | +1.4% | +1.3% |
| 40 | 10081ns | +4.9% | +2.9% | -20.5% | +2.6% | -0.1% | +4.2% | +4.0% | +5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.292 | moderate+ |
| satfold-lanes16 | 0.706 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.489 | moderate+ |
| satfold-lanes4-idx | 0.634 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.364 | moderate+ |
| satfold-neon | 0.579 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.106 | ok |
| satfold-nolaw | 0.746 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.531 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 16/40, lost 22/40
- **satfold-lanes16-constl**: won 18/40, lost 21/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 17/40, lost 20/40
- **satfold-neon**: won 18/40, lost 13/40
- **satfold-neon8**: won 2/40, lost 37/40
- **satfold-nolaw**: won 10/40, lost 27/40
- **satfold-seq**: won 14/40, lost 23/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.7ns | 10220.6ns | 0.0% |  |
| satfold-lanes16 | 3.4ns | 10312.5ns | 0.0% |  |
| satfold-lanes16-constl | 3.2ns | 10352.1ns | 0.0% |  |
| satfold-lanes4-idx | 3.6ns | 8076.6ns | 0.0% |  |
| satfold-lanes64 | 3.7ns | 10270.4ns | 0.0% |  |
| satfold-neon | 2.1ns | 10168.7ns | 0.0% |  |
| satfold-neon8 | 2.3ns | 10586.2ns | 0.0% |  |
| satfold-nolaw | 2.4ns | 10287.5ns | 0.0% |  |
| satfold-seq | 2.2ns | 10315.6ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 9983.1-10510.1 ns)
   9983.1 |
  10009.4 |
  10035.8 |
  10062.1 |###############
  10088.5 |########################################
  10114.8 |####################
  10141.2 |
  10167.5 |###############
  10193.9 |
  10220.2 |#####
  10246.6 |
  10273.0 |##########
  10299.3 |##########
  10325.7 |#####
  10352.0 |
  10378.4 |
  10404.7 |##########
  10431.1 |##############################
  10457.4 |##########
  10483.8 |
  (5 below, 1 above range)

satfold-lanes16 (n=40, range 10077.6-10633.4 ns)
  10077.6 |##########################
  10105.4 |##########################
  10133.1 |####################
  10160.9 |#############
  10188.7 |######
  10216.5 |######
  10244.3 |
  10272.1 |######
  10299.9 |######
  10327.7 |
  10355.5 |#############
  10383.3 |######
  10411.0 |########################################
  10438.8 |####################
  10466.6 |
  10494.4 |
  10522.2 |
  10550.0 |######
  10577.8 |######
  10605.6 |######
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 10003.6-10906.0 ns)
  10003.6 |########################################
  10048.8 |###############
  10093.9 |
  10139.0 |#####
  10184.1 |
  10229.2 |##########
  10274.4 |#####
  10319.5 |###############
  10364.6 |##############################
  10409.7 |#########################
  10454.8 |
  10500.0 |
  10545.1 |
  10590.2 |#####
  10635.3 |#####
  10680.4 |#####
  10725.6 |#####
  10770.7 |
  10815.8 |
  10860.9 |#####
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 7711.4-8688.9 ns)
   7711.4 |############
   7760.3 |#########
   7809.2 |######
   7858.0 |###
   7906.9 |###
   7955.8 |###
   8004.7 |########################################
   8053.5 |###
   8102.4 |######
   8151.3 |###
   8200.1 |######
   8249.0 |
   8297.9 |
   8346.8 |
   8395.6 |
   8444.5 |
   8493.4 |###
   8542.2 |
   8591.1 |
   8640.0 |
  (3 below, 5 above range)

satfold-lanes64 (n=40, range 10004.6-10591.9 ns)
  10004.6 |
  10034.0 |##########
  10063.3 |########################################
  10092.7 |##########
  10122.1 |
  10151.4 |#####
  10180.8 |#####
  10210.2 |#####
  10239.5 |##########
  10268.9 |
  10298.2 |##########
  10327.6 |#####
  10357.0 |##########
  10386.3 |##########
  10415.7 |###################################
  10445.1 |##########
  10474.4 |
  10503.8 |
  10533.1 |
  10562.5 |##########
  (4 below, 1 above range)

satfold-neon (n=40, range 10003.4-10450.6 ns)
  10003.4 |
  10025.8 |#####
  10048.1 |#####
  10070.5 |##############################
  10092.8 |#########################
  10115.2 |########################################
  10137.5 |###############
  10159.9 |#####
  10182.3 |#####
  10204.6 |##########
  10227.0 |
  10249.3 |
  10271.7 |#####
  10294.1 |#####
  10316.4 |
  10338.8 |
  10361.1 |
  10383.5 |
  10405.8 |
  10428.2 |##########
  (4 below, 4 above range)

satfold-neon8 (n=40, range 10129.4-11228.6 ns)
  10129.4 |###
  10184.3 |######
  10239.3 |###
  10294.3 |
  10349.2 |###
  10404.2 |######
  10459.2 |########################################
  10514.1 |#############
  10569.1 |######
  10624.0 |##########
  10679.0 |###
  10734.0 |###
  10788.9 |
  10843.9 |###
  10898.8 |###
  10953.8 |###
  11008.8 |
  11063.7 |###
  11118.7 |
  11173.6 |
  (3 below, 3 above range)

satfold-nolaw (n=40, range 10049.0-10617.8 ns)
  10049.0 |
  10077.4 |######################
  10105.9 |########################################
  10134.3 |##################################
  10162.8 |#####
  10191.2 |#####
  10219.6 |
  10248.1 |#####
  10276.5 |
  10305.0 |
  10333.4 |#####
  10361.9 |#####
  10390.3 |
  10418.7 |###########
  10447.2 |#################
  10475.6 |############################
  10504.1 |
  10532.5 |###########
  10561.0 |
  10589.4 |#####
  (3 below, 2 above range)

satfold-seq (n=40, range 9940.7-10778.2 ns)
   9940.7 |#####
   9982.5 |#####
  10024.4 |#####
  10066.3 |###########
  10108.2 |########################################
  10150.0 |###########
  10191.9 |#####
  10233.8 |#####
  10275.7 |#####
  10317.5 |
  10359.4 |#####
  10401.3 |###########
  10443.2 |############################
  10485.1 |
  10526.9 |#####
  10568.8 |
  10610.7 |#################
  10652.6 |###########
  10694.4 |
  10736.3 |#####
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.53 (measurement drift or warm-up artifact)
