# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 30% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (171 ns) leads bitpack-aligned-rand (222 ns) by 30%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 21% (significant)

bitpack-native-rand is -48 ns (21%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 2.2x slower than the field

bitpack-zeropad-rand (370 ns) is 2.2x the fastest (171 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-rand shows warm-up / thermal drift (autocorr +0.84)

bitpack-native-rand's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-native-rand** at 171.2 ns median (-23.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.16x (fastest 171.2 ns, slowest 370.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 306ns | 301ns | 281ns | 296ns | 357ns | base |
| bitpack-native-rand | 249ns | 245ns | 233ns | 246ns | 274ns | -18.59% |
| bitpack-zeropad-rand | 443ns | 448ns | 421ns | 447ns | 456ns | +45.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 227ns | 210ns | 265ns | base | 1.126 |
| bitpack-native-rand | 171ns | 162ns | 179ns | -24.84% | 1.499 |
| bitpack-zeropad-rand | 366ns | 346ns | 377ns | +60.81% | 0.700 |

## Performance model

- Peak throughput: **1.579 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 1.151 | 72.9% |
| bitpack-native-rand | 1.495 | 94.7% |
| bitpack-zeropad-rand | 0.692 | 43.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 306ns | 306ns | base |
| bitpack-native-rand | 249ns | 249ns | -18.59% |
| bitpack-zeropad-rand | 443ns | 443ns | +45.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 222ns | base | --- | [214, 227] | --- | --- | --- | --- |
| bitpack-native-rand | 171ns | -50.0ns (-22.5%) | [-54, -46]ns | [165, 176] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 370ns | +142.5ns (+64.0%) | [+141, +147]ns | [368, 371] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|
| 1 | 212ns | -15.5% | +74.8% |
| 2 | 212ns | -17.1% | +76.3% |
| 3 | 216ns | -16.2% | +73.0% |
| 4 | 214ns | -15.0% | +74.3% |
| 5 | 216ns | -18.5% | +72.7% |
| 6 | 212ns | -16.5% | +74.7% |
| 7 | 214ns | -17.3% | +71.2% |
| 8 | 215ns | -17.8% | +72.3% |
| 9 | 212ns | -15.7% | +75.0% |
| 10 | 209ns | -15.3% | +75.7% |
| 11 | 228ns | -29.5% | +52.2% |
| 12 | 217ns | -25.0% | +60.0% |
| 13 | 212ns | -22.1% | +62.2% |
| 14 | 207ns | -21.5% | +67.4% |
| 15 | 210ns | -21.5% | +64.8% |
| 16 | 208ns | -22.3% | +66.5% |
| 17 | 213ns | -22.3% | +63.8% |
| 18 | 214ns | -23.8% | +62.6% |
| 19 | 215ns | -24.9% | +62.9% |
| 20 | 208ns | -22.0% | +65.8% |
| 21 | 227ns | -27.5% | +62.9% |
| 22 | 224ns | -26.3% | +64.6% |
| 23 | 228ns | -28.1% | +61.6% |
| 24 | 227ns | -27.3% | +61.8% |
| 25 | 228ns | -27.1% | +62.5% |
| 26 | 229ns | -27.4% | +61.9% |
| 27 | 230ns | -27.2% | +61.5% |
| 28 | 224ns | -27.2% | +64.9% |
| 29 | 228ns | -27.9% | +62.1% |
| 30 | 230ns | -27.8% | +61.7% |
| 31 | 510ns | -65.6% | -27.6% |
| 32 | 229ns | -22.0% | +61.4% |
| 33 | 227ns | -20.8% | +63.6% |
| 34 | 230ns | -22.1% | +62.8% |
| 35 | 229ns | -23.1% | +68.7% |
| 36 | 230ns | -23.4% | +61.6% |
| 37 | 223ns | -20.5% | +69.0% |
| 38 | 228ns | -21.8% | +64.1% |
| 39 | 222ns | -20.7% | +70.7% |
| 40 | 227ns | -22.3% | +65.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | 0.058 | ok |
| bitpack-native-rand | 0.839 | HIGH+ (drift/warm-up) |
| bitpack-zeropad-rand | 0.827 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-native-rand**: won 40/40, lost 0/40
- **bitpack-zeropad-rand**: won 1/40, lost 39/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 1.5ns | 227.3ns | 0.7% |  |
| bitpack-native-rand | 1.4ns | 170.8ns | 0.8% |  |
| bitpack-zeropad-rand | 1.5ns | 365.5ns | 0.4% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 209.7-264.6 ns)
    209.7 |############
    212.4 |############################
    215.1 |################
    217.9 |
    220.6 |########
    223.4 |########
    226.1 |########################################
    228.9 |########################
    231.6 |
    234.4 |
    237.1 |
    239.8 |
    242.6 |
    245.3 |
    248.1 |
    250.8 |
    253.6 |
    256.3 |
    259.1 |
    261.8 |
  (5 below, 1 above range)

bitpack-native-rand (n=40, range 162.1-179.4 ns)
    162.1 |####################
    163.0 |####################
    163.8 |####################
    164.7 |##########################
    165.6 |####################
    166.4 |######
    167.3 |
    168.2 |
    169.0 |
    169.9 |
    170.8 |
    171.6 |
    172.5 |
    173.3 |
    174.2 |
    175.1 |######
    175.9 |########################################
    176.8 |#################################
    177.7 |######
    178.5 |##########################
  (3 below, 3 above range)

bitpack-zeropad-rand (n=40, range 345.9-376.7 ns)
    345.9 |###############
    347.5 |##########
    349.0 |#####
    350.5 |
    352.1 |
    353.6 |
    355.1 |
    356.7 |
    358.2 |
    359.8 |
    361.3 |
    362.8 |
    364.4 |
    365.9 |#####
    367.4 |####################
    369.0 |########################################
    370.5 |###################################
    372.1 |####################
    373.6 |##########
    375.1 |#####
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: CV=20.2% (high variance, measurements may be unstable)
- **bitpack-native-rand**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **bitpack-zeropad-rand**: autocorrelation=0.83 (measurement drift or warm-up artifact)
