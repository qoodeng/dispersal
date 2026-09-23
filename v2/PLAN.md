# Dispersal v2 plan

Status: proposed 23 September 2026. Gate G0 has a first result (see [README](README.md#g0-result)).

## Relationship to v1

- **v1 is frozen at commit `16ad2ae`**, where M1 engineering acceptance passed. It stays in this repository unchanged, apart from bug fixes. Its engine (`engine/`) becomes the Tier 1 mechanism lab, and its recorded M1 evidence is the reference that ported code must reproduce.
- [`docs/DELIVERY-PLAN.md`](../docs/DELIVERY-PLAN.md) still holds for requirements, provenance rules and the numerical gates in its section 10.
- This plan changes three things:
  - **the order of work:** test identifiability before building more mechanisms;
  - **the architecture:** two tiers, native batch runs, and a browser that only views results;
  - **the headline question.**

## The question

> Under reconstructed climate between 120 and 40 ka, when, and for how long, could Arabia and the Levant sustain populations spreading from Africa? And which of those histories can the dated archaeological record tell apart?

- **Arrival dates are a secondary output.** They are reported only for quantities that gate G0 shows the evidence can recover.
- "Could the corridor sustain people, and when?" is the primary result. It still stands even if dates cannot be recovered.

## Architecture

| Part | What it is | Where |
|---|---|---|
| Tier 2: coarse engine | A stochastic model of linked populations on a grid, run natively in large batches, and fitted to evidence | `v2/coarse` (Rust) |
| Tier 1: mechanism lab | The v1 M1 engine: foraging, food ledger, groups. It supplies relationships and priors to Tier 2, such as growth against condition, density against productivity, effective diffusion and crossing probabilities | `engine/` |
| Analysis | Data preparation, evidence designs, inference and reports | `v2/analysis` (Python, managed with uv) |
| Viewer | Plays back exported run packages. No science runs in the browser | later (G6) |

**Rule:** a mechanism moves from Tier 1 into Tier 2 only when a controlled comparison shows it changes an answer compared with the Tier 2 baseline.

## Gates, in order

Each gate produces a committed artifact, a test and a pass criterion.

| Gate | Work | Passes when |
|---|---|---|
| **G0 Identifiability** | Simulation-based calibration of the coarse model under several evidence designs (this deliverable) | Coverage is near nominal, which shows the inference is honest. The report says which quantities contract under which design |
| **G1 Evidence** | Ingest the record properly in the DELIVERY-PLAN evidence schema: sites, samples, errors, attribution, dependency groups. Replace the illustrative design with the real one and rerun G0 | Every observation traces to a source. G0 has been rerun on the real design |
| **G2 Tier 2 numerics** | Timestep and grid convergence. Replace the Bab el-Mandeb latitude rule with bathymetry and sea-level treatment of straits | The section 10 targets are met for onset and occupation outputs |
| **G3 Structure** | Alternative source regions, a southern crossing, freshwater scenarios, suitability forms. Can the record discriminate between them? | Model-comparison calibration is reported, including "cannot tell" |
| **G4 Inference** | Fit the real record with SMC-ABC or neural posterior estimation, with frozen holdouts | Holdout scores beat simple baselines, or failure is reported |
| **G5 Tier 1 → 2** | Use the M1 engine to derive priors only for the mechanisms G3 shows matter | Every transferred prior is documented along with its transfer uncertainty |
| **G6 Viewer** | Run packages and a playback viewer | The viewer shows exported runs only |

## Working rules

These are written for agent-driven development, where no human reviews the code.

1. **Before any push,** `./check.sh` must pass. It runs formatting, lint, tests and a short end-to-end study. CI runs the same script.
2. **Generated tables go in `v2/runtime/`,** which is untracked. Committed results record digests of their inputs, and `analysis/study.py analyze` refuses a table whose parameters, sites, grid or engine have changed.
3. **Results are rewritten in place,** in README.md. Superseded results move to `results/history/` with a note, never silently.
4. **A new mechanism needs a test** showing it changes an answer. Every new parameter needs a prior with a stated source or a named assumption.
5. **Leave v1 alone,** except for bug fixes.
6. **Work tracking:** AGENTS.md makes Beads authoritative. Mirror these gates into Beads (epic `disp-bi6`) where `bd` is available. It was not installed in the session that wrote this plan.
