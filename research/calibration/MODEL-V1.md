# Conditional expansion model v1

This implementation supersedes the earlier research-only status for the bounded expansion model. Run `python scripts/calibrate.py` from the project root to reproduce. Requires numpy, scipy, xarray, h5netcdf/h5py. Inputs and exact formulas are in that script, and hashes plus numerical results are in fit-results.json.

The model calibrates a modern demographic reference and weights an explicit joint prior by one archaeological constraint. It does not claim a fully calibrated ancient population model. Modern intrinsic growth is the target, not an archaeological long-run average. Fertility total is solved numerically with fixed approximate age shape; it is not independently observed ancient fertility.

Al Wusta likelihood: Phi((modeled arrival ka -87.6)/1.25), retaining the measurement's minimum-age interpretation. This is a conservative compatibility likelihood, not a model of preservation/discovery. Effective sample size231.8out of1024. Ancient transfer growth and diffusion remain prior-sensitive. There are no held-out-site accuracy claims.

Route costs use actual reconstructed rainfall averaged120–80ka. This averaging uses the entire stated climatic window, not conditions known to a migrating individual. It is a static environmental approximation. Geography is modern land; masked climate cells are excluded. Missing data can alter route connectivity. Cell coordinates approximate actual site locations at half-degree resolution.

The Rust/WASM custom front was compared against scipy shortest paths for a fitted draw: benchmark date differences below one year. Seven Rust tests passed, including weighted paths and unreachable nodes. Browser custom-scenario execution and playback checked. This establishes numerical behavior, not historical validity.

Population display for individual draws is normalized local density: logistic growth from0.01at arrival with chosen annual growth. It is not an archaeological census or a mass-conservative demographic simulation; the front approximates expansion. The older mass-diffusion engineering fixture is retained in the library for regression but is no longer used by this interface.

Next scientific improvements: independent benchmark chronology, generative observation model, structural and prior sensitivity, time-dependent route geography, and conservative age-structured population transport. These do not block release of the explicitly conditional prototype.
