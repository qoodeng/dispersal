# Births and deaths: modern-reference growth checkpoint

The replacement Rust engine now has an annual birth/death projection. The website exposes recorded 160-year runs with births enabled or disabled, starting from the same synthetic 5,000-person age distribution. These are demographic mechanism experiments, not migration forecasts or estimates of prehistoric census size.

## Evidence and extraction

The fertility shape comes from Tuljapurkar, Puleston and Gurven (2007), *Why Men Matter*, [Figure 2a](https://doi.org/10.1371/journal.pone.0000785.g002), Dobe !Kung 1963–1974. This is an actual population-specific age pattern, replacing the unrelated contemporary global fertility shape used in the legacy model. **It is digitized from a figure, not transcribed from Howell’s original numeric table**, which has not been acquired.

The licensed source figure, checksum, pixel coordinates, extracted rates and method are recorded in `research/demography/`. Red female-curve pixels are fitted as a piecewise-linear trace through the eight published five-year age-bin midpoints. The horizontal scale is calibrated from the 20/40/60-year ticks, not assumed from the left frame. Negative rates caused by pixel error are clipped to zero, and the shape is normalized to integrate to one. Within each five-year bin the annual rate is constant. The fitted pixel RMS residual is 1.78 pixels. This measures trace fidelity, not demographic sampling error. A three-pixel acceptance threshold is an engineering tolerance and is not a confidence interval. Figure uncertainty remains unpropagated in the displayed single reference trajectory.

Survival uses the previously documented modern !Kung Siler coefficients in Gurven and Kaplan (2007), Table 2. These pooled-sex coefficients are used for both sexes as an explicit approximation. The fertility scale is fitted to the 0.26% modern intrinsic growth reference reported by Tallavaara and Jørgensen (2021), Table 1, citing Howell pp212–220. We treat it as an annual log-rate target of 0.0026; this convention is recorded rather than conflated with archaeological long-run growth.

The resulting fertility scale is approximately **4.434 births per woman** under the specific discrete census model. This is a fitted parameter, not an independently measured TFR. It is not a prehistoric fertility estimate. The shared population reference improves comparability but does not remove sampling, sex-specific, digitization or historical-transfer uncertainty.

## Birth mechanism and timing

Each annual step first samples deaths and ages the survivors. Each surviving female then has a Bernoulli birth opportunity, using the probability assigned to her age at the start of the year. Births enter at exact age zero at year end and first experience infant mortality during the next step. Newborn sex is drawn with probability 0.5 for each sex, an explicit reference assumption.

This annual convention permits at most one child per female per step. It has no pregnancy, interbirth interval, postpartum, twin or mating-state mechanism. Muller et al. (2020), Methods equations1–4, is a precedent for an annual Bernoulli fertility model, not validation of all these simplifications. Tuljapurkar et al. explicitly explains why mating structure matters. These missing processes remain in the complete delivery plan.

The implementation rejects missing occupied female ages rather than assuming zero fertility; it rejects invalid probabilities; errors leave both the population and random stream unchanged. Newborns enter the explicit birth ledger. No births arise from males or females already removed by the step’s mortality. Resident and traveling groups use the same reference mechanism; no travel mortality effect has been invented.

## Calibration and independent implementation checks

The preprocessing script calculates the scale through the Euler–Lotka condition for the stated end-of-year birth convention. The verification script independently constructs a Leslie matrix and obtains its dominant eigenvalue. Its log is 0.002599999999999352 per year, recovering the fitting target.

Thirty-two Rust stochastic replicates are compared with a separate expected-value projection at all 161 annual checkpoints. Their largest mean deviation is 2.463 Monte Carlo standard errors, inside the specified six-standard-error software tolerance. Seed123 replays exactly; every population ledger balances. These checks validate implementation and reproduction of the chosen fitting target. **They do not constitute an independent demographic or historical validation.**

## Reproduction

```sh
python scripts/prepare-fertility-reference.py
python scripts/evidence-gate.py research/demography/parameter-contract.json
python scripts/run-growth-reference.py
cargo test --manifest-path engine/Cargo.toml --lib
```

The prepared rates, evidence contract, extraction record, checks and CSV runs are committed. The run-growth-reference.py command generates the browser data from actual Rust output, not a frontend approximation. The website retains the earlier survival experiment, climate explorer and separately labeled legacy expansion.

Next: obtain original fertility tables and sampling uncertainty, refine reproduction timing and partner constraints, and integrate resource budgets and terrain-dependent movement. Historical predictions remain blocked pending transfer modeling and independent archaeological validation.
