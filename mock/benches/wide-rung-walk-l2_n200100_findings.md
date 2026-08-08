# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.57)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (487.81 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.3% of the fastest

All 5 variants sit between 487.81 us and 494.04 us - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 487811.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.01x (fastest 487811.2 ns, slowest 494037.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 490495ns | 488968ns | 485618ns | 489842ns | 497331ns | base |
| wide-rung-ragged | 495288ns | 494209ns | 491472ns | 494529ns | 501382ns | +0.98% |
| wide-rung-ragged-overread | 495904ns | 494936ns | 492448ns | 494895ns | 502385ns | +1.10% |
| wide-rung-wordround | 493041ns | 491364ns | 486701ns | 491343ns | 504474ns | +0.52% |
| wide-rung-wordround-alias | 493666ns | 493046ns | 487825ns | 493087ns | 501242ns | +0.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 489242ns | 484312ns | 496286ns | base | 0.938 |
| wide-rung-ragged | 494266ns | 490448ns | 500349ns | +1.03% | 0.928 |
| wide-rung-ragged-overread | 495029ns | 491832ns | 501515ns | +1.18% | 0.927 |
| wide-rung-wordround | 492079ns | 485776ns | 503467ns | +0.58% | 0.932 |
| wide-rung-wordround-alias | 492622ns | 486851ns | 500134ns | +0.69% | 0.931 |

## Performance model

- Peak throughput: **0.947 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.940 | 99.3% |
| wide-rung-ragged | 0.930 | 98.2% |
| wide-rung-ragged-overread | 0.929 | 98.0% |
| wide-rung-wordround | 0.936 | 98.8% |
| wide-rung-wordround-alias | 0.932 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 490495ns | 490495ns | base |
| wide-rung-ragged | 495288ns | 495288ns | +0.98% |
| wide-rung-ragged-overread | 495904ns | 495904ns | +1.10% |
| wide-rung-wordround | 493041ns | 493041ns | +0.52% |
| wide-rung-wordround-alias | 493666ns | 493666ns | +0.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 487811ns | base | --- | [486536, 490770] | --- | --- | --- | --- |
| wide-rung-ragged | 493391ns | +5580.6ns (+1.1%) | [+4126, +6572]ns | [492506, 494591] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 494037ns | +5751.2ns (+1.2%) | [+5222, +7784]ns | [493067, 494657] | YES | 0.0014 | 0.0007 | 0 |
| wide-rung-wordround | 490335ns | no significant difference | [-1281, +3205]ns | [488403, 491586] | no | 0.1539 | 0.1539 | 0 |
| wide-rung-wordround-alias | 492026ns | +3135.4ns (+0.6%) | [+935, +4496]ns | [490458, 493727] | YES | 0.0086 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 486348ns | +1.0% | +1.3% | +0.6% | +6.5% |
| 2 | 484823ns | +1.3% | +2.0% | +1.3% | +2.3% |
| 3 | 484744ns | +1.0% | +2.2% | +0.1% | +0.8% |
| 4 | 487384ns | +1.2% | +1.8% | +0.2% | +0.2% |
| 5 | 495567ns | -1.1% | +0.4% | -1.0% | -0.4% |
| 6 | 485345ns | +1.4% | +3.4% | +0.3% | +1.1% |
| 7 | 485664ns | +1.4% | +2.1% | +6.6% | +1.6% |
| 8 | 485434ns | +1.8% | +1.9% | +1.3% | +3.2% |
| 9 | 496699ns | -0.7% | -0.5% | -1.7% | -1.4% |
| 10 | 489052ns | +0.7% | +1.2% | +0.5% | +0.3% |
| 11 | 489361ns | +1.4% | +1.2% | -0.4% | +0.1% |
| 12 | 487958ns | +3.4% | +1.6% | +0.1% | +0.8% |
| 13 | 484369ns | +4.6% | +1.6% | +0.1% | +1.0% |
| 14 | 487058ns | +2.3% | +5.3% | +0.0% | +0.4% |
| 15 | 486929ns | +1.6% | +1.2% | +0.3% | +1.7% |
| 16 | 487946ns | +1.3% | +1.1% | -0.4% | +0.7% |
| 17 | 497504ns | -1.3% | -0.6% | -1.9% | -1.4% |
| 18 | 499662ns | +0.6% | -1.3% | -2.3% | -1.8% |
| 19 | 487676ns | +2.4% | +0.8% | +0.3% | +1.0% |
| 20 | 494140ns | +0.4% | -0.3% | -2.3% | -0.9% |
| 21 | 494341ns | -0.6% | -0.2% | -0.8% | -1.8% |
| 22 | 485475ns | +0.9% | +1.6% | +1.6% | -0.2% |
| 23 | 486725ns | +0.6% | +1.2% | +5.8% | -0.3% |
| 24 | 484719ns | +1.3% | +1.9% | +2.5% | +2.0% |
| 25 | 483857ns | +1.5% | +2.2% | +3.6% | +1.4% |
| 26 | 483354ns | +2.0% | +2.3% | +2.9% | +0.7% |
| 27 | 490942ns | +1.0% | +1.2% | +2.1% | +0.3% |
| 28 | 485643ns | +1.5% | +1.3% | +1.7% | +2.1% |
| 29 | 485006ns | +1.3% | +1.6% | +3.2% | +2.1% |
| 30 | 483622ns | +2.3% | +1.9% | +2.9% | +0.7% |
| 31 | 489975ns | +0.9% | +1.0% | +1.1% | -0.2% |
| 32 | 487563ns | +1.4% | +1.1% | +0.7% | +1.5% |
| 33 | 494660ns | -0.4% | -0.4% | -1.7% | -0.4% |
| 34 | 497311ns | -0.8% | -0.6% | -1.1% | -0.4% |
| 35 | 494540ns | +0.0% | -0.8% | -1.2% | +0.2% |
| 36 | 491085ns | +0.9% | +4.7% | -1.0% | +0.2% |
| 37 | 490597ns | +0.4% | +1.2% | -0.5% | +0.6% |
| 38 | 492608ns | +0.8% | +0.3% | -0.5% | +2.2% |
| 39 | 491769ns | +1.3% | -0.1% | +0.6% | +0.8% |
| 40 | 492212ns | +0.3% | +0.1% | -0.2% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.369 | moderate+ |
| wide-rung-ragged | 0.568 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | -0.070 | ok |
| wide-rung-wordround | 0.266 | moderate+ |
| wide-rung-wordround-alias | 0.231 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 6/40, lost 33/40
- **wide-rung-ragged-overread**: won 9/40, lost 30/40
- **wide-rung-wordround**: won 15/40, lost 24/40
- **wide-rung-wordround-alias**: won 11/40, lost 28/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 4.9ns | 489241.7ns | 0.0% |  |
| wide-rung-ragged | 5.5ns | 494265.7ns | 0.0% |  |
| wide-rung-ragged-overread | 6.9ns | 495029.5ns | 0.0% |  |
| wide-rung-wordround | 6.2ns | 492079.0ns | 0.0% |  |
| wide-rung-wordround-alias | 5.6ns | 492622.2ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 484311.6-496285.5 ns)
  484311.6 |########################################
  484910.3 |########################################
  485509.0 |####################
  486107.7 |##########
  486706.4 |##############################
  487305.1 |##############################
  487903.8 |####################
  488502.5 |##########
  489101.2 |##########
  489699.9 |##########
  490298.6 |##########
  490897.3 |####################
  491496.0 |##########
  492094.7 |####################
  492693.4 |
  493292.1 |
  493890.7 |####################
  494489.4 |####################
  495088.1 |##########
  495686.8 |
  (3 below, 4 above range)

wide-rung-ragged (n=40, range 490447.6-500348.8 ns)
  490447.6 |#############
  490942.6 |#################################
  491437.7 |
  491932.8 |#############
  492427.8 |#################################
  492922.9 |#############
  493417.9 |#############
  493913.0 |######
  494408.1 |########################################
  494903.1 |
  495398.2 |#############
  495893.3 |#############
  496388.3 |######
  496883.4 |
  497378.5 |
  497873.5 |#############
  498368.6 |
  498863.6 |
  499358.7 |######
  499853.8 |
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 491831.9-501515.4 ns)
  491831.9 |#####
  492316.0 |########################################
  492800.2 |######################
  493284.4 |###########
  493768.6 |######################
  494252.7 |############################
  494736.9 |#################
  495221.1 |#####
  495705.3 |#################
  496189.4 |#####
  496673.6 |#####
  497157.8 |#####
  497642.0 |
  498126.1 |
  498610.3 |
  499094.5 |
  499578.7 |
  500062.8 |
  500547.0 |
  501031.2 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 485775.5-503467.0 ns)
  485775.5 |####################
  486660.1 |#############
  487544.7 |########################################
  488429.2 |#################################
  489313.8 |
  490198.4 |##########################
  491083.0 |#################################
  491967.5 |
  492852.1 |######
  493736.7 |######
  494621.3 |######
  495505.8 |######
  496390.4 |######
  497275.0 |#############
  498159.6 |
  499044.1 |
  499928.7 |######
  500813.3 |#############
  501697.9 |
  502582.4 |
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 486850.6-500134.4 ns)
  486850.6 |################
  487514.8 |
  488179.0 |################
  488843.2 |########################
  489507.4 |########################
  490171.5 |########################
  490835.7 |########################
  491499.9 |################
  492164.1 |########################
  492828.3 |
  493492.5 |########################
  494156.7 |################
  494820.9 |########################################
  495485.1 |################
  496149.3 |########
  496813.5 |
  497477.7 |
  498141.8 |
  498806.0 |
  499470.2 |
  (3 below, 3 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.57 (measurement drift or warm-up artifact)
