# Controlled food experiment — checkpoint results

19 September 2026. Actor: codex-dispersal-20260919-resource.

**Engineering integration implemented; scientific acceptance remains open.** This checkpoint completes the controlled experiment in RESOURCE-EXPERIMENT.md, not the full DELIVERY-PLAN.md or a calibrated resource model. The existing plan already contains numerical release targets; these remain binding for the complete model. No tracker acceptance criteria were weakened or closed.

## Implemented and integrated

- Existing FoodLedger extended with conserved, atomic transfers between patch and group ledgers.
- Area-scaled synthetic food supplies, capped replenishment, proportional sharing, carried provisions, spoilage and unmet consumption.
- Food-driven stay/move decisions with local knowledge, incoming claims and travel-provision checks; original scheduled policy available as a matched control.
- Independent demographic random stream keeps births and deaths identical across comparison arms; scarcity feedback is deliberately absent.
- Map stock layer, group food inspection, real decision reasons, policy switching and outcome comparison. Journey lookup is indexed instead of scanning every trip per rendered group.
- Exports include both runs, full world input, explicit assumptions and the WASM fingerprint. Native runner uses the same Rust implementation.
- Departure snapshots now export each event exactly once, including events at the current/final snapshot boundary.

## Verified

52 Rust tests pass. Coverage includes food sharing independent of group ID, transfer overflow atomicity, no destination harvest before arrival, abundant/empty limiting cases, exact interval consumption and paired demographic accounting.

Nine paired 100-year Aqaba experiments (seeds 1, 42, 123; renewal 500, 2000, 8000 kcal/km²/day) pass replay, population/energy accounting, nonnegative finite state, whole-run departure completeness and sampled land-route checks. Original reference-mode runtime checks pass.

Both policies complete 100-year runs for 34 of the 36 dated regional inputs. Nile 40 ka and 60 ka correctly refuse initialization for inadequate supported starting sites. They are unavailable, not successful population runs.

Native and WASM produce matching fields at all 1201 monthly snapshots in both central-seed comparison policies, with relative tolerance 1e-8. TypeScript checks and production build pass. Browser interactions, device rendering and frame-rate gates were not tested in this checkpoint.

## Results, not an adoption claim

For Aqaba at 100 ka, seed 123, synthetic renewal 2000:

| Output | Food-driven | Scheduled control |
|---|---:|---:|
| Unmet food demand | 0.00% | 50.15% |
| Departures | 19,226 | 5,621 |
| Total traveled distance, all groups | 310,760.8 km | 208,366.4 km |
| Mean net displacement | 43.8 km | 47.9 km |
| Median completed residence | 8.3 days | 31.1 days |
| Final population | 511 | 511 |

The deterministic maximum-score destination rule was replaced with a seeded weighted choice among food-improving, provisioned destinations. Fully covered planning budgets take priority when available. Returns are allowed; terrain, travel and food constraints still apply. This is an explicit behavioral assumption, not an independently calibrated movement model.

Across central-supply seeds 1, 42 and 123, mean unique destinations per group increased from 53.0 / 51.5 / 45.8 to 164.5 / 167.8 / 178.0. For seed 123, distinct undirected routes per group increased from 166 to 1368.8, and immediate returns fell from 1.96% to 0.43%. All three runs retained zero food deficit. See routes-greedy-baseline.json (source 437ba42) and routes-current.json, reproduced by scripts/check-food-routes.mjs.

The tradeoff is more travel: central-seed distance increased 65% and departures increased 32%. More route variety is not proof of realistic dispersal. The fixed environment, discrete destinations and bounded 120 km domain still permit recurrent routes. Long-term movement calibration remains open.

At high supply, seed 123, food-driven unmet demand is now 0.18% versus 0.83% in the control; seed 1 remains worse (3.19% versus 0.58%). Low-supply runs retain severe deficits (39–47%). Changing destination choice does not solve scarcity response.

Halving the daily step leaves central-seed food deficits at zero, satisfying the narrow registered criterion, but changes net displacement from 43.79 to 24.86 km and departures from 19226 to 19286. Spatial/temporal route convergence is NOT established, and stochastic path divergence needs ensemble-level evaluation. No claim of realistic residence behavior is made.

Serialized paired runs are approximately 21.7–35.6 MB before compression. This is not a mobile memory/performance acceptance result.

## Reproduction

```sh
cargo test --manifest-path engine/Cargo.toml --lib
cargo build --release --target wasm32-unknown-unknown --lib --manifest-path engine/Cargo.toml
cp engine/target/wasm32-unknown-unknown/release/dispersal_engine.wasm public/spatial/engine.wasm
cargo build --release --bin food-run --manifest-path engine/Cargo.toml
node scripts/check-food-experiment.mjs
node scripts/check-food-environments.mjs
node scripts/check-food-native.mjs
python3 scripts/export-food-contract.py
npm run typecheck
npm run build
```

Node 26 supports the type-stripped shared TypeScript runner. Native reproduction: `node scripts/run-food-native.mjs public/spatial/dated/aqaba-100000.json food` (or `scheduled`); write stdout to a chosen run file. No extra inference or native dependencies are required.

## Remaining critical path

1. Diagnose the high-supply stranding failure and test competing decision/information rules against independent movement observations; do not optimize only food deficits.
2. Acquire accepted resource-access and dated water contracts. Uniform synthetic stocks and rainfall do not establish ancient food/water availability.
3. Implement evaluated resource-demography, settlement and fission feedback.
4. Connect regions, advance environmental forcing, and meet the master plan's spatial/temporal convergence gates.
5. Run occupation/arrival inference against independent evidence and complete the application/device acceptance gates.

Shared rocks has no identified Dispersal project. Existing Beads archive and issue export were read and left intact under the current global rocks-only instruction. Related tasks remain open. The earlier integration was a local-preview checkpoint. This route-selection follow-up is prepared for private Site publication; deployment status is reported separately.
