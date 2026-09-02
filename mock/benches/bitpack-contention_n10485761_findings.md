# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 shows warm-up / thermal drift (autocorr +0.88)

bitpack-contend-d64's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} vs {bitpack-contend-d64, bitpack-contend-packed-simd, bitpack-contend-packed} (27% apart)

The field splits into a fast tier {bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} and a slow tier {bitpack-contend-d64, bitpack-contend-packed-simd, bitpack-contend-packed} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-d32** at 87883.3 ns median (-1.2% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.52x (fastest 87883.3 ns, slowest 133254.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 89714ns | 89127ns | 88931ns | 89275ns | 91813ns | base |
| bitpack-contend-d16-control | 89569ns | 89083ns | 88921ns | 89103ns | 91614ns | -0.16% |
| bitpack-contend-d32 | 88581ns | 88099ns | 87875ns | 88174ns | 90508ns | -1.26% |
| bitpack-contend-d64 | 113425ns | 112998ns | 104972ns | 114038ns | 120039ns | +26.43% |
| bitpack-contend-packed | 134189ns | 133552ns | 133124ns | 133656ns | 136854ns | +49.58% |
| bitpack-contend-packed-simd | 125760ns | 125638ns | 125314ns | 125678ns | 126449ns | +40.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 89537ns | 88796ns | 91592ns | base | 11.711 |
| bitpack-contend-d16-control | 89389ns | 88767ns | 91466ns | -0.17% | 11.730 |
| bitpack-contend-d32 | 88361ns | 87679ns | 90277ns | -1.31% | 11.867 |
| bitpack-contend-d64 | 113153ns | 104702ns | 119787ns | +26.38% | 9.267 |
| bitpack-contend-packed | 133975ns | 132987ns | 136623ns | +49.63% | 7.827 |
| bitpack-contend-packed-simd | 125494ns | 125152ns | 126189ns | +40.16% | 8.356 |

## Performance model

- Peak throughput: **11.959 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.791 | 98.6% |
| bitpack-contend-d16-control | 11.801 | 98.7% |
| bitpack-contend-d32 | 11.931 | 99.8% |
| bitpack-contend-d64 | 9.298 | 77.7% |
| bitpack-contend-packed | 7.869 | 65.8% |
| bitpack-contend-packed-simd | 8.366 | 70.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 89714ns | 89714ns | base |
| bitpack-contend-d16-control | 89569ns | 89569ns | -0.16% |
| bitpack-contend-d32 | 88581ns | 88581ns | -1.26% |
| bitpack-contend-d64 | 113425ns | 113425ns | +26.43% |
| bitpack-contend-packed | 134189ns | 134189ns | +49.58% |
| bitpack-contend-packed-simd | 125760ns | 125760ns | +40.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 88932ns | base | --- | [88860, 89144] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 88858ns | no significant difference | [-258, +17]ns | [88813, 88946] | no | 0.4296 | 0.4296 | 0 |
| bitpack-contend-d32 | 87883ns | -1072.3ns (-1.2%) | [-1311, -868]ns | [87791, 88056] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 112779ns | +23638.9ns (+26.6%) | [+22347, +26248]ns | [112331, 115760] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 133255ns | +44186.0ns (+49.7%) | [+44097, +44500]ns | [133091, 133695] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 125337ns | +36374.6ns (+40.9%) | [+36215, +36473]ns | [125263, 125483] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 88792ns | +0.0% | -0.7% | +26.8% | +50.0% | +41.1% |
| 2 | 88797ns | -0.0% | +4.8% | +26.7% | +50.6% | +41.1% |
| 3 | 88812ns | +0.0% | +4.5% | +27.5% | +50.5% | +41.0% |
| 4 | 91110ns | -2.6% | -0.2% | +23.9% | +47.4% | +37.4% |
| 5 | 88755ns | +0.2% | -0.8% | +31.7% | +51.9% | +41.1% |
| 6 | 89071ns | -0.3% | -1.3% | +27.2% | +50.4% | +41.6% |
| 7 | 89033ns | -0.3% | -1.2% | +26.1% | +49.6% | +41.0% |
| 8 | 89165ns | -0.4% | -1.6% | +25.8% | +49.1% | +40.8% |
| 9 | 95528ns | -7.1% | -8.1% | +17.6% | +39.2% | +31.5% |
| 10 | 92016ns | -2.1% | -4.5% | +22.1% | +44.8% | +36.3% |
| 11 | 89272ns | -0.5% | -1.7% | +17.8% | +49.0% | +41.6% |
| 12 | 88932ns | +0.1% | -0.8% | +22.2% | +49.9% | +42.7% |
| 13 | 89124ns | -0.3% | -1.5% | +17.0% | +50.2% | +41.1% |
| 14 | 88815ns | -0.1% | +0.3% | +20.9% | +49.7% | +40.9% |
| 15 | 88807ns | -0.0% | -0.9% | +17.3% | +49.8% | +41.1% |
| 16 | 88852ns | +0.1% | -0.9% | +17.0% | +50.0% | +42.0% |
| 17 | 88843ns | +0.0% | -1.2% | +17.3% | +50.0% | +41.7% |
| 18 | 88820ns | -0.0% | -1.2% | +17.5% | +50.0% | +41.5% |
| 19 | 88824ns | +0.1% | -1.2% | +17.1% | +51.9% | +41.2% |
| 20 | 89007ns | -0.3% | -1.4% | +22.4% | +49.5% | +40.9% |
| 21 | 89298ns | -0.6% | -1.6% | +25.6% | +49.0% | +40.6% |
| 22 | 88929ns | +0.2% | -1.4% | +26.5% | +49.6% | +40.9% |
| 23 | 88931ns | -0.1% | -1.3% | +26.1% | +49.6% | +41.3% |
| 24 | 88868ns | +0.0% | +0.8% | +26.8% | +49.6% | +40.8% |
| 25 | 90558ns | -1.9% | -2.5% | +24.9% | +47.1% | +39.8% |
| 26 | 90762ns | -1.9% | -1.4% | +24.4% | +46.5% | +38.4% |
| 27 | 90118ns | -1.4% | -2.5% | +29.3% | +47.7% | +39.1% |
| 28 | 88796ns | +0.1% | -0.8% | +33.6% | +49.8% | +41.1% |
| 29 | 88795ns | +0.0% | -1.1% | +32.5% | +50.6% | +41.2% |
| 30 | 88908ns | -0.0% | -1.2% | +29.3% | +50.1% | +40.9% |
| 31 | 90658ns | -2.1% | -3.3% | +32.3% | +47.0% | +38.1% |
| 32 | 88910ns | -0.1% | -1.2% | +34.4% | +49.6% | +40.7% |
| 33 | 90151ns | -1.0% | -2.7% | +33.0% | +47.6% | +38.9% |
| 34 | 91950ns | -1.1% | -4.7% | +30.2% | +47.7% | +36.2% |
| 35 | 90123ns | +1.3% | -2.7% | +32.9% | +52.7% | +39.1% |
| 36 | 88932ns | +4.3% | -0.4% | +34.3% | +53.9% | +40.7% |
| 37 | 89736ns | +3.2% | -2.3% | +33.5% | +52.5% | +39.6% |
| 38 | 88853ns | +3.3% | -0.3% | +35.0% | +54.2% | +41.0% |
| 39 | 89010ns | +2.3% | -1.5% | +34.0% | +53.9% | +40.6% |
| 40 | 88829ns | +2.6% | -0.1% | +34.7% | +54.1% | +41.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.288 | moderate+ |
| bitpack-contend-d16-control | 0.861 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | 0.567 | HIGH+ (drift/warm-up) |
| bitpack-contend-d64 | 0.883 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed | 0.806 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.340 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 16/40, lost 12/40
- **bitpack-contend-d32**: won 36/40, lost 4/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 2.5ns | 89537.2ns | 0.0% |  |
| bitpack-contend-d16-control | 2.4ns | 89388.9ns | 0.0% |  |
| bitpack-contend-d32 | 2.5ns | 88360.7ns | 0.0% |  |
| bitpack-contend-d64 | 3.5ns | 113152.9ns | 0.0% |  |
| bitpack-contend-packed | 2.8ns | 133975.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 2.6ns | 125494.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 88796.0-91591.6 ns)
  88796.0 |########################################
  88935.8 |#########
  89075.6 |####
  89215.4 |####
  89355.2 |
  89494.9 |
  89634.7 |##
  89774.5 |
  89914.3 |
  90054.1 |#######
  90193.8 |
  90333.6 |
  90473.4 |##
  90613.2 |##
  90753.0 |##
  90892.8 |
  91032.5 |##
  91172.3 |
  91312.1 |
  91451.9 |
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 88767.4-91466.2 ns)
  88767.4 |########################################
  88902.3 |#########
  89037.2 |#####
  89172.2 |#
  89307.1 |
  89442.1 |
  89577.0 |
  89712.0 |
  89846.9 |
  89981.9 |#
  90116.8 |
  90251.7 |
  90386.7 |
  90521.6 |
  90656.6 |
  90791.5 |
  90926.5 |#
  91061.4 |###
  91196.4 |#
  91331.3 |
  (2 below, 3 above range)

bitpack-contend-d32 (n=40, range 87679.5-90276.8 ns)
  87679.5 |########################################
  87809.3 |###################################
  87939.2 |#################
  88069.1 |#############
  88198.9 |
  88328.8 |####
  88458.7 |####
  88588.5 |########
  88718.4 |
  88848.3 |
  88978.1 |####
  89108.0 |
  89237.9 |
  89367.7 |####
  89497.6 |####
  89627.5 |
  89757.3 |
  89887.2 |
  90017.1 |
  90146.9 |
  (6 below, 3 above range)

bitpack-contend-d64 (n=40, range 104701.8-119786.8 ns)
  104701.8 |####
  105456.0 |
  106210.3 |
  106964.5 |####
  107718.8 |
  108473.0 |########
  109227.3 |
  109981.5 |
  110735.8 |
  111490.0 |#############
  112244.3 |########################################
  112998.5 |#############
  113752.8 |
  114507.0 |####
  115261.3 |
  116015.5 |####
  116769.8 |####
  117524.0 |####
  118278.3 |####
  119032.5 |##########################
  (6 below, 4 above range)

bitpack-contend-packed (n=40, range 132986.9-136623.3 ns)
  132986.9 |########################################
  133168.7 |################################
  133350.5 |###
  133532.3 |###
  133714.2 |##########
  133896.0 |###
  134077.8 |
  134259.6 |###
  134441.4 |
  134623.3 |
  134805.1 |#######
  134986.9 |
  135168.7 |
  135350.5 |
  135532.4 |
  135714.2 |###
  135896.0 |
  136077.8 |
  136259.6 |
  136441.5 |
  (4 below, 6 above range)

bitpack-contend-packed-simd (n=40, range 125152.3-126188.8 ns)
  125152.3 |####################
  125204.1 |########################################
  125255.9 |#################################
  125307.8 |##########################
  125359.6 |####################
  125411.4 |######
  125463.2 |######
  125515.1 |#############
  125566.9 |
  125618.7 |####################
  125670.5 |######
  125722.3 |######
  125774.2 |
  125826.0 |######
  125877.8 |
  125929.6 |
  125981.5 |
  126033.3 |
  126085.1 |######
  126136.9 |######
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16-control**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **bitpack-contend-d64**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **bitpack-contend-packed**: autocorrelation=0.81 (measurement drift or warm-up artifact)
