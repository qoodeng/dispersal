# Calibrated ecology inputs — modern analogue, not a food model

The default map no longer assigns arbitrary renewable food stocks to dots. It uses a fitted modern density reference and observed joint residential-mobility distributions. This is a partial replacement of the earlier hypothetical relationships. It does not calibrate edible food production, deprivation, freshwater, reproduction responses or ancient dispersal.

## Data and fit

The Binford dataset maintained by Marwick, Johnson, White and Eff was downloaded at a pinned Git revision with its key and codebook. Retained observations are non-exceptional subsistence (`subpop=n`), fishing below 40%, at least one residential move/year and positive original annual miles. 111 observations remain. Original ethnic-group names, dates and ethnographic references are preserved. This selection is an analyst-defined terrestrial-mobile analogue, not all hunter-gatherers.

`density` is people/100 km² and is divided by 100. Original `dismov` is miles/year and is multiplied by 1.609344. The derived `kmov` column contains inconsistent conversions, so it is not substituted. Household residential mobility is not long-distance net dispersal.

OLS log-density uses log annual rainfall, temperature/10 and squared temperature/10. See `research/ecology/fit.json` for coefficients. Five folds hold entire 20-degree geographic blocks together. Held-out log RMSE is 1.513 versus 1.576 for the intercept baseline, only modest predictive improvement. 500 geographic-block coefficient bootstraps plus held-out error define approximate 90% predictive bounds. They are not calibrated ancient intervals; coverage has not been externally validated.

Climate models for annual mileage, move frequency and distance/move failed the same baseline comparison. All three were rejected. The engine samples a paired observed annual move-count/distance once per group instead, preserving their empirical association. This is an empirical reference distribution, not an inferred climate response.

## Map coupling

Beyer/pastclim baseline time 0 supplies temperature and annual rainfall at native 0.5 degrees. ETOPO modern terrain remains separate. No fine-scale climatic variation is invented. A convex-hull check in temperature/log-rainfall space rejects extrapolation. 6212.25 km² of 11237.5 km² positive-elevation land is in-domain. 287 of 550 numerical sites have sufficient supported area. Gray means unknown; destination exclusion reflects unavailable model coverage, not proof of uninhabitability.

Physical 0.25 km² raster areas are assigned to their nearest numerical site. Predicted density is integrated across known land area; site number does not create a fixed additional resource supply. This removes the old per-dot budget artifact but does not prove movement-network convergence.

Destinations favor distances near the sampled reference distance/move and lower occupancy pressure within 30 km of connected sites. The pressure preference, 0.35 log-distance width, 60 km candidate limit and evenly scheduled moves are structural assumptions, not fitted path-choice relationships. Journeys use an assumed 24 km/day timing, not a speed inferred from annual mileage. Modern elevation at or below zero blocks movement; this is not complete hydrology or a palaeoshoreline.

Annual demographics complete correctly after subannual movement; all journey records are retained independently of monthly snapshots. No fission is modeled in this mode. Reference and realized annual mileage are shown separately; geometry constrains realized distances. In the default run one group realizes about 535 km/year against an 806 km/year sampled reference. The model has not matched all mobility targets.

## Validation

36 Rust tests pass including strict annual-clock behavior and equivalence after subannual movement. The same WASM and inputs used by the browser were run for 1201 snapshots/100 years, with exact same-seed replay, integer population accounting, annual-only births/deaths, supported destinations, and land traversal checks. The default run has 5631 journeys. Lower density-reference bounds change routes. These are numerical checks, not archaeological validation.

## Reproduce

1. `python scripts/acquire-ecology.py` (source URLs/hashes and compact climate extraction recorded).
2. `python scripts/prepare-spatial.py` (existing terrain/demography inputs).
3. `python scripts/calibrate-ecology.py` (fit, folds, uncertainty and map inputs).
4. Build the Rust WASM library and copy it to `public/spatial/engine.wasm` as in the prior integration document.
5. `node scripts/check-ecology.mjs`.
6. Copy fit/acquisition/observations/runtime-checks into their public names, then build the website.

Next scientific requirements: food and freshwater observations that identify supply and intake separately from density; independent behavioral calibration; full environmental uncertainty; mesh/window sensitivity; ancient transfer and held-out archaeological likelihoods. No food-stock or hunger result is claimed by this calibrated mode.
