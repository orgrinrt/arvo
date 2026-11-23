# The same arms over a 16 MiB column, past every cache level on this host: what the reassociation is worth once the load stream is the binding constraint

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx dominates: 35% faster than the next best (satfold-lanes64)

satfold-lanes4-idx (5.33 ms) leads satfold-lanes64 (7.20 ms) by 35%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes4-idx beats baseline by 27% (significant)

satfold-lanes4-idx is -2.01 ms (27%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {satfold-lanes4-idx} vs {satfold-lanes64, satfold-neon, satfold-nolaw, satfold-seq, satfold-iterfold, satfold-lanes16-constl, satfold-lanes16} (35% apart)

The field splits into a fast tier {satfold-lanes4-idx} and a slow tier {satfold-lanes64, satfold-neon, satfold-nolaw, satfold-seq, satfold-iterfold, satfold-lanes16-constl, satfold-lanes16} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: satfold-lanes4-idx** at 5325243.0 ns median (-27.2% vs baseline)
- 5 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.44x (fastest 5325243.0 ns, slowest 7677558.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The same arms over a 16 MiB column, past every cache level on this host: what the reassociation is worth once the load stream is the binding constraint |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112552305 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 7333090ns | 7310899ns | 7282456ns | 7314694ns | 7438915ns | base |
| satfold-lanes16 | 7753881ns | 7678485ns | 7634554ns | 7713104ns | 7995543ns | +5.74% |
| satfold-lanes16-constl | 7554048ns | 7502751ns | 7460618ns | 7518356ns | 7754557ns | +3.01% |
| satfold-lanes4-idx | 5400417ns | 5326044ns | 5273574ns | 5353955ns | 5666644ns | -26.36% |
| satfold-lanes64 | 7216177ns | 7203450ns | 7182407ns | 7203205ns | 7288863ns | -1.59% |
| satfold-neon | 7253469ns | 7222500ns | 7202806ns | 7232192ns | 7367963ns | -1.09% |
| satfold-nolaw | 7347644ns | 7289831ns | 7268620ns | 7299262ns | 7571815ns | +0.20% |
| satfold-seq | 7368356ns | 7292156ns | 7270685ns | 7317614ns | 7618252ns | +0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 7332156ns | 7281518ns | 7437990ns | base | 2.288 |
| satfold-lanes16 | 7752998ns | 7633744ns | 7994640ns | +5.74% | 2.164 |
| satfold-lanes16-constl | 7553216ns | 7459680ns | 7753823ns | +3.01% | 2.221 |
| satfold-lanes4-idx | 5399556ns | 5272720ns | 5665756ns | -26.36% | 3.107 |
| satfold-lanes64 | 7215322ns | 7181558ns | 7287985ns | -1.59% | 2.325 |
| satfold-neon | 7252732ns | 7202095ns | 7367244ns | -1.08% | 2.313 |
| satfold-nolaw | 7346800ns | 7267619ns | 7571101ns | +0.20% | 2.284 |
| satfold-seq | 7367460ns | 7269789ns | 7617376ns | +0.48% | 2.277 |

## Performance model

- Peak throughput: **3.182 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.295 | 72.1% |
| satfold-lanes16 | 2.185 | 68.7% |
| satfold-lanes16-constl | 2.236 | 70.3% |
| satfold-lanes4-idx | 3.151 | 99.0% |
| satfold-lanes64 | 2.329 | 73.2% |
| satfold-neon | 2.323 | 73.0% |
| satfold-nolaw | 2.302 | 72.3% |
| satfold-seq | 2.301 | 72.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 7333090ns | 7333090ns | base |
| satfold-lanes16 | 7753881ns | 7753881ns | +5.74% |
| satfold-lanes16-constl | 7554048ns | 7554048ns | +3.01% |
| satfold-lanes4-idx | 5400417ns | 5400417ns | -26.36% |
| satfold-lanes64 | 7216177ns | 7216177ns | -1.59% |
| satfold-neon | 7253469ns | 7253469ns | -1.09% |
| satfold-nolaw | 7347644ns | 7347644ns | +0.20% |
| satfold-seq | 7368356ns | 7368356ns | +0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 7310043ns | base | --- | [7300640, 7323321] | --- | --- | --- | --- |
| satfold-lanes16 | 7677558ns | +365144.8ns (+5.0%) | [+340883, +398207]ns | [7657113, 7766270] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 7501902ns | +193290.4ns (+2.6%) | [+171600, +215900]ns | [7486031, 7515039] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 5325243ns | -2005709.6ns (-27.4%) | [-2020905, -1941924]ns | [5298116, 5407330] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 7202394ns | -104850.4ns (-1.4%) | [-123495, -99708]ns | [7198596, 7206505] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 7221663ns | -78260.6ns (-1.1%) | [-82553, -70202]ns | [7219094, 7240304] | YES | 0.0001 | 0.0000 | 0 |
| satfold-nolaw | 7289041ns | -13781.9ns (-0.2%) | [-29980, -1506]ns | [7276413, 7305940] | YES | 0.0385 | 0.0385 | 0 |
| satfold-seq | 7291162ns | -17125.3ns (-0.2%) | [-39606, -2491]ns | [7285463, 7325045] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7318781ns | +4.4% | +2.3% | -27.6% | -1.6% | -1.3% | -0.4% | -0.6% |
| 1 | 2 | 7300811ns | +4.9% | +2.9% | -27.8% | -1.2% | -1.1% | -0.4% | -0.2% |
| 1 | 3 | 7296792ns | +8.6% | +5.6% | -25.4% | -1.4% | -1.1% | -0.1% | +1.5% |
| 1 | 4 | 7359480ns | +5.2% | +1.4% | -25.6% | -2.2% | -1.2% | +1.3% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.200 | ok |
| satfold-lanes16 | -0.129 | ok |
| satfold-lanes16-constl | -0.440 | moderate- |
| satfold-lanes4-idx | 0.247 | moderate+ |
| satfold-lanes64 | -0.291 | moderate- |
| satfold-neon | -0.143 | ok |
| satfold-nolaw | -0.026 | ok |
| satfold-seq | -0.267 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 286.8ns | 7332156.0ns | 0.0% |  |
| satfold-lanes16 | 252.0ns | 7752997.9ns | 0.0% |  |
| satfold-lanes16-constl | 245.7ns | 7553215.6ns | 0.0% |  |
| satfold-lanes4-idx | 227.5ns | 5399556.3ns | 0.0% |  |
| satfold-lanes64 | 307.1ns | 7215321.8ns | 0.0% |  |
| satfold-neon | 251.2ns | 7252732.0ns | 0.0% |  |
| satfold-nolaw | 230.7ns | 7346799.7ns | 0.0% |  |
| satfold-seq | 303.0ns | 7367460.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 7281518.0-7437989.6 ns)
  7281518.0 |####################
  7289341.6 |########################################
  7297165.1 |#################################
  7304988.7 |#################################
  7312812.3 |####################
  7320635.9 |####################
  7328459.5 |#############
  7336283.1 |
  7344106.6 |
  7351930.2 |#############
  7359753.8 |####################
  7367577.4 |
  7375401.0 |
  7383224.6 |
  7391048.1 |#############
  7398871.7 |
  7406695.3 |
  7414518.9 |
  7422342.5 |######
  7430166.1 |
  (3 below, 2 above range)

satfold-lanes16 (n=40, range 7633743.8-7994640.5 ns)
  7633743.8 |########################
  7651788.6 |########################################
  7669833.4 |############
  7687878.3 |
  7705923.1 |####
  7723967.9 |####
  7742012.8 |########
  7760057.6 |
  7778102.5 |####
  7796147.3 |########
  7814192.1 |####
  7832237.0 |########
  7850281.8 |####
  7868326.6 |
  7886371.5 |####
  7904416.3 |
  7922461.2 |
  7940506.0 |####
  7958550.8 |########
  7976595.7 |
  (3 below, 3 above range)

satfold-lanes16-constl (n=40, range 7459679.7-7753823.1 ns)
  7459679.7 |##########################
  7474386.8 |########################################
  7489094.0 |########################################
  7503801.2 |########################################
  7518508.4 |#############
  7533215.5 |
  7547922.7 |
  7562629.9 |
  7577337.0 |
  7592044.2 |
  7606751.4 |
  7621458.6 |######
  7636165.7 |######
  7650872.9 |
  7665580.1 |
  7680287.2 |#############
  7694994.4 |######
  7709701.6 |#############
  7724408.7 |
  7739115.9 |
  (5 below, 4 above range)

satfold-lanes4-idx (n=40, range 5272719.5-5665756.5 ns)
  5272719.5 |################
  5292371.4 |########################################
  5312023.2 |####
  5331675.1 |####
  5351326.9 |############
  5370978.8 |####
  5390630.6 |########
  5410282.5 |####
  5429934.3 |########
  5449586.2 |####
  5469238.0 |
  5488889.9 |########
  5508541.7 |
  5528193.6 |####
  5547845.4 |
  5567497.2 |########
  5587149.1 |
  5606801.0 |
  5626452.8 |
  5646104.7 |
  (5 below, 4 above range)

satfold-lanes64 (n=40, range 7181558.5-7287984.9 ns)
  7181558.5 |#############
  7186879.8 |####
  7192201.1 |#################
  7197522.4 |###################################
  7202843.7 |########################################
  7208165.1 |########
  7213486.4 |########
  7218807.7 |####
  7224129.0 |####
  7229450.3 |
  7234771.7 |
  7240093.0 |
  7245414.3 |
  7250735.6 |
  7256056.9 |
  7261378.3 |
  7266699.6 |
  7272020.9 |
  7277342.2 |
  7282663.5 |####
  (4 below, 4 above range)

satfold-neon (n=40, range 7202095.1-7367244.1 ns)
  7202095.1 |##########
  7210352.6 |####################
  7218610.0 |########################################
  7226867.5 |###
  7235124.9 |######
  7243382.3 |##########
  7251639.8 |
  7259897.2 |
  7268154.7 |
  7276412.1 |###
  7284669.6 |
  7292927.0 |###
  7301184.5 |
  7309441.9 |###
  7317699.4 |
  7325956.8 |
  7334214.3 |###
  7342471.7 |
  7350729.2 |###
  7358986.6 |###
  (3 below, 4 above range)

satfold-nolaw (n=40, range 7267618.5-7571101.0 ns)
  7267618.5 |########################################
  7282792.6 |##########################
  7297966.8 |#############
  7313140.9 |######
  7328315.0 |###
  7343489.1 |###
  7358663.3 |
  7373837.4 |
  7389011.5 |###
  7404185.6 |######
  7419359.7 |
  7434533.9 |
  7449708.0 |
  7464882.1 |
  7480056.2 |###
  7495230.4 |
  7510404.5 |######
  7525578.6 |
  7540752.7 |
  7555926.8 |
  (3 below, 3 above range)

satfold-seq (n=40, range 7269789.0-7617376.1 ns)
  7269789.0 |########################################
  7287168.4 |################
  7304547.7 |#####
  7321927.1 |
  7339306.4 |##
  7356685.8 |
  7374065.1 |##
  7391444.5 |##
  7408823.8 |#####
  7426203.2 |#####
  7443582.5 |##
  7460961.9 |
  7478341.2 |##
  7495720.6 |
  7513100.0 |
  7530479.3 |
  7547858.7 |
  7565238.0 |##
  7582617.4 |
  7599996.7 |
  (3 below, 4 above range)

```
