# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-padal dominates: 109% faster than the next best (bitpack-contend-pipe4)

bitpack-contend-d16-padal (54.13 us) leads bitpack-contend-pipe4 (112.92 us) by 109%, a clear separation rather than a photo finish. CV 48.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d16-padal beats baseline by 56% (significant)

bitpack-contend-d16-padal is -73.10 us (56%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d32 is an outlier: 4.7x slower than the field

bitpack-contend-d32 (256.83 us) is 4.7x the fastest (54.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d16-padal is fastest but the noisiest (CV 48.5%)

bitpack-contend-d16-padal wins on median (54.13 us) yet has the highest variance (CV 48.5%), while bitpack-contend-d32-padal is the steadiest (CV 6.8%, 254.21 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-d16 shows warm-up / thermal drift (autocorr +0.55)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d16-padal} vs {bitpack-contend-pipe4, bitpack-contend-d16, bitpack-contend-packed-simd, bitpack-contend-d32-padal, bitpack-contend-d32} (109% apart)

The field splits into a fast tier {bitpack-contend-d16-padal} and a slow tier {bitpack-contend-pipe4, bitpack-contend-d16, bitpack-contend-packed-simd, bitpack-contend-d32-padal, bitpack-contend-d32} with a 109% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest bitpack-contend-d16-padal (54.13 us) to slowest bitpack-contend-d32 (256.83 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-contend-d16-padal is inconsistent: worst-20% is 2.4x its best-20%

bitpack-contend-d16-padal's best 20% of batches run at 44.37 us but its worst 20% at 105.98 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d16-padal** at 54131.5 ns median (-58.2% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.74x (fastest 54131.5 ns, slowest 256826.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 140735ns | 129823ns | 117449ns | 130147ns | 195788ns | base |
| bitpack-contend-d16-padal | 65542ns | 54352ns | 44745ns | 58728ns | 106780ns | -53.43% |
| bitpack-contend-d32 | 265695ns | 257883ns | 242838ns | 262588ns | 297871ns | +88.79% |
| bitpack-contend-d32-padal | 258229ns | 255466ns | 234827ns | 257842ns | 282793ns | +83.49% |
| bitpack-contend-packed-simd | 202994ns | 190862ns | 159764ns | 195207ns | 269583ns | +44.24% |
| bitpack-contend-pipe4 | 128849ns | 113101ns | 101500ns | 116314ns | 193802ns | -8.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 140358ns | 117213ns | 194975ns | base | 29.883 |
| bitpack-contend-d16-padal | 65044ns | 44369ns | 105975ns | -53.66% | 64.484 |
| bitpack-contend-d32 | 264591ns | 241828ns | 296673ns | +88.51% | 15.852 |
| bitpack-contend-d32-padal | 257356ns | 234160ns | 281862ns | +83.36% | 16.298 |
| bitpack-contend-packed-simd | 202486ns | 159579ns | 268158ns | +44.26% | 20.714 |
| bitpack-contend-pipe4 | 128619ns | 101312ns | 193478ns | -8.36% | 32.610 |

## Performance model

- Peak throughput: **94.532 Gops/s** (bitpack-contend-d16-padal; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 32.366 | 34.2% |
| bitpack-contend-d16-padal | 77.484 | 82.0% |
| bitpack-contend-d32 | 16.331 | 17.3% |
| bitpack-contend-d32-padal | 16.499 | 17.5% |
| bitpack-contend-packed-simd | 22.015 | 23.3% |
| bitpack-contend-pipe4 | 37.145 | 39.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 140735ns | 140735ns | base |
| bitpack-contend-d16-padal | 65542ns | 65542ns | -53.43% |
| bitpack-contend-d32 | 265695ns | 265695ns | +88.79% |
| bitpack-contend-d32-padal | 258229ns | 258229ns | +83.49% |
| bitpack-contend-packed-simd | 202994ns | 202994ns | +44.24% |
| bitpack-contend-pipe4 | 128849ns | 128849ns | -8.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 129591ns | base | --- | [127987, 130980] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 54132ns | -76190.8ns (-58.8%) | [-81640, -67806]ns | [49382, 67546] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32 | 256826ns | +125753.0ns (+97.0%) | [+121550, +141393]ns | [248553, 273611] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 254210ns | +120759.5ns (+93.2%) | [+112933, +129453]ns | [250978, 262733] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 190522ns | +60015.2ns (+46.3%) | [+39484, +81902]ns | [178335, 205306] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 112916ns | -17003.2ns (-13.1%) | [-23822, -11804]ns | [108391, 119262] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 106360ns | -61.9% | +124.9% | +127.8% | +87.6% | -1.1% |
| 2 | 127110ns | -67.4% | +96.0% | +79.5% | +19.9% | -21.6% |
| 3 | 119527ns | -65.2% | +121.7% | +89.4% | +65.4% | -17.6% |
| 4 | 117283ns | -57.2% | +113.8% | +93.0% | +163.1% | +45.7% |
| 5 | 118859ns | -52.1% | +104.7% | +94.0% | +29.3% | -0.7% |
| 6 | 117468ns | -52.0% | +107.7% | +107.2% | +54.5% | +19.2% |
| 7 | 120783ns | -59.3% | +100.2% | +141.9% | +30.1% | +29.3% |
| 8 | 122510ns | -50.6% | +98.5% | +96.0% | +28.8% | -10.9% |
| 9 | 129632ns | -65.6% | +83.1% | +86.6% | +33.0% | -13.2% |
| 10 | 122764ns | -61.4% | +121.3% | +93.5% | +29.6% | -20.4% |
| 11 | 210321ns | -47.2% | +16.8% | +20.6% | -21.5% | -46.3% |
| 12 | 229490ns | -63.4% | +5.3% | +10.9% | -26.2% | -50.8% |
| 13 | 206297ns | -65.4% | +34.7% | +33.2% | -15.0% | -42.1% |
| 14 | 219141ns | -64.3% | +13.2% | +17.8% | -17.6% | -27.6% |
| 15 | 118457ns | -32.5% | +107.6% | +116.6% | +48.6% | +55.1% |
| 16 | 118968ns | -31.2% | +105.2% | +115.5% | +71.9% | +82.6% |
| 17 | 170483ns | -40.2% | +51.2% | +49.0% | +13.3% | -2.6% |
| 18 | 125426ns | -39.5% | +132.6% | +101.7% | +64.4% | +132.6% |
| 19 | 129285ns | -13.8% | +107.6% | +119.7% | +47.3% | +45.4% |
| 20 | 128943ns | -42.4% | +90.1% | +113.7% | +47.6% | -7.5% |
| 21 | 123333ns | -64.1% | +153.1% | +98.4% | +96.9% | -13.1% |
| 22 | 123118ns | -61.7% | +136.8% | +103.4% | +109.8% | -13.8% |
| 23 | 129588ns | -63.0% | +132.3% | +92.0% | +82.4% | -15.1% |
| 24 | 129595ns | -63.0% | +112.0% | +89.7% | +123.9% | -12.7% |
| 25 | 129653ns | -62.9% | +91.3% | +87.6% | +91.4% | -13.0% |
| 26 | 129509ns | -46.3% | +118.8% | +124.6% | +91.5% | -8.0% |
| 27 | 172158ns | -53.9% | +69.4% | +60.8% | +39.8% | -30.7% |
| 28 | 132496ns | -50.5% | +117.9% | +90.7% | +75.8% | -10.5% |
| 29 | 128863ns | -58.0% | +111.5% | +97.0% | +95.0% | -3.5% |
| 30 | 128950ns | -54.7% | +129.6% | +95.1% | +131.0% | +33.1% |
| 31 | 159520ns | -68.9% | +53.2% | +65.0% | +19.5% | -34.1% |
| 32 | 192391ns | -74.5% | +33.0% | +41.5% | -13.6% | -48.6% |
| 33 | 154129ns | -67.6% | +80.5% | +70.1% | +11.1% | -30.2% |
| 34 | 131066ns | -59.8% | +127.8% | +107.4% | +38.5% | -24.4% |
| 35 | 134572ns | -62.4% | +113.6% | +108.3% | +43.7% | -13.5% |
| 36 | 130648ns | -58.5% | +95.7% | +108.6% | +63.6% | -18.5% |
| 37 | 130352ns | -23.0% | +93.7% | +114.4% | +26.7% | -16.8% |
| 38 | 132378ns | +33.8% | +95.5% | +108.1% | +32.2% | -19.1% |
| 39 | 132018ns | -63.3% | +89.5% | +101.8% | +43.2% | -18.0% |
| 40 | 130893ns | -62.9% | +122.0% | +100.3% | +64.9% | -18.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.546 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-padal | 0.335 | moderate+ |
| bitpack-contend-d32 | 0.340 | moderate+ |
| bitpack-contend-d32-padal | 0.470 | moderate+ |
| bitpack-contend-packed-simd | 0.458 | moderate+ |
| bitpack-contend-pipe4 | 0.521 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 39/40, lost 1/40
- **bitpack-contend-d32**: won 0/40, lost 40/40
- **bitpack-contend-d32-padal**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 5/40, lost 35/40
- **bitpack-contend-pipe4**: won 32/40, lost 8/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 12.0ns | 140358.4ns | 0.0% |  |
| bitpack-contend-d16-padal | 10.6ns | 65043.7ns | 0.0% |  |
| bitpack-contend-d32 | 17.8ns | 264590.6ns | 0.0% |  |
| bitpack-contend-d32-padal | 6.9ns | 257356.0ns | 0.0% |  |
| bitpack-contend-packed-simd | 18.9ns | 202486.1ns | 0.0% |  |
| bitpack-contend-pipe4 | 5.7ns | 128619.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 117213.2-194975.0 ns)
  117213.2 |##################
  121101.3 |##########
  124989.4 |########
  128877.5 |########################################
  132765.5 |##
  136653.6 |
  140541.7 |
  144429.8 |
  148317.9 |
  152206.0 |##
  156094.1 |##
  159982.2 |
  163870.3 |
  167758.4 |##
  171646.5 |##
  175534.6 |
  179422.7 |
  183310.8 |
  187198.9 |
  191086.9 |##
  (1 below, 4 above range)

bitpack-contend-d16-padal (n=40, range 44369.1-105975.0 ns)
  44369.1 |############
  47449.4 |########################################
  50529.7 |########
  53610.0 |############
  56690.3 |########
  59770.6 |####
  62850.9 |####
  65931.2 |
  69011.5 |########
  72091.8 |####
  75172.0 |####
  78252.3 |############
  81332.6 |########
  84412.9 |
  87493.2 |
  90573.5 |
  93653.8 |
  96734.1 |
  99814.4 |########
  102894.7 |
  (4 below, 3 above range)

bitpack-contend-d32 (n=40, range 241828.3-296672.8 ns)
  241828.3 |########################################
  244570.5 |####################
  247312.7 |####################
  250055.0 |####################
  252797.2 |
  255539.4 |####################
  258281.6 |######
  261023.9 |
  263766.1 |######
  266508.3 |######
  269250.6 |######
  271992.8 |#############
  274735.0 |
  277477.2 |#############
  280219.5 |
  282961.7 |######
  285703.9 |######
  288446.1 |#############
  291188.4 |####################
  293930.6 |######
  (3 below, 3 above range)

bitpack-contend-d32-padal (n=40, range 234160.2-281862.1 ns)
  234160.2 |
  236545.3 |##########
  238930.4 |##########
  241315.5 |########################################
  243700.6 |####################
  246085.7 |
  248470.8 |####################
  250855.9 |##############################
  253241.0 |########################################
  255626.1 |####################
  258011.2 |##########
  260396.3 |####################
  262781.4 |##########
  265166.5 |##########
  267551.6 |
  269936.6 |####################
  272321.7 |##########
  274706.8 |########################################
  277091.9 |##########
  279477.0 |##########
  (4 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 159578.9-268157.8 ns)
  159578.9 |
  165007.8 |########################################
  170436.8 |########################################
  175865.7 |####################
  181294.7 |####################
  186723.6 |########################################
  192152.6 |####################
  197581.5 |####################
  203010.4 |####################
  208439.4 |##########
  213868.3 |##########
  219297.3 |
  224726.2 |
  230155.2 |##########
  235584.1 |####################
  241013.1 |##########
  246442.0 |##############################
  251871.0 |
  257299.9 |##########
  262728.9 |
  (5 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 101311.7-193478.3 ns)
  101311.7 |########
  105920.0 |########################################
  110528.4 |####################
  115136.7 |############################
  119745.0 |####
  124353.4 |
  128961.7 |
  133570.0 |
  138178.3 |####
  142786.7 |
  147395.0 |
  152003.3 |####
  156611.7 |####
  161220.0 |
  165828.3 |####
  170436.6 |########
  175045.0 |
  179653.3 |####
  184261.6 |####
  188869.9 |
  (5 below, 2 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=21.5% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-padal**: CV=40.4% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: CV=30.0% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: autocorrelation=0.52 (measurement drift or warm-up artifact)
