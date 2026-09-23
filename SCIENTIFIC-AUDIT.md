# Dispersal: evidence review and implementation decisions

Review date: 18 September 2026

**The evidence gate for historical population predictions has not passed.** This audit establishes a sourced environmental experiment and identifies the remaining assumptions. It does not establish that every prehistoric population condition is known, or that the simulator reproduces human history. This is a targeted primary-literature review, not an exhaustive systematic review or external peer review.

## What changed

- Removed the arbitrary 1.8 coastal capacity multiplier and its subsistence selector.
- Removed fabricated desert and river-like habitat gradients. The optional population test now uses an explicitly uniform engineering fixture, with its existing latitude factor.
- Stopped automatic population runs. Running the fixture requires an acknowledgment that its parameters are uncalibrated.
- Added a separate, sourced rainfall screen with selectable reconstruction dates, four threshold scenarios, and downloadable provenance.
- Added a machine-readable register connecting 17 model choices and gaps to evidence status, source scope, and implementation files. Unsupported historical parameters remain unavailable; citations do not turn test settings into empirical estimates.

## Evidence and consequences

### Reconstructed environment

Beyer, Krapp and Manica provide bias-corrected climate and vegetation reconstructions on a 0.5° grid, at 1,000–2,000-year intervals over 120,000 years. These are modeled reconstructions, not observations of each ancient cell. We downloaded the annual NetCDF, checked its published MD5, and retained native coordinates and dates. [Beyer et al. 2020](https://doi.org/10.1038/s41597-020-0552-1)

The exact input is the pastclim v1.1.0 repackaging: ice sheets and internal seas are masked; modern outlines were used for the Black and Caspian seas. We preserve missing values, rather than treating them as dry land or zero rainfall. [Dataset and license](https://zenodo.org/records/7062281), [packaging documentation](https://evolecolgroup.github.io/pastclim/reference/Beyer2020.html)

**Decision:** use annual precipitation for an environmental screen only. Do not infer carrying capacity, freshwater availability, boat access, or settlement from this variable.

### Rainfall constraints

Beyer et al.'s 2021 connectivity study motivates a roughly 90 mm annual-rainfall threshold using an ethnographic sample, and examines alternatives including 110, 130 and 200 mm. It discusses transfer limits and special assumptions about the Nile and southern crossing. The study does not supply validated population growth or movement speeds. [Beyer et al. 2021](https://doi.org/10.1038/s41467-021-24779-1)

**Decision:** retain those four values as sensitivity scenarios. Our regional area screen is not a replication of that paper's 300,000-year route analysis. It does not implement its hydrology or crossing assumptions. A threshold is not a universal human survival limit.

### Population growth

Tallavaara and Jørgensen compare ethnographic, archaeological and simulation-based growth estimates. Their Table 1 combines differing intervals and estimators; short-term demographic growth and long-term net change are not interchangeable. [Tallavaara & Jørgensen 2021](https://doi.org/10.1098/rstb.2019.0708)

**Decision:** the old 0.8% annual setting remains only a software fixture. No scientifically calibrated range has been established for the proposed ancient populations. Do not manufacture one by taking the smallest and largest published estimates as a confidence interval. Fixed logistic growth also remains an unvalidated structural assumption.

### Density and carrying capacity

Tallavaara et al. analyze associations between hunter-gatherer density, productivity, biodiversity and pathogens. Their supporting spreadsheet reports density in **people per 100 km²**. The supporting code excludes suspect subsistence cases and densities of 300 or more. [Tallavaara et al. 2018](https://doi.org/10.1073/pnas.1715638115), [supporting data and analysis](https://zenodo.org/records/1167852)

Our inspection reproduced the reduction from 357 raw rows to 300 after those two filters. Only one retained row falls within our 20–65°E, 5°S–40°N rectangle. That is a diagnostic count from the downloaded spreadsheet, not a claim that the entire region has only one relevant ethnographic source.

**Decision:** do not treat a global median as local ancient carrying capacity. Observed density and ecological carrying capacity are different quantities. The fixture's 2,000-person scale has no historical endorsement.

### Movement and subsistence

Hamilton et al. study residential camp movements and their environmental associations. Annual distance traveled can include repeated moves within a territory; their study also describes unexplained cultural and ecological variation. [Hamilton et al. 2016](https://doi.org/10.1002/evan.21485)

**Decision:** do not convert walking distance or camp mobility directly into a population expansion speed. Our current exchange coefficient has no empirical calibration. Coastal resource use does not quantify a universal capacity bonus or establish seafaring capability. No prehistoric population is assigned a fixed diet or technological capability by this audit.

### Shorelines and archaeological comparison

The Spratt–Lisiecki sea-level stack is a candidate environmental input, but global sea level alone is insufficient to produce locally accurate ancient coastlines. It has not been integrated. [Spratt & Lisiecki 2016](https://doi.org/10.5194/cp-12-1079-2016)

Al Wusta is a candidate independent archaeological benchmark. We have not extracted and verified its complete dating model, contextual uncertainties and coordinates, so it is not an active validation point. [Groucutt et al. 2018](https://doi.org/10.1038/s41559-018-0518-2)

A simulated arrival predating a known site is not evidence of earlier occupation. The earliest discovered site is not necessarily the first arrival, and missing sites cannot simply be treated as absence.

## Reproducible environmental experiment

Input: `Beyer2020_annual_vars_v1.1.0.nc`, published MD5 `9773c80fa406eebd71f84a48cf733056`, CC BY 4.0. SHA-256, source links, units, mask caveats and subset checksum are in `research/data/climate-screen.json`.

Selection: 20–65°E and 5°S–40°N; 41 native snapshots from 120,000 to 40,000 years BP at 2,000-year intervals. BP is relative to 1950. No temporal interpolation, invented 65 ka snapshot, or missing-value filling occurs.

Metric: at each date and threshold, sum spherical cell-area weights for qualifying cells, divided by the weights of the **same 5,599 cells with finite rainfall at all 41 dates**. This common denominator avoids changes in data coverage masquerading as climate change. It also excludes changing land masks, so the statistic is not the fraction of all ancient land that was habitable.

| Reconstruction date | ≥90 mm/year | ≥110 mm/year | ≥130 mm/year | ≥200 mm/year |
| --- | ---: | ---: | ---: | ---: |
| 120,000 BP | 75.2% | 68.5% | 64.1% | 53.8% |
| 40,000 BP | 70.7% | 65.9% | 61.2% | 50.9% |

These are our calculations from the cited climate input. Differences between thresholds demonstrate assumption sensitivity; they are not probability intervals. This uses one reconstruction, not a climate-model ensemble. Rainfall seasonality, water sources, routes, migration speed, population dynamics and archaeological fit are not evaluated.

Reproduce from the repository root:

```sh
python3 -m pip install numpy xarray h5netcdf
python3 scripts/audit-climate.py /path/to/Beyer2020_annual_vars_v1.1.0.nc
```

The script refuses a checksum mismatch, checks the time convention and array dimensions, rejects negative precipitation, checks threshold monotonicity, and saves the actual regional NetCDF subset plus results. The original download URL is in the manifest. These checks establish extraction and calculation behavior, not climate-model truth.

## What must pass before historical population use

1. **Define the estimand.** Specify the dated population, region and question: environmental connectivity, establishment probability, or arrival distribution. These require different models.
2. **Construct paleogeography and water inputs.** Verify bathymetry, sea-level datum, local shoreline limitations, rivers and lakes, including treatment of narrow straits and missing data.
3. **Calibrate demographics and movement.** Specify independently defensible parameter distributions and their geographical/temporal transfer limits. Compare growth, mortality, dispersal and subsistence alternatives; do not fit every assumption to a desired arrival date.
4. **Replace the numerical movement formulation.** The current fixture exchanges population counts, not area-normalized density; its distance and capacity factors do not define a validated diffusion coefficient. Use explicit distance/time units, test uniform-density equilibrium, mass balance, and grid/time-step convergence before historical interpretation. Existing Rust invariants alone are insufficient.
5. **Predefine sensitivity and holdouts.** Separate calibration and evaluation sites before tuning. Preserve dating distributions and uncertainty; include survey/preservation bias. Report withheld-site predictive performance and competing models, including failed scenarios.
6. **Obtain subject-matter review.** Archaeological and paleoclimate review remains necessary. No independent scientific review has occurred.

The completed audit and climate screen are useful research foundations. Historical population predictions, calibrated demographic ranges, dated route validation and claims of earlier occupation remain unsupported.
