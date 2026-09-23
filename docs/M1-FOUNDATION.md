> Historical intermediate checkpoint. Superseded by the integrated [M1 acceptance report](M1-ACCEPTANCE-REPORT.md), 19 September 2026. Retained failures below describe the earlier build.

# M1 foundation checkpoint

Actor: codex-dispersal-20260919-m1. Status: implementation in progress; integrated milestone NOT complete.

Implemented:
- Corrected private-reserve dilution in focal-group food forecasts and aligned learned-mode inspector occupancy with decision occupancy.
- Added current/unfinished residence, long-term residency and provision-blocked counts; kept completed residence separately labeled.
- Removed hard-coded 120 km map/engine geometry. Domain size follows the supplied raster. Added a boundary-crossing population-state fixture and explicit size-limit errors instead of truncation.
- Added reporting fixtures for no departures, extinction and in-transit groups, and a recorded zero-food scientific failure.

Checks: 59 Rust unit tests passed. The zero-food run still produces 511 people after 100 years; scientific viability is BLOCKED, regardless of energy accounting. Low supply now reports median current residence about 99.73 years, six groups resident at least one year, five groups currently blocked by travel provisions and 92.41% unmet food demand. These expose stranding, not successful settlement.

The existing shipped worlds remain 120 km. The larger-domain fixture is flat synthetic terrain, not a connected regional dataset. Existing regional ETOPO and precipitation source files are present; the temporary annual multi-variable climate file used by the dated-window builder is absent and needs recovery before reusing that builder. Its provenance is retained. Site-count and graph-scaling limits still require a regional design and profile; the new input guard makes that limitation explicit.

Remaining M1 work, in order:
1. Prepare and verify the connected regional input domain and traversable corridors; choose graph/refinement representation without turning numerical patch size into foraging territory. Preserve group/cohort/reserve/memory state across internal tiles and report outer-boundary attempts.
2. Define and implement resource condition, lagged scarcity demography and the uncoupled control, with explicit uncertainty. Resolve the no-food failure and failed-exploration/stranding response.
3. Add accessible foraging, minimum settlement/splitting/recolonization mechanisms and their conservation fixtures.
4. Pass integrated distribution-based spatial/timestep, boundary, initial-state and resource-response gates before accepting M1.

Shared rocks mapping remains absent from the prior verified registry; no duplicate project was created. This checkpoint does not close master-plan requirements.
