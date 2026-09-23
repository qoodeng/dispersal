# Dispersal v2

**v2 is a restart of the model code; the scientific requirements stay the same.** See [PLAN.md](PLAN.md) for the reasoning, the two-tier architecture and gates G0–G6. v1, the M1 engine and site, is unchanged at the repository root.

## Run it

```sh
v2/check.sh                                        # fmt, clippy, tests, ruff, grid reproducibility, smoke study
cargo build --release --manifest-path v2/coarse/Cargo.toml
cd v2
uv run python analysis/study.py prepare --sims 3000
uv run python analysis/study.py simulate           # about 10 min on 4 cores
uv run python analysis/study.py analyze            # writes results/identifiability.json
uv run python analysis/study.py timestep --sims 200   # G2: dt 25 / 10 / 5
uv run python analysis/study.py simulate --out table-south.csv --southern-crossing --limit 1000
uv run python analysis/study.py simulate --out table-source5.csv --source-max-lat 5 --limit 1000
uv run python analysis/study.py structure          # G3: sensitivity and discrimination
```

**Requirements:** Rust stable and [uv](https://docs.astral.sh/uv/). `uv` installs numpy and h5py from `uv.lock`.

**Where outputs go:**
- Generated tables go to `v2/runtime/`, which is untracked.
- `analyze` refuses a table if its parameters, sites, grid or engine binary have changed since it was simulated.

## What is here

| Path | Contents |
|---|---|
| `coarse/` | The Tier 2 engine, written in Rust. Integer counts per 1° cell. Logistic birth–death growth, sampled exactly within each step (Kendall 1948), to a carrying capacity set by rainfall. Binomial moves to the four neighbouring cells (the grid form of diffusion). A Poisson process that deposits dated finds. Tests check exact conservation during movement, no crossing of sea or the closed strait, repeatability from a seed, extinction in unsuitable habitat, the samplers' moments, time-step independence of small-population extinction, and invasion speed against Fisher–KPP |
| `analysis/prepare_grid.py` | Builds `data/grid.json` from the checksummed Beyer 2020 rainfall subset in `research/data/`: 1°, 41 snapshots, 120–40 ka |
| `analysis/designs.json` | Evidence designs: where dated evidence could come from, and how precise its dates are |
| `analysis/study.py` | Prior draws, simulation, G0 calibration, G2 time-step check and G3 structure analysis |
| `results/` | Committed results, each carrying the digests of its inputs |
| `../research/evidence/` | The G1 dated-site register (32 records, 17 sites) and its provenance notes |

## Status by gate

| Gate | Status |
|---|---|
| **G0 Identifiability** | Done. Results below |
| **G1 Evidence** | Draft register ingested: [`research/evidence/`](../research/evidence/README.md). It needs expert review; the conflicts and blocked sources are listed there |
| **G2 Tier 2 numerics** | Time step passes. Grid convergence and bathymetry-based straits are still to do |
| **G3 Structure** | Southern crossing and source region tested. Freshwater scenarios are not yet possible, because no data has been acquired |
| G4–G6 | Not started |

## G2: time step

- **The problem:** the first growth step (Ricker–Poisson) drew one round of demographic noise per step, whatever the step length. Shorter steps therefore meant more noise per year. At a 25-year step, 64% of runs reached Arabia; at 10 years, 45% did.
- **The fix:** the step is now sampled exactly from a linear birth–death process (Kendall 1948). The only approximation left is that crowding is held fixed within a step.
- **Result** ([`results/timestep-check.json`](results/timestep-check.json), 200 draws): 50%, 51% and 52.5% of runs reach Arabia at 25, 10 and 5 years, against 52.5% for the 25-year step with different seeds. Median onset is 112.2–112.6 ka at every step. **Pass.**
- **Caveat:** with 200 draws the tolerance is ±0.10, so the check rules out large effects only.

## G0: can the record recover arrival?

**Method:** simulation-based calibration. 3,000 prior draws are made, and 250 are held out as known truths. Each truth is recovered by rejection ABC against the rest (nearest 3%, with random tie-breaking). A 1,500-run subset gives the same picture ([`results/identifiability-subset1500.json`](results/identifiability-subset1500.json)). Full numbers are in [`results/identifiability.json`](results/identifiability.json).

**Prior predictive:** 48% of draws establish a population in Arabia by 40 ka. Their onsets have a median of 113 ka, with a 5–95% range of 60–119 ka.

**The designs compared:**
- the ingested one-location record;
- 10 illustrative sites (typed from memory; not evidence);
- 40 random cells dated to ±2 ka, as an optimistic bound;
- exact onsets (the oracle);
- **the two G1 register designs:**
  - *sapiens* only: 5 locations, namely Al Wusta+Alathar, Tinshemet, Skhul+Qafzeh, Manot and Taramsa;
  - all non-Neanderthal records: 12 locations, adding the lithics-only sites.

In both register designs, each location is dated by its oldest usable record and uses that record's stated error.

| Design | Whether Arabia was reached by 40 ka (Brier skill; 0 = prior, 1 = perfect) | If reached, width of the 90% interval on onset (prior: 55 ka) | Range of 90% coverage across quantities |
|---|---|---|---|
| Ingested (1 location) | 0.24 | 48 ka | 0.85–0.93 |
| **Register, *sapiens* (5)** | **0.71** | **37 ka** (25 ka on the 1,500-run subset) | 0.84–0.94 |
| **Register, all (12)** | **0.86** | **39 ka** | 0.86–0.94 |
| Illustrative (10) | 0.84 | 37 ka | 0.85–0.94 |
| Dense (40, ±2 ka) | 0.91 | 14 ka | 0.88–0.95 |
| Oracle (exact onsets) | 1.00 | 3 ka | 0.88–1.00 |

**Parameter contraction** (0 = no better than the prior, 1 = pinned down). With the 12-site register, growth reaches 0.10, diffusion 0.11, the rainfall threshold 0.30, density 0.26 and detection 0.25. Even the oracle reaches only 0.26 for growth and 0.24 for diffusion.

## G3: do structural choices matter, and can the record tell?

Full numbers are in [`results/structure.json`](results/structure.json). Each alternative uses the same first 1,000 prior draws as the baseline.

| Alternative | Effect on the answer | Can the record tell? (Brier skill; 0 = cannot) |
|---|---|---|
| **Bab el-Mandeb open** (walking crossing allowed) | Every run reaches Arabia (50% in the baseline), and arrival is about 7 ka earlier. The outcome differs in half of all draws | Barely: *sapiens* register 0.10, full register 0.20, dense 0.35. Only exact onsets can (0.93) |
| **Source restricted to south of 5°N** (baseline: 15°N) | Small: only 5% of draws change outcome, and arrival is about 1.4 ka later | No, in any design (about 0) |

## What it means

1. **The inference is honest.** Coverage is at or near nominal in every design, so weak evidence produces wide answers, not false ones.
2. **The real record says a fair amount about *whether* people reached Arabia, and little about *when*.** The 12-site register reaches Brier skill 0.86. Its arrival window is still about 39 ka wide, because each site's oldest surviving find comes long after first arrival. This limit is archaeological, and only many more sites, or earlier sites, would narrow it.
3. **The Bab el-Mandeb assumption drives the result, and the record cannot settle it.** Whether a southern crossing was possible changes the answer more than any parameter does, yet the dated sites barely distinguish the two. This has to be carried as explicit uncertainty (a mixture over routes), or constrained by independent evidence such as bathymetry, sea level and crossing feasibility. It cannot be chosen by fitting sites.
4. **The source-region choice is not important here.** Moving the source from 15°N to 5°N changes little, and it can be fixed without loss.
5. **The mechanism is not identifiable from arrival evidence.** Growth and diffusion stay near the prior even with exact onsets, so mechanisms need their own evidence (Tier 1, gate G5).

## Limits

- **The register is an AI-compiled draft.** Several primary papers were paywalled. Some coordinates are approximate or taken from a nearby place, and several dates conflict (see [`research/evidence/README.md`](../research/evidence/README.md)). These results use the register's *design*: where its sites are and how precise their dates are. Its actual ages have not been fitted; that is G4, after expert review.
- **Still fixed:**
  - walking only, apart from the one crossing scenario;
  - rainfall is the only habitat input;
  - no freshwater data;
  - today's land, apart from the climate source's own mask;
  - arrival times recorded only from 120 ka onward;
  - the gross birth rate is fixed at 0.045 per year, an assumption.
- **Grid convergence (G2) is not yet tested.**
- **Rejection ABC is a conservative estimator.** Better estimators would probably extract more from every design.
