# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.54)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.54, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.58 us) is smaller than the fastest variant's own run-to-run std-dev (4.51 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (495.17 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.5% of the fastest

All 5 variants sit between 495.17 us and 497.75 us - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 495173.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 495173.8 ns, slowest 497752.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 496838ns | 496119ns | 491475ns | 496245ns | 503981ns | base |
| wide-rung-ragged | 499202ns | 497243ns | 491234ns | 496985ns | 513819ns | +0.48% |
| wide-rung-ragged-overread | 497421ns | 496361ns | 490066ns | 496325ns | 508066ns | +0.12% |
| wide-rung-wordround | 509836ns | 498119ns | 490115ns | 498379ns | 563926ns | +2.62% |
| wide-rung-wordround-alias | 500107ns | 498806ns | 492209ns | 498942ns | 511501ns | +0.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 495904ns | 490577ns | 502946ns | base | 0.925 |
| wide-rung-ragged | 498176ns | 490270ns | 512711ns | +0.46% | 0.921 |
| wide-rung-ragged-overread | 496446ns | 488988ns | 507140ns | +0.11% | 0.924 |
| wide-rung-wordround | 508711ns | 489297ns | 562465ns | +2.58% | 0.902 |
| wide-rung-wordround-alias | 499092ns | 491205ns | 510640ns | +0.64% | 0.919 |

## Performance model

- Peak throughput: **0.938 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.926 | 98.8% |
| wide-rung-ragged | 0.925 | 98.6% |
| wide-rung-ragged-overread | 0.926 | 98.7% |
| wide-rung-wordround | 0.923 | 98.4% |
| wide-rung-wordround-alias | 0.922 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 496838ns | 496838ns | base |
| wide-rung-ragged | 499202ns | 499202ns | +0.48% |
| wide-rung-ragged-overread | 497421ns | 497421ns | +0.12% |
| wide-rung-wordround | 509836ns | 509836ns | +2.62% |
| wide-rung-wordround-alias | 500107ns | 500107ns | +0.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 495174ns | base | --- | [493777, 496201] | --- | --- | --- | --- |
| wide-rung-ragged | 496060ns | no significant difference | [-2538, +3129]ns | [494769, 497158] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-ragged-overread | 495269ns | no significant difference | [-1548, +2840]ns | [493803, 496515] | no | 0.6358 | 0.6358 | 0 |
| wide-rung-wordround | 496934ns | no significant difference | [-2429, +6057]ns | [496015, 499052] | no | 0.5364 | 0.2682 | 0 |
| wide-rung-wordround-alias | 497752ns | +4196.5ns (+0.8%) | [+568, +5384]ns | [496093, 499614] | YES (adj: no) | 0.1539 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 490562ns | -0.4% | +0.7% | +1.3% | -0.2% |
| 2 | 490099ns | +1.9% | +1.0% | -0.2% | +6.1% |
| 3 | 501426ns | -0.4% | -0.9% | -2.7% | +0.2% |
| 4 | 499104ns | -0.5% | -0.7% | -0.6% | +0.0% |
| 5 | 497926ns | -1.6% | -1.5% | -1.4% | +5.0% |
| 6 | 493698ns | +1.5% | +4.2% | -0.7% | +0.8% |
| 7 | 501985ns | -1.9% | -0.1% | +2.0% | -1.3% |
| 8 | 499175ns | -1.2% | +5.0% | -0.7% | -1.1% |
| 9 | 494764ns | -0.8% | +1.8% | -1.3% | +1.1% |
| 10 | 496748ns | -0.6% | +0.9% | -1.5% | -0.8% |
| 11 | 494131ns | +1.1% | -0.0% | +41.7% | +0.9% |
| 12 | 495539ns | -1.4% | +0.0% | +27.0% | +0.2% |
| 13 | 502413ns | -1.2% | -2.8% | +25.7% | -1.3% |
| 14 | 496232ns | +0.5% | -1.6% | +1.9% | -1.3% |
| 15 | 509805ns | -2.8% | -4.0% | -2.5% | -4.0% |
| 16 | 502600ns | -1.6% | -2.6% | -1.3% | +1.7% |
| 17 | 494809ns | -0.8% | -1.1% | +0.5% | -0.6% |
| 18 | 495762ns | -0.2% | -1.2% | -0.4% | -0.9% |
| 19 | 499139ns | -1.3% | -2.1% | +2.7% | -1.6% |
| 20 | 492732ns | +1.9% | -1.0% | +2.0% | +1.1% |
| 21 | 494629ns | -1.0% | +0.1% | +0.8% | -0.1% |
| 22 | 493520ns | +0.7% | +0.7% | +1.6% | +0.2% |
| 23 | 491050ns | +0.9% | +1.0% | +2.6% | +3.2% |
| 24 | 493058ns | +2.0% | +0.2% | +2.3% | +2.5% |
| 25 | 491324ns | +3.3% | +0.3% | +1.7% | +0.9% |
| 26 | 506003ns | -1.8% | -2.4% | -1.7% | -1.9% |
| 27 | 496072ns | +1.0% | -0.5% | +1.6% | +2.1% |
| 28 | 488437ns | +2.3% | +0.5% | +1.2% | +1.7% |
| 29 | 490412ns | +0.8% | +0.5% | +0.7% | +1.0% |
| 30 | 496170ns | -0.3% | +1.1% | +0.2% | +1.2% |
| 31 | 499632ns | -0.4% | -0.7% | -0.7% | -0.6% |
| 32 | 495562ns | -0.3% | -0.1% | -1.3% | +1.1% |
| 33 | 495811ns | -0.2% | +0.1% | -1.0% | +0.6% |
| 34 | 493856ns | +0.5% | +0.8% | -1.0% | +1.2% |
| 35 | 490881ns | +1.3% | +3.4% | +1.7% | +1.9% |
| 36 | 499700ns | -0.5% | -0.0% | +0.2% | +2.1% |
| 37 | 492390ns | +0.7% | +1.7% | +0.8% | +2.0% |
| 38 | 494642ns | +0.6% | +0.2% | +0.1% | +0.7% |
| 39 | 492501ns | -0.2% | +2.1% | +1.0% | +1.0% |
| 40 | 491853ns | +19.4% | +1.9% | +1.0% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.170 | ok |
| wide-rung-ragged | -0.049 | ok |
| wide-rung-ragged-overread | 0.434 | moderate+ |
| wide-rung-wordround | 0.536 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | -0.024 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 23/40, lost 17/40
- **wide-rung-ragged-overread**: won 15/40, lost 20/40
- **wide-rung-wordround**: won 16/40, lost 24/40
- **wide-rung-wordround-alias**: won 12/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 6.6ns | 495903.9ns | 0.0% |  |
| wide-rung-ragged | 8.2ns | 498175.8ns | 0.0% |  |
| wide-rung-ragged-overread | 6.4ns | 496446.1ns | 0.0% |  |
| wide-rung-wordround | 16.4ns | 508711.4ns | 0.0% |  |
| wide-rung-wordround-alias | 6.4ns | 499091.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 490577.3-502945.8 ns)
  490577.3 |################
  491195.7 |########
  491814.1 |################
  492432.6 |################
  493051.0 |################
  493669.4 |########################
  494287.8 |################################
  494906.3 |
  495524.7 |########################################
  496143.1 |########################
  496761.5 |
  497380.0 |########
  497998.4 |
  498616.8 |########################
  499235.2 |################
  499853.6 |
  500472.1 |
  501090.5 |########
  501708.9 |########
  502327.3 |################
  (4 below, 2 above range)

wide-rung-ragged (n=40, range 490270.5-512710.7 ns)
  490270.5 |#############
  491392.5 |#############
  492514.5 |#############
  493636.5 |##########################
  494758.5 |#################################
  495880.5 |##########################
  497002.5 |########################################
  498124.6 |######
  499246.6 |##########################
  500368.6 |#############
  501490.6 |######
  502612.6 |######
  503734.6 |
  504856.6 |
  505978.6 |
  507100.7 |######
  508222.7 |
  509344.7 |
  510466.7 |
  511588.7 |
  (4 below, 1 above range)

wide-rung-ragged-overread (n=40, range 488988.2-507140.1 ns)
  488988.2 |########################
  489895.8 |################
  490803.4 |########
  491711.0 |########
  492618.5 |########
  493526.1 |########################################
  494433.7 |########################
  495341.3 |########################################
  496248.9 |########################
  497156.5 |########
  498064.1 |
  498971.7 |########
  499879.3 |
  500786.9 |########################################
  501694.5 |
  502602.1 |################
  503509.7 |
  504417.3 |
  505324.9 |
  506232.5 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 489296.6-562464.6 ns)
  489296.6 |################
  492955.0 |####################################
  496613.4 |########################################
  500271.8 |####################
  503930.2 |########
  507588.6 |
  511247.0 |########
  514905.4 |
  518563.8 |
  522222.2 |
  525880.6 |
  529539.0 |
  533197.4 |
  536855.8 |
  540514.2 |
  544172.6 |
  547831.0 |
  551489.4 |
  555147.8 |
  558806.2 |
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 491204.9-510640.2 ns)
  491204.9 |####################
  492176.7 |##########
  493148.4 |##########
  494120.2 |####################
  495092.0 |########################################
  496063.7 |########################################
  497035.5 |##############################
  498007.3 |########################################
  498979.0 |####################
  499950.8 |##############################
  500922.6 |##########
  501894.3 |####################
  502866.1 |
  503837.9 |
  504809.6 |##########
  505781.4 |####################
  506753.2 |
  507724.9 |
  508696.7 |
  509668.5 |##########
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-wordround**: autocorrelation=0.54 (measurement drift or warm-up artifact)
