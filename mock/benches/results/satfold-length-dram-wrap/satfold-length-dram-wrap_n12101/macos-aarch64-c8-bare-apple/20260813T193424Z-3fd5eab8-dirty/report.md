# The wrapping ceiling over the same 16 MiB column

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (302.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes16 at 278.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (satfold-lanes16, satfold-seq) are a dead heat (<1%)

satfold-lanes16 (278.16 us) and satfold-seq (278.18 us) differ by 0.01%, inside the noise, even though the wider field spreads 8.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes4-idx shows warm-up / thermal drift (autocorr +0.74)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation +0.74, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (24.59 us) is smaller than the fastest variant's own run-to-run std-dev (39.28 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-lanes16 vs stability leader satfold-iterfold (+9% speed for 1.2x steadier)

satfold-lanes16 is fastest (278.16 us, CV 14.1%); satfold-iterfold gives up 8.8% median for 1.2x lower variance (CV 11.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes16-constl is inconsistent: worst-20% is 1.5x its best-20%

satfold-lanes16-constl's best 20% of batches run at 264.48 us but its worst 20% at 398.69 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16** at 278161.1 ns median (-8.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 278161.1 ns, slowest 302748.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 305481ns | 304068ns | 265146ns | 301119ns | 358903ns | base |
| satfold-lanes16 | 297388ns | 279666ns | 259020ns | 288826ns | 361440ns | -2.65% |
| satfold-lanes16-constl | 309353ns | 288161ns | 266180ns | 293503ns | 400075ns | +1.27% |
| satfold-lanes4-idx | 322826ns | 297815ns | 287192ns | 308989ns | 399970ns | +5.68% |
| satfold-lanes64 | 302358ns | 295590ns | 268835ns | 294169ns | 360448ns | -1.02% |
| satfold-neon | 308940ns | 303304ns | 264197ns | 305211ns | 364871ns | +1.13% |
| satfold-nolaw | 307671ns | 298082ns | 273361ns | 299986ns | 365035ns | +0.72% |
| satfold-seq | 298376ns | 279345ns | 261304ns | 282433ns | 383275ns | -2.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 304150ns | 263111ns | 357702ns | base | 55.161 |
| satfold-lanes16 | 295802ns | 257284ns | 360218ns | -2.74% | 56.718 |
| satfold-lanes16-constl | 307672ns | 264481ns | 398691ns | +1.16% | 54.530 |
| satfold-lanes4-idx | 321463ns | 285840ns | 398683ns | +5.69% | 52.190 |
| satfold-lanes64 | 301100ns | 267324ns | 359310ns | -1.00% | 55.720 |
| satfold-neon | 307330ns | 262057ns | 363678ns | +1.05% | 54.590 |
| satfold-nolaw | 306137ns | 271208ns | 363909ns | +0.65% | 54.803 |
| satfold-seq | 296746ns | 259222ns | 382018ns | -2.43% | 56.537 |

## Performance model

- Peak throughput: **65.209 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 55.416 | 85.0% |
| satfold-lanes16 | 60.315 | 92.5% |
| satfold-lanes16-constl | 58.614 | 89.9% |
| satfold-lanes4-idx | 56.623 | 86.8% |
| satfold-lanes64 | 57.000 | 87.4% |
| satfold-neon | 55.577 | 85.2% |
| satfold-nolaw | 56.543 | 86.7% |
| satfold-seq | 60.310 | 92.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 305481ns | 305481ns | base |
| satfold-lanes16 | 297388ns | 297388ns | -2.65% |
| satfold-lanes16-constl | 309353ns | 309353ns | +1.27% |
| satfold-lanes4-idx | 322826ns | 322826ns | +5.68% |
| satfold-lanes64 | 302358ns | 302358ns | -1.02% |
| satfold-neon | 308940ns | 308940ns | +1.13% |
| satfold-nolaw | 307671ns | 307671ns | +0.72% |
| satfold-seq | 298376ns | 298376ns | -2.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 302748ns | base | --- | [284348, 314156] | --- | --- | --- | --- |
| satfold-lanes16 | 278161ns | no significant difference | [-15772, +11614]ns | [266577, 302434] | no | 0.7418 | 0.6358 | 0 |
| satfold-lanes16-constl | 286230ns | no significant difference | [-16931, +23657]ns | [275405, 303019] | no | 1.0000 | 1.0000 | 0 |
| satfold-lanes4-idx | 296298ns | +23216.1ns (+7.7%) | [+511, +35694]ns | [289133, 320329] | YES (adj: no) | 0.5648 | 0.0807 | 0 |
| satfold-lanes64 | 294335ns | no significant difference | [-12233, +5681]ns | [274381, 303252] | no | 0.7418 | 0.4296 | 0 |
| satfold-neon | 301871ns | no significant difference | [-6369, +12292]ns | [290045, 319711] | no | 0.7418 | 0.6358 | 0 |
| satfold-nolaw | 296715ns | no significant difference | [-10320, +14620]ns | [285421, 310413] | no | 0.7418 | 0.6358 | 0 |
| satfold-seq | 278182ns | no significant difference | [-22910, +14242]ns | [264954, 289579] | no | 0.7418 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 262484ns | -2.3% | -0.5% | +9.8% | +9.7% | -0.7% | +8.9% | +0.2% |
| 2 | 275122ns | -6.7% | -5.6% | +5.1% | -0.4% | +2.3% | +5.6% | -2.7% |
| 3 | 298661ns | -13.7% | -10.3% | +8.6% | -8.0% | +1.2% | -9.0% | -13.0% |
| 4 | 346045ns | -25.0% | -18.7% | -16.0% | -20.8% | -12.9% | -21.7% | -24.0% |
| 5 | 340411ns | -24.4% | -23.8% | -15.2% | -12.1% | -17.5% | -20.2% | -22.2% |
| 6 | 328278ns | -22.1% | -18.0% | -12.7% | -17.1% | -16.0% | -15.6% | -20.3% |
| 7 | 300170ns | -14.7% | -6.2% | -3.4% | -9.7% | -3.6% | -4.6% | -12.5% |
| 8 | 274060ns | -1.3% | +7.0% | +14.0% | -0.8% | +4.3% | -1.1% | +5.0% |
| 9 | 279748ns | -0.8% | -1.1% | +2.3% | -3.1% | +7.4% | -2.6% | +0.1% |
| 10 | 298034ns | -2.3% | -6.3% | -0.3% | -8.6% | +0.1% | -9.3% | -11.0% |
| 11 | 287351ns | -7.4% | +19.5% | +2.8% | +3.0% | +5.2% | +14.6% | +7.6% |
| 12 | 306575ns | -13.3% | +14.4% | -3.7% | -3.0% | +30.9% | +25.0% | -13.3% |
| 13 | 378144ns | -30.6% | -19.8% | -24.6% | -18.0% | -1.7% | -21.2% | -30.3% |
| 14 | 301468ns | -7.3% | -7.8% | -5.0% | -0.9% | +18.7% | -5.3% | -7.2% |
| 15 | 313665ns | +5.2% | -13.4% | -8.1% | -10.1% | +4.9% | -9.2% | -5.8% |
| 16 | 324234ns | -8.2% | -6.7% | -5.6% | -1.2% | +12.5% | -12.0% | -18.7% |
| 17 | 310577ns | -4.2% | +0.4% | -8.3% | -5.4% | +13.9% | -4.3% | -15.3% |
| 18 | 351798ns | -24.3% | -17.4% | -18.7% | -2.5% | -15.1% | -7.8% | -24.0% |
| 19 | 388850ns | -30.7% | -29.6% | -26.8% | -8.0% | -14.9% | -16.1% | -25.1% |
| 20 | 380303ns | -20.2% | -27.9% | -24.5% | -27.2% | -2.9% | -18.3% | -17.6% |
| 21 | 266756ns | -3.1% | +14.6% | +7.8% | +2.6% | -2.4% | +4.1% | +5.5% |
| 22 | 262560ns | +6.2% | +13.3% | +11.1% | +22.0% | +0.3% | +18.9% | -1.9% |
| 23 | 263948ns | +23.5% | +10.7% | +23.8% | +1.6% | -1.3% | +7.9% | -2.1% |
| 24 | 269108ns | +11.9% | -0.1% | +17.5% | -0.9% | -3.1% | +11.4% | -4.1% |
| 25 | 281345ns | +20.4% | -3.9% | +2.7% | -4.8% | -6.6% | +4.6% | -8.6% |
| 26 | 262276ns | +5.4% | +2.7% | +9.7% | +1.4% | +1.7% | +13.0% | -1.4% |
| 27 | 263200ns | +1.4% | +0.8% | +17.2% | +0.6% | +5.9% | +12.8% | +7.0% |
| 28 | 262110ns | +1.8% | +1.9% | +26.1% | +12.5% | -0.2% | +4.9% | +8.7% |
| 29 | 261555ns | +1.0% | +11.3% | +50.1% | +5.3% | +25.4% | +3.4% | +5.7% |
| 30 | 273715ns | -3.6% | -2.8% | +11.5% | -4.0% | +11.8% | -0.9% | -0.5% |
| 31 | 330597ns | +2.7% | +20.8% | +25.2% | +2.9% | +5.1% | +9.0% | +10.7% |
| 32 | 304028ns | +19.7% | +21.7% | +23.6% | +33.7% | -4.2% | +25.0% | +13.9% |
| 33 | 315048ns | +19.7% | +25.7% | +24.0% | +11.5% | -3.1% | +4.2% | -1.0% |
| 34 | 315180ns | +17.3% | +10.9% | +33.0% | -5.6% | -7.8% | +28.2% | +10.1% |
| 35 | 314647ns | +17.3% | -11.6% | +21.2% | +6.8% | +0.5% | -1.4% | +38.2% |
| 36 | 305066ns | +13.7% | +3.8% | +26.4% | +7.0% | +4.2% | +0.9% | +19.8% |
| 37 | 312878ns | -2.2% | +22.9% | +28.8% | +25.0% | +2.8% | +5.1% | +16.3% |
| 38 | 293502ns | +11.0% | +33.8% | +37.4% | +18.8% | +10.6% | +26.0% | +11.5% |
| 39 | 345464ns | +7.7% | +28.3% | +0.6% | -11.0% | -4.2% | +2.3% | +23.7% |
| 40 | 317047ns | +8.3% | +42.8% | +10.4% | +5.3% | +8.4% | +4.3% | +27.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.535 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.727 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.664 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.738 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.528 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.542 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.499 | moderate+ |
| satfold-seq | 0.683 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 22/40, lost 18/40
- **satfold-lanes16-constl**: won 20/40, lost 20/40
- **satfold-lanes4-idx**: won 14/40, lost 26/40
- **satfold-lanes64**: won 23/40, lost 17/40
- **satfold-neon**: won 18/40, lost 22/40
- **satfold-nolaw**: won 18/40, lost 22/40
- **satfold-seq**: won 23/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 8.2ns | 304150.2ns | 0.0% |  |
| satfold-lanes16 | 7.0ns | 295802.0ns | 0.0% |  |
| satfold-lanes16-constl | 11.0ns | 307672.0ns | 0.0% |  |
| satfold-lanes4-idx | 8.5ns | 321462.5ns | 0.0% |  |
| satfold-lanes64 | 8.8ns | 301100.1ns | 0.0% |  |
| satfold-neon | 9.4ns | 307330.0ns | 0.0% |  |
| satfold-nolaw | 6.4ns | 306137.5ns | 0.0% |  |
| satfold-seq | 9.2ns | 296745.9ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 263111.0-357701.5 ns)
  263111.0 |########################
  267840.5 |########
  272570.0 |########################
  277299.6 |################
  282029.1 |
  286758.6 |########
  291488.1 |########
  296217.7 |########################
  300947.2 |########################
  305676.7 |########
  310406.2 |########################################
  315135.8 |################
  319865.3 |########
  324594.8 |########
  329324.4 |########
  334053.9 |
  338783.4 |########
  343512.9 |################
  348242.5 |########
  352972.0 |
  (5 below, 3 above range)

satfold-lanes16 (n=40, range 257284.4-360218.3 ns)
  257284.4 |######################
  262431.1 |########################################
  267577.8 |###########
  272724.5 |###########
  277871.2 |###########
  283017.9 |
  288164.6 |#####
  293311.3 |###########
  298458.0 |#####
  303604.7 |###########
  308751.3 |
  313898.0 |
  319044.7 |
  324191.4 |###########
  329338.1 |#####
  334484.8 |#####
  339631.5 |###########
  344778.2 |#####
  349924.9 |
  355071.6 |
  (5 below, 5 above range)

satfold-lanes16-constl (n=40, range 264480.9-398691.2 ns)
  264480.9 |########################################
  271191.4 |####################
  277901.9 |#########################
  284612.4 |##########
  291323.0 |###############
  298033.5 |##########
  304744.0 |#####
  311454.5 |##########
  318165.0 |
  324875.5 |
  331586.0 |
  338296.6 |#####
  345007.1 |##########
  351717.6 |
  358428.1 |
  365138.6 |#####
  371849.1 |
  378559.6 |#####
  385270.2 |
  391980.7 |##########
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 285839.9-398683.2 ns)
  285839.9 |########################################
  291482.1 |###########
  297124.2 |
  302766.4 |#####
  308408.6 |#####
  314050.7 |##
  319692.9 |##
  325335.0 |#####
  330977.2 |
  336619.4 |
  342261.5 |##
  347903.7 |##
  353545.9 |
  359188.0 |
  364830.2 |
  370472.3 |##
  376114.5 |##
  381756.7 |##
  387398.8 |#####
  393041.0 |
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 267324.5-359310.5 ns)
  267324.5 |####################
  271923.8 |########################################
  276523.1 |#####
  281122.4 |#####
  285721.7 |#####
  290321.0 |#####
  294920.3 |##############################
  299519.6 |
  304118.9 |#####
  308718.2 |#####
  313317.5 |
  317916.8 |##########
  322516.1 |#####
  327115.4 |
  331714.7 |##########
  336314.0 |#####
  340913.3 |#####
  345512.6 |#####
  350111.9 |#####
  354711.2 |#####
  (4 below, 2 above range)

satfold-neon (n=40, range 262057.5-363677.8 ns)
  262057.5 |####################
  267138.5 |
  272219.5 |######
  277300.5 |####################
  282381.6 |######
  287462.6 |####################
  292543.6 |
  297624.6 |########################################
  302705.6 |#############
  307786.7 |
  312867.7 |#############
  317948.7 |######
  323029.7 |#############
  328110.7 |####################
  333191.7 |
  338272.8 |
  343353.8 |#############
  348434.8 |
  353515.8 |#############
  358596.8 |
  (5 below, 4 above range)

satfold-nolaw (n=40, range 271207.6-363908.6 ns)
  271207.6 |########################################
  275842.7 |################
  280477.7 |################
  285112.8 |################################
  289747.8 |################
  294382.9 |################################
  299017.9 |########
  303653.0 |########
  308288.0 |########################
  312923.1 |
  317558.1 |
  322193.1 |################
  326828.2 |################################
  331463.2 |
  336098.3 |
  340733.3 |
  345368.4 |
  350003.4 |########
  354638.5 |
  359273.5 |########
  (4 below, 4 above range)

satfold-seq (n=40, range 259222.3-382018.2 ns)
  259222.3 |########################################
  265362.1 |############
  271501.9 |########
  277641.7 |################
  283781.5 |########
  289921.3 |########
  296061.1 |
  302200.9 |
  308340.7 |############
  314480.5 |
  320620.2 |
  326760.0 |####
  332899.8 |
  339039.6 |
  345179.4 |########
  351319.2 |
  357459.0 |
  363598.8 |############
  369738.6 |
  375878.4 |
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.68 (measurement drift or warm-up artifact)
