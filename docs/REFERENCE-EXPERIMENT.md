# Executable demographic and environmental reference

This is an integration experiment for the replacement Rust engine. It is **not** a reconstruction of prehistoric occupancy, a growth calibration or an arrival-date prediction. The live website still uses its explicitly labeled legacy demonstration.

## What runs

`engine/src/bin/reference-run.rs` initializes a synthetic closed cohort of 100,000 age-zero females and advances 80 annual mortality/aging steps. The initial count, sex, age, seed 123, location 28°N/37°E and starting date 100,000 BP are software fixtures. They are not archaeological claims. There are no births, immigration, resource effects or habitat-dependent mortality in this experiment.

Mortality uses the modern !Kung point estimates documented in Gurven and Kaplan (2007), equations 1–2 and Table 2, p327: [author-hosted paper](https://gurven.anth.ucsb.edu/sites/secure.lsit.ucsb.edu.anth.d7_gurven/files/sitefiles/papers/GurvenKaplan2007pdr.pdf), [DOI](https://doi.org/10.1111/j.1728-4457.2007.00171.x). The coefficients are read from the existing parameter contract; no prehistoric transfer or sex-specific adjustment is claimed. Joint parameter uncertainty is not yet available in this implementation.

For age x and interval d, the conditional death probability is 1 − exp(−∫[x,x+d] h(a) da), using the analytical integral of the Siler hazard. Integer deaths are sampled by Bernoulli summation. SplitMix64 supplies reproducible pseudorandom numbers; this is an engineering choice, not a demographic result. All survivors age one year. Deaths are booked at year end, so within-year mortality/travel interactions are approximate. Manual time advancement cannot silently bypass the demographic clock. Ledger events carry time; arrivals are ordered by arrival time and then group ID.

The environment adapter reads native-grid exports of the recorded Beyer et al. rainfall reconstruction ([paper](https://doi.org/10.1038/s41597-020-0552-1), [dataset](https://zenodo.org/records/7062281)) and NOAA ETOPO2022 ([dataset](https://doi.org/10.25921/fd45-gt74)). Their existing source IDs, input artifacts and hashes remain recorded. The export does not downscale rainfall or infer palaeocoasts from modern relief.

Coordinates use a spherical azimuthal-equidistant reference frame bounded to 100 km from its origin. This is a local query adapter, not the planned equal-area world mesh. Resident positions and positions interpolated along existing transit segments can be queried. Environmental values do not yet change movement decisions.

Queries return the containing native cell, with half-open boundaries. Temporal rainfall interpolation requires two finite endpoints; unknown values remain unknown. Queries outside geographic/time coverage fail. Elevation remains meters above EGM2008; rainfall remains mm/year. Neither is silently converted into land availability, food or drinking water.

## Reproduce

From the repository root, with NumPy, SciPy, xarray and h5netcdf installed:

```sh
python scripts/export-runtime-environment.py
python scripts/run-reference-experiment.py
cargo test --manifest-path engine/Cargo.toml --lib
cargo check --manifest-path engine/Cargo.toml --lib --target wasm32-unknown-unknown
```

The exporter uses already recorded source artifacts; generated runtime binaries are reproducible and ignored by git. Their manifest records SHA-256 checksums. The experiment verifies those hashes and the reference parameter contract before running Rust twice to check exact replay. Results are in `research/world-v2/reference-run.csv` and `reference-checks.json`.

Independent Python quadrature checks survival against the continuous hazard, and source-array queries check all 81 environmental snapshots. The stochastic cohort stays within 1.375 binomial standard deviations of expected survival across these checkpoints. This verifies implementation of the reference mechanism; it does not validate the historical transfer of that mechanism.

## Next work before a dispersal result

Matched fertility evidence and uncertainty must support births and demographic calibration. Resource budgets and freshwater constraints must support settlement and movement decisions. Native relief must be connected to route costs alongside defensible dated land/water connectivity. These then require numerical convergence, synthetic parameter recovery and independent archaeological validation before arrival predictions are released. The full delivery plan remains controlling; this checkpoint does not replace its requirements.
