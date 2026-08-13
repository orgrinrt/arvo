# The same arms at the longest reduction length over a 16 MiB column: the row where the hand-written arm reaches the load-bandwidth ceiling on the small column, so it is the row where a working set past every cache level can cap it

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (21.43 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 346.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon dominates: 23% faster than the next best (satfold-lanes64)

satfold-neon (346.29 us) leads satfold-lanes64 (427.25 us) by 23%, a clear separation rather than a photo finish. CV 8.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 98% (significant)

satfold-neon is -21.09 ms (98%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 61.9x slower than the field

satfold-iterfold (21.43 ms) is 61.9x the fastest (346.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon is fastest but the noisiest (CV 8.4%)

satfold-neon wins on median (346.29 us) yet has the highest variance (CV 8.4%), while satfold-seq is the steadiest (CV 0.2%, 21.43 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes16 shows warm-up / thermal drift (autocorr +0.67)

satfold-lanes16's per-pass series has lag-1 autocorrelation +0.67, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16} vs {satfold-lanes4-idx, satfold-nolaw, satfold-seq, satfold-iterfold} (826% apart)

The field splits into a fast tier {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16} and a slow tier {satfold-lanes4-idx, satfold-nolaw, satfold-seq, satfold-iterfold} with a 826% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 61.9x the fastest

Fastest satfold-neon (346.29 us) to slowest satfold-iterfold (21.43 ms): 61.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 346286.0 ns median (-98.4% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 61.89x (fastest 346286.0 ns, slowest 21433080.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 21844910ns | 21435826ns | 21364811ns | 21435817ns | 23552287ns | base |
| satfold-lanes16 | 835225ns | 838426ns | 802954ns | 837823ns | 859703ns | -96.18% |
| satfold-lanes16-constl | 833373ns | 832317ns | 814606ns | 832970ns | 853350ns | -96.19% |
| satfold-lanes4-idx | 7872189ns | 7751022ns | 7729360ns | 7774396ns | 8308394ns | -63.96% |
| satfold-lanes64 | 425299ns | 428744ns | 385399ns | 428055ns | 456933ns | -98.05% |
| satfold-neon | 338225ns | 347619ns | 291354ns | 342985ns | 370818ns | -98.45% |
| satfold-nolaw | 16601354ns | 16444070ns | 16408827ns | 16442626ns | 17270065ns | -24.00% |
| satfold-seq | 21431458ns | 21430994ns | 21365300ns | 21432120ns | 21495632ns | -1.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 21842293ns | 21362457ns | 23549567ns | base | 0.768 |
| satfold-lanes16 | 833910ns | 801697ns | 858296ns | -96.18% | 20.119 |
| satfold-lanes16-constl | 832155ns | 813203ns | 852146ns | -96.19% | 20.161 |
| satfold-lanes4-idx | 7870037ns | 7727461ns | 8305927ns | -63.97% | 2.132 |
| satfold-lanes64 | 423975ns | 383774ns | 455697ns | -98.06% | 39.571 |
| satfold-neon | 336881ns | 290070ns | 369476ns | -98.46% | 49.802 |
| satfold-nolaw | 16598767ns | 16406063ns | 17267537ns | -24.01% | 1.011 |
| satfold-seq | 21428624ns | 21362174ns | 21492847ns | -1.89% | 0.783 |

## Performance model

- Peak throughput: **57.839 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 0.783 | 1.4% |
| satfold-lanes16 | 20.039 | 34.6% |
| satfold-lanes16-constl | 20.186 | 34.9% |
| satfold-lanes4-idx | 2.165 | 3.7% |
| satfold-lanes64 | 39.268 | 67.9% |
| satfold-neon | 48.449 | 83.8% |
| satfold-nolaw | 1.020 | 1.8% |
| satfold-seq | 0.783 | 1.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 21844910ns | 21844910ns | base |
| satfold-lanes16 | 835225ns | 835225ns | -96.18% |
| satfold-lanes16-constl | 833373ns | 833373ns | -96.19% |
| satfold-lanes4-idx | 7872189ns | 7872189ns | -63.96% |
| satfold-lanes64 | 425299ns | 425299ns | -98.05% |
| satfold-neon | 338225ns | 338225ns | -98.45% |
| satfold-nolaw | 16601354ns | 16601354ns | -24.00% |
| satfold-seq | 21431458ns | 21431458ns | -1.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 21433080ns | base | --- | [21417758, 21443361] | --- | --- | --- | --- |
| satfold-lanes16 | 837248ns | -20590801.2ns (-96.1%) | [-20604510, -20574892]ns | [835074, 842179] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 831128ns | -20599986.5ns (-96.1%) | [-20611153, -20580052]ns | [825602, 835917] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 7748741ns | -13687054.6ns (-63.9%) | [-13699922, -13632234]ns | [7742053, 7798885] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 427247ns | -21000909.8ns (-98.0%) | [-21013712, -20988946]ns | [418776, 435708] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 346286ns | -21084825.6ns (-98.4%) | [-21104752, -21074632]ns | [335263, 352992] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 16441629ns | -4994540.4ns (-23.3%) | [-5005908, -4971033]ns | [16430171, 16447922] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 21428094ns | no significant difference | [-18285, +14924]ns | [21420834, 21439520] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 21435609ns | -96.1% | -96.1% | -63.9% | -97.7% | -98.4% | -23.3% | +0.1% |
| 2 | 24238270ns | -96.4% | -96.4% | -68.1% | -98.2% | -98.6% | -32.1% | -11.6% |
| 3 | 22860723ns | -96.4% | -96.4% | -66.1% | -98.1% | -98.4% | -28.1% | -6.2% |
| 4 | 21443604ns | -96.1% | -96.1% | -63.9% | -98.0% | -98.4% | -23.3% | -0.1% |
| 5 | 21395307ns | -96.1% | -96.0% | -63.7% | -98.0% | -98.5% | -23.2% | +0.1% |
| 6 | 21415940ns | -96.0% | -96.1% | -63.8% | -97.9% | -98.4% | -23.2% | +0.3% |
| 7 | 21435210ns | -96.0% | -96.1% | -63.9% | -97.9% | -98.3% | -23.3% | +0.0% |
| 8 | 21443118ns | -96.1% | -96.0% | -63.9% | -97.9% | -98.3% | -23.1% | -0.1% |
| 9 | 21437025ns | -96.0% | -96.1% | -63.9% | -98.0% | -98.3% | -23.3% | -0.1% |
| 10 | 21433584ns | -96.1% | -96.2% | -63.9% | -97.9% | -98.5% | -23.3% | -0.0% |
| 11 | 21442642ns | -96.1% | -96.1% | -62.7% | -97.9% | -98.4% | -23.4% | -0.1% |
| 12 | 21418951ns | -96.0% | -96.0% | -63.9% | -98.0% | -98.5% | -23.3% | +0.4% |
| 13 | 21416565ns | -96.1% | -96.1% | -63.9% | -98.0% | -98.1% | -23.2% | +0.1% |
| 14 | 21414518ns | -96.1% | -96.1% | -63.6% | -98.0% | -98.3% | -23.2% | +0.2% |
| 15 | 22230520ns | -96.2% | -96.3% | -64.5% | -97.9% | -98.4% | -26.2% | -3.7% |
| 16 | 21370393ns | -96.1% | -96.1% | -63.8% | -98.1% | -98.3% | -23.1% | +0.4% |
| 17 | 21891732ns | -96.1% | -96.1% | -64.7% | -98.1% | -98.3% | -24.9% | -2.0% |
| 18 | 21797145ns | -96.2% | -96.2% | -64.4% | -98.1% | -98.4% | -24.5% | -1.7% |
| 19 | 21428360ns | -96.2% | -96.1% | -63.9% | -98.1% | -98.4% | -23.2% | +0.1% |
| 20 | 21404667ns | -96.1% | -96.1% | -63.8% | -98.1% | -98.4% | -9.1% | +0.9% |
| 21 | 21420349ns | -96.1% | -96.1% | -63.9% | -97.9% | -98.4% | -23.2% | +0.1% |
| 22 | 21479849ns | -96.1% | -96.1% | -64.0% | -98.0% | -98.4% | -23.5% | -0.3% |
| 23 | 21446122ns | -96.2% | -96.2% | -63.9% | -98.1% | -98.3% | -23.4% | +0.5% |
| 24 | 21414223ns | -96.1% | -96.1% | -63.7% | -97.9% | -98.4% | -23.3% | +0.0% |
| 25 | 21465435ns | -96.1% | -96.0% | -64.0% | -98.0% | -98.3% | -23.1% | -0.1% |
| 26 | 21432577ns | -96.1% | -96.2% | -56.8% | -98.0% | -98.6% | -23.4% | -0.0% |
| 27 | 21423559ns | -96.0% | -96.1% | -58.8% | -98.0% | -98.4% | -23.3% | +0.0% |
| 28 | 21477679ns | -96.1% | -96.2% | -60.2% | -98.1% | -98.3% | -23.4% | -0.1% |
| 29 | 21421189ns | -96.0% | -96.2% | -64.0% | -97.9% | -98.3% | -23.3% | +0.1% |
| 30 | 21449468ns | -96.1% | -96.1% | -63.3% | -98.0% | -98.4% | -23.4% | -0.1% |
| 31 | 21362187ns | -96.3% | -96.2% | -63.1% | -97.9% | -98.5% | -23.0% | +0.4% |
| 32 | 21348928ns | -96.3% | -96.2% | -63.4% | -98.1% | -98.7% | -23.2% | +0.2% |
| 33 | 21377930ns | -96.3% | -96.1% | -63.0% | -98.2% | -98.7% | -17.8% | -0.0% |
| 34 | 21383920ns | -96.3% | -96.2% | -63.7% | -98.3% | -98.6% | -18.4% | -0.2% |
| 35 | 21353854ns | -96.2% | -96.0% | -63.5% | -98.2% | -98.4% | -18.0% | +0.0% |
| 36 | 21359184ns | -96.2% | -96.1% | -62.2% | -98.2% | -98.4% | -22.0% | +0.1% |
| 37 | 21343258ns | -96.2% | -96.2% | -62.8% | -98.2% | -98.6% | -22.9% | +0.2% |
| 38 | 24757927ns | -96.7% | -96.6% | -68.4% | -98.5% | -98.8% | -33.8% | -13.7% |
| 39 | 27617192ns | -97.0% | -97.0% | -71.8% | -98.4% | -98.9% | -40.6% | -22.7% |
| 40 | 23003025ns | -96.5% | -96.5% | -66.0% | -98.1% | -98.8% | -28.3% | -7.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.498 | moderate+ |
| satfold-lanes16 | 0.673 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.115 | ok |
| satfold-lanes4-idx | 0.515 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.478 | moderate+ |
| satfold-neon | 0.529 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.129 | ok |
| satfold-seq | 0.351 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 13/40, lost 11/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 367.2ns | 21842293.5ns | 0.0% |  |
| satfold-lanes16 | 22.9ns | 833910.1ns | 0.0% |  |
| satfold-lanes16-constl | 31.3ns | 832154.9ns | 0.0% |  |
| satfold-lanes4-idx | 162.8ns | 7870036.9ns | 0.0% |  |
| satfold-lanes64 | 20.1ns | 423974.8ns | 0.0% |  |
| satfold-neon | 13.1ns | 336881.3ns | 0.0% |  |
| satfold-nolaw | 295.8ns | 16598766.6ns | 0.0% |  |
| satfold-seq | 348.7ns | 21428623.9ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 21362456.8-23549566.8 ns)
  21362456.8 |########################################
  21471812.3 |###
  21581167.8 |
  21690523.3 |#
  21799878.8 |#
  21909234.3 |
  22018589.8 |
  22127945.3 |#
  22237300.8 |
  22346656.3 |
  22456011.8 |
  22565367.3 |
  22674722.8 |
  22784078.3 |#
  22893433.8 |
  23002789.3 |#
  23112144.8 |
  23221500.3 |
  23330855.8 |
  23440211.3 |
  (5 below, 3 above range)

satfold-lanes16 (n=40, range 801697.3-858296.1 ns)
  801697.3 |###########
  804527.2 |###########
  807357.2 |#####
  810187.1 |
  813017.0 |
  815847.0 |#####
  818676.9 |
  821506.9 |#####
  824336.8 |#####
  827166.7 |
  829996.7 |
  832826.6 |###########
  835656.6 |########################################
  838486.5 |#################
  841316.4 |#################
  844146.4 |#################
  846976.3 |#################
  849806.3 |
  852636.2 |###########
  855466.1 |#####
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 813203.3-852146.4 ns)
  813203.3 |
  815150.5 |##########################
  817097.6 |
  819044.8 |######
  820991.9 |#############
  822939.1 |#############
  824886.2 |####################
  826833.4 |#############
  828780.6 |####################
  830727.7 |######
  832674.9 |######
  834622.0 |########################################
  836569.2 |
  838516.3 |
  840463.5 |#############
  842410.6 |######
  844357.8 |####################
  846304.9 |
  848252.1 |#############
  850199.2 |######
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 7727460.9-8305927.2 ns)
  7727460.9 |########################################
  7756384.2 |######
  7785307.5 |###########
  7814230.8 |##
  7843154.2 |
  7872077.5 |######
  7901000.8 |##
  7929924.1 |##
  7958847.4 |
  7987770.7 |##
  8016694.1 |
  8045617.4 |##
  8074540.7 |
  8103464.0 |
  8132387.3 |
  8161310.6 |
  8190234.0 |
  8219157.3 |
  8248080.6 |
  8277003.9 |
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 383774.2-455697.5 ns)
  383774.2 |########
  387370.3 |
  390966.5 |
  394562.7 |
  398158.8 |########
  401755.0 |########
  405351.2 |########
  408947.3 |################
  412543.5 |########
  416139.7 |################################
  419735.8 |########
  423332.0 |################
  426928.2 |################
  430524.3 |################
  434120.5 |########################################
  437716.7 |########################
  441312.8 |################
  444909.0 |################
  448505.2 |################
  452101.3 |########
  (5 below, 2 above range)

satfold-neon (n=40, range 290069.9-369475.8 ns)
  290069.9 |########
  294040.2 |################
  298010.5 |
  301980.8 |
  305951.1 |
  309921.4 |################################
  313891.7 |
  317862.0 |
  321832.3 |########
  325802.6 |
  329772.9 |########
  333743.2 |################################
  337713.5 |########
  341683.8 |########
  345654.1 |########################
  349624.4 |########################################
  353594.6 |################################
  357564.9 |################################
  361535.2 |########
  365505.5 |########
  (4 below, 3 above range)

satfold-nolaw (n=40, range 16406063.4-17267536.9 ns)
  16406063.4 |########################################
  16449137.0 |###########
  16492210.7 |#
  16535284.4 |
  16578358.1 |
  16621431.8 |
  16664505.4 |#
  16707579.1 |
  16750652.8 |
  16793726.5 |
  16836800.2 |
  16879873.8 |
  16922947.5 |
  16966021.2 |
  17009094.9 |
  17052168.6 |
  17095242.2 |
  17138315.9 |
  17181389.6 |
  17224463.3 |
  (3 below, 4 above range)

satfold-seq (n=40, range 21362174.3-21492847.2 ns)
  21362174.3 |
  21368707.9 |#############
  21375241.6 |#############
  21381775.2 |######
  21388308.9 |
  21394842.5 |
  21401376.2 |
  21407909.8 |######
  21414443.5 |##########################
  21420977.1 |########################################
  21427510.8 |####################
  21434044.4 |####################
  21440578.1 |#############
  21447111.7 |########################################
  21453645.4 |#############
  21460179.0 |
  21466712.7 |
  21473246.3 |######
  21479780.0 |
  21486313.6 |
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.53 (measurement drift or warm-up artifact)
