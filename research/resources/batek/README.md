# Batek resource–movement development benchmark

Primary paper: Venkataraman, Kraft, Dominy and Endicott (2017), [Hunter-gatherer residential mobility and the marginal value of rainforest patches](https://doi.org/10.1073/pnas.1617542114). Read the Methods, SI Text, Tables S1–S4, Tables 1–2, and author's published R code. Author-maintained data/code: https://github.com/ThomasKraft/HunterGathererMarginalValueTheorem . Original MIT license retained; files, revision and SHA-256 hashes in acquisition.json.

This is a documented paired resource-return/residential-camp dataset, not regional food stock measurements. Observations were collected in Malaysia in 1975–1976. The study economy included trade; rattan returns are rice-equivalent proceeds, not wild edible biomass. The paper reports 40% of calories from agricultural products. Do not transfer these values into ancient Arabia.

## Audit and transformations

The archive has 520 rows: five overlapping resource sets for 93 observed camp-days, plus 55 synthetic zero origins (one per camp/resource). These are not 520 independent observations. Seven complete camps contribute 68 days. Shape analysis excludes camps 1, 4, 7 and 8, following the executable author code and Methods summary; the Methods' earlier camp list is internally inconsistent. All exclusions are retained in protocol.json.

Despite the metadata's daily-return wording, archived `value` is cumulative gain: every sorted series has a zero origin and is nondecreasing, and the author's R code fits the gain curve directly to `value`. `daily-gains.csv` is reconstructed by first differences. Additive identities hold: wild food + rattan equals their combined series; meat + tubers do not exceed total wild food. Missing camp occupation before observation is not reconstructed.

Camp 11 contains 27 days, while the paper prose reports a maximum camp duration of 24 days. The archive and Table S1 date interval support retaining 27 observed days; discrepancy remains explicitly recorded. Caloric values are derived estimates, not calorimeter measurements. The metadata uses both per-capita and forager wording; denominator interpretation should be checked against original field records before any intake requirement is fitted.

`travel.csv` transcribes Table S3. Values already include 3.5 hours for breakdown/setup: do not add that time again. Camps 8 and 10 use imputed mean transition times. The authors' forest and riverside speeds are study-specific estimates, not general walking limits.

## Predeclared development test

protocol.json was written before fitting the four declared candidates. Leave-one-complete-camp-out prediction fits daily increments only on other camps, with equal total training weight per camp. Stored full-sample `Avg` and held-out cumulative trajectories are never supplied to the predictor. Scores average camp RMSE equally. This is stricter than describing a known camp's full trajectory but is not a reproduction or refutation of the published camp-specific analysis.

All three shared depletion curves perform worse than the constant-return baseline. Wild-food RMSE: constant 2033.35, exponential 2161.62, Michaelis–Menten 2226.15, Holling III 2579.22 kcal per capita per day. None is adopted as a regional default. Trade-inclusive sensitivity has the same ranking of baseline versus candidates. This says the pooled curves do not generalize across these camps; it does not establish that depletion is absent.

The source does not identify freshwater constraints, spatial standing stocks, regrowth after abandonment, causal destination choice or demographic responses. Observed gross returns mix effort, availability, sharing and measurement. Camp differences and online local-return estimation require further evaluation. Seven camps from one campaign are development evidence, not independent ancient validation.

## Engine work

`engine/src/population/foraging.rs` provides linear, exponential, Michaelis–Menten and Holling-III cumulative-gain functions, interval gains and descending marginal-return crossings. Units are days and kcal per capita. An externally supplied alternative return must already account for travel/setup costs. It distinguishes unprofitable patches and no crossing before the horizon; it does not force moves from nondepleting patches or confuse a sigmoid's ascending crossing with departure.

This kernel is verified candidate infrastructure, not a fitted regional resource field. It is not wired into the Aqaba default. No new ancient or food calibration claim is made.

## Reproduction

Install Python dependencies `pyreadr`, `numpy`, `pandas`, `scipy`. Run `python scripts/acquire-batek.py`, then `python scripts/audit-batek.py`. Run `cargo test --manifest-path engine/Cargo.toml` for the engine checks. See audit.json and heldout-daily-predictions.csv for all folds, parameters and scores.

## Forward prediction follow-up

The new `forward-protocol.json` was frozen before its scores were computed, but this is explicitly a development follow-up on previously inspected data. Three warmup days leave 47 forecast days across six camps per resource set. Each forecast uses only preceding days; changing future values and other camps verifies the local-model input isolation. All models use identical evaluation rows. The three-day camp has no eligible forecast and is excluded explicitly.

For wild foods, equal-camp RMSE is 1737.91 for the other-camp mean, 1280.02 for the local running mean, 1614.22 for last-day persistence, 1225.58 for exponential and 1223.63 for Michaelis–Menten. The apparent 4.3–4.4% improvement over local mean is uncertain: paired whole-camp bootstrap intervals for RMSE differences include zero (exponential −290.38 to +86.27; Michaelis–Menten −278.35 to +74.40). No candidate passes the predeclared advancement gate. Trade-inclusive forecasts also fail. No tuning was performed after scores.

Forecasts are conditional on the camp still being occupied; outcomes after departure are unobserved. Daily-return accuracy therefore does not validate when to depart. `forward-results.json` records all comparisons and limitations. Run `python scripts/forecast-batek.py` to reproduce.

Independent transfer assessment: Botha et al. 2022 (DOI10.7717/peerj.13066) supplies repeated Cape plant-harvest observations; audited separately in ../cape-geophytes. It is not paired relocation evidence. Wood et al. 2021 (DOI10.1038/s41562-020-01002-7), Methods and Data Availability reviewed, studies Hadza individual movement and landscape use; raw GPS is explicitly not public and requires author agreement. No author contact was made and no restricted tracks were accessed. It cannot currently provide an executable independent benchmark here. Search records and exact eligibility decisions are retained.

## Departure decision evaluation

The frozen departure-protocol.json evaluates end-of-day departure probability in seven complete camps (68days,7events), leaving each entire camp out of fitting. Unlike earlier work, the target is the departure decision itself. A fixed penalized logistic model uses camp age and the ratio of recent food returns to earlier returns; it is compared with a constant hazard and a camp-age-only model. These are exploratory predictive tests, not causal tests of depletion or a reproduction/refutation of the paper's retrospective MVT analysis.

Wild-food equal-camp log loss: constant0.49978, age0.52696, age+food0.55262 (lower is better). Candidate minus constant95%camp-bootstrap interval is[0.00234,0.10662]. Brier scores also worsen. Trade-inclusive sensitivity fails as well. Neither candidate passes. Future-return perturbation leaves earlier features unchanged; focal camps are excluded from all fitting and scaling. With7events, no independent population or ancient calibration is established. No threshold was tuned after seeing the scores.

Run `python scripts/evaluate-departures.py`; predictions, per-camp scores, protocol hash and comparisons are retained. The data do not justify installing this food-decline rule in the regional map.
