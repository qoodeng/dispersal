# Seasonal food and system interactions

Actor: codex-dispersal-20260919-seasons. 19 September 2026. Synthetic mechanism experiment; no empirical calibration claim.

## Connected now

| Link | Implementation and evidence |
|---|---|
| Seasons → food | Exact integrated annual forcing enters the conserved patch ledger. Annual potential supply matches the constant control. |
| Food → decisions | The 14-day budget integrates the same upcoming renewal. No separate decorative season state. |
| Population → food | Living people determine consumption and shared harvest; competing residents deplete the same patches. |
| Movement → food | Travel consumes carried reserves and delays destination harvesting until arrival; departure removes local demand and lets patches recover. |
| Terrain → movement | Routes retain barriers, distance and terrain travel cost; those costs determine required provisions. |
| Food → population | **Missing:** shortages do not modify survival, fertility or splitting. |
| Experience → knowledge | **Missing:** no learned patch memory; current local stocks and the seasonal cycle are assumed known. |
| Climate/water → food | **Missing:** dated climate does not generate this synthetic food cycle; water access remains unmodeled. |

## Measured comparison

Same initial stocks, annual potential renewal, seed and demographic draws. Constant / seasonal food-driven runs:

| Supply | Seed | Food deficit constant / seasonal | Departures constant / seasonal |
|---|---|---|---|
| 500 | 1 | 47.05% / 72.51% | 52288 / 22511 |
| 500 | 42 | 39.26% / 62.56% | 54560 / 30507 |
| 500 | 123 | 40.37% / 62.60% | 54367 / 30370 |
| 2000 | 1 | 0.00% / 0.00% | 24926 / 29035 |
| 2000 | 42 | 0.00% / 0.00% | 22451 / 27972 |
| 2000 | 123 | 0.00% / 0.00% | 19226 / 23042 |
| 8000 | 1 | 3.19% / 0.00% | 469 / 1018 |
| 8000 | 42 | 0.03% / 0.00% | 152 / 700 |
| 8000 | 123 | 0.18% / 0.00% | 111 / 551 |

54 Rust tests pass, including annual supply equivalence, interval splitting across year boundaries, recovery after depletion, seasonal recovery/decision-budget interaction, stock caps and conservation. Seasonal smoke runs pass in all 34 supported dated environments; two Nile inputs correctly reject inadequate starting coverage. Nine seasonal paired scenarios pass replay, energy and population accounting, route barriers and paired demographic checks. Native/WASM match every field of 1201 snapshots for both policies in both seasonal and constant modes.

Central seed 123: constant versus seasonal median completed residence is 8.33 versus 5.96 days; travel is 310,761 versus 342,212 km. Zero food deficit in both. Halving the seasonal step retains zero deficit; departures change 23,042 to 23,476 and mean net displacement 45.36 to 46.00 km. This is sensitivity evidence, not full convergence.

Seasonal low supply produces severe unmet demand and can strand groups without travel provisions. Population remains equal despite these differences because scarcity response is absent. More varied movement is not acceptance of realistic migration.

UI provides Food seasons: constant control or seasonal renewal, plus snapshot renewal percentage and exported forcing settings. Browser checks confirm 180% at year start, 20% at half-year, and constant-control reruns.

Next: replace perfect local knowledge with observations and memory; research and evaluate scarcity response before connecting it to vital rates. Automatic fission, freshwater and regional/dated environmental integration remain open.
