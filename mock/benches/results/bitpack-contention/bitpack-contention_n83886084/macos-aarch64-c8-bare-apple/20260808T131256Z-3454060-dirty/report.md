# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 is an outlier: 4.0x slower than the field

bitpack-contend-d64 (1.24 ms) is 4.0x the fastest (312.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d16 shows warm-up / thermal drift (autocorr +0.75)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation +0.75, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-contend-d16)

The baseline bitpack-contend-d16 is the fastest (312.65 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d32} vs {bitpack-contend-d64} (107% apart)

The field splits into a fast tier {bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 107% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest bitpack-contend-d16 (312.65 us) to slowest bitpack-contend-d64 (1.24 ms): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-contend-packed is inconsistent: worst-20% is 1.7x its best-20%

bitpack-contend-packed's best 20% of batches run at 396.55 us but its worst 20% at 685.98 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-contend-d16) is the fastest** at 312645.7 ns median
- 4 variants significantly slower than baseline
- Spread: 3.96x (fastest 312645.7 ns, slowest 1236972.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 347962ns | 313457ns | 279738ns | 327989ns | 476108ns | base |
| bitpack-contend-d16-control | 327640ns | 326620ns | 285258ns | 323912ns | 381206ns | -5.84% |
| bitpack-contend-d32 | 609754ns | 597842ns | 561069ns | 603957ns | 675832ns | +75.24% |
| bitpack-contend-d64 | 1225149ns | 1238566ns | 1131204ns | 1222299ns | 1327644ns | +252.09% |
| bitpack-contend-packed | 502233ns | 474425ns | 397817ns | 474944ns | 688515ns | +44.34% |
| bitpack-contend-packed-simd | 391074ns | 389047ns | 361059ns | 385368ns | 438209ns | +12.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 346633ns | 278479ns | 473754ns | base | 24.200 |
| bitpack-contend-d16-control | 326392ns | 283612ns | 379887ns | -5.84% | 25.701 |
| bitpack-contend-d32 | 608251ns | 559139ns | 674527ns | +75.47% | 13.791 |
| bitpack-contend-d64 | 1223066ns | 1129092ns | 1325336ns | +252.84% | 6.859 |
| bitpack-contend-packed | 500592ns | 396554ns | 685977ns | +44.42% | 16.757 |
| bitpack-contend-packed-simd | 390319ns | 360469ns | 437260ns | +12.60% | 21.492 |

## Performance model

- Peak throughput: **30.123 Gops/s** (bitpack-contend-d16; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 26.831 | 89.1% |
| bitpack-contend-d16-control | 25.757 | 85.5% |
| bitpack-contend-d32 | 14.056 | 46.7% |
| bitpack-contend-d64 | 6.782 | 22.5% |
| bitpack-contend-packed | 17.756 | 58.9% |
| bitpack-contend-packed-simd | 21.606 | 71.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 347962ns | 347962ns | base |
| bitpack-contend-d16-control | 327640ns | 327640ns | -5.84% |
| bitpack-contend-d32 | 609754ns | 609754ns | +75.24% |
| bitpack-contend-d64 | 1225149ns | 1225149ns | +252.09% |
| bitpack-contend-packed | 502233ns | 502233ns | +44.34% |
| bitpack-contend-packed-simd | 391074ns | 391074ns | +12.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 312646ns | base | --- | [305615, 340650] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 325685ns | no significant difference | [-14166, +6582]ns | [300448, 341721] | no | 0.4296 | 0.4296 | 0 |
| bitpack-contend-d32 | 596800ns | +266757.7ns (+85.3%) | [+247308, +273854]ns | [581683, 630205] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 1236972ns | +862955.0ns (+276.0%) | [+841824, +891888]ns | [1179781, 1258208] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 472433ns | +139243.8ns (+44.5%) | [+105160, +158964]ns | [424505, 501692] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 388252ns | +60831.0ns (+19.5%) | [+43206, +68849]ns | [374662, 392865] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 465415ns | -38.3% | +32.6% | +169.5% | -12.1% | -13.1% |
| 2 | 445849ns | -13.8% | +41.9% | +183.1% | +13.7% | +0.1% |
| 3 | 462663ns | -28.0% | +38.5% | +164.3% | +44.7% | -8.5% |
| 4 | 493895ns | -40.0% | +32.9% | +160.3% | +44.4% | -18.7% |
| 5 | 460878ns | -33.8% | +43.4% | +198.8% | +100.8% | -3.3% |
| 6 | 436308ns | -18.9% | +45.8% | +200.0% | +24.5% | -8.5% |
| 7 | 468448ns | -27.1% | +36.6% | +181.0% | +34.4% | -16.8% |
| 8 | 501939ns | -31.9% | +27.5% | +146.1% | +11.8% | -21.2% |
| 9 | 404158ns | -13.6% | +66.9% | +220.3% | +21.2% | +11.3% |
| 10 | 490945ns | -31.5% | +55.0% | +165.0% | -10.7% | -15.9% |
| 11 | 280682ns | -2.4% | +98.2% | +304.8% | +41.3% | +41.4% |
| 12 | 290973ns | -5.1% | +91.0% | +294.4% | +36.3% | +33.7% |
| 13 | 283450ns | -2.1% | +96.0% | +303.1% | +41.4% | +39.6% |
| 14 | 288649ns | -3.2% | +92.4% | +290.7% | +39.9% | +35.0% |
| 15 | 299940ns | -1.1% | +105.5% | +276.7% | +32.2% | +45.2% |
| 16 | 302847ns | +21.1% | +87.8% | +317.3% | +32.6% | +58.6% |
| 17 | 308704ns | -0.4% | +84.1% | +285.3% | +28.0% | +25.8% |
| 18 | 313432ns | +2.1% | +86.1% | +295.3% | +26.2% | +21.9% |
| 19 | 344117ns | -3.7% | +65.3% | +282.0% | +15.0% | +9.7% |
| 20 | 337464ns | +47.9% | +66.9% | +250.3% | +17.3% | +9.0% |
| 21 | 277534ns | +7.4% | +110.2% | +349.8% | +55.2% | +24.8% |
| 22 | 279042ns | +7.0% | +110.7% | +380.7% | +80.7% | +30.0% |
| 23 | 310910ns | -5.3% | +115.1% | +278.3% | +99.3% | +23.1% |
| 24 | 299420ns | -3.9% | +123.5% | +316.5% | +102.1% | +23.0% |
| 25 | 305207ns | +2.0% | +117.6% | +274.9% | +120.5% | +19.7% |
| 26 | 306023ns | -4.4% | +109.8% | +265.5% | +93.0% | +20.4% |
| 27 | 305000ns | -1.1% | +95.2% | +289.8% | +105.2% | +23.2% |
| 28 | 288312ns | +4.7% | +121.2% | +298.7% | +119.3% | +27.9% |
| 29 | 343836ns | -13.0% | +76.2% | +268.5% | +27.2% | +12.9% |
| 30 | 348334ns | -14.7% | +68.2% | +257.5% | +16.1% | +12.0% |
| 31 | 262263ns | +34.8% | +113.8% | +372.4% | +55.4% | +33.7% |
| 32 | 267901ns | +27.4% | +113.1% | +322.7% | +56.1% | +33.3% |
| 33 | 326502ns | +7.8% | +76.0% | +244.5% | +40.8% | +11.9% |
| 34 | 346401ns | +1.7% | +67.6% | +239.9% | +30.2% | +9.0% |
| 35 | 311472ns | +11.4% | +87.1% | +331.6% | +49.2% | +18.7% |
| 36 | 311859ns | +13.9% | +79.1% | +308.0% | +60.0% | +19.8% |
| 37 | 308397ns | +13.5% | +103.5% | +321.3% | +56.4% | +20.9% |
| 38 | 319682ns | +12.2% | +93.3% | +279.5% | +50.2% | +15.6% |
| 39 | 331298ns | +2.0% | +80.5% | +242.1% | +45.3% | +17.2% |
| 40 | 335184ns | +9.6% | +69.4% | +237.0% | +46.9% | +19.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.748 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.189 | ok |
| bitpack-contend-d32 | 0.504 | HIGH+ (drift/warm-up) |
| bitpack-contend-d64 | 0.399 | moderate+ |
| bitpack-contend-packed | 0.685 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.556 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 23/40, lost 17/40
- **bitpack-contend-d32**: won 0/40, lost 40/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 2/40, lost 38/40
- **bitpack-contend-packed-simd**: won 8/40, lost 32/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 65.1ns | 346633.3ns | 0.0% |  |
| bitpack-contend-d16-control | 21.3ns | 326392.5ns | 0.0% |  |
| bitpack-contend-d32 | 19.6ns | 608250.8ns | 0.0% |  |
| bitpack-contend-d64 | 86.9ns | 1223065.5ns | 0.0% |  |
| bitpack-contend-packed | 87.5ns | 500592.1ns | 0.0% |  |
| bitpack-contend-packed-simd | 13.5ns | 390319.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 278479.2-473753.7 ns)
  278479.2 |####################
  288242.9 |####################
  298006.7 |########################################
  307770.4 |########################################
  317534.1 |#############
  327297.8 |#############
  337061.6 |##########################
  346825.3 |######
  356589.0 |
  366352.7 |
  376116.5 |
  385880.2 |
  395643.9 |######
  405407.6 |
  415171.4 |
  424935.1 |
  434698.8 |######
  444462.5 |######
  454226.3 |#############
  463990.0 |#############
  (3 below, 3 above range)

bitpack-contend-d16-control (n=40, range 283612.3-379887.3 ns)
  283612.3 |################
  288426.0 |########
  293239.8 |################################
  298053.5 |########################################
  302867.3 |################
  307681.0 |########
  312494.8 |
  317308.5 |########
  322122.3 |
  326936.0 |########
  331749.8 |################
  336563.6 |################
  341377.3 |################
  346191.1 |########################
  351004.8 |########################################
  355818.6 |########
  360632.3 |
  365446.1 |################
  370259.8 |
  375073.6 |
  (4 below, 2 above range)

bitpack-contend-d32 (n=40, range 559138.6-674527.1 ns)
  559138.6 |################
  564908.0 |################################
  570677.4 |################
  576446.8 |########
  582216.3 |########################################
  587985.7 |
  593755.1 |################
  599524.6 |
  605294.0 |########
  611063.4 |########
  616832.8 |################
  622602.3 |########
  628371.7 |########
  634141.1 |################
  639910.6 |################################
  645680.0 |
  651449.4 |########
  657218.8 |########
  662988.3 |################
  668757.7 |################
  (5 below, 1 above range)

bitpack-contend-d64 (n=40, range 1129091.7-1325336.2 ns)
  1129091.7 |########################################
  1138903.9 |########################
  1148716.2 |########
  1158528.4 |
  1168340.6 |################
  1178152.8 |########
  1187965.1 |################
  1197777.3 |
  1207589.5 |########
  1217401.7 |########
  1227214.0 |########
  1237026.2 |########################
  1246838.4 |########################
  1256650.6 |################
  1266462.8 |################
  1276275.1 |########
  1286087.3 |########
  1295899.5 |################
  1305711.7 |################
  1315524.0 |########
  (3 below, 3 above range)

bitpack-contend-packed (n=40, range 396553.8-685976.8 ns)
  396553.8 |########################################
  411025.0 |#####
  425496.1 |###############
  439967.3 |#####
  454438.4 |##########
  468909.6 |###############
  483380.7 |##########
  497851.8 |###############
  512323.0 |
  526794.1 |
  541265.3 |#####
  555736.4 |#####
  570207.6 |
  584678.7 |#####
  599149.9 |#####
  613621.0 |##########
  628092.2 |##########
  642563.3 |
  657034.5 |#####
  671505.6 |#####
  (5 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 360469.0-437260.1 ns)
  360469.0 |#####
  364308.6 |#################
  368148.2 |############################
  371987.7 |###########
  375827.3 |#################
  379666.8 |###########
  383506.4 |
  387345.9 |########################################
  391185.5 |
  395025.0 |#################
  398864.6 |#################
  402704.2 |#####
  406543.7 |
  410383.3 |#####
  414222.8 |
  418062.4 |
  421901.9 |#####
  425741.5 |
  429581.0 |
  433420.6 |#####
  (3 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=20.6% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **bitpack-contend-packed**: CV=23.0% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: autocorrelation=0.68 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.56 (measurement drift or warm-up artifact)
