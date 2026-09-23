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
uv run python analysis/study.py timestep --sims 200
```

**Requirements:** Rust stable and [uv](https://docs.astral.sh/uv/). `uv` installs numpy and h5py from `uv.lock`.

**Where outputs go:**
- Generated tables go to `v2/runtime/`, which is untracked.
- `analyze` refuses a table if its parameters, sites, grid or engine binary have changed since it was simulated.

## What is here

| Path | Contents |
|---|---|
| `coarse/` | The Tier 2 engine, written in Rust. Integer counts per 1° cell. Ricker–Poisson growth to a carrying capacity set by rainfall. Binomial moves to the four neighbouring cells (the grid form of diffusion). A Poisson process that deposits dated finds. Tests check exact conservation during movement, no crossing of sea or the closed strait, repeatability from a seed, extinction in unsuitable habitat, the fast sampler's moments, and invasion speed against Fisher–KPP |
| `analysis/prepare_grid.py` | Builds `data/grid.json` from the checksummed Beyer 2020 rainfall subset in `research/data/`: 1°, 41 snapshots, 120–40 ka |
| `analysis/designs.json` | Evidence designs: where dated evidence could come from, and how precise its dates are |
| `analysis/study.py` | The G0 study: prior draws, simulation, leave-one-out calibration with ABC, and the time-step check |
| `results/` | Committed results, each carrying the digests of its inputs |

## G0 result

**The question:** if this model were true, could a dated archaeological record recover its parameters, and when (or whether) people became established in Arabia and the Levant? This is simulation-based calibration. 3,000 prior draws are made. 250 of them are hidden as known truths, and each truth is recovered by rejection ABC against the rest (nearest 3%). A 1,500-run subset reaches the same conclusions ([`results/identifiability-subset1500.json`](results/identifiability-subset1500.json)). Full numbers are in [`results/identifiability.json`](results/identifiability.json).

**Prior predictive:** 62% of draws establish a population in Arabia by 40 ka. Their onsets have a median of 113 ka, with a 5–95% range of 77–119 ka. Most runs that arrive do so early, because 120–110 ka is humid.

**The designs compared:**
- **Ingested:** the only two records in the evidence file. Al Wusta and Alathar fall in the same cell, so this is one location.
- **Illustrative:** 10 cells at approximate locations of well-known Levantine and Arabian sites. None of these has been ingested.
- **Dense:** 40 random cells dated to ±2 ka. This is an optimistic upper bound.
- **Oracle:** the exact time each illustrative cell became established, with no archaeology at all.

| Design | Whether Arabia was reached by 40 ka (Brier skill; 0 = prior, 1 = perfect) | If reached, width of the 90% interval on onset (prior: 41 ka) | Coverage of 90% intervals |
|---|---|---|---|
| Ingested (1 location) | 0.20 | 64 ka | 0.89 |
| Illustrative (10 sites) | 0.72 | 27 ka | 0.88 |
| Dense (40 sites, ±2 ka) | 0.85 | 18 ka | 0.92 |
| Oracle (exact onsets) | 1.00 | 4 ka | 0.93 |

**Contraction of the model parameters** (0 = no better than the prior, 1 = pinned down):

| Parameter | Ingested | Illustrative | Dense | Oracle |
|---|---|---|---|---|
| growth rate r | 0.10 | 0.18 | 0.21 | 0.38 |
| diffusion D | −0.14 | 0.03 | 0.11 | 0.24 |
| rainfall threshold | 0.15 | 0.36 | 0.32 | 0.59 |
| density | 0.07 | 0.41 | 0.33 | 0.27 |
| detection rate | 0.30 | 0.35 | 0.43 | 0.08 |

### What it means

1. **The inference is honest.** 90% intervals cover the truth 83–94% of the time in every design, which is at or near the nominal rate. Where the evidence is weak, the posteriors stay wide rather than becoming falsely precise.
2. **The record currently ingested says very little.** One location gives a Brier skill of 0.2 on whether Arabia was reached, and no useful arrival date. Its "if reached" interval is wider than the prior: when that single site shows nothing, the matching runs are mostly late arrivals, and those are spread widely.
3. **More sites help a lot, and they help most with *whether* people arrived.** With ten sites the model largely determines whether Arabia was occupied. It dates arrival only to within about 27 ka.
4. **Archaeology, not the model, sets the limit on dating.** Exact onsets would date arrival to ±2 ka. Real evidence is the oldest find that happens to be deposited and survive, and that comes thousands of years after arrival. Better dating of each find narrows this gap, but it doesn't close it.
5. **Even perfect onsets don't reveal the mechanism.** Growth and diffusion barely contract, even for the oracle, because many combinations of speed and growth produce the same arrival history. **Adding mechanisms to the model therefore cannot be justified by fitting arrival dates.** Mechanisms need their own evidence (demographic, ecological or ethnographic), which is the Tier 1 role in the plan.

### Limits of this result

- **The time step is not converged (gate G2 fails).** See [`results/timestep-check.json`](results/timestep-check.json). At a 25-year step, about 64% of runs establish in Arabia; at 10 years, about 45% do. That gap is far larger than the difference between two sets of seeds. Median onset times agree to within about 1 ka. The G0 comparison is internally consistent, because the same model generates and fits the synthetic records, so its conclusions stand. Absolute statements, such as how likely an arrival is, must wait until G2 passes.
- **Structural choices are fixed, not explored:**
  - the source is Africa south of 15°N;
  - movement is walking only, and the Bab el-Mandeb strait is closed by a latitude rule;
  - rainfall is the only habitat input, with no freshwater;
  - land is today's land, apart from the climate source's own mask;
  - arrival times are only recorded from 120 ka onward.
  Testing these is gate G3.
- **The illustrative site locations were typed from memory** and are not evidence. Gate G1 replaces them with the real, ingested record.
- **Rejection ABC loses power on long lists of sites.** The dense design is therefore summarised into a few statistics. Better estimators, such as neural posterior estimation, would probably extract more from every design, so these contraction figures are conservative.
