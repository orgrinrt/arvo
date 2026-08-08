# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged dominates: 32% faster than the next best (wide-rung-ragged-overread)

wide-rung-ragged (450.31 us) leads wide-rung-ragged-overread (593.79 us) by 32%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 26% (significant)

wide-rung-ragged is -158.36 us (26%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-wordround-alias shows warm-up / thermal drift (autocorr +0.71)

wide-rung-wordround-alias's per-pass series has lag-1 autocorrelation +0.71, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wide-rung-ragged} vs {wide-rung-ragged-overread, wide-rung-align16, wide-rung-wordround, wide-rung-wordround-alias} (32% apart)

The field splits into a fast tier {wide-rung-ragged} and a slow tier {wide-rung-ragged-overread, wide-rung-align16, wide-rung-wordround, wide-rung-wordround-alias} with a 32% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: wide-rung-ragged** at 450310.7 ns median (-26.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.36x (fastest 450310.7 ns, slowest 613006.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 611311ns | 610577ns | 599138ns | 609605ns | 628605ns | base |
| wide-rung-ragged | 451145ns | 450709ns | 443499ns | 451296ns | 458336ns | -26.20% |
| wide-rung-ragged-overread | 596998ns | 594636ns | 582467ns | 595617ns | 615674ns | -2.34% |
| wide-rung-wordround | 612478ns | 612183ns | 604332ns | 611629ns | 623172ns | +0.19% |
| wide-rung-wordround-alias | 612765ns | 613812ns | 604643ns | 613270ns | 619370ns | +0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 610442ns | 598261ns | 627686ns | base | 0.752 |
| wide-rung-ragged | 450597ns | 442959ns | 457759ns | -26.19% | 1.018 |
| wide-rung-ragged-overread | 596386ns | 581880ns | 615004ns | -2.30% | 0.769 |
| wide-rung-wordround | 611656ns | 603670ns | 622339ns | +0.20% | 0.750 |
| wide-rung-wordround-alias | 611913ns | 603598ns | 618539ns | +0.24% | 0.750 |

## Performance model

- Peak throughput: **1.036 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.753 | 72.7% |
| wide-rung-ragged | 1.019 | 98.4% |
| wide-rung-ragged-overread | 0.773 | 74.6% |
| wide-rung-wordround | 0.750 | 72.4% |
| wide-rung-wordround-alias | 0.748 | 72.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 611311ns | 611311ns | base |
| wide-rung-ragged | 451145ns | 451145ns | -26.20% |
| wide-rung-ragged-overread | 596998ns | 596998ns | -2.34% |
| wide-rung-wordround | 612478ns | 612478ns | +0.19% |
| wide-rung-wordround-alias | 612765ns | 612765ns | +0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 609562ns | base | --- | [604767, 611661] | --- | --- | --- | --- |
| wide-rung-ragged | 450311ns | -158700.0ns (-26.0%) | [-159705, -156032]ns | [449754, 451499] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 593794ns | -14970.8ns (-2.5%) | [-18285, -8919]ns | [591914, 597508] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 611592ns | no significant difference | [-312, +5066]ns | [608500, 613282] | no | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround-alias | 613007ns | no significant difference | [-2199, +6076]ns | [610931, 613879] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 609938ns | -27.8% | -4.9% | -0.8% | -0.6% |
| 2 | 598400ns | -26.3% | -2.4% | +1.1% | +0.9% |
| 3 | 598261ns | -25.9% | -1.5% | +0.9% | +1.1% |
| 4 | 600961ns | -26.6% | -2.4% | +0.7% | +0.5% |
| 5 | 595896ns | -26.2% | -2.1% | +1.8% | +1.0% |
| 6 | 603419ns | -26.1% | -3.9% | -0.2% | -0.1% |
| 7 | 604624ns | -26.5% | -4.2% | -0.5% | -0.3% |
| 8 | 609353ns | -26.2% | -4.5% | +0.3% | -1.2% |
| 9 | 594461ns | -24.3% | -2.0% | +1.4% | +2.8% |
| 10 | 598725ns | -24.6% | -2.5% | +1.1% | +1.6% |
| 11 | 598876ns | -23.8% | -0.8% | +1.5% | +3.8% |
| 12 | 602848ns | -24.9% | -0.7% | +1.5% | +2.9% |
| 13 | 600508ns | -24.5% | -1.4% | +1.3% | +3.1% |
| 14 | 602338ns | -24.4% | -1.1% | +1.1% | +1.9% |
| 15 | 610432ns | -26.1% | +1.5% | -0.4% | +0.4% |
| 16 | 608411ns | -26.1% | +2.2% | -0.6% | +1.3% |
| 17 | 603859ns | -25.7% | +0.4% | +0.1% | +2.0% |
| 18 | 603580ns | -25.5% | +1.0% | +0.8% | +2.3% |
| 19 | 609770ns | -26.4% | -0.0% | -0.1% | +1.2% |
| 20 | 607865ns | -26.1% | +0.6% | +0.3% | +1.4% |
| 21 | 613888ns | -26.5% | -3.5% | +0.0% | -0.7% |
| 22 | 628977ns | -28.4% | -5.9% | -1.5% | -2.0% |
| 23 | 615823ns | -27.0% | -1.4% | +0.0% | -1.1% |
| 24 | 604911ns | -25.6% | -1.4% | +1.4% | +1.0% |
| 25 | 613269ns | -25.6% | -3.2% | +1.7% | -0.6% |
| 26 | 607892ns | -25.5% | -2.5% | +5.2% | +0.9% |
| 27 | 612384ns | -25.6% | -3.7% | +0.7% | -0.5% |
| 28 | 611024ns | -26.1% | -3.8% | +0.3% | +0.4% |
| 29 | 612110ns | -24.8% | -3.9% | +2.0% | +0.7% |
| 30 | 610080ns | -23.6% | -3.0% | +0.7% | +1.5% |
| 31 | 620966ns | -27.9% | -1.2% | -1.4% | -1.1% |
| 32 | 608977ns | -26.1% | -2.6% | +2.2% | +0.7% |
| 33 | 623911ns | -27.0% | -4.7% | -1.8% | -1.6% |
| 34 | 611820ns | -26.0% | -2.6% | +0.3% | +0.0% |
| 35 | 620274ns | -27.3% | -3.9% | -1.3% | -1.4% |
| 36 | 617865ns | -27.6% | -2.6% | -0.3% | -1.0% |
| 37 | 611502ns | -25.5% | +0.1% | +0.3% | +0.1% |
| 38 | 668604ns | -31.7% | -6.9% | -8.0% | -8.2% |
| 39 | 623976ns | -27.8% | -3.0% | -1.7% | -1.8% |
| 40 | 616917ns | -27.1% | -2.8% | -1.1% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.384 | moderate+ |
| wide-rung-ragged | 0.569 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.673 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.578 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.712 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 40/40, lost 0/40
- **wide-rung-ragged-overread**: won 33/40, lost 5/40
- **wide-rung-wordround**: won 14/40, lost 24/40
- **wide-rung-wordround-alias**: won 15/40, lost 23/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 7.6ns | 610442.3ns | 0.0% |  |
| wide-rung-ragged | 5.7ns | 450596.9ns | 0.0% |  |
| wide-rung-ragged-overread | 11.8ns | 596385.7ns | 0.0% |  |
| wide-rung-wordround | 10.3ns | 611656.4ns | 0.0% |  |
| wide-rung-wordround-alias | 7.9ns | 611913.0ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 598261.0-627686.2 ns)
  598261.0 |##############################
  599732.2 |####################
  601203.5 |##########
  602674.8 |########################################
  604146.0 |####################
  605617.3 |
  607088.5 |##############################
  608559.8 |########################################
  610031.1 |##############################
  611502.3 |########################################
  612973.6 |####################
  614444.9 |##########
  615916.1 |##########
  617387.4 |##########
  618858.7 |##########
  620329.9 |##########
  621801.2 |
  623272.5 |####################
  624743.7 |
  626215.0 |
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 442958.9-457759.3 ns)
  442958.9 |####
  443698.9 |####
  444438.9 |
  445178.9 |
  445918.9 |####
  446659.0 |
  447399.0 |########
  448139.0 |
  448879.0 |############
  449619.1 |########################################
  450359.1 |########
  451099.1 |############
  451839.1 |####
  452579.2 |########
  453319.2 |####
  454059.2 |
  454799.2 |################
  455539.3 |
  456279.3 |############
  457019.3 |
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 581879.6-615003.9 ns)
  581879.6 |###########
  583535.8 |###########
  585192.0 |#####
  586848.2 |###########
  588504.4 |###########
  590160.7 |
  591816.9 |########################################
  593473.1 |###########
  595129.3 |######################
  596785.5 |
  598441.7 |###########
  600098.0 |
  601754.2 |#####
  603410.4 |
  605066.6 |###########
  606722.8 |#####
  608379.0 |###########
  610035.3 |#####
  611691.5 |#####
  613347.7 |#####
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 603670.1-622338.7 ns)
  603670.1 |########
  604603.5 |########################################
  605536.9 |
  606470.3 |########
  607403.8 |################
  608337.2 |################################
  609270.6 |########
  610204.1 |################
  611137.5 |
  612070.9 |########################################
  613004.4 |################################
  613937.8 |################
  614871.2 |########
  615804.6 |########################
  616738.1 |
  617671.5 |
  618604.9 |
  619538.4 |########
  620471.8 |
  621405.2 |
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 603598.1-618538.7 ns)
  603598.1 |#####
  604345.1 |#####
  605092.1 |
  605839.2 |#####
  606586.2 |
  607333.2 |
  608080.3 |#####
  608827.3 |###########
  609574.3 |###########
  610321.3 |###########
  611068.4 |###########
  611815.4 |###########
  612562.4 |#################
  613309.5 |########################################
  614056.5 |
  614803.5 |
  615550.5 |#####
  616297.6 |############################
  617044.6 |#####
  617791.6 |
  (5 below, 4 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.71 (measurement drift or warm-up artifact)
