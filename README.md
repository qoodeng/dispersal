# Current checkpoint

Dispersal now includes a controlled food–movement comparison. See [experiment protocol](docs/RESOURCE-EXPERIMENT.md), [results and reproduction](docs/RESOURCE-EXPERIMENT-RESULTS.md), and the [full delivery plan](docs/DELIVERY-PLAN.md). Start the local map with `npm run dev`. Food supplies are synthetic experiment inputs; this is not a historically validated dispersal model.

The original prototype description follows for historical context; it does not describe the replacement population-map experiment.

# Dispersal

Rust population simulation + TypeScript browser workspace. V0.1 is an uncalibrated mechanics sandbox, not a historical reconstruction.

## Run

- `npm ci`
- `npm run dev`
- `npm run typecheck && npm run build`

The browser runs the actual Rust WebAssembly binary in a worker. Runs use 8100 half-degree cells, annual exact-logistic growth and simultaneous conservative neighbor diffusion, retaining snapshots every 30 years. Native CLI: `cargo run --release --manifest-path engine/Cargo.toml` (synthetic corridor example).

## Rebuild engine

`rustup target add wasm32-unknown-unknown`

`cargo test --manifest-path engine/Cargo.toml`

`cargo build --release --target wasm32-unknown-unknown --lib --manifest-path engine/Cargo.toml`

Copy `engine/target/wasm32-unknown-unknown/release/dispersal_engine.wasm` to `public/engine.wasm`, then rebuild the frontend. Public binary is tracked for reproducible site builds without requiring Rust on the hosting builder.

## Geography

`scripts/prepare-geography.py` generates `public/geography.json` from the retained Natural Earth land GeoJSON. This is modern land, not ancient geography. Source and SHA256 are embedded. Each connection is checked at three intermediate points to reduce false land bridges. Native 110m geometry and a 0.5-degree grid cannot resolve every narrow passage. Four-neighbor connectivity is deliberate and can produce directional discretization effects.

Habitat is an explicitly synthetic mathematical surface: a dry latitude band and river-like corridor, not actual paleoclimate or a documented river reconstruction. Coastal capacity is multiplied by 1.8 in the coastal scenario; this is an experimental assumption. Cell capacity accounts approximately for latitude-dependent cell area. There are no boat connections, ice changes, sea-level changes, resource seasonality, stochastic establishment or prehistoric site data.

Population is continuous, not a person-by-person model. Occupied means >=25 people per cell; this is not proof of settlement. The solver is deterministic; exported JSON contains configuration, source provenance, capacity and all snapshots. Annual growth and migration parameters are uncalibrated.

## Next scientific gates

1. Acquire a regional sample of Beyer2020 with pinned data version, units and timestamps.
2. Select paleoshoreline and freshwater sources; preserve missing-data distinctions.
3. Source demographic priors and archaeological benchmark table.
4. Test grid/time-step convergence and validate against independent evidence before historical interpretation.
5. Add explicit stochastic settlement/extinction and ensembles when supported by the specification.

## Assets

Modern land geometry: Natural Earth (public domain), retrieved from nvkelso/natural-earth-vector. Fonts: Space Grotesk and IBM Plex Mono, via Google Fonts (SIL Open Font License). Reference design: typesafe.ai, inspected September 2026; no brand copy or artwork reused.

## Evidence review (18 September 2026)

Read `SCIENTIFIC-AUDIT.md` before interpreting results. The website now includes
an independently displayed reconstruction-based rainfall screen, a parameter
register, and an optional uncalibrated engineering fixture. The rainfall data
is not fed into the population fixture. Coastal multipliers and fabricated
habitat gradients were removed. Historical population predictions remain blocked.

Reproduce the climate screen with `scripts/audit-climate.py`; its input download,
checksums, license, units and limitations are recorded in
`research/data/climate-screen.json`. The regional NetCDF is retained in
`research/data/beyer-regional-precipitation.nc`.
