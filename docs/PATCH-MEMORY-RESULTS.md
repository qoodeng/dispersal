# Patch memory: interaction checkpoint

19 September 2026. Actor: codex-dispersal-20260919-memory.

Groups now observe only their resident patch, recording remaining food and other residents. Memory combines projected seasonal recovery with observed competitor demand; confidence decays toward an explicit half-capacity prior over 90 days. Unvisited patches use that prior with no known competitors. Terrain, capacity and renewal potential remain known. These are synthetic information rules, not calibrated cognition.

Experience → memory → estimated food budget → destination choice → actual arrival food → updated observation now forms a feedback loop. Actual food still uses the shared conserved ledger; mistaken estimates cannot create provisions or bypass terrain. Perfect local information remains a selectable control. Incoming remote reservations are excluded in learned mode. Scheduled movement does not use patch knowledge for decisions.

## Measured outcomes

Seasonal supply, 100-year runs, identical initial conditions and demographic streams:

| Mean supply | Seed | Deficit, perfect / learned | Departures, perfect / learned |
|---|---|---|---|
| 500 | 1 | 72.51% / 92.76% | 22511 / 412 |
| 500 | 42 | 62.56% / 94.27% | 30507 / 290 |
| 500 | 123 | 62.60% / 91.89% | 30370 / 331 |
| 2000 | 1 | 0.00% / 0.00% | 29035 / 42193 |
| 2000 | 42 | 0.00% / 0.00% | 27972 / 41818 |
| 2000 | 123 | 0.00% / 0.00% | 23042 / 34140 |
| 8000 | 1 | 0.00% / 0.00% | 1018 / 955 |
| 8000 | 42 | 0.00% / 0.00% | 700 / 831 |
| 8000 | 123 | 0.00% / 0.00% | 551 / 531 |

At central supply, seed 123, learned knowledge increases travel from 342,212 to 468,197 km and reduces median completed residence from 5.96 to 2.61 days. Both have zero deficit. Low-supply learned runs have 92–94% unmet demand and very few later departures: groups can become stranded. Population continues unaffected by this shortage, making the missing scarcity response especially consequential. This is not evidence of realistic residence or survival.

Half-step central learned run: 34,140 → 34,947 departures, mean net displacement 59.41 → 44.30 km, zero deficit in both. Narrow deficit sensitivity passes; route convergence remains unestablished.

## Verification

57 Rust tests pass, including independent group memories, observation replacement, recovery/competition/aging effects, and a paired world test proving that changing hidden remote stocks cannot change learned first departures. The latter preserves accounting in both worlds.

Nine seasonal memory comparisons pass replay, route barriers, complete departure export, finite bounded stocks, population/food accounting and paired demography. Observation counts and timestamps are checked against completed arrivals to prevent early destination knowledge. Both policies complete all 34 supported dated environments in learned mode; two Nile inputs correctly reject insufficient starting coverage. Eight native/WASM runs (two policies × two forcing modes × two knowledge modes) match every snapshot field within relative 1e-8.

Browser control checks show observed patch counts growing during playback and a working perfect-information comparison. Inspector distinguishes actual snapshot food from the group's last observation; journey reasons distinguish exploration from revisiting a remembered patch. Export settings retain the information policy.

## Remaining work

Scarcity still does not affect survival, fertility, movement speed or group splitting. No social exchange of knowledge, scouting/foraging during travel, dated freshwater, or independently calibrated learning parameters are present. Next, evaluate and integrate scarcity-demography feedback while preserving an uncoupled control; do not infer prehistoric mortality from these synthetic deficit percentages. The master plan remains incomplete.
