# The same arms over a 16 MiB column, past every cache level on this host: what the reassociation is worth once the load stream is the binding constraint

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon dominates: 396% faster than the next best (satfold-lanes4-idx)

satfold-neon (803.21 us) leads satfold-lanes4-idx (3.98 ms) by 396%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 85% (significant)

satfold-neon is -4.51 ms (85%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-lanes64 is an outlier: 6.6x slower than the field

satfold-lanes64 (5.32 ms) is 6.6x the fastest (803.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-constl shows warm-up / thermal drift (autocorr +0.69)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation +0.69, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon} vs {satfold-lanes4-idx, satfold-lanes16-constl, satfold-nolaw, satfold-seq, satfold-lanes16, satfold-iterfold, satfold-lanes64} (396% apart)

The field splits into a fast tier {satfold-neon} and a slow tier {satfold-lanes4-idx, satfold-lanes16-constl, satfold-nolaw, satfold-seq, satfold-lanes16, satfold-iterfold, satfold-lanes64} with a 396% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.6x the fastest

Fastest satfold-neon (803.21 us) to slowest satfold-lanes64 (5.32 ms): 6.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 803211.8 ns median (-84.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 6.62x (fastest 803211.8 ns, slowest 5319164.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 5326499ns | 5314609ns | 5283905ns | 5318910ns | 5391861ns | base |
| satfold-lanes16 | 5335710ns | 5313178ns | 5275364ns | 5322283ns | 5436334ns | +0.17% |
| satfold-lanes16-constl | 5310639ns | 5285550ns | 5212813ns | 5279069ns | 5503175ns | -0.30% |
| satfold-lanes4-idx | 4028009ns | 3982218ns | 3951737ns | 3985489ns | 4231841ns | -24.38% |
| satfold-lanes64 | 5324511ns | 5321050ns | 5287201ns | 5320040ns | 5375233ns | -0.04% |
| satfold-neon | 811596ns | 804373ns | 798199ns | 805168ns | 844280ns | -84.76% |
| satfold-nolaw | 5322628ns | 5304566ns | 5277770ns | 5313328ns | 5395385ns | -0.07% |
| satfold-seq | 5380292ns | 5309757ns | 5283894ns | 5307176ns | 5696039ns | +1.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5324865ns | 5282178ns | 5390221ns | base | 3.151 |
| satfold-lanes16 | 5334062ns | 5273443ns | 5434609ns | +0.17% | 3.145 |
| satfold-lanes16-constl | 5309031ns | 5211105ns | 5501572ns | -0.30% | 3.160 |
| satfold-lanes4-idx | 4026373ns | 3949844ns | 4230192ns | -24.39% | 4.167 |
| satfold-lanes64 | 5322693ns | 5285227ns | 5373665ns | -0.04% | 3.152 |
| satfold-neon | 810432ns | 796969ns | 843195ns | -84.78% | 20.702 |
| satfold-nolaw | 5320918ns | 5275764ns | 5394041ns | -0.07% | 3.153 |
| satfold-seq | 5378508ns | 5281977ns | 5694413ns | +1.01% | 3.119 |

## Performance model

- Peak throughput: **21.051 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 3.158 | 15.0% |
| satfold-lanes16 | 3.159 | 15.0% |
| satfold-lanes16-constl | 3.175 | 15.1% |
| satfold-lanes4-idx | 4.214 | 20.0% |
| satfold-lanes64 | 3.154 | 15.0% |
| satfold-neon | 20.888 | 99.2% |
| satfold-nolaw | 3.164 | 15.0% |
| satfold-seq | 3.161 | 15.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5326499ns | 5326499ns | base |
| satfold-lanes16 | 5335710ns | 5335710ns | +0.17% |
| satfold-lanes16-constl | 5310639ns | 5310639ns | -0.30% |
| satfold-lanes4-idx | 4028009ns | 4028009ns | -24.38% |
| satfold-lanes64 | 5324511ns | 5324511ns | -0.04% |
| satfold-neon | 811596ns | 811596ns | -84.76% |
| satfold-nolaw | 5322628ns | 5322628ns | -0.07% |
| satfold-seq | 5380292ns | 5380292ns | +1.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5312765ns | base | --- | [5295952, 5336906] | --- | --- | --- | --- |
| satfold-lanes16 | 5311479ns | no significant difference | [-14278, +8801]ns | [5292422, 5342059] | no | 0.5012 | 0.4296 | 0 |
| satfold-lanes16-constl | 5283894ns | -62335.2ns (-1.2%) | [-70805, -54101]ns | [5242692, 5301983] | YES | 0.0004 | 0.0002 | 0 |
| satfold-lanes4-idx | 3980918ns | -1329533.8ns (-25.0%) | [-1341565, -1323660]ns | [3966675, 3998838] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5319165ns | no significant difference | [-6335, +12148]ns | [5302020, 5333716] | no | 0.3755 | 0.2682 | 0 |
| satfold-neon | 803212ns | -4503458.6ns (-84.8%) | [-4523599, -4491418]ns | [800705, 807125] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 5302831ns | no significant difference | [-15264, +3815]ns | [5287472, 5339031] | no | 0.2693 | 0.1539 | 0 |
| satfold-seq | 5308087ns | no significant difference | [-15140, +6918]ns | [5292753, 5316825] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 5325571ns | +0.9% | -0.3% | -25.3% | +0.9% | -85.0% | -0.5% | +0.5% |
| 2 | 5356899ns | -1.0% | -2.3% | -25.8% | +0.6% | -84.9% | +2.3% | -0.7% |
| 3 | 5344107ns | +0.3% | -0.9% | -24.9% | -0.4% | -85.0% | -0.1% | +0.1% |
| 4 | 5373172ns | -1.1% | -2.3% | -26.2% | -0.2% | -85.2% | -0.9% | -1.2% |
| 5 | 5361364ns | +0.5% | -1.5% | -24.8% | -0.8% | -85.0% | -0.4% | +0.3% |
| 6 | 5320306ns | -0.0% | -1.2% | -24.5% | +1.1% | -84.3% | +0.6% | -0.0% |
| 7 | 5359107ns | -0.5% | -1.6% | -25.8% | -1.2% | -84.5% | -0.2% | +0.9% |
| 8 | 5336729ns | +0.0% | -1.0% | -24.9% | +0.7% | -84.9% | -0.6% | -0.3% |
| 9 | 5353800ns | -0.5% | -1.0% | -26.2% | -0.2% | -85.0% | +0.7% | +29.1% |
| 10 | 5337084ns | +0.4% | +0.5% | -24.7% | +0.1% | -84.6% | -0.8% | +0.5% |
| 11 | 5282231ns | +0.4% | -1.5% | -24.8% | +1.0% | -84.8% | +0.1% | +0.2% |
| 12 | 5297839ns | -0.3% | -1.6% | -25.1% | +0.7% | -84.9% | -0.3% | -0.3% |
| 13 | 5278226ns | -0.2% | -1.3% | -25.0% | +0.5% | -84.9% | +0.4% | +21.5% |
| 14 | 5297769ns | -0.3% | -1.2% | -25.4% | -0.0% | -84.1% | -0.3% | -0.1% |
| 15 | 5282401ns | +0.5% | -1.3% | -25.1% | +0.9% | -84.7% | +0.2% | +0.2% |
| 16 | 5279568ns | -0.1% | -1.1% | -25.2% | +0.2% | -84.8% | +0.2% | +0.2% |
| 17 | 5289434ns | -0.3% | -1.6% | -25.1% | +0.2% | -84.9% | +0.4% | -0.1% |
| 18 | 5294448ns | -0.3% | -1.1% | +3.0% | +0.8% | -84.9% | -0.1% | +0.4% |
| 19 | 5290688ns | -0.2% | -1.1% | -24.5% | -0.1% | -84.9% | -0.1% | +0.4% |
| 20 | 5305225ns | -0.6% | -1.7% | -25.6% | +2.2% | -85.0% | +0.5% | +0.5% |
| 21 | 5325988ns | +1.6% | -0.4% | -22.1% | +0.1% | -85.0% | +0.7% | -0.2% |
| 22 | 5408018ns | -0.6% | -1.3% | -24.8% | -2.2% | -85.2% | -1.0% | -0.4% |
| 23 | 5345427ns | +0.8% | -1.1% | -24.5% | -0.9% | -85.0% | +0.3% | -0.6% |
| 24 | 5374168ns | -1.2% | -0.8% | -25.1% | -1.7% | -85.1% | -0.0% | -1.3% |
| 25 | 5421115ns | -0.4% | -1.9% | -26.5% | -1.6% | -85.1% | -1.3% | -2.6% |
| 26 | 5331458ns | +0.2% | -0.6% | -24.7% | +0.0% | -84.7% | +0.8% | -0.4% |
| 27 | 5418603ns | -0.6% | -1.8% | -26.2% | -1.9% | -82.5% | -2.1% | -2.5% |
| 28 | 5364214ns | +0.6% | -1.2% | -25.8% | -1.1% | -84.3% | +0.4% | -0.8% |
| 29 | 5401111ns | -1.1% | -1.7% | -25.3% | -1.1% | -84.9% | -1.1% | -1.9% |
| 30 | 5331078ns | +1.5% | -1.2% | -25.1% | +0.2% | -84.7% | +1.8% | -0.8% |
| 31 | 5295942ns | +5.4% | -1.5% | -25.3% | -0.3% | -84.9% | -0.4% | -0.3% |
| 32 | 5280999ns | +4.2% | +2.0% | -25.3% | +0.1% | -84.7% | +0.1% | +0.1% |
| 33 | 5289048ns | +0.1% | -1.2% | -25.4% | -0.3% | -84.8% | -0.3% | -0.1% |
| 34 | 5295979ns | +0.3% | -1.5% | -25.4% | +0.2% | -84.7% | -0.2% | +0.1% |
| 35 | 5295963ns | -0.2% | +3.3% | -24.8% | +0.2% | -84.9% | -0.4% | -0.1% |
| 36 | 5282824ns | -0.2% | +8.8% | -25.0% | +0.2% | -84.8% | -0.2% | -0.0% |
| 37 | 5285761ns | -0.2% | +6.0% | -24.8% | +0.7% | -84.9% | -0.2% | +0.1% |
| 38 | 5305119ns | -0.6% | +6.9% | -23.8% | +0.6% | -85.0% | -0.4% | +0.2% |
| 39 | 5285414ns | +0.1% | +2.7% | -24.6% | +0.1% | -84.9% | -0.2% | +0.1% |
| 40 | 5290422ns | -0.3% | +0.0% | -25.1% | +0.2% | -84.9% | -0.1% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.604 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.537 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.688 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | -0.032 | ok |
| satfold-lanes64 | 0.094 | ok |
| satfold-neon | 0.294 | moderate+ |
| satfold-nolaw | 0.294 | moderate+ |
| satfold-seq | -0.038 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 22/40, lost 15/40
- **satfold-lanes16-constl**: won 32/40, lost 7/40
- **satfold-lanes4-idx**: won 39/40, lost 1/40
- **satfold-lanes64**: won 14/40, lost 20/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-nolaw**: won 20/40, lost 13/40
- **satfold-seq**: won 19/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 56.8ns | 5324865.4ns | 0.0% |  |
| satfold-lanes16 | 81.4ns | 5334061.7ns | 0.0% |  |
| satfold-lanes16-constl | 82.9ns | 5309030.8ns | 0.0% |  |
| satfold-lanes4-idx | 61.6ns | 4026373.0ns | 0.0% |  |
| satfold-lanes64 | 79.5ns | 5322692.5ns | 0.0% |  |
| satfold-neon | 11.4ns | 810432.1ns | 0.0% |  |
| satfold-nolaw | 58.9ns | 5320918.3ns | 0.0% |  |
| satfold-seq | 64.8ns | 5378508.2ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 5282177.9-5390220.6 ns)
  5282177.9 |#################################
  5287580.0 |##########################
  5292982.1 |########################################
  5298384.3 |
  5303786.4 |#############
  5309188.6 |
  5314590.7 |
  5319992.8 |######
  5325395.0 |#############
  5330797.1 |#############
  5336199.2 |#############
  5341601.4 |#############
  5347003.5 |
  5352405.7 |#############
  5357807.8 |#############
  5363209.9 |######
  5368612.1 |######
  5374014.2 |######
  5379416.3 |
  5384818.5 |
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 5273443.4-5434608.6 ns)
  5273443.4 |########################################
  5281501.6 |#################
  5289559.9 |###########
  5297618.2 |###########
  5305676.4 |######################
  5313734.7 |#####
  5321792.9 |#####
  5329851.2 |#####
  5337909.5 |#################
  5345967.7 |
  5354026.0 |###########
  5362084.2 |
  5370142.5 |###########
  5378200.8 |#####
  5386259.0 |###########
  5394317.3 |###########
  5402375.5 |
  5410433.8 |###########
  5418492.1 |
  5426550.3 |
  (3 below, 2 above range)

satfold-lanes16-constl (n=40, range 5211105.1-5501572.0 ns)
  5211105.1 |########################################
  5225628.4 |#################################
  5240151.8 |######
  5254675.1 |#############
  5269198.4 |####################
  5283721.8 |##########################
  5298245.1 |#################################
  5312768.5 |#############
  5327291.8 |#############
  5341815.2 |
  5356338.5 |######
  5370861.9 |
  5385385.2 |######
  5399908.5 |
  5414431.9 |######
  5428955.2 |
  5443478.6 |
  5458001.9 |######
  5472525.3 |
  5487048.6 |
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 3949844.0-4230192.4 ns)
  3949844.0 |##############################
  3963861.4 |########################################
  3977878.9 |#########################
  3991896.3 |###############
  4005913.7 |#########################
  4019931.1 |###############
  4033948.5 |##########
  4047966.0 |
  4061983.4 |#####
  4076000.8 |
  4090018.2 |
  4104035.6 |
  4118053.1 |
  4132070.5 |
  4146087.9 |#####
  4160105.3 |
  4174122.7 |
  4188140.1 |
  4202157.6 |
  4216175.0 |
  (5 below, 1 above range)

satfold-lanes64 (n=40, range 5285227.1-5373665.1 ns)
  5285227.1 |####################
  5289649.0 |##############################
  5294070.9 |####################
  5298492.8 |##############################
  5302914.7 |##############################
  5307336.6 |##########
  5311758.5 |##########
  5316180.4 |##########
  5320602.3 |####################
  5325024.2 |
  5329446.1 |########################################
  5333868.0 |########################################
  5338289.9 |##############################
  5342711.8 |##########
  5347133.7 |
  5351555.6 |
  5355977.5 |
  5360399.4 |##########
  5364821.3 |
  5369243.2 |##########
  (4 below, 4 above range)

satfold-neon (n=40, range 796969.1-843195.4 ns)
  796969.1 |########################################
  799280.4 |#########################
  801591.7 |##############################
  803903.1 |##########
  806214.4 |####################
  808525.7 |##########
  810837.0 |#####
  813148.3 |##########
  815459.6 |#####
  817770.9 |
  820082.2 |#####
  822393.5 |
  824704.9 |
  827016.2 |
  829327.5 |#####
  831638.8 |
  833950.1 |#####
  836261.4 |
  838572.7 |#####
  840884.0 |
  (3 below, 2 above range)

satfold-nolaw (n=40, range 5275764.2-5394041.4 ns)
  5275764.2 |###########
  5281678.1 |########################################
  5287592.0 |###########
  5293505.8 |#################
  5299419.7 |#################
  5305333.5 |
  5311247.4 |#####
  5317161.3 |#####
  5323075.1 |
  5328989.0 |#####
  5334902.8 |###########
  5340816.7 |#####
  5346730.5 |#################
  5352644.4 |#####
  5358558.3 |###########
  5364472.1 |
  5370386.0 |###########
  5376299.8 |
  5382213.7 |#####
  5388127.6 |#####
  (5 below, 2 above range)

satfold-seq (n=40, range 5281976.7-5694413.4 ns)
  5281976.7 |########################################
  5302598.5 |##################################
  5323220.4 |#####
  5343842.2 |########
  5364464.0 |#####
  5385085.9 |
  5405707.7 |##
  5426329.5 |
  5446951.4 |
  5467573.2 |
  5488195.0 |
  5508816.9 |
  5529438.7 |
  5550060.5 |
  5570682.4 |
  5591304.2 |
  5611926.0 |
  5632547.9 |
  5653169.7 |
  5673791.5 |
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.69 (measurement drift or warm-up artifact)
