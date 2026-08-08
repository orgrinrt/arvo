# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 is an outlier: 4.6x slower than the field

bitpack-contend-d64 (575.88 us) is 4.6x the fastest (125.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-packed-simd shows warm-up / thermal drift (autocorr +0.76)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation +0.76, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-contend-d16)

The baseline bitpack-contend-d16 is the fastest (125.53 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d32} vs {bitpack-contend-d64} (126% apart)

The field splits into a fast tier {bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 126% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-contend-d16 (125.53 us) to slowest bitpack-contend-d64 (575.88 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-contend-packed is inconsistent: worst-20% is 1.7x its best-20%

bitpack-contend-packed's best 20% of batches run at 177.49 us but its worst 20% at 293.39 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-contend-d16) is the fastest** at 125529.1 ns median
- 4 variants significantly slower than baseline
- Spread: 4.59x (fastest 125529.1 ns, slowest 575875.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 127924ns | 125801ns | 121019ns | 126941ns | 137777ns | base |
| bitpack-contend-d16-control | 137523ns | 131117ns | 120042ns | 131202ns | 173968ns | +7.50% |
| bitpack-contend-d32 | 266507ns | 256268ns | 246157ns | 258256ns | 311609ns | +108.33% |
| bitpack-contend-d64 | 583220ns | 577546ns | 560878ns | 579948ns | 615379ns | +355.91% |
| bitpack-contend-packed | 217802ns | 197814ns | 177776ns | 205685ns | 294179ns | +70.26% |
| bitpack-contend-packed-simd | 178119ns | 169431ns | 164655ns | 171295ns | 212055ns | +39.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 127606ns | 120658ns | 137361ns | base | 32.869 |
| bitpack-contend-d16-control | 137165ns | 119739ns | 173638ns | +7.49% | 30.579 |
| bitpack-contend-d32 | 265150ns | 245084ns | 310084ns | +107.79% | 15.819 |
| bitpack-contend-d64 | 581644ns | 559086ns | 613774ns | +355.81% | 7.211 |
| bitpack-contend-packed | 217357ns | 177492ns | 293393ns | +70.33% | 19.297 |
| bitpack-contend-packed-simd | 177692ns | 164329ns | 211470ns | +39.25% | 23.604 |

## Performance model

- Peak throughput: **35.029 Gops/s** (bitpack-contend-d16-control; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 33.413 | 95.4% |
| bitpack-contend-d16-control | 32.109 | 91.7% |
| bitpack-contend-d32 | 16.438 | 46.9% |
| bitpack-contend-d64 | 7.283 | 20.8% |
| bitpack-contend-packed | 21.231 | 60.6% |
| bitpack-contend-packed-simd | 24.811 | 70.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 127924ns | 127924ns | base |
| bitpack-contend-d16-control | 137523ns | 137523ns | +7.50% |
| bitpack-contend-d32 | 266507ns | 266507ns | +108.33% |
| bitpack-contend-d64 | 583220ns | 583220ns | +355.91% |
| bitpack-contend-packed | 217802ns | 217802ns | +70.26% |
| bitpack-contend-packed-simd | 178119ns | 178119ns | +39.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 125529ns | base | --- | [123369, 130484] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 130628ns | no significant difference | [-646, +7466]ns | [128578, 133208] | no | 0.4296 | 0.4296 | 0 |
| bitpack-contend-d32 | 255162ns | +129359.6ns (+103.1%) | [+125699, +135679]ns | [252361, 261283] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 575876ns | +448763.8ns (+357.5%) | [+440789, +461152]ns | [571612, 584666] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 197558ns | +70129.8ns (+55.9%) | [+64638, +86401]ns | [194224, 214766] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 169048ns | +44852.9ns (+35.7%) | [+42186, +47205]ns | [166660, 174849] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 140748ns | -13.3% | +81.6% | +356.5% | +24.6% | +14.5% |
| 2 | 120863ns | +3.6% | +106.3% | +388.2% | +45.1% | +33.8% |
| 3 | 121781ns | +6.2% | +108.1% | +379.2% | +62.7% | +35.8% |
| 4 | 122283ns | +4.6% | +112.7% | +356.0% | +100.9% | +35.3% |
| 5 | 123335ns | -3.4% | +105.6% | +357.7% | +177.0% | +33.8% |
| 6 | 123534ns | -3.0% | +106.4% | +367.4% | +103.4% | +37.1% |
| 7 | 123294ns | -0.3% | +104.2% | +351.9% | +132.8% | +44.3% |
| 8 | 121458ns | +1.0% | +107.4% | +361.7% | +152.7% | +36.8% |
| 9 | 123402ns | -0.6% | +106.9% | +373.4% | +120.8% | +34.2% |
| 10 | 123315ns | -2.4% | +114.7% | +356.5% | +100.6% | +37.7% |
| 11 | 118309ns | -0.3% | +121.5% | +399.4% | +65.5% | +42.5% |
| 12 | 119840ns | -1.7% | +148.1% | +371.3% | +70.8% | +42.0% |
| 13 | 136475ns | -13.5% | +86.8% | +337.8% | +39.2% | +23.5% |
| 14 | 124200ns | +2.5% | +113.4% | +420.9% | +52.2% | +35.8% |
| 15 | 129455ns | +1.7% | +107.3% | +343.9% | +45.8% | +31.4% |
| 16 | 134434ns | -4.4% | +108.6% | +327.6% | +39.3% | +30.9% |
| 17 | 133950ns | -2.5% | +263.2% | +351.9% | +48.1% | +29.7% |
| 18 | 133986ns | -1.4% | +133.7% | +327.4% | +79.6% | +31.5% |
| 19 | 136374ns | -4.2% | +92.5% | +342.1% | +149.0% | +31.0% |
| 20 | 132725ns | -0.4% | +96.2% | +339.4% | +126.6% | +38.6% |
| 21 | 120898ns | +8.0% | +103.7% | +359.1% | +45.4% | +37.9% |
| 22 | 121240ns | +83.5% | +102.8% | +382.6% | +48.6% | +37.2% |
| 23 | 120977ns | +14.5% | +105.7% | +359.9% | +52.8% | +37.7% |
| 24 | 121681ns | +10.8% | +100.4% | +392.2% | +57.8% | +37.1% |
| 25 | 121857ns | +8.9% | +112.1% | +394.9% | +57.4% | +36.3% |
| 26 | 124447ns | +3.8% | +94.9% | +349.7% | +75.0% | +32.7% |
| 27 | 126611ns | +11.0% | +91.4% | +345.7% | +54.4% | +31.1% |
| 28 | 126681ns | +14.3% | +95.0% | +355.4% | +69.4% | +47.7% |
| 29 | 124390ns | +50.4% | +121.6% | +358.8% | +73.6% | +39.6% |
| 30 | 130603ns | +95.5% | +86.7% | +363.2% | +64.6% | +36.7% |
| 31 | 123818ns | +8.0% | +101.8% | +369.5% | +33.0% | +73.3% |
| 32 | 139185ns | -3.7% | +81.6% | +312.6% | +26.8% | +66.9% |
| 33 | 142533ns | -8.4% | +76.5% | +298.1% | +42.0% | +49.7% |
| 34 | 126846ns | +12.9% | +97.2% | +352.6% | +53.4% | +85.1% |
| 35 | 131216ns | +4.0% | +93.2% | +361.5% | +48.0% | +63.7% |
| 36 | 130796ns | +5.6% | +104.4% | +352.7% | +50.6% | +51.6% |
| 37 | 130364ns | +6.6% | +108.1% | +339.2% | +49.0% | +27.1% |
| 38 | 130303ns | -1.3% | +103.4% | +332.2% | +50.4% | +26.7% |
| 39 | 130890ns | +0.3% | +120.0% | +355.8% | +62.7% | +27.6% |
| 40 | 135150ns | +15.9% | +92.2% | +321.1% | +79.1% | +45.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.390 | moderate+ |
| bitpack-contend-d16-control | 0.309 | moderate+ |
| bitpack-contend-d32 | 0.295 | moderate+ |
| bitpack-contend-d64 | 0.021 | ok |
| bitpack-contend-packed | 0.592 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.757 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 17/40, lost 23/40
- **bitpack-contend-d32**: won 0/40, lost 40/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 4.4ns | 127606.2ns | 0.0% |  |
| bitpack-contend-d16-control | 5.9ns | 137164.5ns | 0.0% |  |
| bitpack-contend-d32 | 38.8ns | 265149.9ns | 0.0% |  |
| bitpack-contend-d64 | 15.2ns | 581644.4ns | 0.0% |  |
| bitpack-contend-packed | 14.6ns | 217357.0ns | 0.0% |  |
| bitpack-contend-packed-simd | 10.0ns | 177691.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 120658.3-137360.6 ns)
  120658.3 |#################################
  121493.4 |##########################
  122328.5 |
  123163.7 |########################################
  123998.8 |####################
  124833.9 |
  125669.0 |
  126504.1 |####################
  127339.2 |
  128174.3 |
  129009.4 |######
  129844.6 |####################
  130679.7 |####################
  131514.8 |
  132349.9 |######
  133185.0 |#############
  134020.1 |######
  134855.2 |######
  135690.3 |#############
  136525.5 |
  (2 below, 3 above range)

bitpack-contend-d16-control (n=40, range 119738.6-173638.1 ns)
  119738.6 |#############
  122433.6 |#############
  125128.6 |########
  127823.6 |######################
  130518.5 |########################################
  133213.5 |#############
  135908.5 |#############
  138603.5 |########
  141298.4 |####
  143993.4 |####
  146688.4 |
  149383.4 |
  152078.3 |
  154773.3 |####
  157468.3 |
  160163.3 |
  162858.2 |
  165553.2 |
  168248.2 |
  170943.2 |
  (4 below, 3 above range)

bitpack-contend-d32 (n=40, range 245084.0-310084.2 ns)
  245084.0 |#################
  248334.0 |######################
  251584.0 |########################################
  254834.0 |######################
  258084.0 |######################
  261334.1 |###########
  264584.1 |######################
  267834.1 |#####
  271084.1 |#####
  274334.1 |#####
  277584.1 |#####
  280834.1 |
  284084.1 |
  287334.1 |#####
  290584.2 |
  293834.2 |
  297084.2 |#####
  300334.2 |
  303584.2 |
  306834.2 |
  (4 below, 2 above range)

bitpack-contend-d64 (n=40, range 559086.0-613774.5 ns)
  559086.0 |####################
  561820.4 |########################################
  564554.8 |##########
  567289.3 |####################
  570023.7 |##############################
  572758.1 |########################################
  575492.5 |####################
  578227.0 |
  580961.4 |##############################
  583695.8 |####################
  586430.2 |
  589164.7 |####################
  591899.1 |##########
  594633.5 |##########
  597367.9 |####################
  600102.3 |
  602836.8 |########################################
  605571.2 |##########
  608305.6 |
  611040.0 |
  (4 below, 2 above range)

bitpack-contend-packed (n=40, range 177492.1-293393.3 ns)
  177492.1 |######
  183287.2 |##########################
  189082.3 |########################################
  194877.3 |########################################
  200672.4 |#############
  206467.4 |
  212262.5 |#################################
  218057.6 |
  223852.6 |
  229647.7 |
  235442.7 |######
  241237.8 |#############
  247032.9 |#############
  252827.9 |
  258623.0 |
  264418.0 |
  270213.1 |######
  276008.2 |
  281803.2 |######
  287598.3 |
  (5 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 164329.3-211469.7 ns)
  164329.3 |########################################
  166686.3 |####################
  169043.3 |#############
  171400.3 |###
  173757.4 |######
  176114.4 |######
  178471.4 |######
  180828.4 |
  183185.5 |###
  185542.5 |###
  187899.5 |
  190256.5 |
  192613.5 |
  194970.6 |###
  197327.6 |###
  199684.6 |
  202041.6 |
  204398.7 |
  206755.7 |
  209112.7 |
  (2 below, 5 above range)

```

## Diagnostics

- **bitpack-contend-packed**: CV=20.4% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.76 (measurement drift or warm-up artifact)
