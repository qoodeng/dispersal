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
uv run python analysis/strait.py                   # data/strait.json
uv run python analysis/study.py simulate --out table-closed.csv --no-strait --limit 1000
uv run python analysis/study.py simulate --out table-mainland.csv --strait-series mainland --limit 1000
uv run python analysis/study.py gridcheck --sims 200   # G2: 1 vs 0.5 degree
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
| `analysis/strait.py` | Builds `data/strait.json`: the Bab el-Mandeb open-water gap at each snapshot, from ETOPO 2022 bathymetry and the Spratt & Lisiecki 2016 sea-level stack |
| `analysis/study.py` | Prior draws, simulation, G0 calibration, G2 time-step check and G3 structure analysis |
| `results/` | Committed results, each carrying the digests of its inputs |
| `../research/evidence/` | The G1 dated-site register (32 records, 17 sites), the crossing-evidence review behind the `crossing_km` prior, and provenance notes |

## Status by gate

| Gate | Status |
|---|---|
| **G0 Identifiability** | Done. Results below |
| **G1 Evidence** | Draft register ingested: [`research/evidence/`](../research/evidence/README.md). It needs expert review; the conflicts and blocked sources are listed there |
| **G2 Tier 2 numerics** | Time step passes. The Bab el-Mandeb crossing follows bathymetry and sea level. **Grid convergence fails for the Levant and Sinai-only routes** (see below); whether and when Arabia was reached is robust |
| **G3 Structure** | The strait model has been compared with walking only and with a mainland-only gap. The source-region test ran under the old latitude rule ([history](results/history/README.md)). Freshwater scenarios are not yet possible, because no data has been acquired |
| G4–G6 | Not started |

## G2: time step

- **The problem:** the first growth step (Ricker–Poisson) drew one round of demographic noise per step, whatever the step length. Shorter steps therefore meant more noise per year. At a 25-year step, 64% of runs reached Arabia; at 10 years, 45% did.
- **The fix:** the step is now sampled exactly from a linear birth–death process (Kendall 1948). The only approximation left is that crowding is held fixed within a step.
- **Result** ([`results/timestep-check.json`](results/timestep-check.json), 200 draws): 50%, 51% and 52.5% of runs reach Arabia at 25, 10 and 5 years, against 52.5% for the 25-year step with different seeds. Median onset is 112.2–112.6 ka at every step. **Pass.**
- **Caveat:** with 200 draws the tolerance is ±0.10, so the check rules out large effects only.

## The Bab el-Mandeb strait

[`analysis/strait.py`](analysis/strait.py) writes [`data/strait.json`](data/strait.json), the strait's gap at every climate snapshot.
- **Inputs:** ETOPO 2022 bathymetry (checksummed) and the Spratt & Lisiecki 2016 sea-level stack.
- **Default gap:** the island-hopping *minimax leg*, the smallest possible longest single open-water leg between the mainlands. It is 19.9 km today, and 3.8 km at the glacial low stands (about 64–40 ka).
- **Conservative alternative, `mainland`:** straight mainland to mainland, using no islands. It is 28.5 km today, which matches the published width of about 29 km, and 9–10 km at the low stands.
- **Agreement with the literature:** at low sea level both series match Bailey (2009) and Lambeck et al. (2011), who give crossings of 5 km or less at −90 to −100 m.
- **The strait never closed:** a land bridge needs sea level below −134 m.

**How the engine uses it:** a link between an always-land Africa cell and an always-land Arabia cell opens in a snapshot when the gap is no wider than `crossing_km`, the longest open-water leg people can cross. People cross at the diffusive rate for a crossing front of fixed width, D·dt·w/(L·A) with w = 108.6 km (a convention), so the flow does not depend on grid resolution.

**The `crossing_km` prior** is sourced in [`research/evidence/crossing-evidence.md`](../research/evidence/crossing-evidence.md), an AI-compiled review that needs expert review. It is log-normal with a median of 8 km, truncated to 1–100 km, so 5% of draws fall below 2.2 km and 5% above 30 km. The evidence behind it:
- Neanderthal-attributed crossings of 5–12 km to the Ionian Islands, 110–35 ka;
- Sahul crossings of 70 km or more by *H. sapiens*, but only by about 50 ka;
- drift, swim and raft models of Bab el-Mandeb itself.

Under this prior, 83% of draws can cross the 3.8 km low-stand gap and 15% the 18 km high-stand gap.

**What crossing ability does** (prior predictive, 3,000 draws, [`results/identifiability.json`](results/identifiability.json)):

| `crossing_km` | Draws | Share reaching Arabia by 40 ka | Median arrival |
|---|---|---|---|
| Under 3.8 (never open) | 520 | 47% | 112 ka (via Sinai, during humid phases) |
| 3.8–10.4 | 1,405 | 100% | 68–70 ka (once low stands narrow the gap) |
| 10.4 and above | 1,075 | 100% | 112–120 ka (almost at once) |

**Overall:** 91% of draws reach Arabia by 40 ka. Arrival is bimodal: the 5th, 25th and 50th percentiles fall at 66.5, 69.9 and 111.9 ka.

## G2: numerics

**Time step: pass** ([`results/timestep-check.json`](results/timestep-check.json), before the strait link was added; the growth step it tests is unchanged). 50%, 51% and 52.5% of runs reach Arabia at 25, 10 and 5 years, against 52.5% with different seeds.

**Grid, 1° (25-year step) against 0.5° (6.25-year step), same 200 draws: fail** ([`results/grid-check.json`](results/grid-check.json)).

| Outcome | 1° | 1°, other seeds | 0.5° |
|---|---|---|---|
| Arabia reached, all draws | 89.5% (median 109.9 ka) | 89.5% (110.0 ka) | 87.0% (109.4 ka) |
| Arabia reached, draws that cannot cross | 44.7% (114.6 ka) | 44.7% | 31.6% (112.2 ka) |
| Levant reached, all draws | 80.0% | 79.0% | 66.5% |

- **Robust:** whether and when Arabia is reached overall.
- **Not robust:** reaching the Levant, and arrivals through the Sinai corridor alone, fall at the finer grid.
- **Why:** the model has no continuum limit for these outcomes. A grid cell *is* the local population unit. Halving its size quarters its carrying capacity, so pioneer groups in the narrow, marginal Sinai corridor die out more often.
- **The same check exposed a real bug in the first strait link,** which moved people at a rate that scaled with cell population. That is now fixed ([history](results/history/README.md)).
- **What it implies:** cell size has to be treated as a structural parameter, the scale of a local population, and justified or varied (G3/G5). It is not a numerical detail that shrinks away.

## G0: can the record recover arrival?

**Method:** simulation-based calibration. 3,000 prior draws are made, and 250 are held out as known truths. Each truth is recovered by rejection ABC against the rest (nearest 3%, with random tie-breaking). The 1,500-run subset gives the same picture ([`results/identifiability-subset1500.json`](results/identifiability-subset1500.json)).

**The designs compared:**
- the ingested one-location record;
- 10 illustrative sites (typed from memory; not evidence);
- 40 random cells dated to ±2 ka, as an optimistic bound;
- exact onsets (the oracle);
- **the G1 register designs:**
  - *sapiens* only: 5 locations, namely Al Wusta+Alathar, Tinshemet, Skhul+Qafzeh, Manot and Taramsa;
  - all non-Neanderthal records: 12 locations.

In both register designs, each location is dated by its oldest usable record and uses that record's stated error.

| Design | Whether Arabia was reached by 40 ka (Brier skill; 0 = prior, 1 = perfect) | If reached, width of the 90% interval on onset (prior: 54 ka) | Contraction of `crossing_km` | Range of 90% coverage |
|---|---|---|---|---|
| Ingested (1 location) | 0.06 | 53 ka | 0.02 | 0.84–0.96 |
| **Register, *sapiens* (5)** | **0.18** | **51 ka** | **0.09** | 0.84–0.93 |
| **Register, all (12)** | **0.29** | **50 ka** | **0.14** | 0.87–0.94 |
| Illustrative (10) | 0.31 | 50 ka | 0.14 | 0.86–0.95 |
| Dense (40, ±2 ka) | 0.41 | 16 ka | 0.19 | 0.88–0.94 |
| Oracle (exact onsets) | 1.00 | 6 ka | 0.52 | 0.88–0.96 |

**Other parameters,** with the 12-site register: growth 0.13, diffusion 0.09. Even the oracle reaches only 0.38 for growth and 0.28 for diffusion.

## G3: do structural choices matter, and can the record tell?

Full numbers are in [`results/structure.json`](results/structure.json). Each comparison uses the same first 1,000 prior draws.

| Alternative to the island-hopping strait model | Effect | Can the record tell? (Brier skill; 0 = cannot) |
|---|---|---|
| **Walking only** (no crossing) | Arabia is reached in 49% of draws against 91%, and the outcome differs in 43% of draws | Barely: *sapiens* register 0.05, full register 0.13, dense 0.15, oracle 0.29 |
| **Mainland-only gap** (no island stepping stones) | Arabia is reached in 76% against 91%; the outcome differs in 15% of draws | No, in any design (about 0) |

## What it means

1. **The inference is honest.** Coverage is at or near nominal in every design.
2. **Under the sourced crossing evidence, reaching Arabia by 40 ka is very likely in this model (91%), and the route depends on crossing ability.**
   - If people could cross about 4–10 km of water, they arrive at the glacial low stand around 68–70 ka.
   - If they could cross 10 km or more, they arrive almost at once from 120 ka.
   - If they could cross less than about 4 km, only about half the runs reach Arabia, all via Sinai.
3. **The dated record barely constrains any of this.** The 12-site register has Brier skill 0.29 on whether Arabia was reached, contracts `crossing_km` by 0.14, and leaves the onset interval about as wide as the prior (50 against 54 ka). The answer rests on the crossing evidence, not on the sites.
4. **Island use matters, and the record cannot tell whether people used islands.** Treating the strait as mainland to mainland lowers the chance of reaching Arabia from 91% to 76%.
5. **The mechanism is not identifiable from arrival evidence.**
6. **Grid resolution is effectively a model choice for corridor outcomes.** Levant and Sinai-only results should not be interpreted until the size of a local population unit is justified or varied.

## Limits

- **The site register and the crossing review are AI-compiled drafts** that need expert review (see [`research/evidence/`](../research/evidence/)). These results use the register's *design*; its actual ages have not been fitted, which is G4.
- **The strait uses modern bathymetry and global sea level:**
  - no tectonic, sediment or glacio-isostatic correction;
  - no regional relative sea level;
  - 1-arcminute resolution.
- **Crossing is a hard threshold.** There is no crossing mortality and no probability that falls off with distance, and the crossing-front width w is a convention.
- **Still fixed:**
  - rainfall is the only habitat input;
  - no freshwater data;
  - today's land elsewhere;
  - arrival recorded from 120 ka onward;
  - the source is south of 15°N;
  - the gross birth rate is 0.045 per year, an assumption;
  - the cell size, 1°, sets the size of a local population unit.
- **Rejection ABC is a conservative estimator.**
