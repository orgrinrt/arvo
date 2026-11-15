# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (7.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 322 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native dominates: 286% faster than the next best (warm-container-kernel)

warm-container-native (322 ns) leads warm-container-kernel (1.24 us) by 286%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 96% (significant)

warm-container-native is -7.70 us (96%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 24.8x slower than the field

warm-container-headroom (7.98 us) is 24.8x the fastest (322 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.77)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.77, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (518% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 518% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 24.8x the fastest

Fastest warm-container-native (322 ns) to slowest warm-container-headroom (7.98 us): 24.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-headroom is inconsistent: worst-20% is 1.5x its best-20%

warm-container-headroom's best 20% of batches run at 7.83 us but its worst 20% at 12.00 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 321.6 ns median (-96.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 24.81x (fastest 321.6 ns, slowest 7981.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8858ns | 8046ns | 7894ns | 8097ns | 12101ns | base |
| warm-container-kernel | 1374ns | 1368ns | 1342ns | 1371ns | 1413ns | -84.49% |
| warm-container-lanes-deferred | 1442ns | 1415ns | 1344ns | 1422ns | 1600ns | -83.72% |
| warm-container-minimum | 8036ns | 7992ns | 7863ns | 7972ns | 8400ns | -9.28% |
| warm-container-native | 391ns | 386ns | 375ns | 386ns | 424ns | -95.58% |
| warm-container-plusone | 8095ns | 8002ns | 7863ns | 8034ns | 8510ns | -8.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8785ns | 7831ns | 12000ns | base | 1.865 |
| warm-container-kernel | 1246ns | 1216ns | 1282ns | -85.82% | 13.154 |
| warm-container-lanes-deferred | 1308ns | 1218ns | 1455ns | -85.11% | 12.528 |
| warm-container-minimum | 7970ns | 7803ns | 8330ns | -9.28% | 2.056 |
| warm-container-native | 329ns | 315ns | 361ns | -96.26% | 49.823 |
| warm-container-plusone | 8024ns | 7802ns | 8421ns | -8.67% | 2.042 |

## Performance model

- Peak throughput: **51.986 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.053 | 3.9% |
| warm-container-kernel | 13.204 | 25.4% |
| warm-container-lanes-deferred | 12.769 | 24.6% |
| warm-container-minimum | 2.067 | 4.0% |
| warm-container-native | 50.937 | 98.0% |
| warm-container-plusone | 2.066 | 4.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8858ns | 8858ns | base |
| warm-container-kernel | 1374ns | 1374ns | -84.49% |
| warm-container-lanes-deferred | 1442ns | 1442ns | -83.72% |
| warm-container-minimum | 8036ns | 8036ns | -9.28% |
| warm-container-native | 391ns | 391ns | -95.58% |
| warm-container-plusone | 8095ns | 8095ns | -8.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7981ns | base | --- | [7946, 8025] | --- | --- | --- | --- |
| warm-container-kernel | 1241ns | -6730.4ns (-84.3%) | [-6792, -6702]ns | [1226, 1244] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1283ns | -6732.7ns (-84.4%) | [-6772, -6636]ns | [1276, 1286] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 7927ns | no significant difference | [-96, +3]ns | [7837, 7950] | no | 0.1081 | 0.1081 | 1 |
| warm-container-native | 322ns | -7660.2ns (-96.0%) | [-7707, -7625]ns | [320, 323] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 7932ns | no significant difference | [-133, +11]ns | [7866, 7999] | no | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 7869ns | -84.5% | -83.7% | -0.0% | -95.8% | +4.3% |
| 2 | 8208ns | -85.1% | -84.4% | -4.8% | -95.9% | +0.3% |
| 3 | 8014ns | -84.7% | -84.0% | +0.4% | -95.8% | +2.5% |
| 4 | 14967ns | -91.9% | -91.4% | -47.9% | -97.7% | -45.1% |
| 5 | 20690ns | -94.0% | -93.8% | -62.3% | -98.4% | -61.5% |
| 6 | 15771ns | -91.9% | -91.8% | -50.4% | -97.9% | -49.5% |
| 7 | 7806ns | -83.6% | -83.5% | +0.2% | -95.7% | +2.6% |
| 8 | 7804ns | -83.6% | -83.5% | +0.6% | -95.7% | +1.7% |
| 9 | 7803ns | -84.2% | -83.6% | +0.2% | -95.7% | +5.5% |
| 10 | 9105ns | -86.4% | -85.9% | -14.0% | -96.4% | -10.9% |
| 11 | 7934ns | -84.6% | -82.6% | -0.7% | -96.0% | -0.4% |
| 12 | 8848ns | -86.3% | -84.4% | -10.5% | -96.4% | -10.3% |
| 13 | 8847ns | -86.2% | -84.4% | -11.8% | -96.3% | -10.5% |
| 14 | 8925ns | -86.3% | -84.5% | -12.6% | -96.4% | -11.3% |
| 15 | 8850ns | -86.2% | -84.4% | -11.8% | -96.4% | -11.8% |
| 16 | 8700ns | -86.0% | -85.3% | -10.3% | -96.3% | -10.3% |
| 17 | 8160ns | -85.1% | -85.1% | -4.4% | -96.1% | -4.4% |
| 18 | 7850ns | -84.5% | -84.5% | +8.5% | -95.9% | -0.4% |
| 19 | 7803ns | -84.4% | -84.4% | +14.6% | -95.1% | -0.0% |
| 20 | 8123ns | -85.0% | -85.0% | +9.0% | -96.1% | -3.5% |
| 21 | 7908ns | -83.8% | -81.3% | +2.9% | -96.0% | -1.1% |
| 22 | 7961ns | -83.9% | -81.4% | +2.3% | -96.1% | -1.4% |
| 23 | 7803ns | -83.6% | -81.1% | +1.6% | -95.9% | +0.5% |
| 24 | 8015ns | -84.0% | -79.1% | -1.1% | -96.1% | -1.6% |
| 25 | 7985ns | -84.0% | -84.3% | -0.5% | -96.0% | -2.3% |
| 26 | 8123ns | -84.2% | -84.3% | -2.2% | -96.1% | -1.7% |
| 27 | 7921ns | -83.8% | -83.9% | +0.7% | -93.8% | -1.5% |
| 28 | 7992ns | -84.0% | -84.5% | -0.5% | -96.0% | -2.0% |
| 29 | 7923ns | -83.8% | -83.8% | -1.5% | -96.0% | -1.5% |
| 30 | 7927ns | -83.8% | -83.8% | -1.3% | -96.0% | -1.6% |
| 31 | 7977ns | -84.5% | -84.7% | -0.2% | -96.0% | +7.4% |
| 32 | 7971ns | -84.4% | -84.7% | +0.3% | -96.0% | -0.4% |
| 33 | 8035ns | -84.5% | -84.4% | -0.9% | -96.0% | -1.3% |
| 34 | 7938ns | -84.3% | -84.6% | -0.1% | -96.0% | -0.1% |
| 35 | 7940ns | -84.4% | -84.6% | -0.2% | -95.9% | +5.2% |
| 36 | 7960ns | -84.4% | -84.7% | -0.1% | -96.0% | +11.2% |
| 37 | 8015ns | -84.5% | -84.4% | -1.1% | -96.0% | +8.6% |
| 38 | 7953ns | -84.4% | -83.2% | +0.4% | -95.9% | +3.2% |
| 39 | 8006ns | -84.5% | -82.7% | +0.1% | -96.0% | +2.6% |
| 40 | 7977ns | -84.4% | -82.7% | +0.0% | -96.0% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.651 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.769 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.528 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.708 | HIGH+ (drift/warm-up) |
| warm-container-native | -0.065 | ok |
| warm-container-plusone | 0.600 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 24/40, lost 13/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 25/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.8ns | 8785.2ns | 0.0% |  |
| warm-container-kernel | 2.1ns | 1245.5ns | 0.2% |  |
| warm-container-lanes-deferred | 2.4ns | 1307.8ns | 0.2% |  |
| warm-container-minimum | 2.3ns | 7969.7ns | 0.0% |  |
| warm-container-native | 2.4ns | 328.8ns | 0.7% |  |
| warm-container-plusone | 5.6ns | 8023.8ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7830.8-12000.3 ns)
   7830.8 |########################################
   8039.3 |#######
   8247.8 |
   8456.2 |
   8664.7 |#######
   8873.2 |#
   9081.7 |#
   9290.1 |
   9498.6 |
   9707.1 |
   9915.6 |
  10124.0 |
  10332.5 |
  10541.0 |
  10749.5 |
  10957.9 |
  11166.4 |
  11374.9 |
  11583.4 |
  11791.8 |
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1216.2-1281.7 ns)
   1216.2 |####################
   1219.5 |#########################
   1222.8 |
   1226.1 |
   1229.3 |#####
   1232.6 |
   1235.9 |###############
   1239.1 |##############################
   1242.4 |###############
   1245.7 |
   1249.0 |
   1252.2 |
   1255.5 |
   1258.8 |
   1262.1 |
   1265.3 |
   1268.6 |
   1271.9 |
   1275.2 |#####
   1278.4 |########################################
  (5 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1218.0-1455.1 ns)
   1218.0 |####################
   1229.9 |###
   1241.7 |######
   1253.6 |###
   1265.4 |##########
   1277.3 |########################################
   1289.1 |
   1301.0 |
   1312.9 |
   1324.7 |
   1336.6 |###
   1348.4 |
   1360.3 |
   1372.1 |#######################
   1384.0 |
   1395.8 |
   1407.7 |
   1419.5 |
   1431.4 |
   1443.3 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 7802.8-8329.8 ns)
   7802.8 |########################################
   7829.2 |####
   7855.5 |########
   7881.9 |
   7908.2 |########################
   7934.6 |####################
   7960.9 |################
   7987.3 |########
   8013.6 |
   8040.0 |####
   8066.3 |
   8092.7 |
   8119.0 |########
   8145.4 |
   8171.7 |
   8198.1 |
   8224.4 |
   8250.8 |
   8277.1 |
   8303.5 |
  (4 below, 3 above range)

warm-container-native (n=40, range 315.2-360.8 ns)
    315.2 |####################
    317.4 |####################
    319.7 |########################################
    322.0 |###################################
    324.3 |#####
    326.6 |
    328.9 |#####
    331.1 |###############
    333.4 |#########################
    335.7 |
    338.0 |
    340.3 |#####
    342.5 |
    344.8 |
    347.1 |
    349.4 |
    351.7 |
    354.0 |
    356.2 |
    358.5 |
  (4 below, 2 above range)

warm-container-plusone (n=40, range 7802.5-8421.0 ns)
   7802.5 |###################################
   7833.4 |###############
   7864.3 |#####
   7895.3 |###############
   7926.2 |##############################
   7957.1 |##########
   7988.1 |#####
   8019.0 |
   8049.9 |
   8080.8 |
   8111.8 |#####
   8142.7 |
   8173.6 |
   8204.5 |########################################
   8235.5 |
   8266.4 |
   8297.3 |
   8328.3 |#####
   8359.2 |
   8390.1 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=28.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.60 (measurement drift or warm-up artifact)
