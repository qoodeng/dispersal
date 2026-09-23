# Implementation status — replacement engine

18 September2026. Completion is governed by DELIVERY-PLAN.md. This is a foundation checkpoint, not completion of the model or any prediction capability.

## Implemented and checked

- Integer-count cohort/group ledger with timed events and chronologically ordered arrivals. Reference births and mortality now sample integer events and age cohorts annually. Historical demographic calibration is unfinished.
- Atomic rejection of invalid events; no duplicated population on failed operations.
- Group splitting/merging and explicit travel state; population remains counted in transit. Empty groups cannot migrate.
- Continuous local-coordinate displacement and rotation/translation checks; no compass-neighbor movement restriction.
- Whole-segment raster traversal that rejects intervening barriers, diagonal corner cutting and unknown coverage. Analytic barrier fixtures also checked.
- Modern!Kung mortality reference parameter contract with source, table locator, units and uncertainty. Evidence gate rejects missing citations, missing transfer models and missing pinned historical-validation artifacts.
- Climate input artifact retaining41native time slices, native0.5degree coordinates, missing-data masks and spherical cell areas. No extrapolation beyond120–40ka.
- NOAA ETOPO2022 regional60arcsecond relief acquired:2700×2700samples. Original EGM2008 elevation datum retained. Geographic0.1degree block summaries include elevation range/SD, mean slope and fraction above datum. These are preprocessing products, not the final equal-area simulation mesh or ancient land masks.

## Checks run

-20 replacement Rust tests, alongside 7 legacy tests: 27 passed. New checks cover integrated mortality hazard, seeded survival/replay, aging in transit, clock rejection, grid decoding/coverage, geographic frame distances and arrival ordering.
- Rust library checks successfully for wasm32-unknown-unknown; this does not mean new mechanisms are exposed in the website.
- Executable 80-year closed-cohort reference: all 81 checkpoints agree with independently queried environmental data; analytical survival checked using independent Python quadrature; exact replay verified.
-6evidence-boundary checks passed.
-Climate dimensions, time bounds, exact/midpoint interpolation and missing-data behavior checked.
-Terrain dimensions, coordinate subset, units, finite values, slope bounds and input/output hashes recorded.

Rotational tests currently establish geometric invariance of movement primitives. They do NOT establish isotropy/convergence of a complete ecological migration simulation, which does not yet exist.

## Still unfinished

- Equal-area mesh and native-relief path integration; dated coastline/ice connectivity and freshwater coverage.
-140–120ka climate coverage, seasonal food/water fields and evidence-based resource conversion.
- Calibrated stochastic births/deaths, reproductive constraints, scarcity response, local movement decisions, settlement/fission behavior and food accounting. Annual reference aging/mortality exists, with end-of-year event approximation; it is not ancient calibration.
- Route scheduler with environmental changes during travel; explicit maritime hazards and landing mechanics.
- Joint archaeological observation model, synthetic recovery and independent historical validation.
- New-engine website integration and full desktop/phone QA.

The current site remains the old demonstration, now explicitly labeled. No new historical arrival predictions have been released.

## Reproduction

`python scripts/prepare-world-v2.py`

`python scripts/acquire-terrain.py` (network; source pinned in script, output hash in terrain-source.json)

`python scripts/prepare-terrain-v2.py`

`python scripts/evidence-gate.py research/world-v2/parameter-contract.json`

`python scripts/test-evidence-gate.py`

`cargo test --manifest-path engine/Cargo.toml --lib`

New integration: native rainfall and DEM queries in Rust, an explicitly bounded local geographic frame, seeded reference mortality and annual aging. See REFERENCE-EXPERIMENT.md. Terrain does not yet determine migration routes. Next integration: native-relief route costs and dated land/water connectivity, plus matched fertility and resource budgets. No empirical travel speed or fertility rate is supplied by the geometry/ledger tests: all their numbers are synthetic software fixtures.

Additional reproduction: `python scripts/export-runtime-environment.py`, then `python scripts/run-reference-experiment.py`. The runner checks the reference evidence contract and runtime data hashes before execution.

## Website checkpoint

The website now opens with the verified, recorded Rust cohort run (80 years, scrubber, analytical comparison, downloadable results and parameter evidence). A second view exposes all 41 native rainfall slices with cell inspection and explicit missing coverage. The legacy expansion remains separately labeled and available. These are views of completed reference/data work, not a new calibrated dispersal simulation; ecological coupling and new-engine live scenario execution remain unfinished.

Regenerate the website reference bundle with `python scripts/export-web-reference.py` after producing the runtime export and reference run. The exporter verifies the recorded run checksum. No reference view reruns or refits ancient demography in the browser.

## Birth/death checkpoint

Annual reference births now run in Rust with integer offspring, explicit sex draws, aging, mortality and balanced ledgers. Missing occupied fertility ages fail closed. Four additional lifecycle tests bring the library total to31. The annual birth convention is post-survival and at year end; it is a reference approximation, not a pregnancy/mating/spacing model.

A published Dobe !Kung fertility shape has been digitized from Figure2a in Tuljapurkar et al.2007. The original numeric table remains unavailable; figure uncertainty and pooled-sex survival limitations are explicit. Fitting its scale under the discrete census convention to the documented modern growth reference yields4.434 births per woman. This is calibration to a reference target, not independent validation or ancient growth inference.

The website's Births & deaths view displays recorded160-year runs, annual and cumulative counts, the independently calculated expectation, source links and downloadable evidence. Thirty-two replicates and all161annual checkpoints pass the expected-value check; details are in DEMOGRAPHIC-GROWTH-REFERENCE.md. Food, mating states, interbirth spacing, spatial dispersal and archaeological validation remain unfinished.

## Integrated spatial map checkpoint

The website now runs the replacement Rust group engine through WASM, with map playback, cohort demography, resource stock accounting, inspectable relocation decisions, continuous travel and real-terrain barrier checking. See `INTEGRATED-SPATIAL-EXPERIMENT.md` and `research/spatial/model-contract.json`. It is a controlled 120 km modern-terrain experiment, not the completed out-of-Africa model. Resource and relocation parameters are hypotheses, freshwater/climate effects are disconnected, and food deficits do not yet affect mortality or fertility. The default map replaces charts as the primary working surface. Numerical convergence and historical validation remain open.

## Calibrated ecology checkpoint

Default map now uses111 retained ethnographic observations, a modest-skill density/climate fit with blocked-CV and approximate predictive bounds, plus empirical paired mobility. Climate mobility fits failed validation and were rejected. Uniform food budgets were removed from default mode; food production and hunger responses are still uncalibrated. See CALIBRATED-ECOLOGY.md.36 Rust tests and1201 browser-WASM snapshots checked; this is not completion of the scientific delivery plan.

## Dated environment integration

Selectable dates and regions now replace actual inputs to the live Rust population map. Six120km windows × six dates include native temperature/rainfall, global sea-level scenarios and connected marine masks. Inland below-sea depressions are no longer automatically ocean. See DATED-ENVIRONMENTS.md and the artifact manifest. Environments are held fixed for each100-year run; this is not regional long-range dispersal or historical prediction. Research and scientific gates remain tracked in Beads.

## Controlled food integration — 19 September 2026

The food ledger now drives a separately labeled controlled movement experiment and matched scheduled control. Stocks, shared harvest, travel provisions, deficits and decisions are inspectable. Native/WASM parity and engineering accounting checks pass; see RESOURCE-EXPERIMENT-RESULTS.md for full evidence and failures. Synthetic food supplies, no freshwater or scarcity-demography feedback, high-supply model failures and unresolved route convergence prevent scientific acceptance. This checkpoint is available locally; no publication claimed.

## 19 September: route repetition follow-up
Actor: codex-dispersal-20260919-routes. Replaced perfect greedy food destination ranking with seeded gain-weighted choice, preferring covered budgets. Retains provision, terrain, gain and incoming-claim constraints. Three-seed before/after evidence and limitations are in RESOURCE-EXPERIMENT-RESULTS.md and research/food-experiment/routes-*.json. More route diversity costs more travel; calibration and ensemble convergence remain open. No shared rocks project mapping exists; archived issue relationships are unchanged.

## 19 September: seasonal interaction checkpoint
Actor: codex-dispersal-20260919-seasons. Synthetic seasonal forcing now drives patch recovery and decision budgets through one conserved ledger; constant control retained. See SEASONAL-INTERACTIONS.md for the explicit interaction matrix, test evidence and missing feedbacks. Shared rocks epic query still has no Dispersal mapping; no duplicate project or alternative tracker database created.

## 19 September: learned patch information
Actor: codex-dispersal-20260919-memory. Private resident-only observations now drive destination estimates with seasonal recovery, observed competitor demand and confidence decay. Perfect local information is retained as control. Unknown patches are explicit priors, not secretly observed stocks. Tests cover hidden-state independence and no observation before arrival. See PATCH-MEMORY-RESULTS.md. Scarcity demography remains the next missing feedback; no master-plan acceptance is claimed. Existing shared rocks mapping remains absent, so archived relationships are unchanged.

## 19 September: integrated delivery plan revision
Actor: codex-dispersal-20260919-plan. User approved folding connected geography, viability/reporting fixes and all remaining original-scope capabilities into the plan. DELIVERY-PLAN.md section 11 now supersedes the obsolete next batch with M1 connected regional viability and dependent M2–M5 delivery milestones. Independent evaluation, numerical gates and performance checks start alongside implementation. Existing requirements and thresholds remain intact; all new milestone acceptance remains open. Read-only shared rocks lookup confirmed no identified Dispersal epic. Documentation-only change; no engine or live-site update.

## M1 foundation implementation
Actor: codex-dispersal-20260919-m1. Private reserve forecasts, current residence reporting, dynamic domain geometry and explicit capacity guards implemented. See M1-FOUNDATION.md. Connected regional data and scarcity demography are still outstanding; zero-food viability remains a failing scientific gate. No whole-milestone acceptance claimed.

## Integrated M1 implementation in progress
Actor: codex-dispersal-20260919-m1. Connected 3,000 km ETOPO domain generated: 14,882 camps, 127,661 conservative fine-terrain routes, 416,150 unique physical resource quadrature cells. Required mainland anchors share a connected component; islands remain disconnected. Synthetic resources and unknown water do not establish historical habitability.

Added condition with analytically integrated lag/hazard, separately attributed scarcity deaths, condition-dependent fertility, an uncoupled control retaining the same movement/lifecycle rules, fixed daily/arrival decisions, shared physical foraging cells with explicit effort expenditure, provision protection with unmet meals, bounded fission/merging and memory/reserve conservation, continuous residence, established occupation, recolonization, forecast error on arrival and explicit rejected outer-domain exploration proposals. Local fixture behavior remains available as a legacy control.

69 Rust tests currently pass, including established occupation -> resource failure and passage closure -> local extinction -> reopening/recovery -> endogenous recolonization; shared access; physical resource subdivision; phased refuges; initial depletion; provision protection; and integrated fission/merge conservation. The first 32-seed timestep study passed all measured 2% targets; a rerun with explicit first-arrival outcomes and the final implementation is pending. A 32-seed connected-region ensemble completed and is retained; final checks must follow the latest provisioning/accounting optimizations. Browser integration and performance validation are in progress. M1 is still OPEN; no new publication has been made.


## 19 September 2026 — M1 engineering acceptance

Actor: codex-dispersal-m1-20260919. M1 integrated acceptance passed; M2–M5 and historical validity remain unfinished. Canonical evidence: `docs/M1-ACCEPTANCE-REPORT.md` and `public/spatial/m1/results.json`. Connected region, physical shared foraging, exact interval food exposure, lagged demography, conserved lifecycle, explicit establishment/vacancy/recolonization and causal inspection are integrated. Daily event rounding and destination-area quadrature fixed timestep and mesh failures; failed ensembles remain available. All 71 Rust tests, 32-seed timestep suites, 128-seed mesh suite, 49,152-choice isotropy, transformed fixtures, four integrated/eight legacy native-WASM comparisons and browser-export native replay pass. Named-Mac desktop/narrow playback measured 60 fps; physical-phone verification remains in M5. Private exact-build publication follows this source checkpoint; see release receipt for version/deployment IDs. Shared rocks project mapping remains unidentified; no duplicate project created.
