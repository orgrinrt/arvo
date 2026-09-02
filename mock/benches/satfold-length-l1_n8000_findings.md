# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 99% (significant)

satfold-neon is -26.87 us (99%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 52.4x slower than the field

satfold-seq (28.28 us) is 52.4x the fastest (539 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon8, satfold-neon) are a dead heat (<1%)

satfold-neon8 (539 ns) and satfold-neon (542 ns) differ by 0.49%, inside the noise, even though the wider field spreads 5144.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-neon shows warm-up / thermal drift (autocorr +0.90)

satfold-neon's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon} vs {satfold-lanes4-idx, satfold-lanes16-constl, satfold-lanes16, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} (270% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon} and a slow tier {satfold-lanes4-idx, satfold-lanes16-constl, satfold-lanes16, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} with a 270% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 52.4x the fastest

Fastest satfold-neon8 (539 ns) to slowest satfold-seq (28.28 us): 52.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon8** at 539.2 ns median (-98.0% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 52.44x (fastest 539.2 ns, slowest 28276.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 27337ns | 27256ns | 26662ns | 27298ns | 28132ns | base |
| satfold-lanes16 | 4660ns | 4608ns | 4347ns | 4665ns | 4957ns | -82.95% |
| satfold-lanes16-constl | 4596ns | 4524ns | 4356ns | 4525ns | 5049ns | -83.19% |
| satfold-lanes4-idx | 2134ns | 2067ns | 2033ns | 2095ns | 2353ns | -92.19% |
| satfold-lanes64 | 7730ns | 7695ns | 7668ns | 7706ns | 7866ns | -71.72% |
| satfold-neon | 605ns | 618ns | 587ns | 606ns | 622ns | -97.79% |
| satfold-neon8 | 616ns | 600ns | 590ns | 606ns | 670ns | -97.75% |
| satfold-nolaw | 21980ns | 21923ns | 21606ns | 21904ns | 22581ns | -19.60% |
| satfold-seq | 28501ns | 28355ns | 27752ns | 28383ns | 29606ns | +4.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 27233ns | 26579ns | 28005ns | base | 1.203 |
| satfold-lanes16 | 4593ns | 4287ns | 4884ns | -83.13% | 7.134 |
| satfold-lanes16-constl | 4529ns | 4296ns | 4969ns | -83.37% | 7.235 |
| satfold-lanes4-idx | 2069ns | 1974ns | 2279ns | -92.40% | 15.837 |
| satfold-lanes64 | 7667ns | 7613ns | 7796ns | -71.85% | 4.274 |
| satfold-neon | 543ns | 527ns | 558ns | -98.01% | 60.388 |
| satfold-neon8 | 553ns | 528ns | 602ns | -97.97% | 59.303 |
| satfold-nolaw | 21904ns | 21544ns | 22486ns | -19.57% | 1.496 |
| satfold-seq | 28415ns | 27666ns | 29510ns | +4.34% | 1.153 |

## Performance model

- Peak throughput: **62.183 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.206 | 1.9% |
| satfold-lanes16 | 7.229 | 11.6% |
| satfold-lanes16-constl | 7.351 | 11.8% |
| satfold-lanes4-idx | 16.353 | 26.3% |
| satfold-lanes64 | 4.293 | 6.9% |
| satfold-neon | 60.474 | 97.3% |
| satfold-neon8 | 60.772 | 97.7% |
| satfold-nolaw | 1.500 | 2.4% |
| satfold-seq | 1.159 | 1.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 27337ns | 27337ns | base |
| satfold-lanes16 | 4660ns | 4660ns | -82.95% |
| satfold-lanes16-constl | 4596ns | 4596ns | -83.19% |
| satfold-lanes4-idx | 2134ns | 2134ns | -92.19% |
| satfold-lanes64 | 7730ns | 7730ns | -71.72% |
| satfold-neon | 605ns | 605ns | -97.79% |
| satfold-neon8 | 616ns | 616ns | -97.75% |
| satfold-nolaw | 21980ns | 21980ns | -19.60% |
| satfold-seq | 28501ns | 28501ns | +4.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 27175ns | base | --- | [27022, 27425] | --- | --- | --- | --- |
| satfold-lanes16 | 4533ns | -22597.8ns (-83.2%) | [-22748, -22345]ns | [4511, 4722] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 4458ns | -22576.4ns (-83.1%) | [-22949, -22462]ns | [4318, 4521] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2004ns | -25092.9ns (-92.3%) | [-25371, -24909]ns | [1979, 2058] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 7633ns | -19501.5ns (-71.8%) | [-19757, -19343]ns | [7617, 7654] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 542ns | -26631.7ns (-98.0%) | [-26885, -26466]ns | [530, 556] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 539ns | -26605.0ns (-97.9%) | [-26863, -26460]ns | [537, 541] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 21845ns | -5317.1ns (-19.6%) | [-5371, -5095]ns | [21703, 21965] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 28276ns | +1163.6ns (+4.3%) | [+841, +1360]ns | [28242, 28316] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 27074ns | -82.0% | -83.3% | -92.7% | -71.9% | -97.9% | -97.8% | -20.5% | +15.5% |
| 2 | 27476ns | -82.3% | -83.5% | -92.8% | -72.0% | -98.0% | -97.8% | -21.5% | +8.4% |
| 3 | 26552ns | -81.7% | -83.0% | -92.6% | -70.8% | -97.9% | -97.7% | -18.7% | +12.0% |
| 4 | 26569ns | -81.7% | -83.0% | -92.6% | -70.9% | -97.9% | -97.7% | -18.3% | +7.7% |
| 5 | 26572ns | -81.7% | -83.0% | -92.6% | -71.1% | -97.9% | -97.7% | -17.4% | +6.6% |
| 6 | 27190ns | -82.1% | -83.4% | -92.7% | -71.8% | -97.9% | -97.8% | -19.6% | +4.0% |
| 7 | 27048ns | -82.0% | -83.3% | -92.7% | -71.8% | -97.9% | -97.8% | -19.2% | +4.5% |
| 8 | 27540ns | -82.3% | -83.6% | -92.8% | -72.3% | -98.0% | -97.8% | -21.3% | +2.8% |
| 9 | 27096ns | -82.0% | -83.3% | -92.5% | -71.9% | -97.9% | -97.8% | -19.7% | +4.8% |
| 10 | 27616ns | -82.9% | -83.6% | -92.7% | -72.4% | -98.0% | -97.8% | -22.1% | +2.4% |
| 11 | 27368ns | -82.0% | -82.0% | -91.7% | -72.2% | -98.0% | -98.0% | -19.6% | +6.6% |
| 12 | 27673ns | -82.2% | -82.1% | -91.8% | -72.5% | -98.0% | -98.1% | -19.4% | +0.0% |
| 13 | 27020ns | -82.0% | -81.7% | -91.6% | -71.8% | -97.9% | -98.0% | -18.0% | +4.3% |
| 14 | 28144ns | -83.7% | -82.3% | -91.9% | -72.9% | -98.0% | -98.1% | -21.0% | +2.2% |
| 15 | 27944ns | -83.0% | -82.3% | -91.9% | -72.7% | -98.0% | -98.1% | -18.3% | +4.8% |
| 16 | 28106ns | -83.5% | -82.2% | -92.0% | -71.6% | -98.0% | -98.0% | -19.1% | +0.6% |
| 17 | 28215ns | -83.9% | -82.3% | -92.1% | -72.4% | -98.0% | -98.1% | -18.8% | +0.6% |
| 18 | 27360ns | -83.2% | -81.8% | -91.7% | -71.2% | -98.0% | -98.0% | -19.7% | +3.4% |
| 19 | 27398ns | -83.5% | -82.0% | -91.8% | -71.8% | -98.0% | -98.0% | -18.2% | +3.2% |
| 20 | 27024ns | -83.3% | -81.7% | -91.4% | -71.3% | -97.9% | -98.0% | -18.4% | +4.4% |
| 21 | 26597ns | -83.0% | -83.8% | -92.3% | -71.4% | -98.0% | -98.0% | -16.6% | +5.2% |
| 22 | 26586ns | -82.2% | -83.8% | -92.5% | -71.4% | -98.0% | -98.0% | -18.8% | +3.0% |
| 23 | 26761ns | -82.7% | -83.9% | -92.3% | -71.6% | -98.0% | -98.0% | -19.2% | +5.3% |
| 24 | 26845ns | -83.2% | -84.0% | -92.3% | -71.6% | -98.0% | -98.0% | -19.1% | +5.3% |
| 25 | 27160ns | -83.4% | -84.2% | -92.4% | -71.9% | -98.1% | -98.0% | -18.0% | +7.1% |
| 26 | 27866ns | -83.8% | -84.6% | -92.5% | -72.6% | -98.1% | -98.1% | -21.9% | +3.8% |
| 27 | 27667ns | -83.7% | -84.5% | -92.6% | -72.4% | -98.1% | -98.1% | -20.9% | +1.9% |
| 28 | 27376ns | -83.5% | -84.3% | -92.6% | -72.2% | -98.1% | -98.0% | -19.5% | +0.2% |
| 29 | 27101ns | -83.3% | -84.1% | -92.5% | -71.8% | -98.0% | -98.0% | -19.9% | +1.9% |
| 30 | 27241ns | -83.5% | -84.2% | -92.6% | -71.9% | -98.1% | -98.0% | -19.6% | +1.5% |
| 31 | 27453ns | -84.4% | -84.0% | -92.8% | -72.2% | -98.1% | -98.1% | -19.9% | +0.9% |
| 32 | 26647ns | -83.9% | -83.6% | -92.6% | -71.2% | -98.0% | -98.0% | -19.0% | +5.1% |
| 33 | 28423ns | -84.9% | -84.5% | -93.0% | -72.9% | -98.1% | -98.1% | -24.2% | -0.5% |
| 34 | 27456ns | -84.4% | -84.3% | -92.8% | -71.7% | -98.1% | -98.1% | -21.2% | +2.9% |
| 35 | 26887ns | -83.8% | -84.0% | -92.6% | -71.7% | -98.0% | -98.0% | -18.5% | +4.0% |
| 36 | 26520ns | -83.8% | -83.6% | -92.6% | -71.3% | -98.0% | -98.0% | -18.9% | +6.7% |
| 37 | 26703ns | -83.9% | -83.9% | -92.5% | -71.5% | -98.0% | -98.0% | -19.0% | +4.4% |
| 38 | 26896ns | -84.1% | -83.9% | -92.6% | -71.7% | -98.0% | -98.0% | -19.1% | +6.5% |
| 39 | 26588ns | -83.9% | -83.8% | -92.6% | -71.2% | -98.0% | -98.0% | -17.8% | +6.5% |
| 40 | 27563ns | -84.4% | -84.4% | -92.8% | -72.4% | -98.1% | -98.1% | -21.9% | +4.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.399 | moderate+ |
| satfold-lanes16 | 0.873 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.888 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.839 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.466 | moderate+ |
| satfold-neon | 0.904 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.888 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.595 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.464 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 1/40, lost 38/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 3.1ns | 27233.0ns | 0.0% |  |
| satfold-lanes16 | 2.9ns | 4593.4ns | 0.1% |  |
| satfold-lanes16-constl | 2.5ns | 4529.4ns | 0.1% |  |
| satfold-lanes4-idx | 2.8ns | 2069.0ns | 0.1% |  |
| satfold-lanes64 | 2.4ns | 7666.8ns | 0.0% |  |
| satfold-neon | 2.1ns | 542.6ns | 0.4% |  |
| satfold-neon8 | 2.0ns | 552.5ns | 0.4% |  |
| satfold-nolaw | 2.3ns | 21904.2ns | 0.0% |  |
| satfold-seq | 1.8ns | 28415.2ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 26578.9-28004.7 ns)
  26578.9 |########################################
  26650.2 |##########
  26721.4 |##########
  26792.7 |##########
  26864.0 |####################
  26935.3 |
  27006.6 |########################################
  27077.9 |####################
  27149.2 |####################
  27220.5 |##########
  27291.8 |##########
  27363.1 |##############################
  27434.4 |##############################
  27505.6 |####################
  27576.9 |##########
  27648.2 |####################
  27719.5 |
  27790.8 |
  27862.1 |##########
  27933.4 |##########
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 4287.2-4884.2 ns)
   4287.2 |############################
   4317.1 |
   4346.9 |####
   4376.8 |
   4406.6 |
   4436.5 |
   4466.3 |
   4496.2 |########################################
   4526.0 |####
   4555.8 |####
   4585.7 |####
   4615.5 |########
   4645.4 |
   4675.2 |
   4705.1 |########
   4734.9 |####
   4764.8 |
   4794.6 |
   4824.5 |
   4854.3 |########################################
  (2 below, 2 above range)

satfold-lanes16-constl (n=40, range 4296.1-4969.3 ns)
   4296.1 |########################################
   4329.8 |######
   4363.5 |###
   4397.1 |###
   4430.8 |
   4464.4 |
   4498.1 |##############################
   4531.7 |
   4565.4 |
   4599.0 |
   4632.7 |
   4666.4 |
   4700.0 |
   4733.7 |
   4767.3 |
   4801.0 |
   4834.6 |
   4868.3 |
   4902.0 |###
   4935.6 |###############
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 1974.1-2278.6 ns)
   1974.1 |########################################
   1989.3 |########
   2004.6 |
   2019.8 |########
   2035.0 |##
   2050.2 |########
   2065.5 |##
   2080.7 |##
   2095.9 |
   2111.1 |
   2126.3 |
   2141.6 |
   2156.8 |
   2172.0 |
   2187.2 |
   2202.5 |
   2217.7 |
   2232.9 |########
   2248.1 |
   2263.4 |##########
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 7613.2-7796.1 ns)
   7613.2 |########################################
   7622.3 |##########
   7631.5 |#############
   7640.6 |##########
   7649.8 |###
   7658.9 |
   7668.1 |
   7677.2 |##########
   7686.4 |###
   7695.5 |
   7704.7 |###
   7713.8 |###
   7723.0 |
   7732.1 |###
   7741.3 |###
   7750.4 |###
   7759.6 |
   7768.7 |###
   7777.9 |
   7787.0 |
  (4 below, 3 above range)

satfold-neon (n=40, range 527.0-558.2 ns)
    527.0 |###################################
    528.5 |#########################
    530.1 |#########################
    531.7 |
    533.2 |
    534.8 |
    536.3 |
    537.9 |
    539.5 |
    541.0 |
    542.6 |
    544.2 |
    545.7 |
    547.3 |
    548.9 |
    550.4 |
    552.0 |#####
    553.5 |##########
    555.1 |###################################
    556.7 |########################################
  (3 below, 2 above range)

satfold-neon8 (n=40, range 528.4-601.9 ns)
    528.4 |################
    532.1 |####
    535.8 |########################################
    539.5 |################################
    543.1 |####
    546.8 |####
    550.5 |
    554.1 |
    557.8 |
    561.5 |
    565.2 |
    568.8 |
    572.5 |
    576.2 |
    579.8 |
    583.5 |
    587.2 |
    590.9 |
    594.5 |
    598.2 |############################
  (5 below, 3 above range)

satfold-nolaw (n=40, range 21544.4-22486.1 ns)
  21544.4 |########################################
  21591.5 |########################
  21638.5 |########
  21685.6 |########################
  21732.7 |########################
  21779.8 |
  21826.9 |########################
  21874.0 |########################
  21921.1 |########
  21968.2 |########################
  22015.2 |################
  22062.3 |
  22109.4 |########
  22156.5 |########
  22203.6 |########
  22250.7 |########
  22297.8 |########
  22344.9 |
  22392.0 |########
  22439.0 |
  (4 below, 3 above range)

satfold-seq (n=40, range 27665.6-29510.2 ns)
  27665.6 |#######
  27757.8 |
  27850.1 |###
  27942.3 |##########
  28034.5 |
  28126.8 |##########
  28219.0 |########################################
  28311.2 |##################
  28403.4 |
  28495.7 |
  28587.9 |#######
  28680.1 |###
  28772.4 |###
  28864.6 |###
  28956.8 |
  29049.1 |###
  29141.3 |###
  29233.5 |###
  29325.7 |
  29418.0 |
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.59 (measurement drift or warm-up artifact)
