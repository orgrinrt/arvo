# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon dominates: 280% faster than the next best (satfold-lanes4-idx)

satfold-neon (513 ns) leads satfold-lanes4-idx (1.95 us) by 280%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 98% (significant)

satfold-neon is -27.77 us (98%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 55.3x slower than the field

satfold-seq (28.37 us) is 55.3x the fastest (513 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-seq shows warm-up / thermal drift (autocorr +0.87)

satfold-seq's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon} vs {satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} (280% apart)

The field splits into a fast tier {satfold-neon} and a slow tier {satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} with a 280% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 55.3x the fastest

Fastest satfold-neon (513 ns) to slowest satfold-seq (28.37 us): 55.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 512.7 ns median (-98.2% vs baseline)
- 6 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 55.34x (fastest 512.7 ns, slowest 28373.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 28520ns | 28427ns | 27930ns | 28438ns | 29354ns | base |
| satfold-lanes16 | 4097ns | 4081ns | 4068ns | 4088ns | 4154ns | -85.63% |
| satfold-lanes16-constl | 4187ns | 4156ns | 4151ns | 4162ns | 4299ns | -85.32% |
| satfold-lanes4-idx | 2027ns | 2008ns | 2001ns | 2017ns | 2080ns | -92.89% |
| satfold-lanes64 | 7761ns | 7689ns | 7662ns | 7697ns | 8050ns | -72.79% |
| satfold-neon | 576ns | 572ns | 569ns | 574ns | 591ns | -97.98% |
| satfold-nolaw | 21811ns | 21692ns | 21575ns | 21745ns | 22245ns | -23.52% |
| satfold-seq | 29007ns | 28459ns | 28353ns | 28642ns | 30756ns | +1.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 28443ns | 27850ns | 29273ns | base | 1.152 |
| satfold-lanes16 | 4036ns | 4009ns | 4091ns | -85.81% | 8.120 |
| satfold-lanes16-constl | 4126ns | 4091ns | 4234ns | -85.49% | 7.942 |
| satfold-lanes4-idx | 1965ns | 1941ns | 2015ns | -93.09% | 16.673 |
| satfold-lanes64 | 7702ns | 7606ns | 7983ns | -72.92% | 4.255 |
| satfold-neon | 516ns | 510ns | 529ns | -98.19% | 63.528 |
| satfold-nolaw | 21741ns | 21511ns | 22164ns | -23.56% | 1.507 |
| satfold-seq | 28913ns | 28273ns | 30634ns | +1.65% | 1.133 |

## Performance model

- Peak throughput: **64.232 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.156 | 1.8% |
| satfold-lanes16 | 8.153 | 12.7% |
| satfold-lanes16-constl | 8.003 | 12.5% |
| satfold-lanes4-idx | 16.820 | 26.2% |
| satfold-lanes64 | 4.295 | 6.7% |
| satfold-neon | 63.913 | 99.5% |
| satfold-nolaw | 1.516 | 2.4% |
| satfold-seq | 1.155 | 1.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 28520ns | 28520ns | base |
| satfold-lanes16 | 4097ns | 4097ns | -85.63% |
| satfold-lanes16-constl | 4187ns | 4187ns | -85.32% |
| satfold-lanes4-idx | 2027ns | 2027ns | -92.89% |
| satfold-lanes64 | 7761ns | 7761ns | -72.79% |
| satfold-neon | 576ns | 576ns | -97.98% |
| satfold-nolaw | 21811ns | 21811ns | -23.52% |
| satfold-seq | 29007ns | 29007ns | +1.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 28344ns | base | --- | [28247, 28453] | --- | --- | --- | --- |
| satfold-lanes16 | 4019ns | -24316.2ns (-85.8%) | [-24388, -24218]ns | [4015, 4033] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 4095ns | -24228.8ns (-85.5%) | [-24295, -24130]ns | [4093, 4101] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1948ns | -26387.5ns (-93.1%) | [-26468, -26294]ns | [1945, 1975] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 7629ns | -20684.2ns (-73.0%) | [-20747, -20600]ns | [7627, 7650] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 513ns | -27827.8ns (-98.2%) | [-27941, -27725]ns | [512, 514] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 21620ns | -6706.0ns (-23.7%) | [-6807, -6539]ns | [21551, 21829] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 28374ns | +155.2ns (+0.5%) | [+80, +426]ns | [28342, 28603] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 28249ns | -85.8% | -85.5% | -93.1% | -73.1% | -98.2% | -22.0% | +0.3% |
| 2 | 27923ns | -85.6% | -85.3% | -93.0% | -72.8% | -98.1% | -20.9% | +1.4% |
| 3 | 28562ns | -85.9% | -85.7% | -93.2% | -73.4% | -98.2% | -23.1% | -0.9% |
| 4 | 29555ns | -86.4% | -86.2% | -93.4% | -74.2% | -98.2% | -26.7% | -4.0% |
| 5 | 28314ns | -85.8% | -85.5% | -93.1% | -73.0% | -98.2% | -23.9% | +0.3% |
| 6 | 28215ns | -85.6% | -85.5% | -93.1% | -73.1% | -98.1% | -23.8% | +1.4% |
| 7 | 28329ns | -85.8% | -85.6% | -93.1% | -73.2% | -98.2% | -23.7% | -0.2% |
| 8 | 29502ns | -86.4% | -86.1% | -93.4% | -74.2% | -98.2% | -27.1% | -4.0% |
| 9 | 28640ns | -85.7% | -85.7% | -93.2% | -73.3% | -98.1% | -24.7% | -1.4% |
| 10 | 29100ns | -86.2% | -85.9% | -93.3% | -73.9% | -98.2% | -26.0% | -2.8% |
| 11 | 28245ns | -85.8% | -85.5% | -93.1% | -72.5% | -98.2% | -23.8% | +0.4% |
| 12 | 28397ns | -85.9% | -85.6% | -93.1% | -73.1% | -98.2% | -23.8% | -0.1% |
| 13 | 28574ns | -86.0% | -85.7% | -93.2% | -72.4% | -98.2% | -24.4% | -0.8% |
| 14 | 28358ns | -85.8% | -85.6% | -93.1% | -73.1% | -98.2% | -24.1% | +0.1% |
| 15 | 28185ns | -85.8% | -85.5% | -93.1% | -72.6% | -98.2% | -23.1% | +0.6% |
| 16 | 28217ns | -85.8% | -85.5% | -93.1% | -72.8% | -98.2% | -23.8% | +0.4% |
| 17 | 28372ns | -85.9% | -85.6% | -93.0% | -73.1% | -98.2% | -23.2% | -0.3% |
| 18 | 27826ns | -85.5% | -85.3% | -92.7% | -72.5% | -98.2% | -20.8% | +1.5% |
| 19 | 27121ns | -85.1% | -84.9% | -92.4% | -71.9% | -98.1% | -19.4% | +5.0% |
| 20 | 27182ns | -85.1% | -84.9% | -92.7% | -71.3% | -98.1% | -19.0% | +6.7% |
| 21 | 28424ns | -85.9% | -85.6% | -93.2% | -66.7% | -98.2% | -20.4% | +0.2% |
| 22 | 28201ns | -85.8% | -85.5% | -93.1% | -73.0% | -98.2% | -21.8% | +0.5% |
| 23 | 28376ns | -85.9% | -85.6% | -93.2% | -73.1% | -98.2% | -22.9% | -0.3% |
| 24 | 28199ns | -85.8% | -85.5% | -93.1% | -73.0% | -98.2% | -23.5% | +0.4% |
| 25 | 28233ns | -85.7% | -85.5% | -93.1% | -73.0% | -98.2% | -22.2% | +0.3% |
| 26 | 28200ns | -85.6% | -85.5% | -93.1% | -72.9% | -98.2% | -21.4% | +1.3% |
| 27 | 28192ns | -85.5% | -85.5% | -93.1% | -72.9% | -98.2% | -22.5% | +3.2% |
| 28 | 28175ns | -85.5% | -85.5% | -92.8% | -72.9% | -98.2% | -23.7% | +1.5% |
| 29 | 28314ns | -85.6% | -85.5% | -93.1% | -73.1% | -98.2% | -23.9% | -0.2% |
| 30 | 28275ns | -85.6% | -85.5% | -93.1% | -72.8% | -98.2% | -23.8% | +1.2% |
| 31 | 28462ns | -85.6% | -84.6% | -92.9% | -72.8% | -98.2% | -24.3% | +4.1% |
| 32 | 28466ns | -85.4% | -85.2% | -93.0% | -72.8% | -98.2% | -24.3% | +4.2% |
| 33 | 28737ns | -86.1% | -85.4% | -93.1% | -73.2% | -98.2% | -25.0% | +5.6% |
| 34 | 28445ns | -85.7% | -85.4% | -93.0% | -73.2% | -98.2% | -24.3% | +7.7% |
| 35 | 29735ns | -86.4% | -86.0% | -93.4% | -74.3% | -98.3% | -27.7% | +7.3% |
| 36 | 29834ns | -86.6% | -86.0% | -93.3% | -74.4% | -98.3% | -27.7% | +7.6% |
| 37 | 28575ns | -85.9% | -85.5% | -93.1% | -73.3% | -98.2% | -24.6% | +5.4% |
| 38 | 28733ns | -86.0% | -85.4% | -93.1% | -73.2% | -98.2% | -22.5% | +6.1% |
| 39 | 28988ns | -86.1% | -85.2% | -93.2% | -73.5% | -98.2% | -24.5% | +2.9% |
| 40 | 28282ns | -85.8% | -85.0% | -93.0% | -73.1% | -98.2% | -23.0% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.448 | moderate+ |
| satfold-lanes16 | 0.407 | moderate+ |
| satfold-lanes16-constl | 0.533 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.513 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.031 | ok |
| satfold-neon | 0.807 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.560 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.868 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 11/40, lost 28/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 1.8ns | 28442.8ns | 0.0% |  |
| satfold-lanes16 | 2.4ns | 4035.7ns | 0.1% |  |
| satfold-lanes16-constl | 2.1ns | 4126.0ns | 0.1% |  |
| satfold-lanes4-idx | 2.8ns | 1965.4ns | 0.1% |  |
| satfold-lanes64 | 2.7ns | 7701.5ns | 0.0% |  |
| satfold-neon | 2.3ns | 515.8ns | 0.5% |  |
| satfold-nolaw | 2.0ns | 21740.8ns | 0.0% |  |
| satfold-seq | 2.6ns | 28913.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 27850.4-29273.0 ns)
  27850.4 |
  27921.5 |######
  27992.7 |
  28063.8 |
  28134.9 |########################################
  28206.1 |########################################
  28277.2 |##########################
  28348.3 |##########################
  28419.5 |##########################
  28490.6 |######
  28561.7 |#############
  28632.8 |######
  28704.0 |#############
  28775.1 |
  28846.2 |
  28917.4 |######
  28988.5 |
  29059.6 |######
  29130.8 |
  29201.9 |
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 4009.4-4091.2 ns)
   4009.4 |####################
   4013.5 |########################################
   4017.6 |####################
   4021.7 |#####
   4025.8 |###############
   4029.9 |
   4033.9 |#####
   4038.0 |
   4042.1 |#####
   4046.2 |#####
   4050.3 |##########
   4054.4 |#####
   4058.5 |
   4062.6 |
   4066.7 |
   4070.8 |###############
   4074.8 |
   4078.9 |##########
   4083.0 |
   4087.1 |
  (6 below, 3 above range)

satfold-lanes16-constl (n=40, range 4090.7-4234.1 ns)
   4090.7 |########################################
   4097.9 |############
   4105.1 |##
   4112.2 |
   4119.4 |
   4126.6 |
   4133.8 |
   4140.9 |
   4148.1 |
   4155.3 |####
   4162.4 |##
   4169.6 |##
   4176.8 |
   4183.9 |
   4191.1 |##
   4198.3 |####
   4205.4 |
   4212.6 |
   4219.8 |
   4226.9 |
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 1941.3-2014.6 ns)
   1941.3 |########################################
   1944.9 |################################
   1948.6 |########
   1952.3 |########
   1955.9 |
   1959.6 |
   1963.3 |
   1966.9 |
   1970.6 |
   1974.3 |############
   1977.9 |
   1981.6 |########
   1985.3 |############
   1988.9 |########
   1992.6 |
   1996.3 |
   2000.0 |
   2003.6 |####
   2007.3 |
   2011.0 |
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 7606.4-7983.4 ns)
   7606.4 |################
   7625.2 |########################################
   7644.1 |########
   7662.9 |#####
   7681.8 |#####
   7700.6 |##
   7719.5 |##
   7738.3 |#####
   7757.2 |
   7776.0 |#####
   7794.9 |
   7813.7 |
   7832.6 |
   7851.4 |
   7870.3 |
   7889.1 |##
   7908.0 |
   7926.8 |
   7945.7 |
   7964.5 |
  (4 below, 1 above range)

satfold-neon (n=40, range 510.1-528.5 ns)
    510.1 |#######
    511.1 |########################################
    512.0 |#########################
    512.9 |##########
    513.8 |###
    514.7 |
    515.7 |###
    516.6 |
    517.5 |
    518.4 |
    519.3 |###
    520.3 |#######
    521.2 |###
    522.1 |#######
    523.0 |#######
    523.9 |
    524.9 |
    525.8 |
    526.7 |
    527.6 |
  (4 below, 3 above range)

satfold-nolaw (n=40, range 21511.4-22163.5 ns)
  21511.4 |########################################
  21544.0 |##############################
  21576.6 |#####
  21609.2 |###############
  21641.8 |#####
  21674.4 |#####
  21707.1 |
  21739.7 |
  21772.3 |##########
  21804.9 |
  21837.5 |###############
  21870.1 |#####
  21902.7 |
  21935.3 |##########
  21967.9 |
  22000.5 |##########
  22033.1 |##########
  22065.7 |#####
  22098.3 |
  22130.9 |
  (4 below, 3 above range)

satfold-seq (n=40, range 28272.6-30634.5 ns)
  28272.6 |########################################
  28390.7 |####
  28508.8 |#########
  28626.9 |
  28745.0 |
  28863.1 |
  28981.2 |####
  29099.3 |
  29217.4 |##
  29335.5 |
  29453.5 |
  29571.6 |####
  29689.7 |
  29807.8 |##
  29925.9 |
  30044.0 |##
  30162.1 |
  30280.2 |##
  30398.3 |##
  30516.4 |##
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16-constl**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.87 (measurement drift or warm-up artifact)
