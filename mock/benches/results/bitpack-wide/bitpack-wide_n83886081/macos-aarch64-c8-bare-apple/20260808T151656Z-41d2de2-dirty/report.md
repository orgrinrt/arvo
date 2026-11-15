# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (738.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-d16-padal at 292.00 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-d16-padal dominates: 112% faster than the next best (bitpack-wide-pipe4)

bitpack-wide-d16-padal (292.00 us) leads bitpack-wide-pipe4 (618.94 us) by 112%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-d16-padal beats baseline by 60% (significant)

bitpack-wide-d16-padal is -444.17 us (60%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-wide-d16 is an outlier: 2.5x slower than the field

bitpack-wide-d16 (738.46 us) is 2.5x the fastest (292.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-wide-d16-padal is fastest but the noisiest (CV 5.7%)

bitpack-wide-d16-padal wins on median (292.00 us) yet has the highest variance (CV 5.7%), while bitpack-wide-pipe4 is the steadiest (CV 1.1%, 618.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-wide-d16-padal shows warm-up / thermal drift (autocorr +0.60)

bitpack-wide-d16-padal's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-wide-d16-padal} vs {bitpack-wide-pipe4, bitpack-wide-d16-control, bitpack-wide-d16} (112% apart)

The field splits into a fast tier {bitpack-wide-d16-padal} and a slow tier {bitpack-wide-pipe4, bitpack-wide-d16-control, bitpack-wide-d16} with a 112% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-wide-d16-padal** at 291995.0 ns median (-60.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.53x (fastest 291995.0 ns, slowest 738457.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 741847ns | 739332ns | 729169ns | 739891ns | 760395ns | base |
| bitpack-wide-d16-control | 735745ns | 734947ns | 726075ns | 735039ns | 747537ns | -0.82% |
| bitpack-wide-d16-padal | 301175ns | 293689ns | 285781ns | 297227ns | 328413ns | -59.40% |
| bitpack-wide-pipe4 | 619873ns | 619740ns | 610461ns | 619886ns | 629245ns | -16.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 740830ns | 728093ns | 759346ns | base | 11.323 |
| bitpack-wide-d16-control | 734732ns | 724980ns | 746621ns | -0.82% | 11.417 |
| bitpack-wide-d16-padal | 299795ns | 283776ns | 327522ns | -59.53% | 27.981 |
| bitpack-wide-pipe4 | 618923ns | 610032ns | 628011ns | -16.46% | 13.554 |

## Performance model

- Peak throughput: **29.561 Gops/s** (bitpack-wide-d16-padal; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 11.360 | 38.4% |
| bitpack-wide-d16-control | 11.430 | 38.7% |
| bitpack-wide-d16-padal | 28.729 | 97.2% |
| bitpack-wide-pipe4 | 13.553 | 45.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 741847ns | 741847ns | base |
| bitpack-wide-d16-control | 735745ns | 735745ns | -0.82% |
| bitpack-wide-d16-padal | 301175ns | 301175ns | -59.40% |
| bitpack-wide-pipe4 | 619873ns | 619873ns | -16.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 738457ns | base | --- | [733669, 743595] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 733882ns | -5245.4ns (-0.7%) | [-9528, -1790]ns | [730826, 736889] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-wide-d16-padal | 291995ns | -443727.5ns (-60.1%) | [-450770, -437163]ns | [288742, 300970] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-pipe4 | 618941ns | -120026.0ns (-16.3%) | [-127168, -114617]ns | [615802, 621078] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|
| 1 | 739849ns | -2.0% | -62.1% | -16.7% |
| 2 | 732408ns | -1.2% | -60.7% | -16.5% |
| 3 | 728219ns | -0.7% | -60.6% | -13.8% |
| 4 | 742512ns | -0.9% | -60.5% | -17.9% |
| 5 | 734991ns | +1.9% | -60.7% | -16.2% |
| 6 | 744162ns | +2.0% | -61.0% | -16.0% |
| 7 | 731914ns | +1.3% | -60.1% | -14.6% |
| 8 | 726063ns | +1.5% | -59.8% | -14.1% |
| 9 | 726655ns | +0.1% | -60.2% | -13.4% |
| 10 | 755167ns | -4.2% | -62.2% | -17.2% |
| 11 | 734500ns | +0.7% | -61.4% | -14.6% |
| 12 | 739096ns | -0.7% | -61.4% | -15.4% |
| 13 | 752696ns | -1.7% | -62.1% | -18.6% |
| 14 | 751202ns | -2.8% | -61.6% | -18.8% |
| 15 | 779844ns | -6.3% | -60.3% | -19.8% |
| 16 | 747975ns | -2.4% | -60.2% | -17.4% |
| 17 | 755716ns | -1.8% | -62.3% | -18.1% |
| 18 | 750748ns | -2.0% | -62.6% | -17.6% |
| 19 | 746800ns | -1.3% | -61.8% | -16.9% |
| 20 | 750178ns | -2.4% | -58.7% | -16.9% |
| 21 | 776151ns | -3.0% | -58.7% | -22.1% |
| 22 | 742301ns | -0.5% | -54.8% | -17.1% |
| 23 | 737647ns | -0.7% | -57.0% | -17.6% |
| 24 | 732152ns | -0.2% | -60.6% | -15.5% |
| 25 | 731262ns | -0.3% | -60.8% | -15.8% |
| 26 | 729980ns | +0.0% | -60.8% | -15.4% |
| 27 | 731586ns | +1.1% | -58.8% | -16.1% |
| 28 | 739003ns | +1.1% | -54.6% | -16.9% |
| 29 | 747235ns | -2.2% | -57.7% | -17.8% |
| 30 | 730848ns | -0.0% | -55.8% | -15.8% |
| 31 | 727695ns | -0.5% | -53.8% | -14.9% |
| 32 | 724020ns | +2.4% | -56.1% | -14.2% |
| 33 | 734320ns | -1.1% | -56.3% | -15.5% |
| 34 | 737911ns | -0.6% | -59.2% | -15.8% |
| 35 | 734065ns | -1.0% | -55.0% | -15.3% |
| 36 | 732624ns | +0.2% | -59.5% | -14.4% |
| 37 | 744102ns | -0.4% | -61.2% | -18.0% |
| 38 | 753241ns | -2.4% | -61.5% | -17.7% |
| 39 | 743088ns | -1.6% | -58.3% | -14.5% |
| 40 | 733273ns | +0.7% | -60.1% | -16.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | 0.416 | moderate+ |
| bitpack-wide-d16-control | 0.295 | moderate+ |
| bitpack-wide-d16-padal | 0.596 | HIGH+ (drift/warm-up) |
| bitpack-wide-pipe4 | 0.077 | ok |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 27/40, lost 10/40
- **bitpack-wide-d16-padal**: won 40/40, lost 0/40
- **bitpack-wide-pipe4**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 7.1ns | 740830.0ns | 0.0% |  |
| bitpack-wide-d16-control | 5.2ns | 734731.6ns | 0.0% |  |
| bitpack-wide-d16-padal | 3.8ns | 299794.7ns | 0.0% |  |
| bitpack-wide-pipe4 | 5.2ns | 618922.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 728092.8-759345.6 ns)
  728092.8 |######
  729655.4 |#############
  731218.0 |########################################
  732780.7 |####################
  734343.3 |#############
  735906.0 |
  737468.6 |####################
  739031.2 |#############
  740593.9 |
  742156.5 |####################
  743719.2 |#############
  745281.8 |######
  746844.4 |#############
  748407.1 |
  749969.7 |####################
  751532.4 |######
  753095.0 |######
  754657.6 |#############
  756220.3 |
  757782.9 |
  (4 below, 2 above range)

bitpack-wide-d16-control (n=40, range 724980.5-746620.7 ns)
  724980.5 |########
  726062.5 |################
  727144.5 |########
  728226.5 |########
  729308.5 |########################
  730390.5 |########################################
  731472.5 |################
  732554.5 |
  733636.6 |########################
  734718.6 |################
  735800.6 |################
  736882.6 |########
  737964.6 |################
  739046.6 |################
  740128.6 |########################
  741210.6 |########
  742292.6 |########
  743374.7 |
  744456.7 |
  745538.7 |
  (4 below, 4 above range)

bitpack-wide-d16-padal (n=40, range 283776.2-327521.9 ns)
  283776.2 |##############################
  285963.5 |##########
  288150.8 |########################################
  290338.1 |##########
  292525.3 |##########
  294712.6 |#####
  296899.9 |#####
  299087.2 |##########
  301274.5 |
  303461.8 |
  305649.1 |
  307836.3 |###############
  310023.6 |
  312210.9 |
  314398.2 |#####
  316585.5 |##########
  318772.8 |##########
  320960.1 |
  323147.4 |#####
  325334.6 |
  (3 below, 4 above range)

bitpack-wide-pipe4 (n=40, range 610031.5-628010.6 ns)
  610031.5 |####################
  610930.5 |##########
  611829.4 |##########
  612728.4 |##########
  613627.3 |##############################
  614526.3 |####################
  615425.2 |##############################
  616324.2 |
  617223.1 |####################
  618122.1 |####################
  619021.1 |##############################
  619920.0 |##########
  620819.0 |########################################
  621717.9 |
  622616.9 |##########
  623515.8 |##########
  624414.8 |####################
  625313.7 |##############################
  626212.7 |
  627111.6 |##############################
  (3 below, 2 above range)

```

## Diagnostics

- **bitpack-wide-d16-padal**: autocorrelation=0.60 (measurement drift or warm-up artifact)
