# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.55)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.6% of the fastest

All 5 variants sit between 866.12 us and 871.33 us - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged** at 866121.2 ns median (-0.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.01x (fastest 866121.2 ns, slowest 871332.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 871885ns | 870928ns | 868162ns | 871131ns | 877872ns | base |
| wide-rung-ragged | 867959ns | 867132ns | 864918ns | 867377ns | 872746ns | -0.45% |
| wide-rung-ragged-overread | 870198ns | 869895ns | 867990ns | 869951ns | 873147ns | -0.19% |
| wide-rung-wordround | 870918ns | 872168ns | 863468ns | 871786ns | 875766ns | -0.11% |
| wide-rung-wordround-alias | 869316ns | 867896ns | 865261ns | 867988ns | 877356ns | -0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 870988ns | 867126ns | 876941ns | base | 2.107 |
| wide-rung-ragged | 866980ns | 863889ns | 871732ns | -0.46% | 2.117 |
| wide-rung-ragged-overread | 869245ns | 867062ns | 872206ns | -0.20% | 2.111 |
| wide-rung-wordround | 870002ns | 862542ns | 874839ns | -0.11% | 2.109 |
| wide-rung-wordround-alias | 868283ns | 864439ns | 876251ns | -0.31% | 2.113 |

## Performance model

- Peak throughput: **2.127 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.108 | 99.1% |
| wide-rung-ragged | 2.119 | 99.6% |
| wide-rung-ragged-overread | 2.111 | 99.2% |
| wide-rung-wordround | 2.106 | 99.0% |
| wide-rung-wordround-alias | 2.117 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 871885ns | 871885ns | base |
| wide-rung-ragged | 867959ns | 867959ns | -0.45% |
| wide-rung-ragged-overread | 870198ns | 870198ns | -0.19% |
| wide-rung-wordround | 870918ns | 870918ns | -0.11% |
| wide-rung-wordround-alias | 869316ns | 869316ns | -0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 870352ns | base | --- | [868996, 871383] | --- | --- | --- | --- |
| wide-rung-ragged | 866121ns | -3468.8ns (-0.4%) | [-4820, -2738]ns | [865008, 867795] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 869084ns | -970.4ns (-0.1%) | [-2336, -28]ns | [868564, 869368] | YES (adj: no) | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround | 871333ns | no significant difference | [-1840, +1820]ns | [870034, 872195] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround-alias | 866735ns | -3778.4ns (-0.4%) | [-5506, -829]ns | [866243, 867410] | YES | 0.0129 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 871592ns | -0.8% | -0.3% | -1.0% | -0.5% |
| 2 | 872231ns | -0.8% | -0.4% | -1.2% | +0.7% |
| 3 | 873875ns | -1.0% | -0.4% | -1.5% | -0.9% |
| 4 | 872681ns | +1.0% | -0.4% | -1.1% | -0.7% |
| 5 | 874239ns | -0.9% | -0.6% | +0.4% | -0.8% |
| 6 | 868192ns | -0.4% | +0.1% | -0.2% | +0.1% |
| 7 | 870246ns | -0.7% | +0.0% | -0.8% | +2.0% |
| 8 | 870459ns | -0.7% | -0.2% | -0.7% | -0.4% |
| 9 | 869158ns | -0.4% | +0.2% | -0.9% | -0.5% |
| 10 | 870909ns | -0.6% | -0.3% | -1.0% | -0.9% |
| 11 | 871445ns | -0.2% | -0.3% | +0.1% | -0.8% |
| 12 | 872360ns | -0.3% | -0.3% | +0.0% | -0.9% |
| 13 | 871086ns | -0.1% | -0.3% | +0.0% | -0.8% |
| 14 | 870161ns | +0.1% | -0.1% | +0.3% | +0.0% |
| 15 | 871957ns | -0.4% | +0.6% | -0.0% | -0.7% |
| 16 | 868807ns | -0.1% | +0.5% | +0.3% | -0.3% |
| 17 | 873280ns | -0.4% | -0.4% | -0.1% | -0.8% |
| 18 | 871967ns | -0.3% | -0.3% | +0.0% | -0.6% |
| 19 | 869014ns | +0.0% | +0.0% | +0.2% | -0.2% |
| 20 | 868118ns | +0.1% | -0.0% | +0.5% | +0.2% |
| 21 | 869815ns | -0.5% | -0.1% | +0.2% | +0.5% |
| 22 | 883383ns | -2.0% | -1.9% | -1.5% | -1.7% |
| 23 | 867042ns | -0.4% | +0.8% | +0.5% | -0.1% |
| 24 | 868941ns | -0.7% | -0.0% | +0.5% | -0.2% |
| 25 | 866879ns | -0.3% | +0.1% | +0.6% | -0.1% |
| 26 | 867802ns | -0.5% | -0.0% | +0.5% | +0.0% |
| 27 | 867450ns | -0.4% | -0.1% | +0.7% | +0.1% |
| 28 | 866788ns | -0.2% | +0.0% | +0.7% | +0.1% |
| 29 | 866761ns | -0.3% | +0.1% | +0.8% | -0.1% |
| 30 | 874519ns | -1.2% | -1.0% | -0.4% | -0.9% |
| 31 | 871321ns | -0.6% | -0.4% | -0.4% | -0.7% |
| 32 | 868978ns | +0.2% | +0.1% | +0.1% | -0.0% |
| 33 | 867397ns | -0.1% | +0.2% | +0.6% | -0.2% |
| 34 | 868608ns | -0.3% | +0.3% | +0.3% | -0.5% |
| 35 | 879550ns | -0.9% | -1.3% | -0.4% | -1.6% |
| 36 | 866891ns | +0.1% | +0.2% | +1.4% | +0.5% |
| 37 | 883999ns | -1.9% | -1.8% | -1.5% | -1.9% |
| 38 | 870927ns | -0.5% | -0.2% | -0.2% | +2.1% |
| 39 | 871103ns | -0.7% | -0.2% | -0.3% | -0.5% |
| 40 | 869582ns | -0.4% | -0.1% | -0.1% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.143 | ok |
| wide-rung-ragged | 0.247 | moderate+ |
| wide-rung-ragged-overread | 0.298 | moderate+ |
| wide-rung-wordround | 0.546 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.035 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 32/40, lost 5/40
- **wide-rung-ragged-overread**: won 20/40, lost 8/40
- **wide-rung-wordround**: won 17/40, lost 17/40
- **wide-rung-wordround-alias**: won 26/40, lost 8/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 7.7ns | 870987.8ns | 0.0% |  |
| wide-rung-ragged | 6.2ns | 866980.4ns | 0.0% |  |
| wide-rung-ragged-overread | 6.4ns | 869244.6ns | 0.0% |  |
| wide-rung-wordround | 7.0ns | 870001.6ns | 0.0% |  |
| wide-rung-wordround-alias | 8.0ns | 868283.5ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 867126.4-876940.7 ns)
  867126.4 |################
  867617.1 |########
  868107.8 |################
  868598.5 |########################################
  869089.2 |########
  869579.9 |################
  870070.6 |########################
  870561.4 |################
  871052.1 |################################
  871542.8 |########################
  872033.5 |################
  872524.2 |########
  873014.9 |########
  873505.7 |########
  873996.4 |########
  874487.1 |########
  874977.8 |
  875468.5 |
  875959.2 |
  876450.0 |
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 863889.2-871731.6 ns)
  863889.2 |########################################
  864281.3 |################
  864673.4 |########################################
  865065.6 |########
  865457.7 |########################
  865849.8 |################
  866241.9 |########################
  866634.0 |########
  867026.1 |
  867418.3 |########
  867810.4 |################
  868202.5 |########
  868594.6 |
  868986.7 |########
  869378.9 |########################################
  869771.0 |########
  870163.1 |
  870555.2 |########
  870947.3 |########
  871339.4 |########
  (3 below, 1 above range)

wide-rung-ragged-overread (n=40, range 867062.2-872205.9 ns)
  867062.2 |
  867319.4 |#############
  867576.5 |######
  867833.7 |#############
  868090.9 |######
  868348.1 |#################################
  868605.3 |#############
  868862.5 |####################
  869119.7 |########################################
  869376.9 |#################################
  869634.1 |######
  869891.2 |######
  870148.4 |######
  870405.6 |######
  870662.8 |######
  870920.0 |
  871177.2 |
  871434.4 |######
  871691.6 |
  871948.7 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 862542.4-874839.3 ns)
  862542.4 |#############
  863157.2 |######
  863772.1 |######
  864386.9 |
  865001.8 |
  865616.6 |
  866231.5 |######
  866846.3 |
  867461.1 |
  868076.0 |#############
  868690.8 |######
  869305.7 |#############
  869920.5 |####################
  870535.4 |#############
  871150.2 |####################
  871765.0 |########################################
  872379.9 |########################################
  872994.7 |#############
  873609.6 |######
  874224.4 |
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 864439.0-876251.3 ns)
  864439.0 |###########
  865029.6 |###########
  865620.2 |##################################
  866210.8 |########################################
  866801.4 |############################
  867392.0 |#################
  867982.7 |#####
  868573.3 |###########
  869163.9 |#####
  869754.5 |###########
  870345.1 |
  870935.7 |#####
  871526.4 |
  872117.0 |
  872707.6 |
  873298.2 |
  873888.8 |#####
  874479.4 |
  875070.1 |
  875660.7 |
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-wordround**: autocorrelation=0.55 (measurement drift or warm-up artifact)
