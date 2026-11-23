# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed beats baseline by 22% (significant)

bitpack-contend-packed is -55.59 us (22%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 3.5x slower than the field

bitpack-contend-d64 (673.08 us) is 3.5x the fastest (195.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-packed is fastest but the noisiest (CV 8.1%)

bitpack-contend-packed wins on median (195.05 us) yet has the highest variance (CV 8.1%), while bitpack-contend-packed-simd is the steadiest (CV 1.6%, 200.10 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-packed shows alternating (throttle bounce) (autocorr -0.74)

bitpack-contend-packed's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} vs {bitpack-contend-d64} (115% apart)

The field splits into a fast tier {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 115% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.5x the fastest

Fastest bitpack-contend-packed (195.05 us) to slowest bitpack-contend-d64 (673.08 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader bitpack-contend-packed vs stability leader bitpack-contend-packed-simd (+3% speed for 5.2x steadier)

bitpack-contend-packed is fastest (195.05 us, CV 8.1%); bitpack-contend-packed-simd gives up 2.6% median for 5.2x lower variance (CV 1.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-packed** at 195050.4 ns median (-22.6% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.45x (fastest 195050.4 ns, slowest 673080.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 242213ns | 252162ns | 211824ns | 247227ns | 257561ns | base |
| bitpack-contend-d16-control | 271361ns | 272116ns | 253306ns | 271055ns | 290334ns | +12.03% |
| bitpack-contend-d32 | 310068ns | 314149ns | 280763ns | 312627ns | 331697ns | +28.01% |
| bitpack-contend-d64 | 664448ns | 674955ns | 639154ns | 665129ns | 687702ns | +174.32% |
| bitpack-contend-packed | 187271ns | 195238ns | 162739ns | 190833ns | 201117ns | -22.68% |
| bitpack-contend-packed-simd | 201779ns | 200286ns | 199641ns | 200847ns | 206710ns | -16.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 241949ns | 211541ns | 257254ns | base | 17.335 |
| bitpack-contend-d16-control | 271105ns | 253088ns | 290053ns | +12.05% | 15.471 |
| bitpack-contend-d32 | 309149ns | 279963ns | 330651ns | +27.77% | 13.567 |
| bitpack-contend-d64 | 662738ns | 637559ns | 685800ns | +173.92% | 6.329 |
| bitpack-contend-packed | 187061ns | 162551ns | 200917ns | -22.69% | 22.422 |
| bitpack-contend-packed-simd | 201579ns | 199445ns | 206493ns | -16.69% | 20.807 |

## Performance model

- Peak throughput: **25.803 Gops/s** (bitpack-contend-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 16.647 | 64.5% |
| bitpack-contend-d16-control | 15.428 | 59.8% |
| bitpack-contend-d32 | 13.391 | 51.9% |
| bitpack-contend-d64 | 6.232 | 24.2% |
| bitpack-contend-packed | 21.504 | 83.3% |
| bitpack-contend-packed-simd | 20.961 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 242213ns | 242213ns | base |
| bitpack-contend-d16-control | 271361ns | 271361ns | +12.03% |
| bitpack-contend-d32 | 310068ns | 310068ns | +28.01% |
| bitpack-contend-d64 | 664448ns | 664448ns | +174.32% |
| bitpack-contend-packed | 187271ns | 187271ns | -22.68% |
| bitpack-contend-packed-simd | 201779ns | 201779ns | -16.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 251952ns | base | --- | [242638, 253957] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 271860ns | +32802.8ns (+13.0%) | [+27660, +42218]ns | [256938, 283880] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32 | 313214ns | +61360.0ns (+24.4%) | [+57402, +69252]ns | [310308, 315250] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 673080ns | +421915.0ns (+167.5%) | [+395066, +431402]ns | [643153, 675425] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 195050ns | -55593.8ns (-22.1%) | [-59610, -52911]ns | [178371, 200328] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 200096ns | -48078.1ns (-19.1%) | [-51507, -42936]ns | [199709, 200298] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 255686ns | +11.0% | +18.2% | +162.1% | -21.6% | -19.7% |
| 1 | 2 | 242714ns | +18.6% | +29.2% | +173.4% | -33.0% | -17.7% |
| 1 | 3 | 254502ns | +0.5% | +22.3% | +151.6% | -21.5% | -21.3% |
| 1 | 4 | 211723ns | +20.7% | +54.5% | +218.9% | -13.9% | -5.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.278 | moderate- |
| bitpack-contend-d16-control | 0.211 | moderate+ |
| bitpack-contend-d32 | -0.094 | ok |
| bitpack-contend-d64 | -0.422 | moderate- |
| bitpack-contend-packed | -0.739 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.130 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 0/4, lost 4/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 38.5ns | 241949.0ns | 0.0% |  |
| bitpack-contend-d16-control | 53.7ns | 271105.1ns | 0.0% |  |
| bitpack-contend-d32 | 87.0ns | 309149.0ns | 0.0% |  |
| bitpack-contend-d64 | 225.1ns | 662737.8ns | 0.0% |  |
| bitpack-contend-packed | 34.9ns | 187060.6ns | 0.0% |  |
| bitpack-contend-packed-simd | 37.9ns | 201579.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 211541.3-257254.3 ns)
  211541.3 |#########################
  213826.9 |#####
  216112.6 |
  218398.2 |
  220683.9 |
  222969.5 |
  225255.2 |
  227540.8 |
  229826.5 |
  232112.1 |
  234397.8 |
  236683.4 |
  238969.1 |
  241254.7 |###################################
  243540.4 |
  245826.0 |
  248111.7 |
  250397.3 |##############################
  252683.0 |########################################
  254968.6 |#########################
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 253087.7-290052.7 ns)
  253087.7 |########################################
  254935.9 |########################################
  256784.2 |################
  258632.4 |########
  260480.7 |################
  262328.9 |########
  264177.2 |########
  266025.4 |
  267873.7 |
  269721.9 |
  271570.2 |
  273418.4 |
  275266.7 |
  277114.9 |########
  278963.2 |################
  280811.4 |########
  282659.7 |################################
  284507.9 |################################
  286356.2 |################
  288204.4 |########################
  (3 below, 3 above range)

bitpack-contend-d32 (n=40, range 279963.2-330651.1 ns)
  279963.2 |
  282497.5 |
  285031.9 |
  287566.3 |
  290100.7 |
  292635.1 |####################
  295169.5 |##########
  297703.9 |
  300238.3 |#####
  302772.7 |#####
  305307.1 |
  307841.5 |###############
  310375.9 |####################
  312910.3 |########################################
  315444.7 |
  317979.1 |#########################
  320513.5 |#####
  323047.9 |#####
  325582.3 |##########
  328116.7 |
  (4 below, 4 above range)

bitpack-contend-d64 (n=40, range 637558.5-685799.9 ns)
  637558.5 |########################################
  639970.6 |######################
  642382.7 |
  644794.7 |#####
  647206.8 |
  649618.9 |
  652030.9 |
  654443.0 |
  656855.1 |#####
  659267.2 |
  661679.2 |###########
  664091.3 |#####
  666503.4 |
  668915.4 |
  671327.5 |###########
  673739.6 |##################################
  676151.7 |#################
  678563.7 |###########
  680975.8 |###########
  683387.9 |#################
  (3 below, 3 above range)

bitpack-contend-packed (n=40, range 162550.9-200916.8 ns)
  162550.9 |#####################
  164469.2 |
  166387.5 |
  168305.8 |
  170224.1 |
  172142.4 |
  174060.7 |
  175979.0 |
  177897.3 |#############
  179815.6 |
  181733.9 |
  183652.1 |
  185570.4 |##
  187488.7 |
  189407.0 |
  191325.3 |
  193243.6 |##########
  195161.9 |##
  197080.2 |##
  198998.5 |########################################
  (2 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 199445.1-206493.1 ns)
  199445.1 |########################################
  199797.5 |####################
  200149.9 |########
  200502.3 |##
  200854.7 |
  201207.1 |##
  201559.5 |
  201911.9 |
  202264.3 |
  202616.7 |
  202969.1 |
  203321.5 |
  203673.9 |
  204026.3 |
  204378.7 |
  204731.1 |##
  205083.5 |#################
  205435.9 |########
  205788.3 |
  206140.7 |
  (3 below, 1 above range)

```
