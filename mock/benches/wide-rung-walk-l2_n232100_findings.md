# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround-alias shows warm-up / thermal drift (autocorr +0.52)

wide-rung-wordround-alias's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (485.48 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 4.3% of the fastest

All 5 variants sit between 485.48 us and 506.34 us - a 4.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 485482.9 ns median
- 4 variants significantly slower than baseline
- Spread: 1.04x (fastest 485482.9 ns, slowest 506343.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 488978ns | 486368ns | 482649ns | 487295ns | 500355ns | base |
| wide-rung-ragged | 503200ns | 500308ns | 497598ns | 500764ns | 516108ns | +2.91% |
| wide-rung-ragged-overread | 497510ns | 497003ns | 494235ns | 496894ns | 502630ns | +1.74% |
| wide-rung-wordround | 508763ns | 504020ns | 493914ns | 504494ns | 536418ns | +4.05% |
| wide-rung-wordround-alias | 508014ns | 507312ns | 497503ns | 506952ns | 521712ns | +3.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 488044ns | 481739ns | 499476ns | base | 0.940 |
| wide-rung-ragged | 502380ns | 496894ns | 515223ns | +2.94% | 0.913 |
| wide-rung-ragged-overread | 496545ns | 493215ns | 501774ns | +1.74% | 0.924 |
| wide-rung-wordround | 507857ns | 493230ns | 535462ns | +4.06% | 0.903 |
| wide-rung-wordround-alias | 507058ns | 496732ns | 520642ns | +3.90% | 0.905 |

## Performance model

- Peak throughput: **0.952 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.945 | 99.2% |
| wide-rung-ragged | 0.918 | 96.4% |
| wide-rung-ragged-overread | 0.925 | 97.1% |
| wide-rung-wordround | 0.912 | 95.8% |
| wide-rung-wordround-alias | 0.906 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 488978ns | 488978ns | base |
| wide-rung-ragged | 503200ns | 503200ns | +2.91% |
| wide-rung-ragged-overread | 497510ns | 497510ns | +1.74% |
| wide-rung-wordround | 508763ns | 508763ns | +4.05% |
| wide-rung-wordround-alias | 508014ns | 508014ns | +3.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 485483ns | base | --- | [484143, 487936] | --- | --- | --- | --- |
| wide-rung-ragged | 499505ns | +13848.6ns (+2.9%) | [+12676, +15385]ns | [498402, 501164] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 496094ns | +10079.1ns (+2.1%) | [+7311, +10719]ns | [494644, 496671] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 503088ns | +18320.1ns (+3.8%) | [+14847, +20762]ns | [499891, 506241] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 506343ns | +19089.0ns (+3.9%) | [+15344, +23081]ns | [502951, 507742] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 482442ns | +3.5% | +4.1% | +3.4% | +5.1% |
| 2 | 486159ns | +2.8% | +1.4% | +3.9% | +4.1% |
| 3 | 483359ns | +4.0% | +2.1% | +1.5% | +5.3% |
| 4 | 482038ns | +3.8% | +3.5% | +8.6% | +5.4% |
| 5 | 487053ns | +3.4% | +1.6% | +4.3% | +3.8% |
| 6 | 508664ns | -1.8% | -2.8% | -1.5% | -0.3% |
| 7 | 485640ns | +2.6% | +2.2% | +3.0% | +3.6% |
| 8 | 486625ns | +2.9% | +2.0% | +3.9% | +4.1% |
| 9 | 498792ns | +0.9% | -0.4% | +0.8% | -1.9% |
| 10 | 485210ns | +2.5% | +2.3% | +3.1% | +2.3% |
| 11 | 482926ns | +4.0% | +2.0% | +2.0% | +7.1% |
| 12 | 480229ns | +3.4% | +2.5% | +4.5% | +7.6% |
| 13 | 482942ns | +3.7% | +2.9% | +3.6% | +7.1% |
| 14 | 482666ns | +3.1% | +2.2% | +2.5% | +6.2% |
| 15 | 478768ns | +4.0% | +3.2% | +4.3% | +5.0% |
| 16 | 491868ns | +1.9% | +0.4% | +0.8% | +2.9% |
| 17 | 483462ns | +2.7% | +2.2% | +3.1% | +4.4% |
| 18 | 490037ns | +1.7% | +0.8% | +1.3% | +2.2% |
| 19 | 484457ns | +2.6% | +2.1% | +3.9% | +5.1% |
| 20 | 484565ns | +2.6% | +2.5% | +5.4% | +3.8% |
| 21 | 482553ns | +3.2% | +2.8% | +1.6% | +3.2% |
| 22 | 486415ns | +3.4% | +2.1% | +22.2% | +2.1% |
| 23 | 488819ns | +2.0% | +1.4% | +12.0% | +2.3% |
| 24 | 484729ns | +3.0% | +2.0% | +5.8% | +2.9% |
| 25 | 482288ns | +3.3% | +2.3% | +2.8% | +3.2% |
| 26 | 484335ns | +2.9% | +2.4% | +5.6% | +3.6% |
| 27 | 483410ns | +3.3% | +2.4% | +4.8% | +4.9% |
| 28 | 485326ns | +2.6% | +2.2% | +3.7% | +2.7% |
| 29 | 483951ns | +3.2% | +2.1% | +4.5% | +3.4% |
| 30 | 490494ns | +1.0% | +1.1% | +1.9% | +1.6% |
| 31 | 483367ns | +6.3% | +3.3% | +4.2% | +6.3% |
| 32 | 508913ns | -1.7% | -0.3% | -3.2% | -0.5% |
| 33 | 486842ns | +3.1% | +3.0% | +1.3% | +4.2% |
| 34 | 497198ns | +2.0% | +0.2% | +5.3% | +2.2% |
| 35 | 494488ns | +17.0% | +0.7% | +5.2% | +4.5% |
| 36 | 492682ns | +2.8% | +1.1% | +3.5% | +2.6% |
| 37 | 496823ns | +0.2% | +0.7% | +4.0% | +7.4% |
| 38 | 491422ns | +2.0% | +1.4% | +7.1% | +6.4% |
| 39 | 491573ns | +2.7% | +1.0% | +1.5% | +5.8% |
| 40 | 498247ns | +1.2% | +1.6% | +6.6% | +4.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.113 | ok |
| wide-rung-ragged | 0.121 | ok |
| wide-rung-ragged-overread | 0.336 | moderate+ |
| wide-rung-wordround | 0.214 | moderate+ |
| wide-rung-wordround-alias | 0.517 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 2/40, lost 38/40
- **wide-rung-ragged-overread**: won 3/40, lost 37/40
- **wide-rung-wordround**: won 2/40, lost 38/40
- **wide-rung-wordround-alias**: won 3/40, lost 37/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 8.3ns | 488044.4ns | 0.0% |  |
| wide-rung-ragged | 8.1ns | 502379.7ns | 0.0% |  |
| wide-rung-ragged-overread | 5.6ns | 496544.6ns | 0.0% |  |
| wide-rung-wordround | 12.8ns | 507857.4ns | 0.0% |  |
| wide-rung-wordround-alias | 7.5ns | 507057.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 481738.6-499476.0 ns)
  481738.6 |######################
  482625.5 |########################################
  483512.4 |###########
  484399.2 |######################
  485286.1 |#################
  486173.0 |######################
  487059.8 |
  487946.7 |#####
  488833.6 |
  489720.4 |###########
  490607.3 |#####
  491494.2 |###########
  492381.0 |#####
  493267.9 |
  494154.8 |#####
  495041.7 |
  495928.5 |
  496815.4 |###########
  497702.3 |#####
  498589.1 |#####
  (2 below, 2 above range)

wide-rung-ragged (n=40, range 496894.4-515223.1 ns)
  496894.4 |#################
  497810.9 |########################################
  498727.3 |######################
  499643.7 |#############
  500560.2 |#################
  501476.6 |########
  502393.0 |#############
  503309.5 |####
  504225.9 |########
  505142.3 |
  506058.8 |########
  506975.2 |
  507891.7 |
  508808.1 |
  509724.5 |
  510641.0 |
  511557.4 |
  512473.8 |
  513390.3 |####
  514306.7 |
  (3 below, 1 above range)

wide-rung-ragged-overread (n=40, range 493215.5-501773.9 ns)
  493215.5 |########################
  493643.4 |################
  494071.3 |################################
  494499.2 |########################
  494927.2 |########
  495355.1 |################
  495783.0 |########################
  496210.9 |########################################
  496638.9 |########################
  497066.8 |
  497494.7 |########
  497922.6 |################
  498350.5 |########
  498778.5 |########
  499206.4 |########
  499634.3 |
  500062.2 |########
  500490.1 |
  500918.1 |########
  501346.0 |
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 493230.3-535461.5 ns)
  493230.3 |################
  495341.9 |########################
  497453.5 |################################
  499565.0 |########################################
  501676.6 |########################################
  503788.1 |########################
  505899.7 |########
  508011.2 |########
  510122.8 |########################
  512234.4 |########
  514345.9 |
  516457.5 |########
  518569.0 |########
  520680.6 |
  522792.2 |################
  524903.7 |########
  527015.3 |
  529126.8 |
  531238.4 |########
  533350.0 |
  (4 below, 2 above range)

wide-rung-wordround-alias (n=40, range 496732.1-520641.5 ns)
  496732.1 |#############
  497927.5 |##########################
  499123.0 |#############
  500318.5 |######
  501514.0 |######
  502709.4 |####################
  503904.9 |######
  505100.4 |##########################
  506295.9 |########################################
  507491.3 |#############
  508686.8 |#############
  509882.3 |
  511077.8 |
  512273.2 |######
  513468.7 |######
  514664.2 |
  515859.7 |#############
  517055.1 |#############
  518250.6 |
  519446.1 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **wide-rung-wordround-alias**: autocorrelation=0.52 (measurement drift or warm-up artifact)
