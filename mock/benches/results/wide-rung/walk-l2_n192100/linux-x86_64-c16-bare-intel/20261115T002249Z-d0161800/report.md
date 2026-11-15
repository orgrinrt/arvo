# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (535.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 343.18 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged-overread beats baseline by 31% (significant)

wide-rung-ragged-overread is -165.00 us (31%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {wide-rung-ragged-overread, wide-rung-wordround-alias, wide-rung-wordround, wide-rung-ragged} vs {wide-rung-align16} (45% apart)

The field splits into a fast tier {wide-rung-ragged-overread, wide-rung-wordround-alias, wide-rung-wordround, wide-rung-ragged} and a slow tier {wide-rung-align16} with a 45% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-ragged-overread is inconsistent: worst-20% is 1.7x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 262.88 us but its worst 20% at 434.10 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 343181.0 ns median (-35.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.56x (fastest 343181.0 ns, slowest 535411.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 521392ns | 535923ns | 444179ns | 528851ns | 576231ns | base |
| wide-rung-ragged | 365806ns | 369985ns | 306578ns | 360214ns | 441809ns | -29.84% |
| wide-rung-ragged-overread | 345728ns | 343660ns | 263284ns | 343585ns | 434602ns | -33.69% |
| wide-rung-wordround | 403230ns | 366169ns | 330441ns | 389489ns | 517243ns | -22.66% |
| wide-rung-wordround-alias | 387848ns | 363575ns | 357801ns | 366817ns | 480989ns | -25.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 520755ns | 443366ns | 575620ns | base | 0.881 |
| wide-rung-ragged | 365355ns | 306188ns | 441261ns | -29.84% | 1.256 |
| wide-rung-ragged-overread | 345304ns | 262884ns | 434100ns | -33.69% | 1.329 |
| wide-rung-wordround | 402783ns | 329976ns | 516800ns | -22.65% | 1.139 |
| wide-rung-wordround-alias | 387381ns | 357397ns | 480376ns | -25.61% | 1.184 |

## Performance model

- Peak throughput: **1.745 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.857 | 49.1% |
| wide-rung-ragged | 1.242 | 71.1% |
| wide-rung-ragged-overread | 1.337 | 76.6% |
| wide-rung-wordround | 1.254 | 71.9% |
| wide-rung-wordround-alias | 1.263 | 72.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 521392ns | 521392ns | base |
| wide-rung-ragged | 365806ns | 365806ns | -29.84% |
| wide-rung-ragged-overread | 345728ns | 345728ns | -33.69% |
| wide-rung-wordround | 403230ns | 403230ns | -22.66% |
| wide-rung-wordround-alias | 387848ns | 387848ns | -25.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 535412ns | base | --- | [525730, 538781] | --- | --- | --- | --- |
| wide-rung-ragged | 369488ns | -154992.9ns (-28.9%) | [-156725, -124474]ns | [338745, 378936] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 343181ns | -165001.0ns (-30.8%) | [-174912, -145479]ns | [324507, 369672] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 365852ns | -164360.1ns (-30.7%) | [-207050, -80793]ns | [354145, 414169] | YES | 0.0022 | 0.0022 | 0 |
| wide-rung-wordround-alias | 363188ns | -146767.9ns (-27.4%) | [-170115, -95728]ns | [362738, 365021] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 525644ns | -29.7% | -26.4% | -30.2% | -32.1% |
| 1 | 2 | 536105ns | -29.1% | -32.5% | -38.7% | -31.9% |
| 1 | 3 | 567043ns | -46.3% | -54.3% | -39.2% | -36.0% |
| 1 | 4 | 443416ns | -27.8% | -37.0% | +8.0% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.315 | moderate- |
| wide-rung-ragged | 0.109 | ok |
| wide-rung-ragged-overread | 0.237 | moderate+ |
| wide-rung-wordround | -0.073 | ok |
| wide-rung-wordround-alias | -0.316 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 173.7ns | 520755.2ns | 0.0% |  |
| wide-rung-ragged | 75.9ns | 365355.3ns | 0.0% |  |
| wide-rung-ragged-overread | 85.8ns | 345304.3ns | 0.0% |  |
| wide-rung-wordround | 145.5ns | 402783.3ns | 0.0% |  |
| wide-rung-wordround-alias | 135.5ns | 387381.3ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 443366.1-575620.4 ns)
  443366.1 |##########################
  449978.8 |
  456591.6 |
  463204.3 |
  469817.0 |
  476429.7 |
  483042.4 |
  489655.1 |
  496267.8 |
  502880.5 |
  509493.2 |####
  516106.0 |
  522718.7 |######################
  529331.4 |######################
  535944.1 |######################
  542556.8 |
  549169.5 |
  555782.2 |
  562394.9 |########################################
  569007.7 |####
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 306187.8-441260.8 ns)
  306187.8 |
  312941.5 |#################
  319695.1 |#############
  326448.8 |
  333202.4 |
  339956.1 |
  346709.7 |####
  353463.4 |
  360217.0 |
  366970.7 |########################################
  373724.3 |###############################
  380478.0 |####
  387231.6 |
  393985.3 |########
  400738.9 |####
  407492.5 |
  414246.2 |
  420999.8 |####
  427753.5 |
  434507.1 |
  (7 below, 4 above range)

wide-rung-ragged-overread (n=40, range 262883.7-434100.2 ns)
  262883.7 |
  271444.5 |############################
  280005.4 |#####
  288566.2 |
  297127.0 |
  305687.8 |#####
  314248.7 |#####
  322809.5 |
  331370.3 |###########
  339931.1 |############################
  348492.0 |
  357052.8 |############################
  365613.6 |
  374174.4 |#####
  382735.3 |########################################
  391296.1 |#####
  399856.9 |
  408417.7 |
  416978.6 |
  425539.4 |
  (6 below, 5 above range)

wide-rung-wordround (n=40, range 329975.8-516800.0 ns)
  329975.8 |#####
  339317.0 |##############################
  348658.2 |##########
  357999.4 |########################################
  367340.6 |
  376681.8 |
  386023.0 |#####
  395364.2 |##########
  404705.4 |
  414046.7 |
  423387.9 |#####
  432729.1 |
  442070.3 |
  451411.5 |#####
  460752.7 |
  470093.9 |##############################
  479435.1 |##########
  488776.3 |
  498117.6 |
  507458.8 |
  (6 below, 4 above range)

wide-rung-wordround-alias (n=40, range 357396.9-480375.8 ns)
  357396.9 |########################################
  363545.9 |######################
  369694.8 |
  375843.8 |
  381992.7 |
  388141.6 |
  394290.6 |
  400439.5 |
  406588.5 |
  412737.4 |
  418886.4 |
  425035.3 |#######
  431184.2 |##
  437333.2 |
  443482.1 |##
  449631.1 |
  455780.0 |
  461928.9 |
  468077.9 |##
  474226.8 |
  (6 below, 3 above range)

```
