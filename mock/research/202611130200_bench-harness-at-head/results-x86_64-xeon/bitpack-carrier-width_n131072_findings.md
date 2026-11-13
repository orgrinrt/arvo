# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

5 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 dominates: 33% faster than the next best (bitpack-carrier-packed)

bitpack-carrier-d64 (14.03 us) leads bitpack-carrier-packed (18.68 us) by 33%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-d64 beats baseline by 48% (significant)

bitpack-carrier-d64 is -12.97 us (48%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d16-control is an outlier: 2.0x slower than the field

bitpack-carrier-d16-control (28.12 us) is 2.0x the fastest (14.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-carrier-d16-control shows alternating (throttle bounce) (autocorr -0.71)

bitpack-carrier-d16-control's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-carrier-packed is inconsistent: worst-20% is 1.7x its best-20%

bitpack-carrier-packed's best 20% of batches run at 16.62 us but its worst 20% at 29.04 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-carrier-d64** at 14032.5 ns median (-47.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.00x (fastest 14032.5 ns, slowest 28117.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 27098ns | 27071ns | 24681ns | 27065ns | 29612ns | base |
| bitpack-carrier-d16-control | 30771ns | 28340ns | 25629ns | 29470ns | 39817ns | +13.56% |
| bitpack-carrier-d32 | 19439ns | 19964ns | 16418ns | 19936ns | 20967ns | -28.26% |
| bitpack-carrier-d64 | 14451ns | 14194ns | 13702ns | 14278ns | 15721ns | -46.67% |
| bitpack-carrier-packed | 20820ns | 18825ns | 16756ns | 19334ns | 29341ns | -23.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 26909ns | 24524ns | 29377ns | base | 4.871 |
| bitpack-carrier-d16-control | 30560ns | 25447ns | 39575ns | +13.57% | 4.289 |
| bitpack-carrier-d32 | 19260ns | 16278ns | 20776ns | -28.42% | 6.805 |
| bitpack-carrier-d64 | 14271ns | 13522ns | 15546ns | -46.96% | 9.184 |
| bitpack-carrier-packed | 20609ns | 16623ns | 29043ns | -23.41% | 6.360 |

## Performance model

- Peak throughput: **9.693 Gops/s** (bitpack-carrier-d64; best 20% batches)
- Ops per call: 131072

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 4.874 | 50.3% |
| bitpack-carrier-d16-control | 4.662 | 48.1% |
| bitpack-carrier-d32 | 6.623 | 68.3% |
| bitpack-carrier-d64 | 9.341 | 96.4% |
| bitpack-carrier-packed | 7.017 | 72.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 27098ns | 27098ns | base |
| bitpack-carrier-d16-control | 30771ns | 30771ns | +13.56% |
| bitpack-carrier-d32 | 19439ns | 19439ns | -28.26% |
| bitpack-carrier-d64 | 14451ns | 14451ns | -46.67% |
| bitpack-carrier-packed | 20820ns | 20820ns | -23.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 26894ns | base | --- | [26457, 27401] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 28117ns | no significant difference | [-792, +6699]ns | [27225, 31965] | no | 0.8746 | 0.8746 | 0 |
| bitpack-carrier-d32 | 19790ns | -6671.1ns (-24.8%) | [-7034, -6344]ns | [19560, 20145] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 14032ns | -12970.2ns (-48.2%) | [-13391, -12558]ns | [13896, 14245] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 18680ns | -7956.8ns (-29.6%) | [-8360, -6398]ns | [16853, 21117] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed |
|---|---|---|---|---|---|---|
| 1 | 1 | 24882ns | +29.0% | -21.4% | -41.2% | -32.7% |
| 1 | 2 | 26593ns | -4.2% | -25.4% | -48.3% | -23.1% |
| 1 | 3 | 27091ns | +46.7% | -24.7% | -48.1% | -38.0% |
| 1 | 4 | 28561ns | -3.9% | -43.3% | -49.1% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.123 | ok |
| bitpack-carrier-d16-control | -0.714 | HIGH- (thermal bounce) |
| bitpack-carrier-d32 | -0.212 | moderate- |
| bitpack-carrier-d64 | -0.306 | moderate- |
| bitpack-carrier-packed | -0.305 | moderate- |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 2/4, lost 2/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 4/4, lost 0/4
- **bitpack-carrier-packed**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 64.6ns | 26908.5ns | 0.2% |  |
| bitpack-carrier-d16-control | 74.9ns | 30559.7ns | 0.2% |  |
| bitpack-carrier-d32 | 76.9ns | 19260.2ns | 0.4% |  |
| bitpack-carrier-d64 | 68.9ns | 14271.2ns | 0.5% |  |
| bitpack-carrier-packed | 60.1ns | 20609.3ns | 0.3% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 24524.0-29376.8 ns)
  24524.0 |##############################
  24766.6 |########################################
  25009.2 |##########
  25251.9 |##########
  25494.5 |
  25737.2 |
  25979.8 |####################
  26222.4 |####################
  26465.1 |########################################
  26707.7 |##############################
  26950.4 |########################################
  27193.0 |##########
  27435.6 |####################
  27678.3 |##########
  27920.9 |##########
  28163.6 |##############################
  28406.2 |####################
  28648.8 |
  28891.5 |##########
  29134.1 |##########
  (1 below, 3 above range)

bitpack-carrier-d16-control (n=40, range 25446.9-39574.9 ns)
  25446.9 |######################
  26153.3 |
  26859.7 |########################################
  27566.1 |########
  28272.5 |####
  28978.9 |
  29685.3 |
  30391.7 |
  31098.1 |#############
  31804.5 |##########################
  32510.9 |####
  33217.3 |
  33923.7 |
  34630.1 |
  35336.5 |
  36042.9 |
  36749.3 |####
  37455.7 |
  38162.1 |
  38868.5 |####
  (5 below, 6 above range)

bitpack-carrier-d32 (n=40, range 16278.0-20776.3 ns)
  16278.0 |######
  16502.9 |
  16727.8 |
  16952.7 |
  17177.6 |
  17402.5 |
  17627.5 |######
  17852.4 |
  18077.3 |
  18302.2 |######
  18527.1 |
  18752.0 |
  18976.9 |#############
  19201.9 |#############
  19426.8 |##########################
  19651.7 |########################################
  19876.6 |#############
  20101.5 |#################################
  20326.4 |########################################
  20551.3 |######
  (6 below, 3 above range)

bitpack-carrier-d64 (n=40, range 13522.4-15545.9 ns)
  13522.4 |########################
  13623.5 |########################
  13724.7 |################################
  13825.9 |########################
  13927.1 |########################################
  14028.2 |########################
  14129.4 |########################
  14230.6 |########
  14331.8 |########
  14432.9 |################
  14534.1 |
  14635.3 |########
  14736.5 |################
  14837.6 |########
  14938.8 |
  15040.0 |################
  15141.2 |
  15242.3 |########
  15343.5 |########
  15444.7 |
  (2 below, 2 above range)

bitpack-carrier-packed (n=40, range 16623.2-29043.0 ns)
  16623.2 |########################################
  17244.2 |
  17865.2 |
  18486.2 |
  19107.2 |
  19728.2 |###########
  20349.1 |##
  20970.1 |
  21591.1 |#########
  22212.1 |
  22833.1 |##
  23454.1 |##
  24075.1 |####
  24696.1 |
  25317.0 |
  25938.0 |
  26559.0 |
  27180.0 |
  27801.0 |##
  28422.0 |
  (3 below, 5 above range)

```

## Diagnostics

- **bitpack-carrier-packed**: CV=23.7% (high variance, measurements may be unstable)
