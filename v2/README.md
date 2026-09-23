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
| `../research/evidence/` | The G1 dated-site register (32 records, 17 sites) and its provenance notes |

## Status by gate

| Gate | Status |
|---|---|
| **G0 Identifiability** | Done. Results below |
| **G1 Evidence** | Draft register ingested: [`research/evidence/`](../research/evidence/README.md). It needs expert review; the conflicts and blocked sources are listed there |
| **G2 Tier 2 numerics** | Time step passes. The Bab el-Mandeb crossing now follows bathymetry and sea level. Grid convergence is still to do |
| **G3 Structure** | The strait model has been compared with walking only. The source-region test ran under the old latitude rule ([history](results/history/README.md)). Freshwater scenarios are not yet possible, because no data has been acquired |
| G4–G6 | Not started |

## G2: time step

- **The problem:** the first growth step (Ricker–Poisson) drew one round of demographic noise per step, whatever the step length. Shorter steps therefore meant more noise per year. At a 25-year step, 64% of runs reached Arabia; at 10 years, 45% did.
- **The fix:** the step is now sampled exactly from a linear birth–death process (Kendall 1948). The only approximation left is that crowding is held fixed within a step.
- **Result** ([`results/timestep-check.json`](results/timestep-check.json), 200 draws): 50%, 51% and 52.5% of runs reach Arabia at 25, 10 and 5 years, against 52.5% for the 25-year step with different seeds. Median onset is 112.2–112.6 ka at every step. **Pass.**
- **Caveat:** with 200 draws the tolerance is ±0.10, so the check rules out large effects only.

## The Bab el-Mandeb strait

[`analysis/strait.py`](analysis/strait.py) computes the strait's gap at every climate snapshot and writes [`data/strait.json`](data/strait.json).
- **What the gap is:** the smallest possible *longest single open-water leg* on any island-hopping route between the African and Arabian mainlands.
- **Inputs:** ETOPO 2022 bathymetry (checksummed) and the Spratt & Lisiecki 2016 sea-level stack (2.5%, median and 97.5%).
- **Sanity checks:**
  - today's gap comes out at 19.9 km, which is the Perim Island channel;
  - a land bridge would need sea level below −134 m, which matches the Hanish sill (about −137 m).
- **Result:** between 120 and 40 ka the strait never closed. The gap was about 18 km at high sea level, and 3.8 km during the glacial low stands (about 64–40 ka, and briefly near 72 ka).
- **How the engine uses it:** an edge between the 1° cells on either side of the strait opens in a snapshot only when that snapshot's gap is no wider than the new parameter `crossing_km`, the longest open-water leg people can cross.
- **The prior on `crossing_km`:** log-uniform over 1–40 km. This is an assumption, chosen to be agnostic, so about a third of draws cannot cross even the narrowest gap (they are effectively walking only).

**What crossing ability does to the answer** (prior predictive, 3,000 draws, [`results/identifiability.json`](results/identifiability.json)):

| `crossing_km` | Draws | Share reaching Arabia by 40 ka | Median arrival |
|---|---|---|---|
| Under 3.8 (never open) | 1,112 | 48% | 113 ka (via Sinai, during humid phases) |
| 3.8–10.4 | 835 | 100% | 70–81 ka (once low stands narrow the gap) |
| 10.4 and above | 1,053 | 100% | 113–120 ka (almost at once) |

## G0: can the record recover arrival?

**Method:** simulation-based calibration. 3,000 prior draws are made, and 250 are held out as known truths. Each truth is recovered by rejection ABC against the rest (nearest 3%, with random tie-breaking). The 1,500-run subset gives the same picture ([`results/identifiability-subset1500.json`](results/identifiability-subset1500.json)).

**Prior predictive:** under the strait model, 81% of draws reach Arabia by 40 ka.

**The designs compared:**
- the ingested one-location record;
- 10 illustrative sites (typed from memory; not evidence);
- 40 random cells dated to ±2 ka, as an optimistic bound;
- exact onsets (the oracle);
- **the G1 register designs:**
  - *sapiens* only: 5 locations, namely Al Wusta+Alathar, Tinshemet, Skhul+Qafzeh, Manot and Taramsa;
  - all non-Neanderthal records: 12 locations.

In both register designs, each location is dated by its oldest usable record and uses that record's stated error.

| Design | Whether Arabia was reached by 40 ka (Brier skill; 0 = prior, 1 = perfect) | If reached, width of the 90% interval on onset (prior: 53 ka) | Contraction of `crossing_km` | Range of 90% coverage |
|---|---|---|---|---|
| Ingested (1 location) | 0.08 | 53 ka | 0.02 | 0.84–0.92 |
| **Register, *sapiens* (5)** | **0.23** | **52 ka** | **0.10** | 0.85–0.94 |
| **Register, all (12)** | **0.41** | **52 ka** | **0.20** | 0.84–0.92 |
| Illustrative (10) | 0.47 | 50 ka | 0.23 | 0.86–0.92 |
| Dense (40, ±2 ka) | 0.60 | 18 ka | 0.27 | 0.88–0.94 |
| Oracle (exact onsets) | 1.00 | 6 ka | 0.55 | 0.88–0.95 |

**Other parameters,** with the 12-site register: growth 0.11, diffusion 0.08, rainfall threshold 0.29, density 0.20. Even the oracle reaches only 0.33 for growth and 0.25 for diffusion.

## G3: does the strait model matter, and can the record tell?

Full numbers are in [`results/structure.json`](results/structure.json). Both sides use the same first 1,000 prior draws.

| Comparison | Effect on the answer | Can the record tell? (Brier skill; 0 = cannot) |
|---|---|---|
| **Strait model against walking only** | Arabia is reached in 82.5% of draws against 49%, and the outcome differs in a third of them | Hardly: *sapiens* register 0.02, full register 0.10, dense 0.14. Even exact onsets reach only 0.23, because draws that cannot cross are identical to walking only by construction |

The earlier all-or-nothing test of the strait (every run reached Arabia when it was open) and the source-region test are in [`results/history/`](results/history/README.md).

## What it means

1. **The inference is honest.** Coverage is at or near nominal in every design.
2. **Arabia was almost certainly reachable if people could cross about 4 km of water.** From about 64 ka the strait narrowed to 3.8 km, so any crossing ability at that level guarantees arrival in this model. A crossing ability above about 10 km would have made arrival almost immediate from 120 ka.
3. **The dated record cannot say whether they could.** Crossing ability barely contracts (0.20 with the 12-site register), and the record hardly distinguishes the strait model from walking only. The question has to be answered by independent evidence of Pleistocene seafaring or swimming capability. It cannot be answered by fitting sites. Until then, results should be reported conditional on crossing ability, as in the table above.
4. **Adding the strait makes the register less informative about *whether* people arrived** (0.41, against 0.86 when walking only was assumed). That is because arrival now depends on a quantity the sites barely constrain. The earlier, sharper number came from an assumption, not from the evidence.
5. **Timing stays out of reach.** With the register, the onset interval is about as wide as the prior (52 against 53 ka). Only a dense, well-dated record narrows it much.
6. **The mechanism is not identifiable from arrival evidence.** Growth and diffusion stay near the prior even with exact onsets.

## Limits

- **The register is an AI-compiled draft.** It needs expert review (see [`research/evidence/README.md`](../research/evidence/README.md)). These results use the register's *design*; its actual ages have not been fitted, which is G4.
- **The strait uses modern bathymetry and global sea level:**
  - no tectonic, sediment or glacio-isostatic correction;
  - no regional relative sea level;
  - 1-arcminute resolution.
- **Crossing is a hard threshold.** Legs up to `crossing_km` are crossed at the ordinary movement rate, and longer ones not at all. There is no crossing mortality and no probability that falls off with distance.
- **Still fixed:**
  - rainfall is the only habitat input;
  - no freshwater data;
  - today's land elsewhere;
  - arrival recorded from 120 ka onward;
  - the source is south of 15°N;
  - the gross birth rate is 0.045 per year, an assumption.
- **Grid convergence is not yet tested.** The time-step check was run before the strait was added; the growth step it tests is unchanged.
- **Rejection ABC is a conservative estimator.**
