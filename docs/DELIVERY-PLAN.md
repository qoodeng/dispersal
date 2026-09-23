# Dispersal — complete product and scientific delivery plan

**Status: approved requirements from the conversation translated into an execution plan; implementation unfinished.**

Created 18 September 2026; execution sequence revised 19 September 2026 after the adversarial review and connected-region discussion. This plan supersedes the release standard in `research/calibration/MODEL-V1.md`, including its statement that missing scientific components need not block release. The current live site is an experimental demonstration, not completion of the requested product. A successful deployment, passing software tests, a bibliography, or fitting a demographic target cannot individually establish completion.

## 1. The product we are building

An interactive, evidence-grounded simulation of human dispersal out of Africa. Populations must move, persist, grow, contract, disappear and re-establish in response to geography, food, water, climate and their subsistence capabilities. The user can compare terrestrial, coastal and limited watercraft scenarios, watch their consequences, inspect why a route was used, and compare modeled occupation with independently assessed archaeological evidence.

Keep the TypeSafe.AI-inspired interface: squared instrument windows, black title bars, pale blue water, pink population marks, Space Grotesk and restrained measurement typography. The map and playback remain the first screen. The model supplies the visual patterns; cosmetic noise, smoothing and hand-authored paths must not create apparent realism.

### Finished scope

- African source populations and dispersal into the Levant, Arabia and connected Eurasia; a Southeast Asia/Sahul maritime test domain for checking the crossing mechanism. These are geographically explicit scenario domains, not a claim to explain every migration everywhere.
- A core configurable 140–40 ka experiment window, covering early and later departures as separate hypotheses. The start date is an experiment boundary, not an assertion about the origin of humans. Earlier occupations require an older domain or initial occupancy, never forced exclusion. Data acquisition must verify coverage before a scenario is enabled.
- Walking, coastal foraging, freshwater access, seasonal resource use, optional limited boats, sea-level change, ice boundaries, demographic variation, local extinction and recolonization.
- Population numbers and uncertainty where the model supports them; distribution, movement, arrival, persistence, resource pressure and evidence layers everywhere the relevant inputs exist.
- Reproducible ensembles, comparisons and exports; responsive desktop and phone access. Larger scientific runs execute outside the browser, with results available in the same interface.

Build and verify these systems first in Northeast Africa–Levant–Arabia, then extend the validated implementation to the remaining domains. A functioning regional checkpoint is useful progress, but it is not the finished scope above.

### Requirements traceability

| Original request | Required implementation | Evidence of completion |
|---|---|---|
| How far and fast people travel on foot | Resource-constrained population movement, realistic travel budgets, settlement decisions | Independent movement benchmarks and regional occupation tests |
| Mountains and difficult terrain | DEM-derived slope, roughness, passes and path connectivity | Pass/barrier fixtures and geographical review |
| Follow coastlines or use basic boats | Coastal food access plus explicit embarkation, travel and landing | Crossing survival, conservation and changing-strait tests |
| Follow animals and seasons | Seasonal resource fields and foraging response, with uncertainty in prehistoric prey distribution | Seasonal refuge/return experiments; source-linked resource assumptions |
| Climate changes over long periods | Transient temperature, precipitation and productivity; environmental uncertainty | Opening and closing corridors, retreat and recolonization |
| Ice and sea levels change | Dated ice masks and bathymetry-derived shorelines | No traversal of ice/water without a supported mechanism |
| Different subsistence strategies | Continuous dietary and technological traits affecting resource access and mobility | Controlled comparisons that explain cause, not preset route bonuses |
| Calibrated growth | Joint fertility, survival, density dependence and resource calibration | Parameter recovery plus independent demographic checks |
| Credible arrival estimates | Explicit occupation/detection model and held-out archaeology | Out-of-sample predictive assessment and sensitivity results |
| Record all sources | Claim, parameter and dataset provenance with corrections | Every default and scientific result resolves to evidence or a named assumption |
| A fine-grained, watchable site | Real spatial variation, pan/zoom, inspectable populations and movement | Numerical convergence, geographic plausibility, desktop/phone QA |

“Complete” does not mean every ancient behavior is known. It means every promised mechanism is implemented and tested, uncertainty is propagated, and the product refuses unsupported precision. An unresolved component is reported as unresolved; it is not quietly removed from the completion criteria.

## 2. What to keep and what to replace

| Existing component | Decision |
|---|---|
| TypeScript shell, typography, colors, playback and export concept | Keep and adapt |
| Source register and downloaded/extracted evidence | Keep; audit each entry before adoption |
| Actual climate subset and original input hashes | Keep as an input/reference, not as sufficient environmental coverage |
| Modern demographic reproduction | Keep as a reference test; not the sole ancient growth calibration |
| Four-neighbor shortest-path front and imposed wave speed | Retire from the scientific engine; retain only as an explicitly named baseline |
| Static mean rainfall resistance | Replace with transient environmental constraints and resource mechanisms |
| Post-arrival logistic fill with no migrating population | Replace with explicit demographic state and movement accounting |
| Fixed Upper Nile source and narrow departure assumption | Replace with alternative, documented initial-state hypotheses |
| One-sided Al Wusta weighting labeled as sufficient calibration | Replace with a joint observation model and multiple independent evidence groups |
| Current arrival ranges | Archive as outputs of the deprecated experiment; do not inherit them as targets |

First implementation change: label the existing build as a legacy demonstration, version the scientific status of its results, and make the new engine the only source of future research-mode outputs. Do not silently rewrite historical run files. No site changes are being claimed as completed by this planning document.

## 3. Spatial world: fine detail without invented information

### Representation

Use an area-aware equal-area mesh with local refinement, and continuous locations for mobile population groups. The environment is raster/mesh based; movement is not limited to four or eight compass directions. Candidate moves sample direction and distance in physical units and follow local traversable terrain. Crossed cell boundaries affect resources and hazards, not a preferred compass axis.

Start engineering evaluation at **10 km environmental cells**, compare against 20 km and 5 km, and refine selected passes, shorelines and straits to roughly 1–2 km where source topography supports it. These are design targets, not evidence-derived constants. Choose the shipped mesh using convergence and runtime measurements. Do not advertise 1 km palaeoclimate when the climate product is much coarser.

Separate three resolutions: source-data resolution, simulation resolution and rendered detail. Interpolating a climate field must preserve and display the uncertainty and native resolution. A high-resolution coastline does not create high-resolution ancient rainfall evidence.

### Geographic state

Each cell/patch stores physical area, elevation distribution, slope/roughness, land fraction, traversable boundaries, climate coverage, vegetation/productivity, water availability and uncertainty. Time slices also carry coastline and ice state. Maintain spatial indexes and boundary geometry for travel; forbid corner-cutting through barriers.

Use bathymetry plus sea-level scenarios, with vertical datum verification and regional relative sea-level uncertainty. A coarse grid must not accidentally join Africa to Arabia or close a viable passage. Assess crossings using actual water distance along the tested route. Submerged settlements must relocate, remain in transit, or suffer explicitly accounted losses; they cannot teleport.

Freshwater consists of evidence-supported perennial sources, seasonal water and uncertain palaeohydrology. Modern rivers are geometry leads, not proof of past flow. Unknown water coverage is a distinct state, not automatically dry or permanently wet.

## 4. Environmental and subsistence model

### Climate through time

Ingest transient climate reconstructions with temperature, precipitation and relevant seasonality, plus productivity/vegetation and ice. Preserve snapshot dates and interpolation rules. Use seasonal climatology only where available. If seasonal phases are synthesized, identify them as stochastic scenarios, not reconstructed historical weather.

Interpolate continuous fields cautiously; handle shorelines, ice and connectivity as changing topology. Environmental uncertainty is spatially and temporally correlated. Do not add independent random noise to every fine cell and call it uncertainty. Include alternative climate reconstructions or response models; shared model ancestry does not count as independent evidence.

### Food and water

Represent accessible terrestrial plant resources, terrestrial animal resources, freshwater resources and coastal/aquatic resources. Productivity is an input proxy, not a direct count of edible calories. Accessibility and renewable harvest depend on biome, season, technology and subsistence traits. Fit conversion and depletion/recovery relationships against appropriate analogues; expose their transfer uncertainty.

Maintain one coherent resource budget so the same scarcity is not double-counted as several unrelated penalties. Resource intake affects survival, reproduction, willingness to move and local depletion through defined response functions. Water access constrains feasible travel and residence; rainfall alone cannot replace it.

Model prehistoric animal-following as a response to uncertain seasonal animal-resource fields and refuges. Do not draw asserted ancient migration routes without evidence. Validate the mechanism against modern ecological analogues separately from claims about where ancient herds actually traveled.

### Subsistence traits

Populations carry continuous proportions/capabilities for terrestrial gathering, hunting, freshwater exploitation and coastal foraging, plus travel/transport capabilities and seasonal flexibility. Use presets as transparent experiments, not immutable ethnic categories or claims that all hunters behaved alike. Traits can be fixed per experiment or adapt through a documented, bounded transition model. No unexplained “coastal speed ×1.8” multipliers.

## 5. Population engine

### State and units

Represent mobile bands or local population groups with explicit individuals aggregated into age/sex cohorts, location/home range, subsistence traits, resource condition and destination/transit status. Track counts as integers in stochastic runs. If a deterministic approximation uses fractional expected counts, label them as expectations. Never interchange genetic effective population size and census population.

Use SI-compatible explicit units at API boundaries: kilometers, square kilometers, elapsed years, years BP, individuals, annual hazards and resource units. Define BP relative to 1950. Track sex for demographic calculations without making unsupported cultural assumptions.

### Demography

Births depend on age-specific fertility, reproductive population and resource condition. Deaths depend on age-specific survival and explicitly modeled hazards. Cohorts age; groups split, merge and relocate; births, deaths and migrants appear in the accounting ledger. Low numbers must be able to cause demographic extinction.

Calibrate matched fertility and survival schedules where available. Fit uncertain reference populations hierarchically rather than taking a mixed list of census growth, intrinsic growth and millennia-long archaeological growth as interchangeable observations. Ancient transfer effects remain estimated or sensitivity parameters.

Scarcity can depress fertility and raise mortality with biologically plausible lags. Include stochastic demographic variation and environmental shocks with justified priors. Disease, catastrophes, competition and small-population effects require evidence or explicitly bracketed alternatives; do not introduce precise historical hazard rates merely to make curves look realistic.

### Movement and settlement

Distinguish daily/seasonal foraging from migration and new settlement. Foraging travel is not net dispersal. Migration decisions use local resources, crowding, water, route feasibility, known nearby opportunities and uncertain exploration. Populations do not know a globally optimal path to a future archaeological site.

Movement includes direction persistence, exploration, returns, travel time, fatigue/resource consumption and failed destinations. Settlement requires sufficient resources and water over a defined evaluation interval, not just reaching a cell. Group fission contributes settlers; migrants are deducted from their source, carried in transit and added to destinations only on arrival. Extinction and recolonization are ordinary outcomes.

For other hominins, support alternative background occupancy/competition scenarios where evidence warrants them. Do not make a full genetics/admixture simulator a hidden prerequisite or pretend a movement simulation identifies ancestry. Genetic evidence, if used, needs a separate demographic observation model.

### Maritime movement

Implement explicit walking-only, coastal-without-crossings and limited-watercraft scenarios. A voyage requires an embarkation location, capability, potential landing, water/food budget and travel-time/hazard model. Bound propulsion, endurance, visibility and weather/current assumptions with source-backed scenarios. If palaeocurrent data are unavailable, propagate a plausible range instead of substituting present currents as ancient truth.

Count people at sea; failed crossings cause recorded losses. Model crossing opportunities as coastlines change. Boats must not turn all sea cells into ordinary walkable terrain. Crossing frequency and technology onset remain uncertain until evidence supports more precise values.

### Time integration

Use event scheduling for relocation, fission, embarkation and arrival. Integrate demographic/resource hazards with stable substeps; begin with seasonal environmental updates and finer movement/event timing, then select the step using convergence tests. Climate snapshots are slower-changing forcing, not the simulation tick. Preserve checkpoints and deterministic replay of random streams.

## 6. Calibration and archaeological comparison

### Evidence schema

For every dated observation store site and sample IDs, coordinates and spatial uncertainty, taxonomic attribution, material, dating method, original units, error convention, complete distribution when available, stratigraphic relationship, minimum/maximum/finite status, shared systematics, publication/correction, and whether the observation represents a visit, occupation or environmental context.

Keep dates from one tooth/site/phase dependent where appropriate. Never multiply raw sample likelihoods by a published posterior derived from the same samples. A lithic industry alone does not automatically identify a species. Preserve contested associations as alternate inclusion scenarios. Blank parts of the map are not absence observations without survey evidence.

### Distinct output quantities

1. Earliest modeled visit.
2. First established local occupation, with a declared persistence criterion.
3. Occupation probability through time, including gaps and recolonization.
4. Earliest detectable archaeological evidence under the observation model.

These are not interchangeable. Historical arrival predictions must state which quantity they estimate. An earlier feasible simulated arrival than the earliest known site indicates a hypothesis compatible with assumptions, not a discovery of an earlier migration.

### Fitting sequence

1. Freeze the evidence register and declare training, development and final holdout groups before fitting. Group by sites, regions and shared dating dependencies.
2. Fit demographic reference distributions and habitat/resource response models using independent analogue data; quantify transfer uncertainty.
3. Generate prior-predictive populations and occupation histories. Reject implausible priors for documented reasons, not because they miss a favored route.
4. Implement a generative observation process linking occupation, deposition, preservation, survey and dated finds. Fit only identifiable components; analyze broad detection scenarios where survey data are absent.
5. Run a small synthetic recovery study. Demonstrate recovery of known generating conditions and correct detection of non-identifiability before fitting historical data.
6. Use sequential Monte Carlo or another justified inference method chosen after profiling the likelihood and runtime. Simulation-based methods need calibrated summary discrepancies; no arbitrary “looks close” tolerance.
7. Fit across multiple independent evidence groups. Treat origin, initial occupancy, departure episodes, growth, resources, movement and detection jointly or propagate their uncertainty between stages.
8. Reserve final holdouts until model choices are frozen. After looking at a holdout and changing the model, that evidence becomes development data; obtain another final test or report the lack of one.
9. Compare simple baselines, alternative mechanisms, prior choices and structural variants. Report non-arrival probability, censored arrivals and multimodal intervals rather than averaging incompatible routes into one story.

### What qualifies as credible

Numerical correctness, geographic plausibility, identifiability, independent predictive performance and honest interval reporting must all be assessed. “Fitted” describes an estimation procedure; it is not a quality certificate. If evidence cannot constrain an arrival, the complete product returns a broad or unidentifiable result and explains why. It must not produce a precise date merely because the interface has a date slot.

## 7. Data acquisition and provenance work

The existing 33-source catalog is a starting inventory, not an implementation-complete dataset. Acquire and inspect full methodological detail for each adopted parameter. The following work is mandatory.

| Input | Starting evidence | Remaining acquisition/processing |
|---|---|---|
| Terrain and bathymetry | NOAA ETOPO [S1] | Tile acquisition, vertical datum, derivatives, coastline/strait QA |
| Transient climate | Beyer 2020; Krapp 2021 [S2–S3] | Full-domain variables, older boundary coverage, monthly/seasonal fields where available, uncertainty variants |
| Sea level and ice | Spratt–Lisiecki; climate/ice archives [S4] | Datum reconciliation, regional relative-level scenarios, masks and transition tests |
| Palaeowater | Breeze 2016 [S5] | Georeferenced features, chronology/activation uncertainty, missing-coverage classification |
| Food and habitat | Tallavaara 2018 and related primary ecological data [S6] | Actual resource conversions, seasonality, depletion/recovery and independent calibration |
| Fertility and survival | Gurven–Kaplan; underlying life tables; growth review [S7–S8] | Matched schedules, sample errors, between-population effects, transfer model |
| Mobility and group behavior | Hamilton 2016 and its underlying ethnography [S9] | Separate camp moves, foraging, settlement distance, group sizes and fission evidence |
| Boats and coastal subsistence | Sahul models and underlying maritime/archaeological studies [S10] | Direct technology/endurance/landing evidence; current/wind treatment; dating of capability |
| Occupation and dates | Al Wusta, Alathar, Skhul/Qafzeh, Arabia and later Eurasian domains [S11–S13] | Full chronologies, locations, attribution, survey context, corrections and independent splits |

Each acquisition records DOI/URL, author/year, version, exact locator, access date, license, checksum, native units/resolution, transformations, uncertainties and supported uses. Parameters link to these records or to an explicit project-assumption ID. Missing inputs become blocking tickets or declared uncertainty experiments; never silent invented constants.

## 8. Architecture and execution

**Rust scientific core:** units, mesh/geography, resources, demography, groups, movement, maritime travel, scheduler, observations, outputs and numerical tests as separate modules. Native batch runner and WASM runner use the same model code. No scientific computation hidden in presentation code.

**Python evidence/inference pipeline:** versioned acquisitions, geospatial processing, dated-evidence modeling, demographic fitting, inference orchestration, convergence diagnostics and benchmark reports. Production inference must call the same Rust core; an independent small implementation remains a verification oracle, not a competing engine.

**TypeScript web application:** map renderer, accessible controls, playback, layer inspector, scenario comparison, evidence details and exports. Use geographic tiles/levels of detail and spatial indexing; a smooth display must never alter simulation state.

**Execution service:** native ensemble jobs with job IDs, progress, cancel/resume, input hashes, cached environments, immutable run manifests and checkpoints. Browser WASM supports bounded interactive experiments; do not force continent-scale ensembles onto a phone. Choose hosting/compute only after measured CPU, memory, storage and runtime budgets. Recurring paid compute requires an explicit cost decision; local profiling and test datasets do not.

**Run package:** engine revision, dataset hashes, initial conditions, parameters/priors, seed streams, observation version, calibration split, solver settings, validity domain, outputs and diagnostics. Save all particles/weights needed to reproduce reported quantiles. Never substitute a small sampled playback ensemble for the full inference distribution when calculating scientific summaries.

## 9. Website behavior

Open on an informative paused state with obvious Play, time scrub, scenario and legend controls. Provide map pan/zoom, fine local inspection and a regional overview without changing the modeled scale.

Layers: estimated population/density, actual movement flux or group trajectories, resources/water, terrain cost, climate, coastline/ice, archaeological observations and uncertainty. Movement lines must represent simulated movement, not decorative routes. Distinguish simulated individual runs from ensemble probability maps.

Clicking a group/cell shows population, births/deaths/migration, food/water state, local environmental inputs, source confidence and the reasons behind migration decisions. Explain “why not here?” through actual constraints. Suppress false precision in dates and headcounts.

Compare matched-seed scenarios such as walking-only versus coastal subsistence versus limited boats, wetter versus drier forcing, or alternative source populations. Summarize what changed and what remained fixed. Custom settings are identified as custom; they do not retain a calibrated badge automatically.

Evidence overlays keep fit and holdout sites visibly distinct. Arrival distributions, occupation histories and comparison residuals are available beside the map. Include evidence links, source downloads, run exports and immutable share links under the intended access policy. Private deployment must not be called publicly shareable; verify access explicitly before handing a friend a link.

Keep the research detail accessible without making the landing view a wall of warnings. Scientific status appears once clearly and follows exported results.

## 10. Acceptance tests and release gates

The numeric thresholds below are initial **project acceptance targets**, not values claimed by academic papers. Freeze the final test protocol before tuning the full model. Changes to a gate require a documented rationale and before/after results, not quiet relaxation to pass.

| Gate | Required check | Initial acceptance target |
|---|---|---|
| Accounting | Closed system without births/deaths; movement, fission, merging, voyages | Exact integer population conservation; every real loss attributed |
| State validity | Long runs and edge cases | No negative counts, invalid ages, NaNs, duplicated groups or hidden migration sources |
| Grid artifacts | Flat isotropic world; rotate/translate mesh and initial state | Radial spread anisotropy below 5% outside stochastic sampling error; no persistent compass lobes |
| Spatial convergence | 20, 10 and 5 km reference runs | Last refinement changes regional median arrivals under 5% and route occupancy within Monte Carlo uncertainty; otherwise refine/report failure |
| Temporal convergence | Halve integration steps/event tolerances | Key demographic/arrival summaries change under 2%; stochastic comparisons use distributions |
| Barriers and travel | Ridge with a pass, dry gap, island, estuary, flooded shelf, moving ice | No illegal crossing/teleportation; every route has feasible transit state and budget |
| Resource response | Seasonal refuges and severe persistent scarcity | Mechanistic movement, contraction, extinction and recolonization; no guaranteed monotonic expansion |
| Demographic verification | Analytic/reference life-table cases and small-population tests | Reproduce reference expectations; correct stochastic extinction behavior |
| Calibration recovery | At least 100 manageable synthetic cases before historical fit | Bias/interval coverage assessed against known truth; unidentifiable parameters flagged rather than falsely recovered |
| Inference stability | Repeated seeds/replicates and particle/budget increases | Key interval endpoints stable within 5% of interval width or explicit unresolved status |
| Predictive validity | Untouched site/region blocks; simple baselines | Report proper predictive scores, coverage and uncertainty; claimed improvements require evidence beyond sampling error |
| Sensitivity | Prior width/origin, resource rules, dating assumptions, climate variants | Material changes visible; no robust conclusion asserted if alternatives reverse it |
| Reproducibility | Clean-run rebuild and export/import | Same seeded scientific results within documented platform tolerance |
| Interface | Desktop, narrow phone, touch, keyboard, reduced motion | All core flows work; no clipping, inaccessible controls or mislabeled layers |
| Performance | Measured named devices and standard datasets | Target 60 fps desktop playback, at least 30 fps supported phone playback; controls respond within 100 ms except disclosed computation |
| Publication | Exact tested build and intended audience | Successful deployment, working assets/runs, verified access and rollback |

Rotational invariance alone is insufficient: a circular expansion can be just as arbitrary as a diamond. The real-geography tests must show routes, stoppages and local persistence caused by actual inputs. Conversely, irregularity is not a success metric; a valid homogeneous-world control should spread symmetrically.

Do not demand that an uncertain prehistoric model “predict everything correctly.” Require that it is evaluated honestly, does not contradict its stated process, and restricts its claims to demonstrated capability. If historical predictive validity fails, the research system may be functional, but the promised credible-prediction capability remains unfinished.

## 11. Work packages, dependencies and completion artifacts

All work packages are currently **not complete** unless an artifact is explicitly listed as reusable above. Each implementation ticket must include its input contract, scientific assumption IDs, tests, output artifact, status and blocking dependency.

| Package | Deliverables | Depends on | Exit evidence |
|---|---|---|---|
| P0 — Reset and lock scope | Legacy-model status, requirements matrix, issue backlog, frozen baseline outputs | This plan | No obsolete calibration claim presented as completed science |
| P1 — Acquire world/evidence | Full data manifests, geography tiles, dated-site database, calibration/holdout split | P0 | Coverage, licenses, units and chronology reviewed |
| P2 — Numerical foundation | Mesh, continuous movement geometry, units, accounting ledger, scheduler | P0; P1 fixtures | Conservation, isotropy, resolution and barrier gates |
| P3 — Living populations | Cohort births/deaths, groups, resources, water, foraging/migration, extinction | P1–P2 | Reference demography and resource-response experiments |
| P4 — Changing world and boats | Transient climate, ice/coast changes, seasonal traits, explicit voyages | P1–P3 | Corridor opening/closing, seasonal returns and crossing fixtures |
| P5 — Calibration | Analogue fits, observation model, synthetic recovery, historical ensemble fit | P1–P4 | Parameter diagnostics, uncertainty and reproducible fit artifacts |
| P6 — Independent validation | Frozen holdout results, baseline comparisons, sensitivity/convergence report | P5 | Scientific claims pass their registered tests or are withheld |
| P7 — Complete application | Scientific layers, inspect/compare, run jobs, export/share, responsive QA | P2 interfaces; then P3–P6 | Full end-to-end flows using the validated engine |
| P8 — Scale and release | Full domain packs, compute benchmarks, reproducibility bundle, deployment | P1–P7 | All finished-scope requirements and release gates accounted for |

P1 data curation and P2 analytic fixtures can proceed independently. P7 interface work can use clearly marked development fixtures while the engine is built, but final UI acceptance uses real model outputs. Do not wait for cosmetic completion before checking numerical artifacts. Do not wait until final deployment to discover that phone playback cannot load an ensemble.

### Revised execution sequence — approved 19 September 2026

Actor: codex-dispersal-20260919-plan. The sequence below replaces the earlier next-batch instruction and the incremental local-feature sequence. It does not remove any finished-scope requirement or relax the numerical gates in section 10. M1 is **accepted for its declared engineering scope**, with evidence in [M1-ACCEPTANCE-REPORT.md](M1-ACCEPTANCE-REPORT.md). M2–M5 remain **planned, not accepted**. Existing food, seasonal and memory implementations are reusable experimental components, not independently validated behavior.

#### M1 — Viable movement across a connected region (engineering acceptance passed)

Combine the geographical and model corrections as one milestone, with separately testable changes. Draw from P1, P2, P3 and P7 rather than waiting for each broad package to finish sequentially.

| Workstream | Required change | Acceptance evidence |
|---|---|---|
| Connected geography | One Northeast Africa–Sinai–Levant–Arabia domain. Preserve group identity, cohorts, reserves, memory and simulation time across internal tiles. Use coarser environmental cells with justified terrain refinement at passages. Retain 120 km windows as test fixtures, not the main dispersal domain. | Internal seams produce no reset, duplication, artificial wall or teleportation. Compare equivalent tiled/untiled fixtures. Outer boundaries explicitly record attempted exits or transfers and trigger domain-sensitivity tests; never wrap or silently reflect groups. |
| Accessible food and forecasting | Correct shared harvest versus private reserve arithmetic. Separate numerical patch allocation from accessible foraging area; account for barriers and access effort. | Equal-group sharing and private-reserve fixtures; inaccessible food cannot support residents. Patch refinement does not silently create different biological territories. |
| Viability and population feedback | Track resource condition and sustained deficits; connect condition to bounded, lagged survival and reproduction. Preserve the uncoupled demographic control. Declare parameter uncertainty and source/assumption status before tuning. | No-food scenarios cannot sustain unchanged reference demography indefinitely. Abundant food supports the declared reference behavior. Shortage, recovery, contraction and extinction are accounted for; no invented precise prehistoric mortality rate is treated as calibrated. |
| Movement, settlement and population renewal | Establish the minimum explicit distinction between foraging, full-group relocation and settlement. Include bounded splitting/merging needed for establishment and recolonization, with exact cohort and reserve accounting. Address failed exploration and inability to obtain departure provisions rather than adding deaths alone. | Resource-supported residence is distinguishable from stranding. Groups can establish, decline and recolonize when fixture conditions permit; this outcome is not forced in every run. |
| Honest outcomes and causal inspection | Correct residence reporting to include unfinished stays and censoring. Show current residence, person-time traveling/resident, blocked departure causes, viable occupancy and prediction error on arrival. Reconcile inspector quantities with the actual decision inputs. | The recorded low-supply failure is reported as approximately 99 years stranded, not summarized solely as 1.45-day completed stays. Each outcome is traceable to the input/state that caused it. |

**M1 acceptance:** groups can leave, establish viable populations, decline and recolonize across the connected region under declared test conditions. Longer tracks or more varied routes are not acceptance evidence. Synthetic food/water fixtures may establish engineering behavior, but must not be described as geographically validated habitability. Where regional water coverage is unknown, withhold a real-world settlement-viability claim until M2.

Before implementation tuning, freeze a small integrated suite: no food; sustained abundance; crowding and recovery; depleted initial stocks; seasonal refuges with different spatial timing; barriers and inaccessible resources; mistaken exploration; failed departure provisions; extinction and recolonization; internal seams and external boundaries. Compare controls and seed distributions. Apply the section 10 spatial/timestep gates to relevant movement, occupancy, condition and demographic outputs—not food deficit alone. A failed gate keeps the milestone open.

#### Remaining delivery milestones

These fold the eight outstanding capabilities into the existing P0–P8 scope. Acquisition, benchmark definition and early evaluation begin during M1; the dependencies below govern acceptance, not permission to prepare inputs.

| Milestone | Deliverables and original packages | Depends on | Exit evidence |
|---|---|---|---|
| M2 — Resource access and subsistence | Evidence-supported freshwater, plant, animal and coastal resources; access effort, recovery and foraging ranges; subsistence traits that change actual access. Complete the foraging/migration distinction using appropriate analogue benchmarks. P1, P3, P5. | M1 accounting/interfaces; reviewed regional inputs | Dry gaps, water-limited residence, seasonal refuges and alternative diets produce explainable outcomes. Unknown coverage remains unknown. Resource and movement assumptions receive independent evaluation. |
| M3 — Changing world and transport | Configurable 140–40 ka experiments; dated climate, shoreline, ice and connectivity changes. Walking, coastal-without-crossings and limited-watercraft scenarios with embarkation, landing, provisions and failure. Preserve the eventual connected Eurasian scope and Southeast Asia/Sahul maritime test. P1, P4, P8. | M1–M2 mechanisms; verified temporal/domain coverage | Opening/closing corridors, displacement by environmental change, seasonal resource shifts and crossing outcomes conserve people/resources and respect water/ice. Repeating a global synthetic season is not completion. |
| M4 — Calibration and occupation inference | Joint resource, movement and demographic calibration; synthetic parameter recovery; observation/detection model distinguishing visits, established occupation and detectable evidence. P5–P6. | Evaluated component contracts; M1–M3 outputs for each claimed scenario | Frozen training/development/holdout groups; proper predictive scores and coverage; uncertainty in dates, detection and transfer. At least 100 manageable synthetic recovery cases per section 10. Unsupported arrival claims withheld. |
| M5 — Reliable research application and full delivery | Larger native/off-browser ensembles, reproducible saved scenarios, result import/export, comparisons, uncertainty and causal inspection; final domains and desktop/phone experience. P7–P8. | Functional slices start in M1; final acceptance requires M2–M4 | Clean replay/export/import, versioned inputs/parameters, measured named-device performance and memory, core interaction/accessibility checks, exact-build publication and rollback. All original finished-scope requirements accounted for. |

**Cross-cutting requirement:** uncertainty, sensitivity, numerical reliability and provenance apply at every milestone. Compare origins, initial stocks, information priors, resource rules, climate alternatives, random seeds, mesh and timestep. Use distributions for stochastic comparisons. Do not promote a new behavior to the default merely because it changes paths or passes conservation tests. Record implementation, integration, scientific evaluation and release status separately.

**Priority:** M1 connected regional viability; then M2 water/resource access and M3 changing environments; then full M4 occupation/arrival inference. Calibration design, independent evidence selection, recoverability checks and application profiling run alongside the early milestones. Visual expansion follows demonstrated model capability.

Shared planning registry: the current rocks epic list has no identified Dispersal project. Do not fabricate a mapping or duplicate a project. Keep this repository plan and its implementation-status log as the concrete revision until a shared mapping is established; preserve archived issue relationships. This planning update does not close any scientific gate or authorize unsupported completion claims.

## 12. Delivery discipline and realistic limits

Report progress as **implemented / scientifically checked / independently evaluated / integrated / released**, separately for each component. “Completed” requires its exit artifact. A progress screenshot never substitutes for that artifact. Retain failures, rejected assumptions and model changes in an audit log.

Do not claim the task is done because the site is online. Do not stop at a bibliography or a newly fitted parameter. Do not silently replace the population model with a wavefront to meet a deadline. Do not turn unavailable evidence into fabricated precision or permanently block all implementation while searching for impossible certainty.

This is a multi-stage scientific software project, not a credible one-night finish. Runtime and implementation estimates should be made after the regional P1/P2 profiling slice, using measured cells, groups, events, ensemble evaluations and data acquisition throughput. Provide estimates with dependencies and ranges; do not promise unattended overnight progress unless an actual execution job is running with monitoring and checkpoints.

Before final release, review the numerical/model report and the archaeology/observation report separately. An external archaeological or demographic expert review is strongly desirable before publication-grade claims; it is not permission needed to build the software. No expert endorsement may be implied if none occurred.

**Final acceptance:** the user can run and compare all stated scenario families, inspect their causal mechanisms and sources, recover population accounting, obtain appropriately uncertain occupation/arrival results, see independent validation and sensitivity evidence, reproduce/export a run, and use the deployed application reliably. Every requirement above is either passed or explicitly identified as preventing completion. The current prototype does not meet that definition.

## 13. Primary-source starting points

These references support data selection and methodological review, not an assertion that all proposed mechanisms are already validated. Proposed mesh sizes, performance targets and acceptance tolerances are project decisions. Preserve the larger existing source register and add every new source used during implementation.

- **S1.** NOAA ETOPO 2022, official terrain/bathymetry product: https://www.ncei.noaa.gov/products/etopo-global-relief-model (DOI10.25921/fd45-gt74).
- **S2.** Beyer, Krapp & Manica (2020), climate/bioclimate/vegetation reconstruction: https://doi.org/10.1038/s41597-020-0552-1.
- **S3.** Krapp et al. (2021), longer climate reconstruction: https://doi.org/10.1038/s41597-021-01009-3; data https://osf.io/8n43x/.
- **S4.** Spratt & Lisiecki (2016), sea-level stack: https://doi.org/10.5194/cp-12-1079-2016.
- **S5.** Breeze et al. (2016), palaeohydrological corridors: https://doi.org/10.1016/j.quascirev.2016.05.012.
- **S6.** Tallavaara et al. (2018), environmental correlates of density: https://doi.org/10.1073/pnas.1715638115. Observed density must not simply be relabeled carrying capacity.
- **S7.** Gurven & Kaplan (2007), survival/life-table evidence: https://doi.org/10.1111/j.1728-4457.2007.00171.x.
- **S8.** Tallavaara & Jørgensen (2021), distinct growth estimands: https://doi.org/10.1098/rstb.2019.0708.
- **S9.** Hamilton et al. (2016), residential mobility: https://doi.org/10.1002/evan.21485.
- **S10.** Bradshaw et al. (2021), Sahul model and underlying assumptions: https://doi.org/10.1038/s41467-021-21551-3. Its parameters are not automatic Arabian defaults.
- **S11.** Groucutt et al. (2018), Al Wusta: https://doi.org/10.1038/s41559-018-0518-2.
- **S12.** Stewart et al. (2020), Alathar: https://doi.org/10.1126/sciadv.aba8940.
- **S13.** Groucutt et al. (2021), repeated Arabian dispersals: https://doi.org/10.1038/s41586-021-03863-y; correction https://doi.org/10.1038/s41586-021-04289-2.
- **S14.** Shao et al. (2024), coupled demographic/environmental model: https://doi.org/10.1038/s41467-024-51349-y; correction https://doi.org/10.1038/s41467-025-61311-1. Study-specific fitted coefficients must remain study-specific until transfer is tested.
- **S15.** Roberts et al. (2017), structured validation: https://doi.org/10.1111/ecog.02881. Dependencies must be respected when splitting data.
- **S16.** Saltré et al. (2024), incomplete evidence and expansion inference: https://doi.org/10.1038/s41467-024-48762-8. Its data/time restrictions require review before reuse.
- **S17.** Hallett et al. (2025), changing human niche: https://doi.org/10.1038/s41586-025-09154-0.

Sources S1, S14 and S15 were checked again while preparing this plan; the remaining references carry the review statuses in `research/calibration/sources.json`. No acquisition-only or abstract-only entry is upgraded to fully reviewed by its inclusion here.
