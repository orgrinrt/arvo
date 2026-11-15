# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 98% (significant)

satfold-neon is -27.84 us (98%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 55.5x slower than the field

satfold-seq (28.75 us) is 55.5x the fastest (518 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes64 shows warm-up / thermal drift (autocorr +0.83)

satfold-lanes64's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-neon8} vs {satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} (269% apart)

The field splits into a fast tier {satfold-neon, satfold-neon8} and a slow tier {satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} with a 269% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 55.5x the fastest

Fastest satfold-neon (518 ns) to slowest satfold-seq (28.75 us): 55.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 517.7 ns median (-98.2% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 55.53x (fastest 517.7 ns, slowest 28745.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 28588ns | 28569ns | 27989ns | 28524ns | 29380ns | base |
| satfold-lanes16 | 4156ns | 4143ns | 4065ns | 4138ns | 4304ns | -85.46% |
| satfold-lanes16-constl | 4275ns | 4244ns | 4151ns | 4255ns | 4457ns | -85.05% |
| satfold-lanes4-idx | 2039ns | 2044ns | 2007ns | 2043ns | 2058ns | -92.87% |
| satfold-lanes64 | 7783ns | 7805ns | 7691ns | 7790ns | 7854ns | -72.78% |
| satfold-neon | 581ns | 579ns | 567ns | 580ns | 599ns | -97.97% |
| satfold-neon8 | 605ns | 597ns | 585ns | 598ns | 648ns | -97.88% |
| satfold-nolaw | 22192ns | 22059ns | 21668ns | 22124ns | 22922ns | -22.37% |
| satfold-seq | 28844ns | 28818ns | 28472ns | 28822ns | 29283ns | +0.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 28497ns | 27903ns | 29267ns | base | 1.150 |
| satfold-lanes16 | 4094ns | 4005ns | 4239ns | -85.63% | 8.004 |
| satfold-lanes16-constl | 4210ns | 4090ns | 4390ns | -85.23% | 7.783 |
| satfold-lanes4-idx | 1977ns | 1948ns | 1995ns | -93.06% | 16.571 |
| satfold-lanes64 | 7723ns | 7632ns | 7791ns | -72.90% | 4.243 |
| satfold-neon | 519ns | 507ns | 536ns | -98.18% | 63.117 |
| satfold-neon8 | 544ns | 525ns | 585ns | -98.09% | 60.275 |
| satfold-nolaw | 22112ns | 21602ns | 22835ns | -22.40% | 1.482 |
| satfold-seq | 28761ns | 28392ns | 29177ns | +0.93% | 1.139 |

## Performance model

- Peak throughput: **64.661 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.150 | 1.8% |
| satfold-lanes16 | 8.032 | 12.4% |
| satfold-lanes16-constl | 7.837 | 12.1% |
| satfold-lanes4-idx | 16.541 | 25.6% |
| satfold-lanes64 | 4.228 | 6.5% |
| satfold-neon | 63.295 | 97.9% |
| satfold-neon8 | 61.055 | 94.4% |
| satfold-nolaw | 1.491 | 2.3% |
| satfold-seq | 1.140 | 1.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 28588ns | 28588ns | base |
| satfold-lanes16 | 4156ns | 4156ns | -85.46% |
| satfold-lanes16-constl | 4275ns | 4275ns | -85.05% |
| satfold-lanes4-idx | 2039ns | 2039ns | -92.87% |
| satfold-lanes64 | 7783ns | 7783ns | -72.78% |
| satfold-neon | 581ns | 581ns | -97.97% |
| satfold-neon8 | 605ns | 605ns | -97.88% |
| satfold-nolaw | 22192ns | 22192ns | -22.37% |
| satfold-seq | 28844ns | 28844ns | +0.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 28495ns | base | --- | [28272, 28580] | --- | --- | --- | --- |
| satfold-lanes16 | 4080ns | -24321.0ns (-85.4%) | [-24475, -24226]ns | [4038, 4089] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 4181ns | -24282.5ns (-85.2%) | [-24397, -24021]ns | [4162, 4207] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1981ns | -26512.3ns (-93.0%) | [-26599, -26317]ns | [1978, 1987] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 7749ns | -20746.9ns (-72.8%) | [-20824, -20634]ns | [7734, 7751] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 518ns | -27983.3ns (-98.2%) | [-28070, -27737]ns | [515, 519] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 537ns | -27957.5ns (-98.1%) | [-28044, -27746]ns | [534, 538] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 21983ns | -6404.8ns (-22.5%) | [-6602, -6081]ns | [21891, 22077] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 28746ns | +201.2ns (+0.7%) | [+154, +454]ns | [28691, 28771] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 28561ns | -85.2% | -85.4% | -93.0% | -72.9% | -98.2% | -98.0% | -21.5% | +0.8% |
| 2 | 28565ns | -85.1% | -85.4% | -93.0% | -72.9% | -98.2% | -98.0% | -23.2% | +0.6% |
| 3 | 28475ns | -85.1% | -85.3% | -93.0% | -72.8% | -98.2% | -98.0% | -20.9% | +2.6% |
| 4 | 28945ns | -85.4% | -85.6% | -93.1% | -73.1% | -98.2% | -98.1% | -21.6% | -0.6% |
| 5 | 28704ns | -85.2% | -85.4% | -93.1% | -73.0% | -98.2% | -98.1% | -19.9% | +0.2% |
| 6 | 28632ns | -85.2% | -85.4% | -93.1% | -72.9% | -98.2% | -98.1% | -23.0% | +1.8% |
| 7 | 28487ns | -85.1% | -85.3% | -93.0% | -72.8% | -98.2% | -98.1% | -23.2% | +0.7% |
| 8 | 28452ns | -85.1% | -85.3% | -93.0% | -72.8% | -98.2% | -98.1% | -22.5% | +1.2% |
| 9 | 28636ns | -85.6% | -85.4% | -93.1% | -72.8% | -98.2% | -98.1% | -18.5% | +1.6% |
| 10 | 28545ns | -85.7% | -85.2% | -93.0% | -72.9% | -98.2% | -98.1% | -20.3% | +0.5% |
| 11 | 31011ns | -87.1% | -86.8% | -93.6% | -75.0% | -98.3% | -98.3% | -27.0% | -6.8% |
| 12 | 29062ns | -85.9% | -85.9% | -93.2% | -73.3% | -98.2% | -98.2% | -25.9% | +1.0% |
| 13 | 28074ns | -85.0% | -85.4% | -92.9% | -72.2% | -98.1% | -98.1% | -23.3% | +2.8% |
| 14 | 27835ns | -84.9% | -85.3% | -92.9% | -72.2% | -98.1% | -97.2% | -21.9% | +3.1% |
| 15 | 28638ns | -85.7% | -85.5% | -93.1% | -73.0% | -98.2% | -98.1% | -24.5% | +2.1% |
| 16 | 28810ns | -86.0% | -85.8% | -93.1% | -73.1% | -98.2% | -98.1% | -24.8% | -0.3% |
| 17 | 28101ns | -85.6% | -85.4% | -92.9% | -72.4% | -98.2% | -98.0% | -21.6% | +2.0% |
| 18 | 28045ns | -85.5% | -85.4% | -92.9% | -72.1% | -98.2% | -98.1% | -19.7% | +4.1% |
| 19 | 27963ns | -85.7% | -84.8% | -92.9% | -72.2% | -98.1% | -98.1% | -22.6% | +2.8% |
| 20 | 27482ns | -85.4% | -84.3% | -92.8% | -71.7% | -98.1% | -98.1% | -21.6% | +5.5% |
| 21 | 28588ns | -85.8% | -85.7% | -93.1% | -72.8% | -98.2% | -98.1% | -23.4% | +0.5% |
| 22 | 28650ns | -85.8% | -85.7% | -93.1% | -72.9% | -98.2% | -98.1% | -23.4% | +0.5% |
| 23 | 28569ns | -85.7% | -85.7% | -93.1% | -72.9% | -98.2% | -98.1% | -23.2% | +0.7% |
| 24 | 28588ns | -85.7% | -85.4% | -93.1% | -72.9% | -98.2% | -98.1% | -23.4% | +0.5% |
| 25 | 28502ns | -85.7% | -84.6% | -93.1% | -72.8% | -98.2% | -98.1% | -23.2% | +0.7% |
| 26 | 28468ns | -85.7% | -84.7% | -93.0% | -72.8% | -98.2% | -98.1% | -19.4% | +1.0% |
| 27 | 28622ns | -85.7% | -84.9% | -93.1% | -73.0% | -98.2% | -98.1% | -20.7% | +0.0% |
| 28 | 29297ns | -86.0% | -85.3% | -93.2% | -73.6% | -98.3% | -98.2% | -22.6% | -2.1% |
| 29 | 29657ns | -86.3% | -85.5% | -93.3% | -74.2% | -98.3% | -98.2% | -23.7% | -3.3% |
| 30 | 28572ns | -85.7% | -84.9% | -93.1% | -73.2% | -98.2% | -98.1% | -23.0% | +0.7% |
| 31 | 27702ns | -85.5% | -85.0% | -92.9% | -72.5% | -98.1% | -98.1% | -21.9% | +2.6% |
| 32 | 28145ns | -85.8% | -85.2% | -92.8% | -72.7% | -98.1% | -98.1% | -21.5% | +1.9% |
| 33 | 28185ns | -85.8% | -85.3% | -93.1% | -72.9% | -98.1% | -98.1% | -22.1% | +0.5% |
| 34 | 28020ns | -85.7% | -83.9% | -93.1% | -72.8% | -98.1% | -98.1% | -20.4% | +1.6% |
| 35 | 28273ns | -85.8% | -84.1% | -93.1% | -73.0% | -98.1% | -98.1% | -23.2% | +0.2% |
| 36 | 28130ns | -85.8% | -85.2% | -93.1% | -72.8% | -98.1% | -98.1% | -21.6% | +0.5% |
| 37 | 28123ns | -85.7% | -84.2% | -93.1% | -72.9% | -98.1% | -98.1% | -21.5% | +3.8% |
| 38 | 28292ns | -85.8% | -84.8% | -93.0% | -73.0% | -98.1% | -98.1% | -23.5% | -0.2% |
| 39 | 28272ns | -85.8% | -85.2% | -93.1% | -72.9% | -98.1% | -98.1% | -23.5% | +0.5% |
| 40 | 28188ns | -85.8% | -85.0% | -93.1% | -72.9% | -98.1% | -98.1% | -22.7% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.349 | moderate+ |
| satfold-lanes16 | 0.825 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.461 | moderate+ |
| satfold-lanes4-idx | 0.445 | moderate+ |
| satfold-lanes64 | 0.835 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.725 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.094 | ok |
| satfold-nolaw | 0.457 | moderate+ |
| satfold-seq | 0.090 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 6/40, lost 33/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.5ns | 28496.7ns | 0.0% |  |
| satfold-lanes16 | 2.5ns | 4094.0ns | 0.1% |  |
| satfold-lanes16-constl | 2.5ns | 4210.1ns | 0.1% |  |
| satfold-lanes4-idx | 3.0ns | 1977.4ns | 0.2% |  |
| satfold-lanes64 | 2.5ns | 7723.4ns | 0.0% |  |
| satfold-neon | 1.9ns | 519.2ns | 0.4% |  |
| satfold-neon8 | 1.8ns | 543.6ns | 0.3% |  |
| satfold-nolaw | 5.4ns | 22112.3ns | 0.0% |  |
| satfold-seq | 1.8ns | 28760.9ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 27902.8-29267.0 ns)
  27902.8 |#####
  27971.0 |#####
  28039.2 |#################
  28107.4 |#################
  28175.6 |###########
  28243.8 |#################
  28312.0 |
  28380.2 |
  28448.4 |############################
  28516.7 |############################
  28584.9 |########################################
  28653.1 |#####
  28721.3 |
  28789.5 |#####
  28857.7 |
  28925.9 |#####
  28994.1 |
  29062.3 |#####
  29130.6 |
  29198.8 |
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 4005.4-4239.2 ns)
   4005.4 |########################################
   4017.1 |
   4028.8 |###
   4040.5 |###
   4052.2 |###
   4063.9 |#######
   4075.5 |#########################
   4087.2 |##########
   4098.9 |###
   4110.6 |###
   4122.3 |
   4134.0 |
   4145.7 |
   4157.4 |
   4169.1 |
   4180.8 |###
   4192.4 |
   4204.1 |###
   4215.8 |
   4227.5 |##############
  (2 below, 4 above range)

satfold-lanes16-constl (n=40, range 4089.6-4390.5 ns)
   4089.6 |##################################
   4104.6 |
   4119.7 |
   4134.7 |
   4149.8 |######################
   4164.8 |######################
   4179.9 |########################################
   4194.9 |#####
   4209.9 |#####
   4225.0 |#####
   4240.0 |#####
   4255.1 |
   4270.1 |
   4285.2 |#####
   4300.2 |######################
   4315.3 |#####
   4330.3 |
   4345.4 |
   4360.4 |#####
   4375.5 |#####
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 1947.7-1994.7 ns)
   1947.7 |####
   1950.0 |
   1952.4 |
   1954.7 |
   1957.1 |
   1959.4 |
   1961.8 |
   1964.1 |
   1966.5 |####
   1968.8 |
   1971.2 |####
   1973.5 |#############
   1975.9 |#################
   1978.2 |#################
   1980.6 |########
   1982.9 |########
   1985.3 |#############
   1987.6 |########################################
   1990.0 |####
   1992.3 |
  (6 below, 3 above range)

satfold-lanes64 (n=40, range 7631.9-7791.3 ns)
   7631.9 |######
   7639.8 |######
   7647.8 |######
   7655.8 |
   7663.7 |###
   7671.7 |
   7679.7 |
   7687.7 |
   7695.6 |
   7703.6 |
   7711.6 |###
   7719.5 |
   7727.5 |###
   7735.5 |###
   7743.5 |########################################
   7751.4 |############
   7759.4 |
   7767.4 |######
   7775.4 |
   7783.3 |######
  (5 below, 4 above range)

satfold-neon (n=40, range 506.8-536.5 ns)
    506.8 |
    508.2 |#####
    509.7 |######################
    511.2 |#################
    512.7 |#####
    514.2 |###########
    515.7 |###########
    517.2 |########################################
    518.6 |#################
    520.1 |###########
    521.6 |#####
    523.1 |
    524.6 |###########
    526.1 |
    527.6 |
    529.0 |
    530.5 |
    532.0 |#####
    533.5 |
    535.0 |######################
  (4 below, 3 above range)

satfold-neon8 (n=40, range 525.1-584.6 ns)
    525.1 |######################
    528.1 |#############
    531.1 |########
    534.0 |###############################
    537.0 |########################################
    540.0 |
    542.9 |####
    545.9 |####
    548.9 |####
    551.9 |
    554.8 |#############
    557.8 |########
    560.8 |####
    563.8 |
    566.7 |
    569.7 |
    572.7 |
    575.6 |
    578.6 |
    581.6 |
  (4 below, 1 above range)

satfold-nolaw (n=40, range 21601.8-22835.3 ns)
  21601.8 |########################################
  21663.5 |######
  21725.2 |######
  21786.8 |######
  21848.5 |##########################
  21910.2 |##########################
  21971.9 |#############
  22033.5 |#################################
  22095.2 |
  22156.9 |
  22218.6 |
  22280.3 |######
  22341.9 |
  22403.6 |######
  22465.3 |
  22527.0 |#############
  22588.6 |#############
  22650.3 |####################
  22712.0 |######
  22773.7 |
  (3 below, 3 above range)

satfold-seq (n=40, range 28392.1-29177.3 ns)
  28392.1 |###########
  28431.4 |#####
  28470.6 |
  28509.9 |
  28549.2 |
  28588.4 |#####
  28627.7 |
  28666.9 |########################################
  28706.2 |############################
  28745.4 |########################################
  28784.7 |#################
  28824.0 |
  28863.2 |###########
  28902.5 |
  28941.7 |#####
  28981.0 |
  29020.3 |
  29059.5 |
  29098.8 |#####
  29138.0 |#####
  (4 below, 5 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.72 (measurement drift or warm-up artifact)
