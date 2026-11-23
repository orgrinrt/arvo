# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (1.13 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-pipe4 at 928.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-pipe4 dominates: 21% faster than the next best (bitpack-wide-d16-control)

bitpack-wide-pipe4 (928.68 us) leads bitpack-wide-d16-control (1.12 ms) by 21%, a clear separation rather than a photo finish. CV 8.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 beats baseline by 20% (significant)

bitpack-wide-pipe4 is -230.85 us (20%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-wide-d16-padal shows alternating (throttle bounce) (autocorr -0.70)

bitpack-wide-d16-padal's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 928677.7 ns median (-17.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 928677.7 ns, slowest 1129203.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112815569 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1177652ns | 1130358ns | 1114283ns | 1133607ns | 1373153ns | base |
| bitpack-wide-d16-control | 1163005ns | 1120274ns | 1077106ns | 1152015ns | 1281876ns | -1.24% |
| bitpack-wide-d16-padal | 1214223ns | 1128844ns | 1121484ns | 1189749ns | 1380383ns | +3.11% |
| bitpack-wide-pipe4 | 942162ns | 929809ns | 851973ns | 942448ns | 1031494ns | -20.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 1176504ns | 1113126ns | 1372018ns | base | 14.260 |
| bitpack-wide-d16-control | 1161865ns | 1075949ns | 1280722ns | -1.24% | 14.440 |
| bitpack-wide-d16-padal | 1213078ns | 1120338ns | 1379240ns | +3.11% | 13.830 |
| bitpack-wide-pipe4 | 940976ns | 850856ns | 1030294ns | -20.02% | 17.830 |

## Performance model

- Peak throughput: **19.718 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 14.858 | 75.4% |
| bitpack-wide-d16-control | 14.992 | 76.0% |
| bitpack-wide-d16-padal | 14.877 | 75.4% |
| bitpack-wide-pipe4 | 18.066 | 91.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 1177652ns | 1177652ns | base |
| bitpack-wide-d16-control | 1163005ns | 1163005ns | -1.24% |
| bitpack-wide-d16-padal | 1214223ns | 1214223ns | +3.11% |
| bitpack-wide-pipe4 | 942162ns | 942162ns | -20.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1129204ns | base | --- | [1125086, 1131022] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 1119087ns | no significant difference | [-13536, +55354]ns | [1116882, 1209127] | no | 0.1539 | 0.1539 | 0 |
| bitpack-wide-d16-padal | 1127760ns | no significant difference | [-2112, +12004]ns | [1125458, 1248089] | no | 0.1539 | 0.1539 | 0 |
| bitpack-wide-pipe4 | 928678ns | -230847.3ns (-20.4%) | [-241255, -191393]ns | [888525, 1003792] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 1113540ns | +9.0% | +1.0% | -20.4% |
| 1 | 2 | 1125535ns | +14.8% | +22.5% | -21.0% |
| 1 | 3 | 1360359ns | -20.8% | -17.0% | -24.6% |
| 1 | 4 | 1129606ns | -1.1% | +8.6% | -16.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.367 | moderate- |
| bitpack-wide-d16-control | -0.045 | ok |
| bitpack-wide-d16-padal | -0.702 | HIGH- (thermal bounce) |
| bitpack-wide-pipe4 | -0.062 | ok |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 2/4, lost 2/4
- **bitpack-wide-d16-padal**: won 1/4, lost 3/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 48.3ns | 1176504.5ns | 0.0% |  |
| bitpack-wide-d16-control | 40.5ns | 1161864.9ns | 0.0% |  |
| bitpack-wide-d16-padal | 45.0ns | 1213078.1ns | 0.0% |  |
| bitpack-wide-pipe4 | 79.7ns | 940975.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 1113126.1-1372017.7 ns)
  1113126.1 |########################################
  1126070.6 |########################################
  1139015.2 |###
  1151959.8 |######
  1164904.4 |
  1177849.0 |###
  1190793.6 |
  1203738.1 |
  1216682.7 |
  1229627.3 |
  1242571.9 |
  1255516.5 |
  1268461.1 |
  1281405.6 |
  1294350.2 |
  1307294.8 |
  1320239.4 |
  1333184.0 |
  1346128.5 |
  1359073.1 |#######################
  (4 below, 1 above range)

bitpack-wide-d16-control (n=40, range 1075948.9-1280721.6 ns)
  1075948.9 |####################
  1086187.5 |#####
  1096426.1 |
  1106664.8 |####################
  1116903.4 |########################################
  1127142.1 |#####
  1137380.7 |
  1147619.3 |
  1157858.0 |
  1168096.6 |
  1178335.2 |
  1188573.9 |
  1198812.5 |###############
  1209051.2 |##############################
  1219289.8 |#####
  1229528.4 |#####
  1239767.1 |
  1250005.7 |
  1260244.3 |
  1270483.0 |
  (5 below, 6 above range)

bitpack-wide-d16-padal (n=40, range 1120337.7-1379240.0 ns)
  1120337.7 |########################################
  1133282.8 |
  1146228.0 |
  1159173.1 |###
  1172118.2 |
  1185063.3 |
  1198008.4 |
  1210953.5 |
  1223898.6 |
  1236843.8 |
  1249788.9 |
  1262734.0 |
  1275679.1 |
  1288624.2 |
  1301569.3 |
  1314514.4 |#
  1327459.6 |
  1340404.7 |
  1353349.8 |#
  1366294.9 |#############
  (3 below, 5 above range)

bitpack-wide-pipe4 (n=40, range 850856.1-1030293.7 ns)
  850856.1 |
  859828.0 |
  868799.9 |
  877771.7 |########################################
  886743.6 |########################################
  895715.5 |
  904687.4 |
  913659.3 |
  922631.1 |##########
  931603.0 |##########
  940574.9 |
  949546.8 |
  958518.7 |
  967490.5 |
  976462.4 |#####
  985434.3 |
  994406.2 |##########
  1003378.1 |#####
  1012350.0 |###############
  1021321.8 |###################################
  (3 below, 3 above range)

```
