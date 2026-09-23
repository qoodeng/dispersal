# Integrated population map

This checkpoint connects the replacement Rust population engine to live browser map runs. It does not complete the scientific delivery plan or provide ancient arrival forecasts.

The default surface now runs 100 elapsed years on a 120 km square centered on the Gulf of Aqaba. Actual age/sex cohorts persist at continuous positions. The simulation combines births, deaths, group fission, renewable finite resource accounts, resident demand, destination selection, barrier-checked journeys and arrival events. Users rerun the Rust/WASM engine with a seed and explicit resource/travel assumptions, scrub or play time, inspect groups, inspect resource stock, zoom/pan and export the full result. The older charts remain supporting reference views. The deprecated front remains separately labeled.

## Evidence boundary

`research/spatial/model-contract.json` records each source and assumption. Terrain comes from the already acquired ETOPO2022 dataset. Mortality and digitized fertility reuse the modern !Kung reference. The graph's numerical sites, resource quantities, knowledge radius, score rule, climbing penalty, travel budget, provisioning and split threshold are explicit hypotheses, not calibrated prehistoric facts. No unsupported rainfall-to-food conversion is present. No new paper is claimed to validate those rules.

The 0.5 km collision grid is nearest-cell resampling of approximately1.6–1.85 km source elevation. It adds no information. All raster cells touched by an edge must be open, including corner/boundary contacts. Nonpositive elevations are blocked conservatively; this is not a palaeocoast or a complete present-day hydrology classification. The view has no archaeological date axis.

Shortages alter relocation decisions and resource accounts. They do not alter births or deaths. Unmet demand is explicit so that impossible supply scenarios are visible rather than silently made viable. This is the central next scientific gap: evidence-based food/water production, intake and demographic response. The current uniform site budget is a controlled integration experiment only.

## Reproduce

1. `python scripts/prepare-spatial.py` extracts the bounded map input from acquired regional ETOPO and demographic data.
2. `cargo test --manifest-path engine/Cargo.toml --lib`
3. `cargo build --manifest-path engine/Cargo.toml --target wasm32-unknown-unknown --release --lib`
4. Copy `engine/target/wasm32-unknown-unknown/release/dispersal_engine.wasm` to `public/spatial/engine.wasm`.
5. `node scripts/check-spatial.mjs` exercises the same WASM module and data used by the website.
6. `npm run build`

Default seed123, renewal25 and travel budget8 produced426 initial people,585 people at year100 and312 departures with110 distinct rounded bearings.111 proposed geometric edges were blocked. Population and resource identities hold at every annual snapshot; repeated same-seed runs match; changing renewal changes the run. These results are software checks, not evidence for the historical accuracy of the trajectories. The count of bearings does not establish isotropy. Mesh/site density convergence remains required.

## Next integration gates

- Replace uniform site budgets with calibrated subsistence/resource and freshwater models; connect intake to reproduction/survival with uncertainty.
- Verify density/resolution and directional invariance before expanding the domain.
- Add dated palaeogeography and climate rather than labeling modern terrain as ancient.
- Calibrate behavior against independent analogues, then compare historical occupation with held-out archaeology.

Travelers now withdraw and carry provisions at departure; they cannot consume later renewal from their former site. Carried stock and discarded return overflow are included in the exact resource identity. The inspector follows fractional playback, and backward scrubbing resolves selection before a group exists. The resource key explains green intensity; terrain remains default, trails show the past10years and selected journeys have destination/direction marks. Desktop browser interactions checked; mobile rendering was not exercised because the supplied browser has no viewport-control capability.
