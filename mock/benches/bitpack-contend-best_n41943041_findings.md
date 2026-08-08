# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-padal dominates: 116% faster than the next best (bitpack-contend-pipe4)

bitpack-contend-d16-padal (138.28 us) leads bitpack-contend-pipe4 (299.14 us) by 116%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d16-padal beats baseline by 62% (significant)

bitpack-contend-d16-padal is -227.10 us (62%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-packed-simd is an outlier: 3.7x slower than the field

bitpack-contend-packed-simd (507.20 us) is 3.7x the fastest (138.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d16 shows warm-up / thermal drift (autocorr +0.74)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation +0.74, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d16-padal} vs {bitpack-contend-pipe4, bitpack-contend-d32-padal, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-packed-simd} (116% apart)

The field splits into a fast tier {bitpack-contend-d16-padal} and a slow tier {bitpack-contend-pipe4, bitpack-contend-d32-padal, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-packed-simd} with a 116% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.7x the fastest

Fastest bitpack-contend-d16-padal (138.28 us) to slowest bitpack-contend-packed-simd (507.20 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-contend-d16-padal** at 138278.2 ns median (-62.2% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.67x (fastest 138278.2 ns, slowest 507196.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 367047ns | 366379ns | 360233ns | 366295ns | 376117ns | base |
| bitpack-contend-d16-padal | 140613ns | 138593ns | 136208ns | 139257ns | 149086ns | -61.69% |
| bitpack-contend-d32 | 393593ns | 386562ns | 372441ns | 388567ns | 429822ns | +7.23% |
| bitpack-contend-d32-padal | 313362ns | 302850ns | 287077ns | 308013ns | 355694ns | -14.63% |
| bitpack-contend-packed-simd | 511868ns | 507955ns | 503003ns | 510283ns | 525486ns | +39.46% |
| bitpack-contend-pipe4 | 326665ns | 300241ns | 295009ns | 300825ns | 435844ns | -11.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 366628ns | 359666ns | 375771ns | base | 11.440 |
| bitpack-contend-d16-padal | 140282ns | 135858ns | 148768ns | -61.74% | 29.899 |
| bitpack-contend-d32 | 392133ns | 371220ns | 428570ns | +6.96% | 10.696 |
| bitpack-contend-d32-padal | 312108ns | 285394ns | 354584ns | -14.87% | 13.439 |
| bitpack-contend-packed-simd | 511377ns | 502427ns | 525149ns | +39.48% | 8.202 |
| bitpack-contend-pipe4 | 326080ns | 294527ns | 435211ns | -11.06% | 12.863 |

## Performance model

- Peak throughput: **30.873 Gops/s** (bitpack-contend-d16-padal; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.466 | 37.1% |
| bitpack-contend-d16-padal | 30.332 | 98.2% |
| bitpack-contend-d32 | 10.885 | 35.3% |
| bitpack-contend-d32-padal | 13.902 | 45.0% |
| bitpack-contend-packed-simd | 8.270 | 26.8% |
| bitpack-contend-pipe4 | 14.021 | 45.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 367047ns | 367047ns | base |
| bitpack-contend-d16-padal | 140613ns | 140613ns | -61.69% |
| bitpack-contend-d32 | 393593ns | 393593ns | +7.23% |
| bitpack-contend-d32-padal | 313362ns | 313362ns | -14.63% |
| bitpack-contend-packed-simd | 511868ns | 511868ns | +39.46% |
| bitpack-contend-pipe4 | 326665ns | 326665ns | -11.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 365788ns | base | --- | [363109, 368743] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 138278ns | -226217.3ns (-61.8%) | [-228617, -224715]ns | [137383, 140605] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32 | 385320ns | +16529.4ns (+4.5%) | [+9525, +34126]ns | [378280, 395573] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 301706ns | -61787.5ns (-16.9%) | [-72785, -47704]ns | [292744, 317117] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 507196ns | +147269.6ns (+40.3%) | [+140180, +149696]ns | [505285, 514022] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 299136ns | -64379.0ns (-17.6%) | [-68097, -59639]ns | [296655, 302692] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 359363ns | -60.9% | +11.7% | -14.6% | +41.0% | -15.5% |
| 2 | 360593ns | -57.7% | +9.9% | -12.1% | +47.2% | -16.1% |
| 3 | 359541ns | -58.7% | +12.4% | -15.3% | +40.5% | -15.8% |
| 4 | 359172ns | -60.7% | +7.6% | -16.7% | +41.1% | -15.8% |
| 5 | 358768ns | -61.2% | +11.9% | -12.7% | +41.1% | -17.5% |
| 6 | 358088ns | -60.5% | +15.1% | -18.4% | +41.7% | -17.0% |
| 7 | 359583ns | -62.6% | +9.0% | -19.8% | +42.2% | -17.8% |
| 8 | 366774ns | -62.5% | +7.1% | -21.4% | +40.2% | -19.5% |
| 9 | 363455ns | -62.2% | +22.4% | -20.5% | +43.2% | -18.8% |
| 10 | 362438ns | -62.0% | +19.0% | -20.4% | +42.5% | -18.5% |
| 11 | 382620ns | -63.3% | -1.1% | -25.5% | +31.0% | -23.1% |
| 12 | 383909ns | -61.9% | -4.0% | -16.5% | +32.0% | -23.3% |
| 13 | 380142ns | -62.8% | -2.3% | -24.4% | +31.7% | +98.0% |
| 14 | 375101ns | -62.3% | +3.7% | -24.1% | +33.7% | +110.5% |
| 15 | 370938ns | -61.4% | -0.2% | -23.2% | +35.4% | -13.9% |
| 16 | 366448ns | -58.5% | +1.0% | -20.0% | +37.9% | -13.2% |
| 17 | 368857ns | -60.7% | +0.8% | -6.6% | +37.3% | -8.1% |
| 18 | 368629ns | -59.9% | +2.6% | -22.3% | +39.4% | -15.0% |
| 19 | 368954ns | -60.6% | +2.5% | -22.9% | +42.5% | -13.0% |
| 20 | 369413ns | -58.7% | +1.7% | -23.2% | +44.3% | -15.8% |
| 21 | 363027ns | -62.6% | +10.4% | -5.3% | +41.3% | -18.2% |
| 22 | 363160ns | -62.5% | +2.4% | -3.1% | +42.7% | -18.9% |
| 23 | 362642ns | -62.4% | +2.3% | -12.5% | +39.0% | -18.8% |
| 24 | 363058ns | -62.3% | +8.7% | -21.1% | +41.2% | -18.8% |
| 25 | 365536ns | -62.7% | +19.4% | -20.1% | +41.1% | -19.5% |
| 26 | 362435ns | -62.0% | +18.7% | -18.1% | +44.5% | -18.0% |
| 27 | 362432ns | -61.3% | +19.1% | -2.6% | +43.2% | -17.6% |
| 28 | 368184ns | -63.0% | +19.0% | -15.6% | +40.2% | -19.5% |
| 29 | 367650ns | -63.0% | +5.2% | -19.7% | +36.8% | -18.5% |
| 30 | 369670ns | -63.2% | +3.5% | -20.6% | +35.9% | -18.8% |
| 31 | 369592ns | -63.1% | +2.3% | -19.4% | +40.6% | -11.6% |
| 32 | 369654ns | -63.1% | +3.5% | -13.7% | +41.3% | -14.8% |
| 33 | 370522ns | -63.1% | +1.2% | -6.7% | +41.7% | -17.8% |
| 34 | 373264ns | -63.2% | +0.5% | -3.1% | +36.5% | -19.5% |
| 35 | 369605ns | -62.6% | +1.7% | -8.2% | +36.7% | -17.4% |
| 36 | 364489ns | -62.6% | +5.4% | -6.0% | +38.4% | -17.6% |
| 37 | 363828ns | -62.0% | +3.0% | -3.9% | +38.6% | -18.9% |
| 38 | 362215ns | -61.8% | +11.4% | -2.6% | +39.5% | -17.6% |
| 39 | 366040ns | -61.9% | +2.6% | +3.1% | +37.7% | -19.5% |
| 40 | 365314ns | -61.5% | +9.8% | -13.7% | +37.9% | -18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.737 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-padal | 0.538 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | 0.544 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32-padal | 0.527 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.402 | moderate+ |
| bitpack-contend-pipe4 | 0.483 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 40/40, lost 0/40
- **bitpack-contend-d32**: won 4/40, lost 36/40
- **bitpack-contend-d32-padal**: won 39/40, lost 1/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40
- **bitpack-contend-pipe4**: won 38/40, lost 2/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 5.1ns | 366627.6ns | 0.0% |  |
| bitpack-contend-d16-padal | 3.6ns | 140282.2ns | 0.0% |  |
| bitpack-contend-d32 | 11.5ns | 392132.7ns | 0.0% |  |
| bitpack-contend-d32-padal | 4.5ns | 312108.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 5.4ns | 511377.2ns | 0.0% |  |
| bitpack-contend-pipe4 | 8.9ns | 326080.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 359665.5-375770.7 ns)
  359665.5 |
  360470.8 |########
  361276.1 |
  362081.3 |########################################
  362886.6 |################################
  363691.8 |################
  364497.1 |
  365302.4 |########################
  366107.6 |################
  366912.9 |########
  367718.1 |########
  368523.4 |########################
  369328.7 |########################################
  370133.9 |################
  370939.2 |
  371744.4 |
  372549.7 |########
  373355.0 |
  374160.2 |
  374965.5 |########
  (6 below, 3 above range)

bitpack-contend-d16-padal (n=40, range 135858.2-148768.3 ns)
  135858.2 |########################################
  136503.7 |###############
  137149.2 |#########################
  137794.7 |###############
  138440.2 |
  139085.7 |##########
  139731.2 |#####
  140376.7 |####################
  141022.2 |###############
  141667.7 |
  142313.2 |
  142958.7 |#####
  143604.2 |
  144249.7 |#####
  144895.2 |#####
  145540.7 |#####
  146186.2 |
  146831.7 |
  147477.3 |#####
  148122.8 |#####
  (2 below, 3 above range)

bitpack-contend-d32 (n=40, range 371220.1-428570.2 ns)
  371220.1 |####################
  374087.6 |########################################
  376955.1 |##########################
  379822.6 |######
  382690.1 |#############
  385557.6 |#############
  388425.1 |######
  391292.6 |#############
  394160.1 |#############
  397027.6 |
  399895.1 |##########################
  402762.6 |#############
  405630.1 |
  408497.6 |
  411365.1 |######
  414232.6 |
  417100.2 |
  419967.7 |
  422835.2 |
  425702.7 |
  (4 below, 6 above range)

bitpack-contend-d32-padal (n=40, range 285393.9-354584.2 ns)
  285393.9 |########################################
  288853.4 |####################
  292312.9 |####################
  295772.4 |####################
  299231.9 |
  302691.5 |######
  306151.0 |######
  309610.5 |######
  313070.0 |#############
  316529.5 |####################
  319989.0 |######
  323448.5 |
  326908.1 |
  330367.6 |
  333827.1 |
  337286.6 |######
  340746.1 |#############
  344205.6 |#############
  347665.1 |######
  351124.6 |####################
  (5 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 502427.1-525148.9 ns)
  502427.1 |################
  503563.2 |########################################
  504699.3 |################################
  505835.3 |########################################
  506971.4 |########
  508107.5 |
  509243.6 |########
  510379.7 |########
  511515.8 |########
  512651.9 |########
  513788.0 |################
  514924.1 |########
  516060.2 |################
  517196.3 |########
  518332.4 |########
  519468.4 |################
  520604.5 |
  521740.6 |########
  522876.7 |########
  524012.8 |########
  (4 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 294526.9-435211.2 ns)
  294526.9 |########################################
  301561.1 |#############
  308595.3 |######
  315629.6 |######
  322663.8 |##
  329698.0 |
  336732.2 |##
  343766.4 |
  350800.6 |
  357834.8 |
  364869.0 |
  371903.2 |
  378937.5 |
  385971.7 |
  393005.9 |
  400040.1 |
  407074.3 |
  414108.5 |
  421142.7 |
  428176.9 |
  (6 below, 2 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-padal**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **bitpack-contend-d32-padal**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe4**: CV=31.5% (high variance, measurements may be unstable)
