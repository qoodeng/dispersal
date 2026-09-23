# Dispersal: research and calibration brief

18 September 2026 · Northeast Africa, the Levant and Arabia · research package v1

## What is ready

The package records **33 academic or primary data references**, including two published corrections, and separates material actually inspected from abstracts and pending leads. It includes a BibTeX bibliography, machine-readable source register, 10 transcribed archaeological measurements, six modern mortality parameter rows, the 799-row published sea-level table, and a reproducible demographic diagnostic. The earlier climate sensitivity extraction remains in the project.

This is a foundation for calibration, **not a completed calibrated simulation**. No historical arrival posterior has been fitted. Collecting more citations alone cannot make an unidentified parameter scientifically known. The practical target is a regional, conditional prediction system that reports what changes when defensible assumptions change.

## Scientific scope and claims

Model movement and population persistence across Northeast Africa–Levant–Arabia first. Separate **first visit**, **established occupation**, and **ancestry of later populations**. A footprint, a fossil and a genetic divergence estimate do not measure the same event. Model multiple occupation episodes and local extinction; do not assume every early departure contributed to living Eurasian ancestry. Arabian records support repeated occupations, while recent work explicitly tests changing human environmental niches. [Groucutt et al. 2021](https://doi.org/10.1038/s41586-021-03863-y), [Hallett et al. 2025](https://doi.org/10.1038/s41586-025-09154-0).

Use 120–40 ka for the existing climate-backed regional screen. Do not predict a first arrival near its starting boundary without either extending the environmental data and initial-state model, or marking the arrival as left-censored. The older climate reconstruction is a possible extension, but it shares model ancestry with the shorter reconstruction and is not an independent climate replicate. [Beyer et al. 2020](https://doi.org/10.1038/s41597-020-0552-1), [Krapp et al. 2021](https://doi.org/10.1038/s41597-021-01009-3).

## Growth: what can be grounded now

Modern ethnographic growth, intrinsic growth calculated from life tables, and archaeological growth over millennia are different estimands. A collection of these numbers must not become a single fitted prehistoric growth distribution. [Tallavaara & Jørgensen 2021, Table 1](https://doi.org/10.1098/rstb.2019.0708).

The mortality coefficients in the downloaded Sahul code were checked against the original modern demographic paper. Table 2 supplies population-specific Siler parameters; the extracted table preserves missing entries and marks the pooled row as non-independent. These are modern analogues, not measurements of MIS5 populations. [Gurven & Kaplan 2007](https://doi.org/10.1111/j.1728-4457.2007.00171.x).

An independent calculation using the published Sahul inputs gives annual log growth of approximately **0.003728** with the specified survival schedule and **0.030441** after survival is artificially set to one. The inputs combine a modern global fertility shape with a rescaled fertility total. The code defines multiple growth quantities; the inspected projection branch uses the no-mortality construction. These results explain the code, not prehistoric growth. They must not be promoted into our default prior. [Bradshaw et al. 2021](https://doi.org/10.1038/s41467-021-21551-3), [archived code](https://doi.org/10.5281/zenodo.4453767).

**Implementation decision:** use an age-structured demographic reference model offline, with matched fertility and survival inputs and between-population uncertainty. Fit a simpler annual growth approximation only over the conditions in which that approximation reproduces the reference model. Estimate low-density growth separately from density regulation, extinction and migration. Generation intervals are uncertain and variable; annual units prevent a fixed generation length from silently changing rates. [Wang et al. 2023](https://doi.org/10.1126/sciadv.abm7047).

**Remaining evidence gap:** matched fertility schedules and their sampling uncertainty have not been extracted. Mortality alone cannot deliver calibrated growth. No defensible prehistoric posterior exists yet for a universal catastrophe frequency, an inbreeding threshold, or fixed hunting-versus-fishing demographic coefficients.

## Movement and carrying capacity

Recorded residential travel is not equivalent to persistent dispersal away from an origin. Turning, return travel and settlement determine how travel translates into spread. Modern hunter-gatherer density is an observed outcome, not direct environmental carrying capacity. The published density dataset includes 357 rows; its analysis filter retains 300, only one within our chosen regional rectangle. Transfer to ancient Arabia is therefore a substantial assumption. [Hamilton et al. 2016](https://doi.org/10.1002/evan.21485), [Tallavaara et al. 2018](https://doi.org/10.1073/pnas.1715638115).

**Implementation decision:** fit movement and demographic parameters jointly, but constrain them with independent evidence. In a synthetic homogeneous logistic-diffusion example, growth 0.002/year with diffusion 125 km²/year and growth 0.008/year with diffusion 31.25 km²/year produce the same limiting front speed of 1 km/year. This is an identifiability demonstration, not an estimate of human spread. Matching arrival times alone need not identify growth.

Keep coastal movement, boats, freshwater dependence and subsistence differences as named hypotheses until supported parameters are available. Coarse climate cells cannot resolve narrow straits. HydroRIVERS is modern hydrography; a modern channel must not become an automatically perennial ancient river. Palaeohydrological reconstruction provides a stronger candidate basis, but GIS extraction and time-specific activation remain unfinished. [Breeze et al. 2016](https://doi.org/10.1016/j.quascirev.2016.05.012), [HydroRIVERS documentation](https://www.hydrosheds.org/products/hydrorivers).

## Arrival evidence and observation model

Al Wusta has a directly dated fossil with a minimum-age constraint, associated tooth measurements, and surrounding sediment ages. The source distinguishes two-sigma U-series errors from one-sigma US-ESR errors. Preserve these distinctions, shared samples and stratigraphic relationships. The published combined interval is not an additional independent observation and is not automatically a uniform posterior. [Groucutt et al. 2018, supplement section 4](https://doi.org/10.1038/s41559-018-0518-2).

At Alathar, sediment below and above footprints brackets the event. Do not average those dates into a supposed first arrival. The extracted OSL rows remain blocked from automated likelihood ingestion until their uncertainty convention is explicitly verified. [Stewart et al. 2020, supplementary Table S4](https://doi.org/10.1126/sciadv.aba8940).

For a site occupation age O and regional first-arrival age A, positive years-before-present imply **A ≥ O**. That constraint does not imply equality: survival and discovery can leave an unknown lag. Later occupations do not rule out earlier local extinction and recolonization. An observation process needs preservation, survey and discovery assumptions; absence of a recorded site is not evidence of absence without adequate survey information. The northern-expansion study supplies a relevant inference precedent, but its radiocarbon-focused dataset does not directly solve older Arabian dating. [Saltré et al. 2024](https://doi.org/10.1038/s41467-024-48762-8).

Use dated evidence as the measured quantity. Integrate uncertainty in sample age, association and detection when evaluating simulated occupation histories. Show regional arrival probabilities conditional on model and data; show “not reached during the modeled interval” separately. Report predictive intervals, not only the spread across repeated random seeds at a fixed parameter setting.

IntCal20 applies to the relevant Northern Hemisphere atmospheric radiocarbon measurements within its stated range; it is not a calibration function for OSL, U-series, or 95 ka observations. [Reimer et al. 2020](https://doi.org/10.1017/RDC.2020.41).

## Environmental inputs and uncertainty

Retain the verified regional climate subset and its spherical cell areas. Precipitation thresholds are sensitivity hypotheses, not universal biological limits. Ocean, ice, missing climate and unsuitable habitat must be distinct states. [Beyer et al. 2021](https://doi.org/10.1038/s41467-021-24779-1).

The sea-level supplement was extracted without changing its PC1, median or quantile columns. Before using it with bathymetry, verify the vertical reference and scaling, and account for regional relative sea-level differences. Do not subtract the first value simply to force modern sea level to zero. Sea-level uncertainty must affect route connectivity, rather than only an uncertainty ribbon on a chart. High-resolution regional bathymetry remains to be acquired. [Spratt & Lisiecki 2016](https://doi.org/10.5194/cp-12-1079-2016), [NOAA ETOPO](https://www.ncei.noaa.gov/products/etopo-global-relief-model).

## Rust prototype contract

These are project design choices, not claims that a paper uniquely prescribes them.

1. **Offline evidence pipeline:** versioned source IDs, raw-file hashes, typed dates, explicit units, missing-value flags, licensing and transformations. Metadata validation rejects unresolved uncertainty conventions instead of guessing.
2. **Rust numerical core:** population counts and cell areas stored separately; transport operates on density with conservative fluxes. Explicit years, kilometers and persons. Handle coast changes, blocked edges, empty cells and extinction without negative populations or invented land bridges.
3. **Calibration runner:** reproducible seeds; independent prior-predictive checks; a tested synthetic parameter-recovery experiment; then joint demographic/movement fitting with an explicit observation model. Do not silently select the best-looking simulation. Keep failed and unreached outcomes in the denominator.
4. **Prediction export:** ensemble IDs, parameter draws, source/data hashes, prior and posterior summaries, arrival definitions, non-arrival probability, interval convention and validation results. The web UI consumes these outputs; browser animation does not establish credibility.
5. **Website:** map and timeline, arrival uncertainty, scenario comparison, evidence drawer, and a visible distinction between exploratory and validated results. Every displayed parameter and benchmark links to its source or is labeled a project assumption. Keep the present engineering fixture labeled as such.

The OurWay study is useful numerical precedent, but its European habitat and growth settings are not transferable calibration. Its correction removes Essen-Fischlaken from a comparison; use the corrected source. [Shao et al. 2024](https://doi.org/10.1038/s41467-024-51349-y), [2025 correction](https://doi.org/10.1038/s41467-025-61311-1).

## Shareable prediction gates

| Gate | Required evidence | Current state |
|---|---|---|
| Traceability | Source IDs, corrections, exact locators, transformations | Register and initial extracted tables ready |
| Demography | Matched fertility/survival and transfer uncertainty | Survival extracted; fertility and calibration unfinished |
| Geography | Resolved land/sea topology and palaeowater assumptions | Coarse climate available; detailed topology unfinished |
| Numerical validity | Conservation, units, grid/time convergence, barriers | New scientific solver not yet implemented |
| Identifiability | Synthetic recovery; posterior sensitivity to priors | Demonstration ready; recovery experiment unfinished |
| Independent evidence | Held-out site/region groups, no habitat-data leakage | Initial dates curated; benchmark collection incomplete |
| Predictive reliability | Held-out dated-evidence checks and interval calibration | Not run; no historical arrival posterior |
| Reproducibility | Pinned inputs, seeds, versions and rerunnable outputs | Initial scripts and checksums provided |

Split validation by independent sites or spatial groups, keeping related samples and site-trained habitat models together. Random splitting of dates from the same site would inflate apparent performance. With only a handful of independent sites, do not advertise an empirically verified 95% coverage claim. [Roberts et al. 2017](https://doi.org/10.1111/ecog.02881).

## Execution order and unresolved work

**Next:** complete matched fertility extraction and regional benchmark chronologies; acquire regional bathymetry; resolve the OSL conventions and temporal boundary. Then implement the conservative Rust solver and synthetic recovery tests. Fit the regional model only after those checks, freeze a held-out comparison, and expose conditional predictions with sensitivity results.

Skhul and Jebel Faya remain candidate benchmarks with partial review, not calibration-ready rows. The Hallett shared-data link returned an empty response during this session. The older climate repository was located but its large NetCDFs were not downloaded. The Europe-wide OurWay archive is approximately 5.5 GB and was cataloged rather than acquired. These are explicit unfinished tasks, not missing evidence disguised by a default number.

Research cannot promise that every prehistoric condition is known. It can make the evidence, uncertainty and consequences inspectable. The first shareable scientific milestone should be a reproducible regional experiment that passes the gates above; a polished animation with dates attached would not meet that target.

## Files

- `sources.json`, `SOURCE-REGISTER.md`, `references.bib`: source ledger and bibliography.
- `data/`: transcriptions, demographic diagnostic, raw sea-level table and audit outputs.
- `scripts/`: extraction and verification utilities.
- `acquisition-manifest.json`: successful source acquisitions, stable identifiers, sizes and checksums; access failures and pending leads remain explicit.
- `validation.json`: checks actually run and calibration blockers.

Acquired literature and archives are not all redistributed in this package. Stable links and checksums preserve provenance; publisher and code licenses still apply. Read status is deliberately narrower than download status.
