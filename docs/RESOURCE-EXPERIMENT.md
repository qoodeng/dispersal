# Controlled food–movement experiment, protocol 5

Actor: codex-dispersal-20260919-resource. Status: controlled integration implemented; engineering checks passed; scientific acceptance remains open. See RESOURCE-EXPERIMENT-RESULTS.md.
Related existing tasks: disp-bi6.1.1, disp-bi6.4.2, disp-bi6.5.1, disp-bi6.9.1. The shared rocks tracker has no Dispersal project mapping; the archived Beads records are preserved without creating another database or marking scientific gates complete.

## Question and scope
Does an explicit food-budget decision rule alter food deficits, residence and net displacement relative to scheduled residential movement, when both rules share terrain, initial cohorts, food supplies and demographic random draws?

This is a controlled engineering experiment on the existing 120 km dated terrain windows over 100 elapsed years. It is NOT a reconstruction of ancient food supplies. Default: Gulf of Aqaba, selected 100 ka environment, seed 123. Terrain/coastlines/climate remain fixed. The six initial groups and modern demographic reference are retained.

## Explicit assumptions, frozen before comparison
- F1: Uniform, accessible edible-food renewal supplied in kcal per physical land km² per day. Default 2,000; comparison scenarios 500 and 8,000. This is a test input, not inferred from population density, precipitation or an accepted resource fit.
- F1b: Select constant renewal (amplitude 0) or a synthetic annual cosine cycle (amplitude 0.8; 20–180% of mean), peaking at elapsed year start everywhere. Browser default is seasonal. This is a mechanism test, not regional phenology inferred from climate. Integrate forcing exactly over each interval and the 14-day planning budget. Annual potential supply equals the constant control; capped realized replenishment may differ. Initial stocks and capacity remain unchanged. Groups have perfect knowledge of this cycle.
- F2: Each destination site's existing disjoint land-area allocation determines its patch area. Initial food equals 60 days of renewal; replenishment is capped at that capacity. No finer-scale ecological variation is invented.
- F3: Demand 2,500 kcal/person/day for all ages; three days of portable food; carried food decays exponentially at 1% per day. All are synthetic control settings, not fitted demographic requirements.
- F4: At each integration step residents share harvest in proportion to food demand plus reserve deficit. Travelers consume their own carried food. Advance exactly to arrivals so they cannot harvest destination food while traveling. No foraging along journeys.
- F5: Food-driven groups check a 14-day budget. Below full coverage they consider terrain-connected destinations within 30 km, choose an improvement of at least 0.1 coverage units after travel-time cost, and leave only with sufficient carried provisions. Among provisioned improvements, prefer destinations covering the whole budget when available, then sample with probability proportional to coverage gain above the 0.1 threshold. This seeded behavioral assumption replaces deterministic maximum-score selection; it is not an empirically fitted preference. Returns remain allowed. In the perfect-information control, local stocks and incoming reservations are known within that radius; learned mode uses the private-observation rules below.
- F6: Scheduled control retains the existing empirically sampled departure schedule and destination weighting; identical food accounting runs alongside it. It may schedule journeys without enough provisions: those deficits are recorded, not repaired.
- F7: Resource deficits do not yet modify births/deaths. Keep independent demographic and movement random streams in both food comparison arms, so changes in movement cannot accidentally change demography. Report this restriction next to results.
- F8: No freshwater field is inferred from rainfall. Water feasibility is untested; food feasibility must not be labeled complete journey feasibility. Automatic fission, transient forcing and historical arrival inference remain open.

## Acceptance and evaluation
Required: replay; exact population accounting; each patch and group energy balance and global energy balance within relative 1e-8; nonnegative finite stocks; no food acquired at destination before arrival; fair sharing independent of group IDs; abundant-food stay and depleted-food departure limiting cases; refusal of unprovisioned food-driven trips; coastline barrier preservation; original reference-mode regression checks.

Compare seeds 1, 42 and 123 at supplies 500, 2,000 and 8,000 in Aqaba. Report total deficit fraction, departures, completed travel distance, net displacement from initial group locations and residence durations. Repeat the central seed at half the daily step. Require deficit fraction difference <= 0.02 absolute and report route/departure differences without assuming convergence. These are engineering targets, not scientific validation thresholds. Record runtime and serialized size. Run central-input smoke checks across all 36 existing environments.

No candidate is adopted as calibrated behavior based on these synthetic experiments. Batek departure evaluation remains rejected, and these runs do not reuse its held-out observations. Historical and external predictive validation remain separate gates.

## Next integrated milestones
1. Replace controlled food inputs with independently evaluated access/harvest contracts and dated freshwater coverage.
2. Add evaluated scarcity–demography and group-splitting/settlement mechanisms.
3. Connect regional domains and advance environmental forcing through time.
4. Run the master plan's convergence, held-out evidence and occupation/arrival evaluation.

## Seasonal interaction acceptance (registered before ensemble runs)
Actor: codex-dispersal-20260919-seasons. Compare constant and seasonal runs at the same seed, mean supply, initial stocks, terrain and demographic draws. Require exact seed replay, finite nonnegative energy accounting, capacity bounds, unchanged demographic pairing and terrain barriers. Unit tests must establish annual potential-supply equivalence, integration across year boundaries, recovery after depletion, slower low-season recovery, and lower low-season decision budgets. Run three central-supply seeds, both policies, plus high/low supplies and a half-step sensitivity check. Report deficits, travel and residence without requiring prettier routes or claiming realism. Native/WASM comparison must include both forcing modes. Population affects food demand, but food does not yet affect vital rates. Memory, water, scarcity demography and fission remain explicit missing links.

## Learned patch information (registered before comparison)
Actor: codex-dispersal-20260919-memory. Browser default: observed knowledge. Keep perfect local information as an explicit control. Groups observe only the patch where they are resident: post-harvest food and other residents. No early observation during travel, global incoming reservations, shared memories, or remote current stock is available to destination choice. Known terrain, patch capacities and the seasonal renewal function remain assumptions.

For remembered patches, project observed stock with integrated potential renewal minus last-observed competitor demand, capped to [0, capacity]. Blend toward a half-capacity prior with exponential confidence exp(-age/90 days); estimated competitors decay with the same confidence. Unvisited patches use half capacity and zero known competitors. These choices are transparent synthetic hypotheses, not inferred ancient cognition. Existing provision, route and improvement constraints apply to estimated budgets. Returns are permitted; arrival replaces estimates with observations. Scheduled control does not use memory for decisions.

Acceptance: a hidden remote stock change must not alter the first learned-information decision; memories remain private and update only on observation; no destination observation before arrival; aging, recovery and observed competition affect estimates; all existing food/population and route constraints remain valid. Compare three seeds and all three supplies with seasonal forcing, learned/perfect food decisions and the scheduled control. Record errors and deficit tradeoffs without requiring the new mode to outperform an omniscient control. Half-step deficit comparison and native/WASM equivalence remain required checks; scientific validity is still open.

## M1 foundation corrections
Actor: codex-dispersal-20260919-m1. The focal-group budget now equals shared environmental food (stock plus potential 14-day renewal) divided by all competing demand, plus its private reserves divided only by its own demand. Forecast harvest sharing is still a proportional approximation; this is not a fitted foraging model. Learned-mode inspector claims use resident counts consistently with decisions.

Outcome summaries now include median and longest current residence among living residents, number resident for at least one year, and final-snapshot provision-blocked groups. In-transit and extinct groups are excluded from current-residence statistics; completed stays remain separately reported. Long residence does not establish settlement viability.

Square domain geometry derives from width times cell size in the engine and map. Existing 120 km inputs remain unchanged. The current runner rejects dimensions over 512 or more than 2000 sites rather than silently truncating input. A 240 km flat fixture crosses the old boundary; this is not acceptance of connected Northeast Africa–Arabia geography or a tiling implementation.

No-food survival remains an explicitly failing scientific acceptance case, recorded in m1-foundation-checks.json. Conservation and interface checks cannot override that failure. M1 remains open.
