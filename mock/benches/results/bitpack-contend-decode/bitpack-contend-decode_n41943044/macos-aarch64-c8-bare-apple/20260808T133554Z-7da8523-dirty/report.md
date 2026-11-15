# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-control shows warm-up / thermal drift (autocorr +0.80)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16-control, bitpack-contend-d16} vs {bitpack-contend-packed-simd} (33% apart)

The field splits into a fast tier {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16-control, bitpack-contend-d16} and a slow tier {bitpack-contend-packed-simd} with a 33% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-packed-simd is inconsistent: worst-20% is 2.1x its best-20%

bitpack-contend-packed-simd's best 20% of batches run at 162.08 us but its worst 20% at 343.57 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 118434.1 ns median (-16.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.60x (fastest 118434.1 ns, slowest 189131.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 148246ns | 142105ns | 126846ns | 141594ns | 189603ns | base |
| bitpack-contend-d16-control | 152912ns | 135855ns | 118276ns | 141532ns | 221689ns | +3.15% |
| bitpack-contend-packed-simd | 224562ns | 189328ns | 162249ns | 205177ns | 345029ns | +51.48% |
| bitpack-contend-pipe2 | 132199ns | 126621ns | 104418ns | 126539ns | 176961ns | -10.82% |
| bitpack-contend-pipe4 | 127879ns | 118701ns | 103430ns | 119691ns | 176895ns | -13.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 147923ns | 126525ns | 189222ns | base | 28.355 |
| bitpack-contend-d16-control | 152563ns | 118006ns | 221102ns | +3.14% | 27.492 |
| bitpack-contend-packed-simd | 223936ns | 162083ns | 343570ns | +51.39% | 18.730 |
| bitpack-contend-pipe2 | 131984ns | 104274ns | 176684ns | -10.77% | 31.779 |
| bitpack-contend-pipe4 | 127622ns | 103247ns | 176580ns | -13.72% | 32.865 |

## Performance model

- Peak throughput: **40.624 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 29.578 | 72.8% |
| bitpack-contend-d16-control | 30.962 | 76.2% |
| bitpack-contend-packed-simd | 22.177 | 54.6% |
| bitpack-contend-pipe2 | 33.187 | 81.7% |
| bitpack-contend-pipe4 | 35.415 | 87.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 148246ns | 148246ns | base |
| bitpack-contend-d16-control | 152912ns | 152912ns | +3.15% |
| bitpack-contend-packed-simd | 224562ns | 224562ns | +51.48% |
| bitpack-contend-pipe2 | 132199ns | 132199ns | -10.82% |
| bitpack-contend-pipe4 | 127879ns | 127879ns | -13.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 141803ns | base | --- | [132956, 146040] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 135465ns | no significant difference | [-10108, +4780]ns | [129522, 149280] | no | 0.2682 | 0.2682 | 0 |
| bitpack-contend-packed-simd | 189131ns | +44748.8ns (+31.6%) | [+35953, +82720]ns | [178044, 238266] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 126384ns | -16532.4ns (-11.7%) | [-20960, -7008]ns | [116429, 132704] | YES | 0.0009 | 0.0007 | 0 |
| bitpack-contend-pipe4 | 118434ns | -26924.0ns (-19.0%) | [-32443, -21647]ns | [112260, 123369] | YES | 0.0004 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|
| 1 | 127549ns | -8.7% | +35.4% | -22.4% | -18.8% |
| 2 | 128318ns | -10.9% | +24.7% | -16.2% | +54.1% |
| 3 | 126625ns | -6.4% | +28.5% | -15.0% | -21.7% |
| 4 | 124012ns | -3.5% | +33.1% | +1.3% | +11.8% |
| 5 | 132521ns | -8.8% | +20.7% | -3.6% | -23.4% |
| 6 | 144133ns | -16.7% | +12.1% | -27.3% | -24.8% |
| 7 | 130348ns | -9.6% | +23.7% | -23.4% | -22.3% |
| 8 | 123246ns | -3.1% | +34.0% | -14.0% | -14.1% |
| 9 | 123572ns | -2.5% | +30.1% | -14.1% | +39.6% |
| 10 | 132135ns | -10.9% | +25.0% | -19.7% | -22.8% |
| 11 | 156722ns | -18.9% | +15.6% | -11.6% | -32.0% |
| 12 | 173021ns | -26.1% | +13.6% | +30.6% | -34.2% |
| 13 | 146082ns | -7.4% | +36.7% | -4.7% | -18.0% |
| 14 | 129922ns | +3.9% | +35.7% | -2.6% | -4.9% |
| 15 | 129535ns | +4.9% | +34.2% | -8.5% | -15.0% |
| 16 | 134238ns | +3.4% | +35.3% | +27.1% | -12.5% |
| 17 | 149335ns | -13.5% | +21.9% | +5.1% | -21.9% |
| 18 | 145073ns | -6.9% | +24.0% | -10.3% | +0.0% |
| 19 | 139472ns | +7.4% | +25.6% | -7.3% | -16.1% |
| 20 | 139463ns | -2.7% | +26.1% | +26.0% | -0.4% |
| 21 | 245150ns | -10.5% | -21.2% | -53.1% | -44.1% |
| 22 | 243580ns | +1.0% | -22.6% | -54.9% | -54.6% |
| 23 | 173784ns | +26.5% | +9.2% | -28.9% | -16.2% |
| 24 | 176667ns | +23.5% | +3.4% | -12.1% | -34.0% |
| 25 | 161493ns | +26.1% | +37.7% | -3.3% | -25.9% |
| 26 | 178360ns | +18.6% | +62.7% | -9.6% | -26.2% |
| 27 | 130925ns | +45.2% | +198.4% | +0.4% | +90.8% |
| 28 | 145997ns | +14.5% | +74.4% | -4.7% | +52.0% |
| 29 | 145656ns | +62.2% | +79.3% | -10.9% | -15.5% |
| 30 | 145820ns | +46.7% | +76.0% | -8.2% | -14.3% |
| 31 | 133392ns | +24.4% | +122.8% | -5.4% | -19.8% |
| 32 | 138229ns | -4.2% | +101.9% | -15.3% | -23.3% |
| 33 | 149599ns | -8.1% | +69.8% | -22.6% | -21.9% |
| 34 | 129706ns | +14.7% | +153.4% | -18.4% | +5.1% |
| 35 | 129340ns | -0.0% | +157.9% | -17.6% | -17.6% |
| 36 | 144238ns | -10.0% | +86.4% | -25.6% | -23.6% |
| 37 | 161719ns | -2.8% | +192.9% | -25.8% | -25.3% |
| 38 | 137519ns | -2.4% | +145.5% | +21.0% | +2.4% |
| 39 | 154773ns | +3.9% | +91.8% | +1.6% | -22.5% |
| 40 | 155640ns | -6.5% | +29.3% | +27.9% | -23.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.573 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.800 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.708 | HIGH+ (drift/warm-up) |
| bitpack-contend-pipe2 | 0.371 | moderate+ |
| bitpack-contend-pipe4 | 0.197 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 23/40, lost 16/40
- **bitpack-contend-packed-simd**: won 2/40, lost 38/40
- **bitpack-contend-pipe2**: won 31/40, lost 9/40
- **bitpack-contend-pipe4**: won 32/40, lost 7/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 7.8ns | 147922.7ns | 0.0% |  |
| bitpack-contend-d16-control | 11.9ns | 152563.5ns | 0.0% |  |
| bitpack-contend-packed-simd | 29.5ns | 223936.4ns | 0.0% |  |
| bitpack-contend-pipe2 | 5.9ns | 131984.1ns | 0.0% |  |
| bitpack-contend-pipe4 | 6.4ns | 127622.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 126524.8-189221.6 ns)
  126524.8 |#################################
  129659.6 |########################################
  132794.5 |#############
  135929.3 |#############
  139064.2 |#############
  142199.0 |####################
  145333.8 |##########################
  148468.7 |#############
  151603.5 |
  154738.4 |####################
  157873.2 |
  161008.0 |#############
  164142.9 |
  167277.7 |
  170412.6 |######
  173547.4 |#############
  176682.3 |######
  179817.1 |
  182951.9 |
  186086.8 |
  (3 below, 2 above range)

bitpack-contend-d16-control (n=40, range 118005.8-221101.6 ns)
  118005.8 |##################################
  123160.6 |###########
  128315.4 |######################
  133470.2 |########################################
  138625.0 |#####
  143779.8 |###########
  148934.6 |#####
  154089.4 |#####
  159244.1 |#####
  164398.9 |###########
  169553.7 |
  174708.5 |
  179863.3 |
  185018.1 |#####
  190172.9 |
  195327.7 |
  200482.5 |#####
  205637.2 |
  210792.0 |###########
  215946.8 |#################
  (4 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 162083.3-343570.4 ns)
  162083.3 |##########################
  171157.7 |########################################
  180232.0 |#################################
  189306.4 |####################
  198380.7 |#############
  207455.1 |
  216529.4 |######
  225603.8 |
  234678.1 |
  243752.5 |
  252826.8 |##########################
  261901.2 |######
  270975.5 |######
  280049.9 |
  289124.3 |####################
  298198.6 |
  307273.0 |
  316347.3 |
  325421.7 |#############
  334496.0 |######
  (5 below, 2 above range)

bitpack-contend-pipe2 (n=40, range 104274.1-176684.4 ns)
  104274.1 |########################################
  107894.6 |####
  111515.1 |####
  115135.6 |#############
  118756.1 |####
  122376.6 |########
  125997.2 |#################
  129617.7 |#############
  133238.2 |####
  136858.7 |#############
  140479.2 |
  144099.8 |
  147720.3 |
  151340.8 |
  154961.3 |#################
  158581.8 |####
  162202.4 |
  165822.9 |####
  169443.4 |####
  173063.9 |####
  (2 below, 2 above range)

bitpack-contend-pipe4 (n=40, range 103246.8-176579.9 ns)
  103246.8 |########################################
  106913.5 |################################
  110580.1 |################
  114246.8 |########################################
  117913.4 |########################################
  121580.1 |########################
  125246.7 |
  128913.4 |########
  132580.0 |
  136246.7 |################################
  139913.4 |########
  143580.0 |################
  147246.7 |
  150913.3 |
  154580.0 |
  158246.6 |
  161913.3 |
  165579.9 |
  169246.6 |########
  172913.2 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: CV=25.0% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: CV=32.4% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe2**: CV=21.1% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: CV=24.8% (high variance, measurements may be unstable)
