# M1 — viable movement across a connected region

Engineering acceptance: passed, 19 September 2026. Exact-build private publication is recorded in the release receipt. This completes M1, not the original full scientific product. Water is unknown regionally; synthetic food and assumed available water do not establish historical habitability.

## What now interacts

A 3,000 km Northeast Africa–Sinai–Levant–Arabia domain contains 14,882 camps, 127,661 terrain routes and 416,150 unique resource cells. The named mainland anchors share a connected component; disconnected islands are retained. Routes are sampled at no more than 0.5 km on the native terrain/marine mask, including corner crossings; the map has a separate 2 km overview.

Food depletion, seasonal recovery, shared access effort, carried reserves, learned local information, relocation and lagged population condition now interact. Shortage can suppress reproduction, cause accounted deaths and extinction, or induce provision protection and movement. Healthy groups can split; small co-residents can merge without creating people, food or historical person-time. A year of continuously supported residence establishes an experimental site; recolonization requires genuine vacancy and subsequent supported residence. Current viable occupation is distinguished from historical establishment and unfinished residence is censored explicitly.

## Registered numerical gates

| Gate | Result |
|---|---|
| Mechanisms and accounting | 71 Rust tests pass, including exact food exposure, shared access, private bags, resource barriers, depletion/recovery, empty stocks, no-food extinction, abundant reference control, seam continuity, splitting/merging and endogenous recolonization |
| Physical-foraging timestep, 32 seeds × 50 years | 1 versus 0.5 day: every paired arrival/displacement summary agrees; median population, births, deaths and departures agree; person-time differences are numerical precision. All registered metrics below 2% |
| Local coupled timestep, 32 seeds | Arrival, displacement and demographic medians agree; person-time differences are numerical precision. All metrics below 2% |
| Actual camp mesh, 128 seeds × 20 years | 20/10/5 km numerical meshes use identical physical origins and fixed resource support. Final median arrival refinement 0.93%, below 5%; all 64 route-occupancy bins within simultaneous Monte Carlo bounds |
| Directional bias | 49,152 first departures over 8,192 seeds. Four directional harmonics, including the simultaneous 99% uncertainty bound, remain below 5% |
| Internal seams / outer boundaries | State and arrival time preserved across equivalent representations. Rejected outside proposals recorded; 320/640 km domain sensitivity reported, with no illegal exit or reflected/wrapped trajectory |
| Native / browser engine | Four integrated policy/response combinations, 1,201 monthly frames each; every field agrees to relative tolerance 1e-8. Eight legacy combinations retained |
| Region | 32 seeded 100-year runs conserve people and food at every frame; 22 establish sites and 16 recolonize |
| Parameter sensitivity | Four seeds each for low/empty stocks, lag 15/60 days, hazard 6/24 per year, threshold 0.1/0.4, information and season controls. Empty low-supply regional runs incur 67–345 scarcity deaths in ten years. This is sensitivity evidence, not historical calibration |
| Sustained abundance | Seed 123, supply 8,000: six viable sites, 100-year censored residence, five splits and one merge; no forced departure or scarcity death |

Earlier failures are preserved alongside the passing reports. The first physical timestep comparison failed at 32 seeds and still failed displacement at 512. Daily/arrival foraging events plus analytical intake/spoilage/exhaustion removed interval-dependent extra harvests; consistent midnight handling removed priority changes from floating-point rounding. The subsequent mesh occupancy failure at 32 and 128 seeds exposed equal-per-camp sampling bias. Numerical destination land-area weights corrected this without changing biological foraging area, input food supply or acceptance thresholds.

## Interface and reproducibility

Measured on Mac17,3 / Apple M5 / 10 CPU cores, Codex in-app browser. Desktop 1440×900 playback: 60 fps, p99 frame interval 17.7 ms, p99 update 2.1 ms over 3,360 frames. At 390×844 with the resource layer: 60 fps, p99 frame interval 17.7 ms, p99 update 3.5 ms over 1,740 frames. No horizontal page overflow; narrow map controls have 44 px targets. These are responsive viewport measurements on a Mac, not physical-phone results.

Play/pause, time scrubbing, policy and layer changes, group selection, keyboard pan, pointer drag, zoom and fit-map were exercised. Reduced-motion CSS removes decorative transitions and smooth scrolling; simulation playback remains explicit opt-in. The worker reports progress; the two default 100-year runs took 6.4 seconds in the final local UI, separately from playback latency. Large run export is an explicit computation/download. The 92.4 MB browser download includes complete inputs, both policies and the engine fingerprint; its native replay compares every monthly field.

`public/spatial/m1/results.json` contains hashes, machine-readable gate summaries and links to full evidence, including retained failures. `scripts/report-m1.py` refuses to publish an acceptance bundle unless all recorded mandatory numerical and interface checks pass. Reproduction uses the source repository's `check-m1-*.mjs` scripts, the documented Python input builders, `cargo test --manifest-path engine/Cargo.toml --lib`, and `npm run typecheck && npm run build`.

## Limits and next milestone

Synthetic food production, access radius/effort, daily harvest opportunity, condition response and seasonal phase remain explicit hypotheses. The regional model uses a projected planar distance/area approximation, modern relief and a static 100 ka global sea-level scenario; it is not a validated palaeoshoreline or palaeohydrology reconstruction. Resource quadrature does not resolve every sub-kilometer obstacle. The condition sensitivities have four seeds per alternative, insufficient for robust historical inference.

M2 supplies independently evaluated freshwater/resource access and subsistence inputs. M3 adds the changing world and transport; M4 calibration and occupation inference; M5 complete research workflows, broader domains and named physical-device performance. No historical arrival, archaeological validation or final-product completion is claimed by M1.
