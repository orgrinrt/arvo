# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (8.21 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 994 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 94% (significant)

warm-container-native is -7.68 us (94%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 8.3x slower than the field

warm-container-headroom (8.21 us) is 8.3x the fastest (994 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 8.9%)

warm-container-kernel wins on median (994 ns) yet has the highest variance (CV 8.9%), while warm-container-plusone is the steadiest (CV 6.0%, 8.19 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (994 ns) and warm-container-native (1000 ns) differ by 0.53%, inside the noise, even though the wider field spreads 725.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.90)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (718% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 718% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.3x the fastest

Fastest warm-container-kernel (994 ns) to slowest warm-container-headroom (8.21 us): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone's edge over baseline is significant but tiny (-24 ns, 0.29%)

warm-container-plusone differs from baseline warm-container-headroom by -24 ns (0.29%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 994.3 ns median (-87.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 8.26x (fastest 994.3 ns, slowest 8210.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8624ns | 8305ns | 8201ns | 8473ns | 9502ns | base |
| warm-container-kernel | 1107ns | 1056ns | 1045ns | 1074ns | 1270ns | -87.16% |
| warm-container-minimum | 1109ns | 1064ns | 1045ns | 1076ns | 1271ns | -87.15% |
| warm-container-native | 1097ns | 1060ns | 1044ns | 1057ns | 1267ns | -87.28% |
| warm-container-plusone | 8454ns | 8286ns | 8149ns | 8272ns | 9306ns | -1.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8526ns | 8108ns | 9396ns | base | 4.804 |
| warm-container-kernel | 1042ns | 983ns | 1195ns | -87.78% | 39.304 |
| warm-container-minimum | 1043ns | 984ns | 1194ns | -87.77% | 39.285 |
| warm-container-native | 1032ns | 983ns | 1192ns | -87.89% | 39.686 |
| warm-container-plusone | 8358ns | 8056ns | 9203ns | -1.97% | 4.901 |

## Performance model

- Peak throughput: **41.670 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.989 | 12.0% |
| warm-container-kernel | 41.193 | 98.9% |
| warm-container-minimum | 40.944 | 98.3% |
| warm-container-native | 40.976 | 98.3% |
| warm-container-plusone | 5.003 | 12.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8624ns | 8624ns | base |
| warm-container-kernel | 1107ns | 1107ns | -87.16% |
| warm-container-minimum | 1109ns | 1109ns | -87.15% |
| warm-container-native | 1097ns | 1097ns | -87.28% |
| warm-container-plusone | 8454ns | 8454ns | -1.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8211ns | base | --- | [8195, 8498] | --- | --- | --- | --- |
| warm-container-kernel | 994ns | -7205.0ns (-87.7%) | [-7318, -7190]ns | [987, 1005] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1000ns | -7202.5ns (-87.7%) | [-7513, -7191]ns | [987, 1005] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1000ns | -7205.2ns (-87.8%) | [-7326, -7195]ns | [986, 1001] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8187ns | no significant difference | [-400, +3]ns | [8140, 8211] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 8157ns | -87.9% | -87.9% | -87.7% | -1.3% |
| 2 | 8075ns | -87.8% | -87.8% | -87.6% | -0.3% |
| 3 | 8059ns | -87.8% | -87.8% | -87.6% | +0.0% |
| 4 | 8155ns | -87.9% | -87.9% | -87.8% | -1.2% |
| 5 | 8798ns | -88.8% | -88.8% | -88.8% | -8.5% |
| 6 | 9141ns | -89.3% | -89.2% | -89.2% | -10.9% |
| 7 | 9174ns | -89.3% | -89.3% | -89.3% | -11.9% |
| 8 | 9147ns | -89.2% | -89.3% | -89.2% | -10.4% |
| 9 | 9134ns | -89.3% | -89.1% | -89.2% | -10.5% |
| 10 | 9238ns | -89.3% | -89.3% | -89.3% | -11.1% |
| 11 | 9774ns | -87.8% | -89.9% | -87.7% | -16.8% |
| 12 | 9778ns | -87.8% | -89.8% | -87.8% | -16.4% |
| 13 | 9782ns | -87.8% | -89.9% | -87.8% | -16.0% |
| 14 | 8957ns | -86.7% | -89.0% | -86.7% | -8.4% |
| 15 | 8358ns | -85.7% | -88.2% | -85.7% | -2.2% |
| 16 | 8401ns | -85.8% | -88.3% | -85.8% | -2.2% |
| 17 | 8520ns | -86.0% | -88.4% | -86.0% | -5.4% |
| 18 | 8490ns | -85.9% | -88.4% | -86.1% | -4.3% |
| 19 | 8506ns | -85.9% | -88.4% | -88.4% | -5.1% |
| 20 | 8185ns | -85.4% | -88.0% | -88.0% | +1.4% |
| 21 | 8196ns | -87.7% | -87.7% | -87.8% | -0.1% |
| 22 | 8198ns | -87.8% | -87.8% | -87.8% | -0.1% |
| 23 | 8203ns | -87.7% | -87.7% | -87.8% | -0.0% |
| 24 | 8190ns | -87.7% | -87.8% | -87.8% | +0.1% |
| 25 | 8193ns | -87.8% | -87.8% | -87.8% | +0.1% |
| 26 | 8192ns | -87.8% | -87.8% | -87.8% | +2.6% |
| 27 | 8197ns | -87.7% | -87.7% | -87.8% | +0.5% |
| 28 | 8211ns | -87.8% | -87.7% | -87.8% | +0.0% |
| 29 | 8210ns | -87.7% | -87.8% | -87.8% | +3.4% |
| 30 | 8211ns | -87.8% | -87.7% | -87.7% | +5.2% |
| 31 | 8180ns | -88.0% | -85.4% | -88.0% | -1.5% |
| 32 | 8179ns | -87.9% | -85.4% | -88.0% | -1.1% |
| 33 | 8309ns | -88.2% | -85.6% | -88.1% | +12.9% |
| 34 | 8211ns | -88.0% | -85.5% | -88.0% | +19.1% |
| 35 | 8164ns | -87.9% | -85.4% | -88.0% | +19.7% |
| 36 | 8055ns | -87.7% | -85.2% | -87.8% | +21.2% |
| 37 | 8071ns | -87.8% | -85.2% | -87.8% | +16.5% |
| 38 | 8127ns | -87.9% | -85.3% | -87.9% | +0.6% |
| 39 | 8773ns | -88.8% | -86.4% | -88.8% | -7.5% |
| 40 | 9137ns | -89.2% | -86.9% | -89.2% | -11.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.852 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.865 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.898 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.837 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.781 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 23/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.6ns | 8525.9ns | 0.0% |  |
| warm-container-kernel | 2.8ns | 1042.1ns | 0.3% |  |
| warm-container-minimum | 2.9ns | 1042.6ns | 0.3% |  |
| warm-container-native | 2.9ns | 1032.1ns | 0.3% |  |
| warm-container-plusone | 2.3ns | 8358.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8107.8-9396.4 ns)
   8107.8 |###########
   8172.3 |########################################
   8236.7 |
   8301.1 |#####
   8365.5 |##
   8430.0 |##
   8494.4 |#####
   8558.8 |
   8623.2 |
   8687.7 |
   8752.1 |#####
   8816.5 |
   8880.9 |
   8945.4 |##
   9009.8 |
   9074.2 |#####
   9138.6 |########
   9203.1 |##
   9267.5 |
   9331.9 |
  (4 below, 3 above range)

warm-container-kernel (n=40, range 983.0-1195.0 ns)
    983.0 |########################################
    993.6 |###########
   1004.2 |###########
   1014.8 |
   1025.4 |
   1036.0 |
   1046.6 |
   1057.2 |
   1067.8 |
   1078.4 |
   1089.0 |
   1099.6 |
   1110.2 |
   1120.8 |
   1131.4 |
   1142.0 |
   1152.6 |
   1163.2 |
   1173.8 |
   1184.4 |###########
  (3 below, 5 above range)

warm-container-minimum (n=40, range 983.7-1193.9 ns)
    983.7 |########################################
    994.2 |########################
   1004.7 |########
   1015.2 |
   1025.8 |
   1036.3 |
   1046.8 |
   1057.3 |
   1067.8 |
   1078.3 |
   1088.8 |
   1099.3 |
   1109.8 |
   1120.4 |
   1130.9 |
   1141.4 |
   1151.9 |
   1162.4 |
   1172.9 |
   1183.4 |##################
  (3 below, 3 above range)

warm-container-native (n=40, range 983.4-1192.2 ns)
    983.4 |########################################
    993.8 |##################################
   1004.2 |##
   1014.7 |
   1025.1 |
   1035.6 |
   1046.0 |
   1056.5 |
   1066.9 |
   1077.3 |
   1087.8 |
   1098.2 |
   1108.7 |
   1119.1 |
   1129.5 |
   1140.0 |
   1150.4 |
   1160.9 |
   1171.3 |##
   1181.7 |##
  (3 below, 6 above range)

warm-container-plusone (n=40, range 8056.5-9203.4 ns)
   8056.5 |################
   8113.8 |##########
   8171.2 |########################################
   8228.5 |##
   8285.9 |##
   8343.2 |
   8400.6 |##
   8457.9 |##
   8515.2 |
   8572.6 |
   8629.9 |##
   8687.3 |
   8744.6 |
   8802.0 |
   8859.3 |
   8916.7 |
   8974.0 |
   9031.3 |
   9088.7 |
   9146.0 |
  (5 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.78 (measurement drift or warm-up artifact)
