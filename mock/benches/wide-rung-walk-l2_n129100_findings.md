# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged dominates: 32% faster than the next best (wide-rung-ragged-overread)

wide-rung-ragged (446.84 us) leads wide-rung-ragged-overread (589.18 us) by 32%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 27% (significant)

wide-rung-ragged is -163.54 us (27%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {wide-rung-ragged} vs {wide-rung-ragged-overread, wide-rung-align16, wide-rung-wordround-alias, wide-rung-wordround} (32% apart)

The field splits into a fast tier {wide-rung-ragged} and a slow tier {wide-rung-ragged-overread, wide-rung-align16, wide-rung-wordround-alias, wide-rung-wordround} with a 32% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: wide-rung-ragged** at 446842.3 ns median (-26.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.37x (fastest 446842.3 ns, slowest 611600.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 607590ns | 607556ns | 596950ns | 607661ns | 618015ns | base |
| wide-rung-ragged | 448236ns | 447265ns | 442995ns | 447822ns | 454718ns | -26.23% |
| wide-rung-ragged-overread | 589737ns | 589772ns | 582057ns | 589484ns | 598175ns | -2.94% |
| wide-rung-wordround | 614492ns | 612579ns | 601590ns | 612362ns | 633785ns | +1.14% |
| wide-rung-wordround-alias | 611299ns | 608725ns | 603459ns | 609296ns | 625147ns | +0.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 606546ns | 595896ns | 616957ns | base | 0.756 |
| wide-rung-ragged | 447644ns | 442487ns | 454042ns | -26.20% | 1.025 |
| wide-rung-ragged-overread | 589031ns | 581272ns | 597474ns | -2.89% | 0.779 |
| wide-rung-wordround | 613565ns | 600590ns | 632907ns | +1.16% | 0.748 |
| wide-rung-wordround-alias | 610399ns | 602722ns | 623995ns | +0.64% | 0.752 |

## Performance model

- Peak throughput: **1.037 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.757 | 73.0% |
| wide-rung-ragged | 1.027 | 99.0% |
| wide-rung-ragged-overread | 0.779 | 75.1% |
| wide-rung-wordround | 0.750 | 72.3% |
| wide-rung-wordround-alias | 0.755 | 72.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 607590ns | 607590ns | base |
| wide-rung-ragged | 448236ns | 448236ns | -26.23% |
| wide-rung-ragged-overread | 589737ns | 589737ns | -2.94% |
| wide-rung-wordround | 614492ns | 614492ns | +1.14% |
| wide-rung-wordround-alias | 611299ns | 611299ns | +0.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 606402ns | base | --- | [604870, 608146] | --- | --- | --- | --- |
| wide-rung-ragged | 446842ns | -159471.2ns (-26.3%) | [-161645, -156297]ns | [446092, 448383] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 589180ns | -17154.8ns (-2.8%) | [-19581, -14087]ns | [586332, 591143] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 611601ns | +4391.0ns (+0.7%) | [+1624, +5878]ns | [610525, 613112] | YES | 0.0009 | 0.0007 | 0 |
| wide-rung-wordround-alias | 607986ns | +2492.9ns (+0.4%) | [+588, +4501]ns | [606625, 611665] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 599805ns | -26.1% | -2.8% | +0.3% | +0.5% |
| 2 | 599183ns | -26.9% | -3.3% | +0.2% | +0.8% |
| 3 | 607330ns | -25.4% | -4.6% | -1.2% | -0.6% |
| 4 | 602621ns | -25.4% | -2.9% | -0.3% | +0.1% |
| 5 | 594332ns | -25.4% | -2.6% | +1.1% | +1.4% |
| 6 | 596496ns | -25.4% | -3.0% | +0.6% | +1.3% |
| 7 | 593916ns | -24.8% | -1.1% | +1.0% | +1.3% |
| 8 | 594986ns | -25.4% | -0.3% | +0.9% | +1.3% |
| 9 | 596539ns | -26.2% | -1.5% | +2.0% | +1.1% |
| 10 | 606470ns | -27.3% | -1.9% | +0.4% | -0.7% |
| 11 | 623157ns | -27.4% | -6.4% | -0.9% | -2.2% |
| 12 | 607135ns | -25.1% | -3.7% | -0.2% | -0.6% |
| 13 | 602735ns | -24.9% | -2.8% | +0.4% | +0.0% |
| 14 | 602365ns | -25.9% | -3.1% | +2.1% | +1.6% |
| 15 | 602496ns | -24.8% | -2.6% | +1.9% | +0.7% |
| 16 | 591911ns | -23.5% | -0.5% | +3.7% | +15.6% |
| 17 | 610970ns | -26.0% | -4.5% | +0.3% | +0.8% |
| 18 | 611779ns | -26.4% | -4.5% | +0.4% | +0.1% |
| 19 | 604420ns | -26.0% | -3.2% | +7.3% | +0.8% |
| 20 | 617376ns | -27.0% | -5.2% | +0.3% | -2.1% |
| 21 | 605027ns | -25.8% | -2.4% | +0.9% | +0.2% |
| 22 | 611902ns | -26.9% | -3.4% | -0.1% | -0.1% |
| 23 | 619747ns | -27.6% | -1.0% | -0.2% | -1.7% |
| 24 | 605458ns | -26.3% | -2.2% | +2.0% | +0.5% |
| 25 | 604714ns | -26.1% | -2.2% | +1.1% | +0.3% |
| 26 | 607063ns | -26.6% | -1.9% | +0.7% | +0.1% |
| 27 | 605400ns | -25.8% | -1.3% | +1.4% | +0.5% |
| 28 | 605480ns | -26.4% | -1.4% | +1.0% | +0.4% |
| 29 | 605136ns | -26.4% | -2.0% | +1.0% | +0.4% |
| 30 | 606333ns | -26.4% | -2.3% | +0.9% | +0.1% |
| 31 | 613738ns | -27.4% | -4.2% | +2.0% | -0.2% |
| 32 | 614555ns | -27.2% | -4.0% | -0.3% | -0.2% |
| 33 | 611102ns | -24.5% | -2.8% | +14.3% | +0.5% |
| 34 | 618623ns | -27.6% | -4.2% | -0.1% | -0.7% |
| 35 | 616559ns | -27.7% | -4.3% | -1.0% | -0.4% |
| 36 | 609696ns | -26.9% | -3.1% | +0.0% | +0.6% |
| 37 | 607930ns | -26.4% | -3.3% | +0.9% | +1.0% |
| 38 | 608362ns | -26.6% | -2.7% | +0.7% | +0.9% |
| 39 | 607828ns | -26.6% | -2.3% | +0.8% | +1.5% |
| 40 | 611164ns | -27.0% | -3.5% | +0.1% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.463 | moderate+ |
| wide-rung-ragged | 0.263 | moderate+ |
| wide-rung-ragged-overread | 0.488 | moderate+ |
| wide-rung-wordround | 0.173 | ok |
| wide-rung-wordround-alias | 0.144 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 40/40, lost 0/40
- **wide-rung-ragged-overread**: won 40/40, lost 0/40
- **wide-rung-wordround**: won 9/40, lost 29/40
- **wide-rung-wordround-alias**: won 11/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 7.2ns | 606546.0ns | 0.0% |  |
| wide-rung-ragged | 4.9ns | 447644.3ns | 0.0% |  |
| wide-rung-ragged-overread | 6.3ns | 589031.5ns | 0.0% |  |
| wide-rung-wordround | 9.6ns | 613565.5ns | 0.0% |  |
| wide-rung-wordround-alias | 8.2ns | 610398.6ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 595895.9-616957.1 ns)
  595895.9 |####################
  596949.0 |
  598002.0 |
  599055.1 |####################
  600108.2 |
  601161.2 |
  602214.3 |########################################
  603267.3 |
  604320.4 |########################################
  605373.5 |########################################
  606426.5 |########################################
  607479.6 |##############################
  608532.7 |
  609585.7 |##########
  610638.8 |##############################
  611691.8 |####################
  612744.9 |##########
  613798.0 |##########
  614851.0 |
  615904.1 |##########
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 442487.3-454041.6 ns)
  442487.3 |
  443065.0 |#################
  443642.7 |
  444220.4 |#####
  444798.2 |###########
  445375.9 |#################
  445953.6 |########################################
  446531.3 |#####
  447109.0 |############################
  447686.7 |#####
  448264.5 |#####
  448842.2 |###########
  449419.9 |#####
  449997.6 |#####
  450575.3 |#####
  451153.0 |
  451730.7 |
  452308.5 |######################
  452886.2 |###########
  453463.9 |
  (3 below, 2 above range)

wide-rung-ragged-overread (n=40, range 581271.8-597474.3 ns)
  581271.8 |
  582081.9 |
  582892.0 |##############################
  583702.1 |##############################
  584512.3 |####################
  585322.4 |####################
  586132.5 |
  586942.7 |########################################
  587752.8 |##########
  588562.9 |##########
  589373.0 |##############################
  590183.2 |##############################
  590993.3 |##########
  591803.4 |##############################
  592613.6 |##############################
  593423.7 |####################
  594233.8 |
  595043.9 |####################
  595854.1 |
  596664.2 |##########
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 600589.7-632906.9 ns)
  600589.7 |#############
  602205.6 |
  603821.5 |####
  605437.3 |####
  607053.2 |####
  608669.0 |########
  610284.9 |########################################
  611900.8 |######################
  613516.6 |#################
  615132.5 |####
  616748.3 |#############
  618364.2 |########
  619980.0 |
  621595.9 |
  623211.8 |
  624827.6 |####
  626443.5 |
  628059.3 |
  629675.2 |
  631291.1 |
  (5 below, 2 above range)

wide-rung-wordround-alias (n=40, range 602722.2-623994.6 ns)
  602722.2 |########################################
  603785.8 |##########
  604849.4 |
  605913.1 |####################
  606976.7 |###############
  608040.3 |##########
  609103.9 |###############
  610167.5 |#####
  611231.2 |#####
  612294.8 |###############
  613358.4 |##############################
  614422.0 |
  615485.7 |#####
  616549.3 |#####
  617612.9 |#####
  618676.5 |
  619740.1 |
  620803.8 |
  621867.4 |
  622931.0 |
  (3 below, 1 above range)

```
