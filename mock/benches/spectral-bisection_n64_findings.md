# Spectral bisection: Fiedler vector + sign-cut partition

1 variants, 40 samples per variant.
Baseline: **fiedler-bisect-dense**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 105510ns | 98225ns | 94507ns | 99983ns | 133092ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| fiedler-bisect-dense | 105344ns | 94390ns | 132864ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fiedler-bisect-dense | 105510ns | 105510ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 97995ns | base | --- | [97150, 101100] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fiedler-bisect-dense |
|---|---|
| 1 | 105578ns |
| 2 | 95842ns |
| 3 | 115154ns |
| 4 | 118631ns |
| 5 | 109336ns |
| 6 | 99179ns |
| 7 | 166555ns |
| 8 | 159228ns |
| 9 | 114167ns |
| 10 | 106794ns |
| 11 | 97119ns |
| 12 | 95546ns |
| 13 | 95580ns |
| 14 | 96648ns |
| 15 | 98023ns |
| 16 | 97181ns |
| 17 | 135445ns |
| 18 | 121473ns |
| 19 | 97347ns |
| 20 | 97325ns |
| 21 | 101971ns |
| 22 | 95945ns |
| 23 | 98628ns |
| 24 | 97463ns |
| 25 | 95621ns |
| 26 | 91010ns |
| 27 | 94960ns |
| 28 | 95075ns |
| 29 | 94992ns |
| 30 | 92950ns |
| 31 | 97967ns |
| 32 | 100229ns |
| 33 | 95170ns |
| 34 | 132260ns |
| 35 | 95417ns |
| 36 | 99185ns |
| 37 | 111250ns |
| 38 | 105882ns |
| 39 | 97476ns |
| 40 | 98175ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fiedler-bisect-dense | 0.417 | moderate+ |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fiedler-bisect-dense | 8.1ns | 105344.4ns | 0.0% |  |

## Distribution (algo ns)

```
fiedler-bisect-dense (n=40, range 94389.9-132864.1 ns)
  94389.9 |########################################
  96313.6 |########################################
  98237.3 |############
  100161.0 |########
  102084.7 |
  104008.4 |########
  105932.1 |####
  107855.9 |####
  109779.6 |####
  111703.3 |
  113627.0 |########
  115550.7 |
  117474.4 |####
  119398.1 |
  121321.8 |####
  123245.5 |
  125169.2 |
  127092.9 |
  129016.6 |
  130940.4 |####
  (2 below, 3 above range)

```
