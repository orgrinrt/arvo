# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bitpack-carrier-d16, bitpack-carrier-d16-control) are a dead heat (<1%)

bitpack-carrier-d16 (177.83 us) and bitpack-carrier-d16-control (179.02 us) differ by 0.67%, inside the noise, even though the wider field spreads 51.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-carrier-d32 shows warm-up / thermal drift (autocorr +0.80)

bitpack-carrier-d32's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-carrier-d16)

The baseline bitpack-carrier-d16 is the fastest (177.83 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d32} vs {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d64} (38% apart)

The field splits into a fast tier {bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d32} and a slow tier {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d64} with a 38% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (bitpack-carrier-d16) is the fastest** at 177825.6 ns median
- 4 variants significantly slower than baseline
- Spread: 1.51x (fastest 177825.6 ns, slowest 268453.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 178782ns | 178126ns | 177848ns | 178314ns | 181122ns | base |
| bitpack-carrier-d16-control | 179991ns | 179310ns | 177866ns | 179641ns | 183169ns | +0.68% |
| bitpack-carrier-d32 | 183877ns | 182631ns | 179243ns | 183006ns | 191125ns | +2.85% |
| bitpack-carrier-d64 | 291380ns | 270676ns | 265875ns | 279967ns | 351125ns | +62.98% |
| bitpack-carrier-packed | 268577ns | 267104ns | 266648ns | 267198ns | 274642ns | +50.23% |
| bitpack-carrier-packed-simd | 251951ns | 251113ns | 250689ns | 251251ns | 255311ns | +40.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 178481ns | 177602ns | 180820ns | base | 11.750 |
| bitpack-carrier-d16-control | 179734ns | 177641ns | 182977ns | +0.70% | 11.668 |
| bitpack-carrier-d32 | 183451ns | 178792ns | 190691ns | +2.78% | 11.432 |
| bitpack-carrier-d64 | 289652ns | 263709ns | 349979ns | +62.29% | 7.240 |
| bitpack-carrier-packed | 268155ns | 266233ns | 274353ns | +50.24% | 7.821 |
| bitpack-carrier-packed-simd | 251548ns | 250306ns | 255005ns | +40.94% | 8.337 |

## Performance model

- Peak throughput: **11.808 Gops/s** (bitpack-carrier-d16; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 11.793 | 99.9% |
| bitpack-carrier-d16-control | 11.714 | 99.2% |
| bitpack-carrier-d32 | 11.511 | 97.5% |
| bitpack-carrier-d64 | 7.812 | 66.2% |
| bitpack-carrier-packed | 7.864 | 66.6% |
| bitpack-carrier-packed-simd | 8.369 | 70.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 178782ns | 178782ns | base |
| bitpack-carrier-d16-control | 179991ns | 179991ns | +0.68% |
| bitpack-carrier-d32 | 183877ns | 183877ns | +2.85% |
| bitpack-carrier-d64 | 291380ns | 291380ns | +62.98% |
| bitpack-carrier-packed | 268577ns | 268577ns | +50.23% |
| bitpack-carrier-packed-simd | 251951ns | 251951ns | +40.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 177826ns | base | --- | [177761, 178000] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 179024ns | no significant difference | [-61, +1677]ns | [178229, 180695] | no | 0.1539 | 0.1539 | 0 |
| bitpack-carrier-d32 | 182182ns | +4328.7ns (+2.4%) | [+2782, +5958]ns | [181947, 183848] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 268453ns | +90574.1ns (+50.9%) | [+89203, +101797]ns | [267482, 279602] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 266664ns | +88782.7ns (+49.9%) | [+88547, +89043]ns | [266448, 266867] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 250601ns | +72778.3ns (+40.9%) | [+72592, +73134]ns | [250514, 250988] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 178000ns | -0.2% | +0.4% | +52.5% | +49.7% | +44.3% |
| 2 | 177651ns | +0.8% | +0.9% | +50.0% | +49.8% | +44.5% |
| 3 | 177820ns | +1.0% | +0.5% | +95.2% | +51.3% | +44.5% |
| 4 | 177607ns | +0.8% | +0.6% | +51.0% | +51.2% | +43.1% |
| 5 | 177648ns | +0.6% | +0.6% | +50.6% | +50.2% | +41.1% |
| 6 | 177780ns | -0.1% | +0.6% | +48.6% | +50.1% | +40.9% |
| 7 | 177621ns | +0.0% | +0.7% | +48.4% | +50.0% | +40.9% |
| 8 | 177568ns | +0.2% | +0.7% | +52.1% | +54.0% | +41.4% |
| 9 | 180480ns | -1.5% | -1.0% | +48.4% | +59.3% | +38.8% |
| 10 | 180302ns | -1.4% | -0.7% | +46.1% | +57.8% | +38.9% |
| 11 | 177598ns | +1.8% | +3.3% | +79.6% | +49.9% | +42.5% |
| 12 | 177728ns | +1.9% | +3.0% | +102.9% | +49.9% | +41.5% |
| 13 | 177719ns | +1.7% | +3.8% | +65.8% | +49.8% | +41.1% |
| 14 | 177700ns | +1.9% | +3.5% | +50.1% | +49.9% | +41.0% |
| 15 | 181551ns | -0.4% | +0.2% | +46.4% | +48.1% | +38.0% |
| 16 | 183374ns | -1.5% | -0.6% | +44.4% | +45.2% | +36.6% |
| 17 | 180923ns | -0.1% | +0.6% | +44.0% | +47.2% | +38.4% |
| 18 | 179165ns | +0.9% | +1.4% | +48.2% | +48.8% | +39.8% |
| 19 | 177875ns | +1.8% | +2.3% | +49.1% | +49.8% | +40.7% |
| 20 | 177960ns | +1.5% | +2.6% | +49.6% | +49.6% | +40.8% |
| 21 | 179738ns | -0.1% | +4.2% | +69.8% | +52.1% | +40.5% |
| 22 | 180596ns | -1.3% | +3.4% | +90.7% | +49.2% | +38.6% |
| 23 | 179145ns | -0.8% | +1.7% | +90.1% | +48.9% | +39.8% |
| 24 | 177752ns | +1.6% | +2.4% | +48.8% | +50.1% | +40.9% |
| 25 | 177775ns | +2.5% | +2.5% | +50.9% | +49.9% | +40.8% |
| 26 | 178040ns | +2.6% | +2.3% | +50.9% | +49.6% | +40.6% |
| 27 | 178000ns | +2.5% | +2.3% | +50.3% | +49.6% | +43.4% |
| 28 | 177770ns | +3.7% | +2.4% | +50.8% | +50.1% | +41.4% |
| 29 | 177942ns | +4.8% | +2.6% | +48.0% | +50.1% | +40.9% |
| 30 | 178855ns | +2.6% | +3.2% | +49.9% | +49.0% | +40.1% |
| 31 | 177931ns | -0.1% | +3.4% | +56.2% | +50.1% | +41.2% |
| 32 | 177640ns | +0.1% | +3.5% | +99.8% | +50.2% | +40.9% |
| 33 | 177556ns | +0.0% | +3.8% | +114.7% | +50.1% | +41.6% |
| 34 | 177662ns | -0.0% | +4.1% | +91.5% | +50.0% | +41.2% |
| 35 | 179592ns | -0.4% | +6.8% | +84.1% | +48.4% | +39.4% |
| 36 | 177578ns | +0.5% | +8.1% | +83.8% | +51.1% | +41.1% |
| 37 | 178189ns | +0.0% | +5.8% | +55.2% | +49.7% | +43.0% |
| 38 | 177779ns | -0.0% | +10.0% | +57.7% | +49.8% | +41.0% |
| 39 | 177789ns | -0.1% | +7.8% | +69.4% | +50.3% | +40.9% |
| 40 | 177831ns | +0.6% | +8.0% | +56.8% | +50.1% | +41.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.544 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d16-control | 0.759 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d32 | 0.795 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d64 | 0.526 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed | 0.551 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed-simd | 0.495 | moderate+ |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 10/40, lost 22/40
- **bitpack-carrier-d32**: won 3/40, lost 37/40
- **bitpack-carrier-d64**: won 0/40, lost 40/40
- **bitpack-carrier-packed**: won 0/40, lost 40/40
- **bitpack-carrier-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 3.8ns | 178480.7ns | 0.0% |  |
| bitpack-carrier-d16-control | 3.0ns | 179733.9ns | 0.0% |  |
| bitpack-carrier-d32 | 2.9ns | 183451.0ns | 0.0% |  |
| bitpack-carrier-d64 | 3.5ns | 289652.2ns | 0.0% |  |
| bitpack-carrier-packed | 4.3ns | 268154.6ns | 0.0% |  |
| bitpack-carrier-packed-simd | 3.1ns | 251547.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 177601.8-180819.6 ns)
  177601.8 |########################################
  177762.7 |################################
  177923.6 |########################
  178084.5 |####
  178245.4 |
  178406.3 |
  178567.1 |
  178728.0 |####
  178888.9 |
  179049.8 |########
  179210.7 |
  179371.6 |
  179532.5 |####
  179693.4 |####
  179854.2 |
  180015.1 |
  180176.0 |####
  180336.9 |####
  180497.8 |####
  180658.7 |
  (4 below, 3 above range)

bitpack-carrier-d16-control (n=40, range 177640.9-182976.5 ns)
  177640.9 |########################################
  177907.7 |
  178174.4 |########
  178441.2 |####
  178708.0 |#############
  178974.8 |########
  179241.6 |####
  179508.3 |####
  179775.1 |
  180041.9 |
  180308.7 |
  180575.5 |###################################
  180842.3 |########
  181109.0 |####
  181375.8 |
  181642.6 |
  181909.4 |
  182176.2 |####
  182442.9 |########
  182709.7 |
  (4 below, 3 above range)

bitpack-carrier-d32 (n=40, range 178791.9-190690.9 ns)
  178791.9 |########################
  179386.9 |
  179981.8 |
  180576.8 |
  181171.7 |####
  181766.7 |########################################
  182361.6 |########
  182956.6 |########
  183551.5 |############
  184146.5 |############
  184741.4 |####
  185336.4 |
  185931.3 |
  186526.3 |####
  187121.2 |####
  187716.1 |
  188311.1 |####
  188906.0 |
  189501.0 |
  190095.9 |
  (4 below, 5 above range)

bitpack-carrier-d64 (n=40, range 263708.5-349978.8 ns)
  263708.5 |########################################
  268022.1 |#######################
  272335.6 |###
  276649.1 |##########
  280962.6 |
  285276.1 |
  289589.6 |
  293903.2 |###
  298216.7 |###
  302530.2 |###
  306843.7 |
  311157.2 |
  315470.7 |###
  319784.2 |
  324097.8 |###
  328411.3 |###
  332724.8 |
  337038.3 |######
  341351.8 |###
  345665.3 |###
  (4 below, 3 above range)

bitpack-carrier-packed (n=40, range 266232.9-274353.4 ns)
  266232.9 |########################################
  266638.9 |#####################
  267044.9 |####
  267450.9 |
  267857.0 |
  268263.0 |####
  268669.0 |##
  269075.0 |####
  269481.1 |
  269887.1 |
  270293.1 |
  270699.1 |
  271105.2 |
  271511.2 |
  271917.2 |
  272323.3 |
  272729.3 |
  273135.3 |####
  273541.3 |
  273947.4 |
  (3 below, 2 above range)

bitpack-carrier-packed-simd (n=40, range 250305.7-255004.6 ns)
  250305.7 |########################################
  250540.7 |####################
  250775.6 |##
  251010.5 |##
  251245.5 |########
  251480.4 |##
  251715.4 |
  251950.3 |
  252185.3 |##
  252420.2 |##
  252655.1 |
  252890.1 |##
  253125.0 |
  253360.0 |
  253594.9 |
  253829.9 |
  254064.8 |##
  254299.7 |
  254534.7 |
  254769.6 |##
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-carrier-d16**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **bitpack-carrier-d16-control**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **bitpack-carrier-d32**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **bitpack-carrier-d64**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed**: autocorrelation=0.55 (measurement drift or warm-up artifact)
