# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 32% faster than the next best (warm-clamp-head)

warm-clamp-accfit (197 ns) leads warm-clamp-head (260 ns) by 32%, a clear separation rather than a photo finish. CV 8.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 64% (significant)

warm-clamp-accfit is -332 ns (64%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 37.3x slower than the field

warm-clamp-minimum (7.36 us) is 37.3x the fastest (197 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.87)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64} vs {warm-clamp-min-lanes, warm-clamp-minimum} (279% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum} with a 279% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 37.3x the fastest

Fastest warm-clamp-accfit (197 ns) to slowest warm-clamp-minimum (7.36 us): 37.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 197.2 ns median (-62.0% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 37.30x (fastest 197.2 ns, slowest 7358.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 590ns | 581ns | 573ns | 585ns | 621ns | base |
| warm-clamp-accfit | 271ns | 260ns | 249ns | 265ns | 310ns | -54.11% |
| warm-clamp-accfit-dyn | 372ns | 368ns | 359ns | 368ns | 398ns | -36.84% |
| warm-clamp-head | 327ns | 325ns | 300ns | 326ns | 358ns | -44.51% |
| warm-clamp-min-lanes | 2130ns | 2033ns | 1944ns | 2051ns | 2552ns | +261.25% |
| warm-clamp-minimum | 7488ns | 7424ns | 7055ns | 7445ns | 8049ns | +1169.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 527ns | 512ns | 556ns | base | 15.540 |
| warm-clamp-accfit | 205ns | 189ns | 232ns | -61.20% | 40.050 |
| warm-clamp-accfit-dyn | 308ns | 297ns | 331ns | -41.49% | 26.561 |
| warm-clamp-head | 261ns | 239ns | 285ns | -50.57% | 31.439 |
| warm-clamp-min-lanes | 2061ns | 1881ns | 2470ns | +290.94% | 3.975 |
| warm-clamp-minimum | 7412ns | 6990ns | 7955ns | +1306.01% | 1.105 |

## Performance model

- Peak throughput: **43.367 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 15.799 | 36.4% |
| warm-clamp-accfit | 41.531 | 95.8% |
| warm-clamp-accfit-dyn | 26.899 | 62.0% |
| warm-clamp-head | 31.508 | 72.7% |
| warm-clamp-min-lanes | 4.164 | 9.6% |
| warm-clamp-minimum | 1.113 | 2.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 590ns | 590ns | base |
| warm-clamp-accfit | 271ns | 271ns | -54.11% |
| warm-clamp-accfit-dyn | 372ns | 372ns | -36.84% |
| warm-clamp-head | 327ns | 327ns | -44.51% |
| warm-clamp-min-lanes | 2130ns | 2130ns | +261.25% |
| warm-clamp-minimum | 7488ns | 7488ns | +1169.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 518ns | base | --- | [515, 527] | --- | --- | --- | --- |
| warm-clamp-accfit | 197ns | -327.2ns (-63.1%) | [-336, -321]ns | [193, 205] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 305ns | -217.5ns (-41.9%) | [-227, -215]ns | [300, 309] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 260ns | -262.5ns (-50.6%) | [-275, -249]ns | [251, 269] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1968ns | +1408.5ns (+271.7%) | [+1388, +1461]ns | [1905, 2006] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 7358ns | +6806.4ns (+1312.7%) | [+6618, +6977]ns | [7132, 7522] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 512ns | -61.9% | -39.3% | -53.5% | +268.5% | +1298.1% |
| 2 | 512ns | -62.2% | -30.5% | -53.1% | +271.2% | +1328.2% |
| 3 | 513ns | -62.6% | -33.6% | -53.2% | +263.4% | +1303.7% |
| 4 | 514ns | -61.9% | -24.0% | -52.8% | +266.3% | +1305.9% |
| 5 | 513ns | -63.0% | -40.4% | -52.9% | +268.8% | +1262.2% |
| 6 | 515ns | -62.4% | -40.3% | -53.5% | +269.2% | +1273.6% |
| 7 | 512ns | -63.0% | -39.6% | -53.7% | +275.3% | +1265.4% |
| 8 | 517ns | -63.1% | -40.0% | -54.0% | +269.4% | +1320.4% |
| 9 | 515ns | -62.6% | -39.0% | -53.8% | +264.6% | +1279.3% |
| 10 | 545ns | -64.4% | -44.6% | -55.7% | +247.3% | +1187.5% |
| 11 | 514ns | -54.6% | -42.1% | -47.7% | +384.6% | +1441.7% |
| 12 | 512ns | -54.6% | -41.7% | -47.2% | +416.9% | +1448.3% |
| 13 | 515ns | -55.1% | -42.3% | -48.9% | +381.3% | +1454.4% |
| 14 | 512ns | -55.4% | -41.4% | -47.5% | +383.7% | +1447.5% |
| 15 | 513ns | -54.9% | -41.6% | -47.5% | +385.2% | +1443.9% |
| 16 | 513ns | -55.3% | -41.7% | -48.6% | +389.6% | +1445.4% |
| 17 | 515ns | -55.1% | -42.2% | -48.9% | +355.1% | +1439.7% |
| 18 | 514ns | -54.2% | -42.1% | -48.3% | +351.3% | +1446.7% |
| 19 | 515ns | -55.2% | -41.7% | -49.0% | +344.9% | +1438.2% |
| 20 | 515ns | -55.7% | -41.9% | -47.9% | +347.4% | +1441.7% |
| 21 | 592ns | -64.6% | -47.7% | -57.7% | +236.7% | +1148.5% |
| 22 | 549ns | -61.4% | -43.5% | -53.6% | +264.7% | +1246.8% |
| 23 | 564ns | -63.3% | -44.7% | -54.7% | +249.8% | +1209.3% |
| 24 | 545ns | -63.0% | -43.2% | -52.8% | +265.6% | +1373.4% |
| 25 | 545ns | -62.7% | -43.3% | -53.2% | +268.6% | +1260.3% |
| 26 | 548ns | -63.3% | -43.3% | -54.9% | +281.8% | +1296.3% |
| 27 | 544ns | -63.5% | -42.8% | -54.6% | +273.0% | +1282.5% |
| 28 | 547ns | -63.7% | -43.1% | -54.5% | +268.8% | +1274.7% |
| 29 | 562ns | -62.9% | -45.1% | -54.9% | +253.2% | +1288.9% |
| 30 | 544ns | -62.9% | -43.0% | -53.5% | +268.1% | +1255.7% |
| 31 | 531ns | -64.1% | -43.3% | -46.9% | +253.5% | +1216.7% |
| 32 | 520ns | -63.2% | -42.3% | -44.5% | +264.2% | +1245.4% |
| 33 | 533ns | -64.0% | -44.5% | -47.6% | +253.3% | +1217.1% |
| 34 | 519ns | -64.0% | -41.6% | -45.0% | +264.5% | +1257.0% |
| 35 | 533ns | -64.1% | -44.1% | -46.8% | +257.8% | +1210.6% |
| 36 | 518ns | -63.7% | -42.6% | -45.4% | +278.7% | +1283.7% |
| 37 | 524ns | -64.1% | -42.9% | -45.4% | +268.9% | +1233.4% |
| 38 | 523ns | -63.7% | -43.0% | -45.6% | +266.6% | +1239.0% |
| 39 | 519ns | -62.6% | -42.2% | -46.4% | +263.5% | +1251.6% |
| 40 | 520ns | -63.8% | -40.6% | -44.3% | +266.1% | +1244.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.547 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.871 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.476 | moderate+ |
| warm-clamp-head | 0.823 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.848 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.740 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.1ns | 527.1ns | 0.6% |  |
| warm-clamp-accfit | 2.9ns | 204.5ns | 1.4% |  |
| warm-clamp-accfit-dyn | 2.7ns | 308.4ns | 0.9% |  |
| warm-clamp-head | 2.8ns | 260.6ns | 1.1% |  |
| warm-clamp-min-lanes | 2.6ns | 2060.8ns | 0.1% |  |
| warm-clamp-minimum | 3.0ns | 7411.7ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 512.4-556.4 ns)
    512.4 |########################################
    514.6 |##########################
    516.8 |########
    519.0 |#############
    521.2 |####
    523.4 |####
    525.6 |
    527.8 |
    530.0 |####
    532.2 |########
    534.4 |
    536.6 |
    538.8 |
    541.0 |
    543.2 |######################
    545.4 |####
    547.6 |########
    549.8 |
    552.0 |
    554.2 |
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 188.9-231.9 ns)
    188.9 |########################################
    191.1 |########################################
    193.2 |########################################
    195.4 |########
    197.5 |################
    199.7 |########################
    201.8 |########
    204.0 |
    206.1 |########
    208.3 |################
    210.4 |########
    212.6 |
    214.7 |
    216.9 |
    219.0 |
    221.2 |
    223.3 |
    225.5 |
    227.6 |########################
    229.8 |################################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 297.5-330.8 ns)
    297.5 |########################################
    299.2 |###################################
    300.8 |##########
    302.5 |#####
    304.1 |#####
    305.8 |
    307.5 |###############
    309.1 |########################################
    310.8 |####################
    312.5 |
    314.1 |#####
    315.8 |
    317.5 |
    319.1 |
    320.8 |
    322.4 |
    324.1 |
    325.8 |
    327.4 |
    329.1 |
  (2 below, 3 above range)

warm-clamp-head (n=40, range 239.0-285.4 ns)
    239.0 |########################################
    241.3 |####################
    243.6 |
    246.0 |####################
    248.3 |####################
    250.6 |##########
    252.9 |##############################
    255.2 |####################
    257.5 |
    259.9 |
    262.2 |########################################
    264.5 |##########
    266.8 |####################
    269.1 |##############################
    271.4 |
    273.8 |
    276.1 |##########
    278.4 |##########
    280.7 |####################
    283.0 |####################
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1881.2-2470.1 ns)
   1881.2 |########################################
   1910.6 |#########
   1940.1 |###
   1969.5 |############
   1999.0 |############
   2028.4 |###
   2057.8 |
   2087.3 |###
   2116.7 |
   2146.2 |
   2175.6 |
   2205.1 |
   2234.5 |
   2264.0 |###
   2293.4 |######
   2322.8 |###
   2352.3 |
   2381.7 |
   2411.2 |
   2440.6 |
  (3 below, 6 above range)

warm-clamp-minimum (n=40, range 6990.2-7955.1 ns)
   6990.2 |##########################
   7038.5 |########
   7086.7 |####
   7135.0 |########
   7183.2 |########
   7231.5 |
   7279.7 |####
   7327.9 |####
   7376.2 |######################
   7424.4 |
   7472.7 |
   7520.9 |########
   7569.2 |
   7617.4 |####
   7665.7 |
   7713.9 |
   7762.2 |####
   7810.4 |
   7858.6 |
   7906.9 |########################################
  (5 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.74 (measurement drift or warm-up artifact)
