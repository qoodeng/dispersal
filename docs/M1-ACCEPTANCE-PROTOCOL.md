# M1 integrated protocol

Frozen before coupled-model tuning, 19 September 2026. This supplements, and does not lower, DELIVERY-PLAN section 10. A passing component check is not milestone acceptance.

Use a separate uncoupled demographic control, identical initial cohorts, initial stocks and seeds. Synthetic food and assumed available water establish engineering behavior only. Regional water remains unknown; no historical habitability claim is permitted.

The initial condition hypothesis is a bounded exponentially weighted unmet-demand fraction, with a 30-day time constant. Excess mortality begins above 0.25 sustained deficit and increases quadratically to a maximum hazard of 12/year. Fertility is multiplied by one minus mean annual deficit condition. These are uncalibrated engineering parameters, not prehistoric estimates. Sensitivity must include lag 15/60 days, hazard 6/24 per year and threshold 0.1/0.4 before robustness claims. Survival and births remain integer stochastic events; every excess death is separately counted. No scarcity means exactly the reference demographic behavior.

Movement decisions occur on a fixed daily calendar and at arrivals, independent of resource integration substeps. Observation, trip predictions, failed provisions and residence must be retained. Foraging uses a physical reach and terrain access, independent of environmental cell allocation. Splits/merges conserve cohorts and carried food.

Required fixtures: no food, abundance, crowding/recovery, empty initial stocks, spatially phased seasonal refuges, blocked resources, mistaken exploration, insufficient provisions, extinction/recolonization, internal tile seams and external boundaries. Compare at least 32 seeds for stochastic distribution gates. Record population, births, total and scarcity deaths, condition, arrival time, route occupancy, resident/travel person-days, viable residence and censored stays. Recolonization requires actual prior vacancy, not a visit labeled settlement.

Spatial: 20/10/5 km environmental representations with fixed physical foraging support; final refinement median arrivals below 5% and occupancy within Monte Carlo uncertainty. Temporal: 1/0.5-day integration, demographics and arrivals below 2% with distributions. Isotropic rotated/translated fixtures below 5% beyond sampling error. Preserve failures and test artifacts. Regional connectivity, real terrain barriers and absence of state changes at internal seams are independently checked. Interface and measured playback gates remain required.

## Numerical correction, 19 September

The initial 1/0.5-day physical-foraging run failed at 32 seeds; expanding to 512 still left final displacement above 2%. Both failures are retained. Correction: harvest opportunities are explicit daily/arrival events; consumption, spoilage and reserve exhaustion integrate analytically between them, with exact condition exposure before/after exhaustion. Subdivision no longer invents extra harvesting opportunities. Near-midnight floating-point values are snapped to the same calendar event (1e-9 day tolerance); daily group priority uses that same convention. This is an explicit daily-foraging hypothesis, not a calibrated behavior. All thresholds and comparison metrics remain unchanged, and all affected suites are rerun.

The event-corrected mesh check passed arrivals but failed 1/64 occupancy bins at 32 seeds and 4/64 at 128 seeds. These reports are retained. Candidate sampling now weights each numerical camp by its represented land area (nearest-camp quadrature over fixed land cells), separate from accessible/shared food. This removes equal-per-camp sampling bias when numerical camp density changes. The 128-seed mesh suite is rerun at the original thresholds.
