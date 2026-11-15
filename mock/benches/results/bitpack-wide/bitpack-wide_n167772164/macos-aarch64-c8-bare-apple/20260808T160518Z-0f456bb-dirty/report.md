# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (691.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-pipe4 at 557.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-pipe4 dominates: 13% faster than the next best (bitpack-wide-d16-control)

bitpack-wide-pipe4 (557.75 us) leads bitpack-wide-d16-control (628.79 us) by 13%, a clear separation rather than a photo finish. CV 33.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 beats baseline by 26% (significant)

bitpack-wide-pipe4 is -182.99 us (26%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-wide-d16-control shows warm-up / thermal drift (autocorr +0.65)

bitpack-wide-d16-control's per-pass series has lag-1 autocorrelation +0.65, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (133.41 us) is smaller than the fastest variant's own run-to-run std-dev (187.42 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-wide-d16 is inconsistent: worst-20% is 2.3x its best-20%

bitpack-wide-d16's best 20% of batches run at 571.25 us but its worst 20% at 1.32 ms (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 557748.3 ns median (-19.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.24x (fastest 557748.3 ns, slowest 691161.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 803109ns | 692820ns | 573455ns | 707902ns | 1318381ns | base |
| bitpack-wide-d16-control | 693430ns | 630254ns | 581470ns | 651586ns | 930923ns | -13.66% |
| bitpack-wide-d16-padal | 713156ns | 643666ns | 570345ns | 659291ns | 1017559ns | -11.20% |
| bitpack-wide-pipe4 | 647803ns | 559110ns | 482743ns | 595865ns | 968675ns | -19.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 801111ns | 571251ns | 1315893ns | base | 20.942 |
| bitpack-wide-d16-control | 691602ns | 579602ns | 929017ns | -13.67% | 24.258 |
| bitpack-wide-d16-padal | 711373ns | 569271ns | 1014970ns | -11.20% | 23.584 |
| bitpack-wide-pipe4 | 646214ns | 481290ns | 966483ns | -19.34% | 25.962 |

## Performance model

- Peak throughput: **34.859 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 24.274 | 69.6% |
| bitpack-wide-d16-control | 26.682 | 76.5% |
| bitpack-wide-d16-padal | 26.132 | 75.0% |
| bitpack-wide-pipe4 | 30.080 | 86.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 803109ns | 803109ns | base |
| bitpack-wide-d16-control | 693430ns | 693430ns | -13.66% |
| bitpack-wide-d16-padal | 713156ns | 713156ns | -11.20% |
| bitpack-wide-pipe4 | 647803ns | 647803ns | -19.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 691162ns | base | --- | [620308, 776123] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 628794ns | no significant difference | [-67275, +2060]ns | [609136, 687595] | no | 0.0807 | 0.0807 | 0 |
| bitpack-wide-d16-padal | 642020ns | -42474.2ns (-6.1%) | [-80885, -7792]ns | [616181, 710738] | YES (adj: no) | 0.0577 | 0.0385 | 0 |
| bitpack-wide-pipe4 | 557748ns | -94359.8ns (-13.7%) | [-156230, -69401]ns | [534824, 628317] | YES | 0.0001 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|
| 1 | 3072987ns | -69.5% | -71.8% | -76.3% |
| 2 | 869761ns | +9.3% | -15.2% | +5.1% |
| 3 | 1238042ns | -40.0% | -35.5% | -20.0% |
| 4 | 1116082ns | -41.1% | -28.2% | -3.0% |
| 5 | 1098762ns | -44.6% | -32.2% | -16.6% |
| 6 | 835018ns | +15.8% | +14.4% | +17.8% |
| 7 | 887588ns | +5.9% | -0.6% | -45.1% |
| 8 | 999276ns | -1.0% | -25.5% | -20.4% |
| 9 | 1147411ns | -17.2% | -36.8% | -34.2% |
| 10 | 922782ns | -0.6% | -4.5% | +33.3% |
| 11 | 718872ns | -12.6% | -20.3% | +11.7% |
| 12 | 667733ns | +17.1% | +8.4% | +21.6% |
| 13 | 692519ns | -16.4% | -10.1% | -17.7% |
| 14 | 803178ns | -9.1% | -20.8% | -20.8% |
| 15 | 613809ns | +20.0% | +13.2% | +1.1% |
| 16 | 777098ns | -11.2% | -6.4% | -25.2% |
| 17 | 719047ns | +1.4% | -2.9% | -6.3% |
| 18 | 728898ns | -16.3% | -4.9% | -29.9% |
| 19 | 689805ns | -6.9% | -6.3% | -10.1% |
| 20 | 931801ns | -29.1% | -32.2% | -41.2% |
| 21 | 561292ns | +21.4% | +3.8% | -16.5% |
| 22 | 561960ns | +27.3% | +12.1% | -12.4% |
| 23 | 564841ns | +10.5% | +2.9% | -13.2% |
| 24 | 575219ns | +6.9% | +10.8% | -7.8% |
| 25 | 563403ns | +8.3% | +5.4% | -3.3% |
| 26 | 567427ns | +3.2% | +0.3% | +29.4% |
| 27 | 584210ns | +17.3% | -3.1% | -0.6% |
| 28 | 651090ns | -9.5% | -13.8% | -14.9% |
| 29 | 607606ns | -4.0% | -7.4% | -7.6% |
| 30 | 641587ns | -7.3% | -5.0% | -14.4% |
| 31 | 643080ns | -4.2% | -11.1% | -28.1% |
| 32 | 591651ns | +6.3% | +7.0% | -20.6% |
| 33 | 622339ns | -6.3% | +4.0% | -23.4% |
| 34 | 616859ns | -5.8% | -7.3% | -18.6% |
| 35 | 598073ns | -4.3% | -1.8% | -16.1% |
| 36 | 618278ns | -2.8% | +25.7% | -13.5% |
| 37 | 613748ns | -6.1% | +251.3% | -13.4% |
| 38 | 775149ns | -25.7% | -13.8% | -30.8% |
| 39 | 826812ns | -26.8% | -30.0% | -35.3% |
| 40 | 729332ns | -18.6% | -19.8% | -27.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | 0.184 | ok |
| bitpack-wide-d16-control | 0.646 | HIGH+ (drift/warm-up) |
| bitpack-wide-d16-padal | 0.123 | ok |
| bitpack-wide-pipe4 | 0.627 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 26/40, lost 14/40
- **bitpack-wide-d16-padal**: won 27/40, lost 13/40
- **bitpack-wide-pipe4**: won 33/40, lost 7/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 60.7ns | 801110.6ns | 0.0% |  |
| bitpack-wide-d16-control | 38.9ns | 691601.9ns | 0.0% |  |
| bitpack-wide-d16-padal | 85.3ns | 711372.8ns | 0.0% |  |
| bitpack-wide-pipe4 | 56.7ns | 646214.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 571250.5-1315892.9 ns)
  571250.5 |############################
  608482.6 |########################################
  645714.8 |###########
  682946.9 |######################
  720179.0 |###########
  757411.1 |###########
  794643.2 |###########
  831875.4 |#####
  869107.5 |###########
  906339.6 |###########
  943571.7 |
  980803.8 |#####
  1018036.0 |
  1055268.1 |
  1092500.2 |###########
  1129732.3 |#####
  1166964.4 |
  1204196.6 |#####
  1241428.7 |
  1278660.8 |
  (5 below, 1 above range)

bitpack-wide-d16-control (n=40, range 579601.5-929017.3 ns)
  579601.5 |########################################
  597072.3 |############################
  614543.1 |############################
  632013.9 |#####
  649484.7 |###########
  666955.5 |#####
  684426.3 |###########
  701897.1 |#####
  719367.9 |#################
  736838.6 |#####
  754309.4 |
  771780.2 |#####
  789251.0 |
  806721.8 |
  824192.6 |
  841663.4 |
  859134.2 |
  876605.0 |
  894075.8 |
  911546.5 |#####
  (4 below, 6 above range)

bitpack-wide-d16-padal (n=40, range 569271.4-1014970.1 ns)
  569271.4 |########################################
  591556.3 |########
  613841.3 |######################
  636126.2 |#############
  658411.2 |####
  680696.1 |#############
  702981.0 |########
  725266.0 |#################
  747550.9 |
  769835.8 |####
  792120.8 |########
  814405.7 |
  836690.6 |
  858975.6 |########
  881260.5 |####
  903545.5 |
  925830.4 |
  948115.3 |####
  970400.3 |
  992685.2 |
  (3 below, 1 above range)

bitpack-wide-pipe4 (n=40, range 481290.1-966483.2 ns)
  481290.1 |######################
  505549.8 |####
  529809.4 |########################################
  554069.1 |#############
  578328.7 |########
  602588.4 |########
  626848.0 |####
  651107.7 |####
  675367.3 |
  699627.0 |
  723886.6 |########
  748146.3 |####
  772405.9 |####
  796665.6 |########
  820925.2 |
  845184.9 |
  869444.6 |
  893704.2 |########
  917963.9 |
  942223.5 |
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-wide-d16**: CV=50.5% (high variance, measurements may be unstable)
- **bitpack-wide-d16-control**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **bitpack-wide-d16-padal**: CV=35.5% (high variance, measurements may be unstable)
- **bitpack-wide-pipe4**: CV=29.0% (high variance, measurements may be unstable)
- **bitpack-wide-pipe4**: autocorrelation=0.63 (measurement drift or warm-up artifact)
