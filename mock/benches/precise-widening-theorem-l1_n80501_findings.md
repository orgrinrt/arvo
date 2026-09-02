# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 101% (significant)

warm-container-kernel is -5.39 us (101%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 33.1x slower than the field

warm-container-plusone (5.47 us) is 33.1x the fastest (165 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 7.4%)

warm-container-kernel wins on median (165 ns) yet has the highest variance (CV 7.4%), while warm-container-minimum is the steadiest (CV 2.2%, 5.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.74)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.74, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-minimum, warm-container-headroom, warm-container-native, warm-container-plusone} (3055% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-minimum, warm-container-headroom, warm-container-native, warm-container-plusone} with a 3055% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 33.1x the fastest

Fastest warm-container-kernel (165 ns) to slowest warm-container-plusone (5.47 us): 33.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone's edge over baseline is significant but tiny (-42 ns, 0.79%)

warm-container-plusone differs from baseline warm-container-headroom by -42 ns (0.79%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 165.2 ns median (-96.9% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 33.13x (fastest 165.2 ns, slowest 5473.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5451ns | 5394ns | 5324ns | 5432ns | 5636ns | base |
| warm-container-kernel | 234ns | 228ns | 222ns | 230ns | 259ns | -95.70% |
| warm-container-lanes-deferred | 236ns | 233ns | 225ns | 235ns | 253ns | -95.67% |
| warm-container-minimum | 5423ns | 5399ns | 5299ns | 5407ns | 5597ns | -0.50% |
| warm-container-native | 5638ns | 5507ns | 5375ns | 5520ns | 6255ns | +3.43% |
| warm-container-plusone | 5589ns | 5539ns | 5376ns | 5518ns | 6012ns | +2.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5385ns | 5252ns | 5566ns | base | 3.043 |
| warm-container-kernel | 171ns | 162ns | 190ns | -96.83% | 95.926 |
| warm-container-lanes-deferred | 172ns | 164ns | 186ns | -96.80% | 95.126 |
| warm-container-minimum | 5356ns | 5234ns | 5530ns | -0.53% | 3.059 |
| warm-container-native | 5565ns | 5315ns | 6149ns | +3.35% | 2.944 |
| warm-container-plusone | 5520ns | 5310ns | 5938ns | +2.51% | 2.968 |

## Performance model

- Peak throughput: **101.284 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.072 | 3.0% |
| warm-container-kernel | 99.177 | 97.9% |
| warm-container-lanes-deferred | 97.090 | 95.9% |
| warm-container-minimum | 3.077 | 3.0% |
| warm-container-native | 3.012 | 3.0% |
| warm-container-plusone | 2.994 | 3.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5451ns | 5451ns | base |
| warm-container-kernel | 234ns | 234ns | -95.70% |
| warm-container-lanes-deferred | 236ns | 236ns | -95.67% |
| warm-container-minimum | 5423ns | 5423ns | -0.50% |
| warm-container-native | 5638ns | 5638ns | +3.43% |
| warm-container-plusone | 5589ns | 5589ns | +2.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5332ns | base | --- | [5321, 5411] | --- | --- | --- | --- |
| warm-container-kernel | 165ns | -5168.0ns (-96.9%) | [-5249, -5143]ns | [165, 166] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 169ns | -5163.8ns (-96.8%) | [-5246, -5149]ns | [166, 173] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5324ns | no significant difference | [-95, +79]ns | [5276, 5398] | no | 0.6358 | 0.6358 | 0 |
| warm-container-native | 5439ns | +96.7ns (+1.8%) | [+28, +164]ns | [5393, 5500] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-plusone | 5473ns | +125.5ns (+2.4%) | [+8, +186]ns | [5406, 5494] | YES (adj: no) | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5214ns | -96.4% | -96.8% | +2.8% | +4.4% | +0.8% |
| 2 | 5250ns | -96.5% | -96.9% | +1.7% | +3.8% | +3.8% |
| 3 | 5250ns | -96.5% | -96.8% | +0.8% | +1.9% | +1.2% |
| 4 | 5259ns | -96.5% | -96.9% | +1.1% | +7.7% | +1.3% |
| 5 | 5303ns | -96.5% | -96.8% | -1.2% | +1.5% | +2.9% |
| 6 | 5333ns | -96.5% | -96.8% | -1.4% | +3.0% | -0.2% |
| 7 | 5324ns | -96.6% | -96.9% | -1.5% | +1.8% | -0.2% |
| 8 | 5354ns | -96.6% | -96.9% | -2.3% | +0.8% | -0.6% |
| 9 | 5323ns | -96.6% | -97.0% | -1.5% | +1.9% | +2.5% |
| 10 | 5821ns | -96.8% | -97.2% | -10.1% | -3.9% | -8.5% |
| 11 | 5367ns | -96.9% | -96.9% | -1.3% | -0.3% | +1.7% |
| 12 | 5485ns | -97.1% | -97.0% | -4.8% | -2.0% | -0.4% |
| 13 | 5325ns | -96.9% | -96.9% | +1.3% | +1.3% | +3.1% |
| 14 | 5320ns | -96.9% | -96.9% | +1.7% | +0.9% | +3.0% |
| 15 | 5305ns | -96.9% | -96.9% | -0.1% | +1.6% | +0.4% |
| 16 | 5401ns | -97.0% | -97.0% | -1.9% | +0.2% | -0.1% |
| 17 | 5422ns | -97.0% | -97.0% | -3.0% | -1.7% | -0.1% |
| 18 | 5422ns | -97.0% | -96.9% | -3.0% | -1.9% | -1.3% |
| 19 | 5572ns | -97.1% | -97.0% | -4.3% | -4.6% | -4.4% |
| 20 | 5504ns | -95.9% | -97.0% | -5.1% | -3.5% | -3.2% |
| 21 | 5218ns | -96.8% | -96.4% | +4.0% | +13.5% | +7.4% |
| 22 | 5220ns | -96.8% | -96.4% | +2.4% | +25.3% | +13.5% |
| 23 | 5385ns | -96.9% | -96.6% | -0.8% | +3.6% | +10.2% |
| 24 | 5493ns | -97.0% | -96.6% | -1.6% | +0.1% | +7.9% |
| 25 | 5492ns | -97.0% | -96.6% | -4.7% | +2.5% | +7.9% |
| 26 | 5492ns | -97.0% | -96.7% | -4.3% | +15.2% | +15.9% |
| 27 | 5492ns | -97.0% | -96.7% | -4.3% | +18.2% | +3.9% |
| 28 | 5492ns | -97.0% | -96.6% | -3.7% | +9.6% | +6.7% |
| 29 | 5555ns | -97.0% | -96.7% | -5.2% | +7.5% | +5.6% |
| 30 | 5594ns | -97.1% | -96.7% | -2.6% | +11.7% | -1.6% |
| 31 | 5332ns | -96.9% | -96.8% | +3.1% | +3.2% | -0.5% |
| 32 | 5332ns | -96.9% | -96.5% | +3.2% | +3.2% | -0.0% |
| 33 | 5309ns | -96.9% | -96.7% | +3.6% | +3.6% | +3.8% |
| 34 | 5332ns | -96.9% | -96.8% | +3.2% | +2.0% | +3.1% |
| 35 | 5306ns | -96.9% | -96.7% | +3.7% | +0.0% | +3.6% |
| 36 | 5303ns | -96.9% | -96.7% | +3.8% | +0.1% | +3.6% |
| 37 | 5308ns | -96.9% | -96.8% | +3.8% | +0.1% | +3.5% |
| 38 | 5307ns | -96.9% | -96.8% | +7.8% | +0.3% | +3.5% |
| 39 | 5373ns | -97.0% | -96.8% | +2.3% | +2.3% | +2.2% |
| 40 | 5498ns | -97.0% | -96.9% | +0.0% | +0.1% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.342 | moderate+ |
| warm-container-kernel | 0.343 | moderate+ |
| warm-container-lanes-deferred | 0.741 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.710 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.599 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.713 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 22/40, lost 17/40
- **warm-container-native**: won 7/40, lost 28/40
- **warm-container-plusone**: won 10/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.9ns | 5384.7ns | 0.1% |  |
| warm-container-kernel | 2.7ns | 170.8ns | 1.6% |  |
| warm-container-lanes-deferred | 2.8ns | 172.2ns | 1.6% |  |
| warm-container-minimum | 3.3ns | 5356.3ns | 0.1% |  |
| warm-container-native | 3.3ns | 5564.8ns | 0.1% |  |
| warm-container-plusone | 3.0ns | 5519.9ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5252.1-5566.2 ns)
   5252.1 |#####
   5267.8 |
   5283.5 |
   5299.2 |########################################
   5314.9 |######################
   5330.6 |######################
   5346.3 |#####
   5362.0 |###########
   5377.7 |#####
   5393.5 |#####
   5409.2 |###########
   5424.9 |
   5440.6 |
   5456.3 |
   5472.0 |#####
   5487.7 |##################################
   5503.4 |#####
   5519.1 |
   5534.8 |
   5550.5 |#####
  (5 below, 3 above range)

warm-container-kernel (n=40, range 161.8-190.4 ns)
    161.8 |#################
    163.2 |###################################
    164.6 |########################################
    166.1 |#################
    167.5 |
    168.9 |
    170.4 |
    171.8 |
    173.2 |
    174.6 |
    176.1 |
    177.5 |
    178.9 |
    180.4 |####
    181.8 |########
    183.2 |####
    184.7 |#############
    186.1 |########
    187.5 |####
    189.0 |
  (4 below, 1 above range)

warm-container-lanes-deferred (n=40, range 163.8-186.1 ns)
    163.8 |#################
    164.9 |########################################
    166.0 |#################
    167.1 |###########
    168.2 |#################
    169.3 |###########
    170.5 |#####
    171.6 |###########
    172.7 |###########
    173.8 |#####
    174.9 |
    176.0 |
    177.2 |
    178.3 |
    179.4 |
    180.5 |
    181.6 |###########
    182.7 |###########
    183.9 |#####
    185.0 |#####
  (3 below, 5 above range)

warm-container-minimum (n=40, range 5233.8-5530.3 ns)
   5233.8 |#################################
   5248.6 |#################################
   5263.4 |######
   5278.3 |#############
   5293.1 |####################
   5307.9 |######
   5322.7 |######
   5337.6 |####################
   5352.4 |######
   5367.2 |
   5382.0 |######
   5396.9 |#############
   5411.7 |######
   5426.5 |
   5441.3 |######
   5456.2 |
   5471.0 |
   5485.8 |####################
   5500.6 |########################################
   5515.5 |
  (3 below, 1 above range)

warm-container-native (n=40, range 5315.3-6149.0 ns)
   5315.3 |############################
   5357.0 |##################################
   5398.7 |######################
   5440.4 |###########
   5482.0 |########################################
   5523.7 |
   5565.4 |###########
   5607.1 |#####
   5648.8 |#####
   5690.5 |
   5732.2 |
   5773.8 |
   5815.5 |
   5857.2 |
   5898.9 |#####
   5940.6 |#####
   5982.3 |#####
   6023.9 |
   6065.6 |
   6107.3 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 5309.8-5938.3 ns)
   5309.8 |########################################
   5341.2 |####
   5372.6 |####
   5404.1 |####
   5435.5 |####################
   5466.9 |####################################
   5498.3 |########
   5529.8 |
   5561.2 |
   5592.6 |####
   5624.0 |
   5655.5 |
   5686.9 |####
   5718.3 |
   5749.7 |
   5781.2 |
   5812.6 |
   5844.0 |########
   5875.4 |
   5906.9 |################
  (2 below, 1 above range)

```

## Diagnostics

- **warm-container-lanes-deferred**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.71 (measurement drift or warm-up artifact)
