"""Identifiability study: can a dated archaeological record recover the coarse
model's parameters and arrival times?

Method (simulation-based calibration with rejection ABC):
  1. Draw N parameter sets from the prior and simulate each once (Rust engine).
  2. For each evidence design, turn every simulation into the record that
     design would observe: the oldest dated find at each site cell, with
     Gaussian dating error; "no find in 120-40 ka" is its own outcome.
  3. Hold out T simulations as known truths. For each, run rejection ABC
     against the other N-T simulations, then ask whether the posterior
     (a) contracts relative to the prior and (b) covers the truth at the
     nominal rate. Good coverage with no contraction means the evidence is
     honest but uninformative; poor coverage means the inference is broken.

Usage:
  uv run python analysis/study.py prepare --sims 12000
  uv run python analysis/study.py simulate            # runs the Rust engine
  uv run python analysis/study.py analyze
  uv run python analysis/study.py timestep --sims 200  # dt 25 vs 10 check
"""

import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

import numpy as np

V2 = Path(__file__).resolve().parents[1]
GRID = V2 / "data/grid.json"
DESIGNS = V2 / "analysis/designs.json"
WORK = V2 / os.environ.get("DISPERSAL_STUDY_WORK", "runtime/study")  # generated tables; untracked
RESULTS = V2 / "results"
ENGINE = V2 / "coarse/target/release/dispersal-coarse"

# name: (kind, low, high, units, description)
PRIORS = {
    "growth": ("log_uniform", 0.001, 0.03, "per year", "intrinsic growth rate r"),
    "diffusion": ("log_uniform", 2.0, 60.0, "km^2/year", "diffusion coefficient D"),
    "rain_half": ("uniform", 50.0, 250.0, "mm/year", "precipitation at half habitat suitability"),
    "density": ("log_uniform", 1.0, 30.0, "people/100 km^2", "carrying capacity in suitable habitat"),
    "detection": ("log_uniform", 1e-9, 1e-5, "finds/person-year", "dated-find deposition and recovery rate"),
}
NO_EVIDENCE_KA = 30.0  # encoding for "nothing in 120-40 ka"; 10 ka beyond the window edge
NEVER_KA = 30.0
T_TESTS = 250
MAX_PER_SITE_SUMMARIES = 12  # above this, observations are reduced to summary statistics
ACCEPT_FRACTION = 0.03  # nearest 3%: wider-than-ideal posteriors, i.e. conservative contraction


def to_unit(name, x):
    kind, lo, hi = PRIORS[name][:3]
    return (np.log(x) - np.log(lo)) / (np.log(hi) - np.log(lo)) if kind == "log_uniform" else (x - lo) / (hi - lo)


def from_unit(name, u):
    kind, lo, hi = PRIORS[name][:3]
    return np.exp(np.log(lo) + u * (np.log(hi) - np.log(lo))) if kind == "log_uniform" else lo + u * (hi - lo)


def load_grid():
    g = json.loads(GRID.read_text())
    ever_land = np.array(g["landAreaKm2"]).max(axis=0) > 0
    always_land = np.array(g["landAreaKm2"]).min(axis=0) > 0
    return g, ever_land, always_land


def cell_of(g, lat, lon):
    row = int(np.argmin(np.abs(np.array(g["latitudes"]) - lat)))
    col = int(np.argmin(np.abs(np.array(g["longitudes"]) - lon)))
    return row, col


def resolve_designs():
    """Map every design site to a land cell; coastal sites snap to the nearest land cell."""
    g, ever_land, always_land = load_grid()
    w = g["width"]
    spec = json.loads(DESIGNS.read_text())["designs"]
    resolved = {}
    for name, d in spec.items():
        if "same_sites_as" in d:
            continue
        sites = []
        if "random_sites" in d:
            r = d["random_sites"]
            candidates = [c for c in range(len(g["regions"])) if g["regions"][c] in r["regions"] and always_land[c]]
            picks = np.random.default_rng(r["seed"]).choice(candidates, size=r["count"], replace=False)
            sites = [{"name": f"random_{c}", "cell": int(c), "region": g["regions"][c]} for c in sorted(picks)]
        for s in d.get("sites", []):
            row, col = cell_of(g, s["lat"], s["lon"])
            own = row * w + col
            best, best_km = None, None
            for dr in range(-2, 3):
                for dc in range(-2, 3):
                    rr, cc = row + dr, col + dc
                    if 0 <= rr < g["height"] and 0 <= cc < w and ever_land[rr * w + cc]:
                        km = 111.2 * np.hypot(
                            g["latitudes"][rr] - s["lat"],
                            (g["longitudes"][cc] - s["lon"]) * np.cos(np.deg2rad(s["lat"])),
                        )
                        if best_km is None or km < best_km:
                            best, best_km = rr * w + cc, km
            if best is None:
                sys.exit(f"site {s['name']} has no land cell within two cells")
            merged = next((x for x in sites if x["cell"] == best), None)
            if merged:  # one cell has one record; keep the first name and note the merge
                merged["name"] += "+" + s["name"]
                continue
            sites.append(
                {
                    "name": s["name"],
                    "cell": int(best),
                    "ownCellLand": bool(ever_land[own]),
                    "kmFromCellCentre": round(float(best_km), 1),
                    "region": g["regions"][best],
                }
            )
        resolved[name] = {"sigma": d["sigma"], "sites": sites}
    for name, d in spec.items():
        if "same_sites_as" in d:
            resolved[name] = {"sigma": d["sigma"], "sites": resolved[d["same_sites_as"]]["sites"]}
    return g, resolved


def prepare(args):
    g, designs = resolve_designs()
    WORK.mkdir(parents=True, exist_ok=True)
    rng = np.random.default_rng(args.seed)
    u = rng.uniform(size=(args.sims, len(PRIORS)))
    lines = ["id,seed," + ",".join(PRIORS)]
    for i in range(args.sims):
        values = [from_unit(n, u[i, j]) for j, n in enumerate(PRIORS)]
        lines.append(f"{i},{int(rng.integers(1, 2**63))}," + ",".join(f"{v:.10g}" for v in values))
    (WORK / "params.csv").write_text("\n".join(lines) + "\n")
    cells = sorted({s["cell"] for d in designs.values() for s in d["sites"]})
    lat, lon = np.array(g["latitudes"]), np.array(g["longitudes"])
    (WORK / "sites.csv").write_text(
        "name,lat,lon\n" + "".join(f"c{c},{lat[c // g['width']]},{lon[c % g['width']]}\n" for c in cells)
    )
    (WORK / "designs.resolved.json").write_text(json.dumps(designs, indent=1))
    print(
        f"{args.sims} prior draws, {len(cells)} site cells; designs: "
        + ", ".join(f"{k}={len(v['sites'])}" for k, v in designs.items())
    )
    for s in designs["illustrative"]["sites"]:
        if not s["ownCellLand"]:
            print(f"  note: {s['name']} is coastal; its own cell is sea, snapped to the nearest land cell")


def simulate(args):
    cmd = [
        str(ENGINE),
        "--grid",
        str(GRID),
        "--params",
        str(WORK / "params.csv"),
        "--sites",
        str(WORK / "sites.csv"),
        "--out",
        str(WORK / args.out),
        "--dt",
        str(args.dt),
    ]
    if args.southern_crossing:
        cmd.append("--southern-crossing")
    subprocess.run(cmd, check=True)
    (WORK / f"{args.out}.meta.json").write_text(
        json.dumps(inputs_digest(args.dt, args.southern_crossing), indent=1) + "\n"
    )


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inputs_digest(dt, southern_crossing):
    return {
        "params": sha(WORK / "params.csv"),
        "sites": sha(WORK / "sites.csv"),
        "grid": sha(GRID),
        "engine": sha(ENGINE),
        "dt": dt,
        "southernCrossing": southern_crossing,
    }


def read_table(path):
    header = path.read_text().splitlines()[0].split(",")
    data = np.genfromtxt(path, delimiter=",", skip_header=1, filling_values=np.nan)
    return {h: data[:, i] for i, h in enumerate(header)}


def observations(table, design, rng):
    """Observed record under a design: one column per site, in ka."""
    cols = []
    for s in design["sites"]:
        kind = design["sigma"]["kind"]
        if kind == "exact_established":
            age = table[f"c{s['cell']}__established_bp"]
        else:
            age = table[f"c{s['cell']}__oldest_find_bp"]
            sigma = design["sigma"]["value"] * (age if kind == "fraction_of_age" else 1.0)
            age = age + rng.normal(size=age.shape) * np.nan_to_num(sigma)
        cols.append(np.where(np.isfinite(age), age / 1000.0, NO_EVIDENCE_KA))
    S = np.column_stack(cols)
    if S.shape[1] <= MAX_PER_SITE_SUMMARIES:
        return S
    # Many sites: nearest-neighbour ABC degrades with dimension, so reduce the
    # record to the number of sites with evidence, age quantiles across sites,
    # and the oldest age per region.
    regions = np.array([s["region"] for s in design["sites"]])
    reduced = [(S > NO_EVIDENCE_KA).sum(axis=1).astype(float)]
    reduced += list(np.quantile(S, [1.0, 0.9, 0.75, 0.5], axis=1))
    reduced += [S[:, regions == r].max(axis=1) for r in sorted(set(regions))]
    return np.column_stack(reduced)


def weighted_quantile(x, w, q):
    order = np.argsort(x)
    cw = np.cumsum(w[order])
    return np.interp(np.asarray(q) * cw[-1], cw, x[order])


def abc(S, targets, tests):
    """Rejection ABC with Epanechnikov weights, leave-one-out over `tests`."""
    ref = np.setdiff1d(np.arange(len(S)), tests)
    scale = np.maximum(np.median(np.abs(S[ref] - np.median(S[ref], axis=0)), axis=0) * 1.4826, 1.0)
    k = max(20, int(ACCEPT_FRACTION * len(ref)))
    out = {
        name: {
            "covered50": [],
            "covered90": [],
            "post_var": [],
            "width90": [],
            "truth": [],
            "median": [],
            "p_reached": [],
            "median_reached": [],
            "width90_reached": [],
            "covered90_reached": [],
        }
        for name in targets
    }
    for t in tests:
        d = np.sqrt((((S[ref] - S[t]) / scale) ** 2).sum(axis=1))
        idx = np.argpartition(d, k)[:k]
        h = d[idx].max() * 1.0000001 + 1e-12
        w = 1.0 - (d[idx] / h) ** 2
        for name, values in targets.items():
            x, truth = values[ref][idx], values[t]
            lo50, hi50, lo90, hi90, med = weighted_quantile(x, w, [0.25, 0.75, 0.05, 0.95, 0.5])
            mean = np.average(x, weights=w)
            r = out[name]
            r["covered50"].append(lo50 <= truth <= hi50)
            r["covered90"].append(lo90 <= truth <= hi90)
            r["post_var"].append(np.average((x - mean) ** 2, weights=w))
            r["width90"].append(hi90 - lo90)
            r["truth"].append(truth)
            r["median"].append(med)
            reached = x > NEVER_KA
            r["p_reached"].append(float(np.average(reached, weights=w)))
            if name.endswith("_onset_ka") and reached.sum() >= 3:
                lo, hi, m = weighted_quantile(x[reached], w[reached], [0.05, 0.95, 0.5])
                r["median_reached"].append(m)
                r["width90_reached"].append(hi - lo)
                r["covered90_reached"].append(lo <= truth <= hi)
            else:
                r["median_reached"].append(np.nan)
                r["width90_reached"].append(np.nan)
                r["covered90_reached"].append(False)
    return ref, out


def summarize(targets, ref, out, prior_width90):
    rows = {}
    for name, r in out.items():
        prior_var = np.var(targets[name][ref])
        rows[name] = {
            "contraction": round(float(1 - np.mean(r["post_var"]) / prior_var), 3) if prior_var > 0 else None,
            "coverage50": round(float(np.mean(r["covered50"])), 3),
            "coverage90": round(float(np.mean(r["covered90"])), 3),
            "medianWidth90": round(float(np.median(r["width90"])), 3),
            "priorWidth90": round(float(prior_width90[name]), 3),
        }
        if name.endswith("_onset_ka"):
            truth = np.array(r["truth"])
            reached = truth > NEVER_KA
            p = np.array(r["p_reached"])
            base = targets[name][ref] > NEVER_KA
            brier_prior = np.mean((base.mean() - reached) ** 2)
            rows[name]["whetherReached"] = {
                "brierSkill": round(float(1 - np.mean((p - reached) ** 2) / brier_prior), 3)
                if brier_prior > 0
                else None,
                "priorProbability": round(float(base.mean()), 3),
            }
            if reached.sum() >= 5:
                # Posterior restricted to accepted runs that also reached: "if they arrived, when?"
                mc = np.array(r["median_reached"])[reached]
                rows[name]["whenReached"] = {
                    "tests": int(reached.sum()),
                    "medianAbsErrorKa": round(float(np.nanmedian(np.abs(mc - truth[reached]))), 2),
                    "medianWidth90Ka": round(float(np.nanmedian(np.array(r["width90_reached"])[reached])), 2),
                    "coverage90": round(float(np.mean(np.array(r["covered90_reached"])[reached])), 3),
                    "priorWidth90Ka": round(
                        float(np.subtract(*np.quantile(targets[name][ref][base], [0.95, 0.05]))), 2
                    ),
                    "priorMedianAbsErrorKa": round(
                        float(np.median(np.abs(np.median(targets[name][ref][base]) - truth[reached]))), 2
                    ),
                }
    return rows


def analyze(args):
    designs = json.loads((WORK / "designs.resolved.json").read_text())
    meta = json.loads((WORK / f"{args.table}.meta.json").read_text())
    current = inputs_digest(meta["dt"], meta["southernCrossing"])
    stale = [k for k in ("params", "sites", "grid", "engine") if meta[k] != current[k]]
    if stale:
        sys.exit(f"{args.table} is stale: {', '.join(stale)} changed since it was simulated; rerun simulate")
    table = read_table(WORK / args.table)
    if args.subset:
        table = {k: v[: args.subset] for k, v in table.items()}
    params = read_table(WORK / "params.csv")
    n = len(table["id"])
    assert np.array_equal(table["id"], params["id"][:n])
    ids = table["id"].astype(int)
    targets = {name: to_unit(name, params[name][ids]) for name in PRIORS}
    targets["arabia_onset_ka"] = np.nan_to_num(table["arabia_onset_bp"] / 1000.0, nan=NEVER_KA)
    targets["levant_onset_ka"] = np.nan_to_num(table["levant_onset_bp"] / 1000.0, nan=NEVER_KA)
    targets["arabia_occupied_fraction"] = table["arabia_occupied_fraction"]
    tests = np.arange(min(T_TESTS, n // 10))
    prior_width90 = {k: np.subtract(*np.quantile(v, [0.95, 0.05])) for k, v in targets.items()}

    reached = np.isfinite(table["arabia_onset_bp"])
    report = {
        "schema": "dispersal-v2-identifiability/1",
        "inputs": meta,
        "simulations": int(n),
        "tests": int(len(tests)),
        "acceptFraction": ACCEPT_FRACTION,
        "table": args.table,
        "subset": args.subset,
        "priors": {
            k: {"kind": v[0], "low": v[1], "high": v[2], "units": v[3], "meaning": v[4]} for k, v in PRIORS.items()
        },
        "encoding": {
            "noEvidenceKa": NO_EVIDENCE_KA,
            "neverKa": NEVER_KA,
            "parameters": "contraction and interval width are in prior-unit space (0-1)",
            "onsets": "ka; runs that never establish in the region are coded as 30 ka",
        },
        "priorPredictive": {
            "fractionReachingArabia": round(float(reached.mean()), 3),
            "arabiaOnsetKaQuantiles": [
                round(float(q), 1)
                for q in np.quantile(table["arabia_onset_bp"][reached] / 1000, [0.05, 0.25, 0.5, 0.75, 0.95])
            ]
            if reached.any()
            else None,
            "fractionReachingLevant": round(float(np.isfinite(table["levant_onset_bp"]).mean()), 3),
        },
        "designs": {},
    }
    rng = np.random.default_rng(args.seed)
    for name, design in designs.items():
        S = observations(table, design, rng)
        has_evidence = (S > NO_EVIDENCE_KA).any(axis=1)
        ref, out = abc(S, targets, tests)
        report["designs"][name] = {
            "sites": len(design["sites"]),
            "fractionOfRunsWithAnyEvidence": round(float(has_evidence.mean()), 3),
            "results": summarize(targets, ref, out, prior_width90),
        }
    out_dir = V2 / args.report_dir
    out_dir.mkdir(parents=True, exist_ok=True)
    path = out_dir / args.report
    path.write_text(json.dumps(report, indent=1) + "\n")
    print_report(report)
    print(f"\nwrote {path.relative_to(V2)}")


def print_report(report):
    pp = report["priorPredictive"]
    print(
        f"{report['simulations']} simulations, {report['tests']} held-out truths; "
        f"prior: {pp['fractionReachingArabia']:.0%} reach Arabia, onset quantiles {pp['arabiaOnsetKaQuantiles']} ka"
    )
    keys = list(next(iter(report["designs"].values()))["results"])
    print(f"\n{'quantity':26}" + "".join(f"{d:>22}" for d in report["designs"]))
    print(f"{'':26}" + "".join(f"{'contr  cov90  w90':>22}" for _ in report["designs"]))
    for k in keys:
        cells = []
        for d in report["designs"].values():
            r = d["results"][k]
            cells.append(f"{r['contraction']:>6.2f} {r['coverage90']:>6.2f} {r['medianWidth90']:>7.2f}")
        print(f"{k:26}" + "".join(f"{c:>22}" for c in cells))
    print(
        "\nprior 90% widths: "
        + ", ".join(f"{k} {v['priorWidth90']}" for k, v in next(iter(report["designs"].values()))["results"].items())
    )
    for q in ("arabia_onset_ka", "levant_onset_ka"):
        print(
            f"\n{q}: whether reached by 40 ka (Brier skill; 0 = no better than prior) / "
            "when reached (median abs error, ka; median 90% width, ka; coverage)"
        )
        for d, v in report["designs"].items():
            r = v["results"][q]
            wr, wn = r["whetherReached"], r.get("whenReached")
            when = (
                (
                    f"error {wn['medianAbsErrorKa']:>5} (prior {wn['priorMedianAbsErrorKa']}), "
                    f"width {wn['medianWidth90Ka']:>5} (prior {wn['priorWidth90Ka']}), cov {wn['coverage90']}"
                )
                if wn
                else "n/a"
            )
            print(f"  {d:14} skill {wr['brierSkill']:>6}   {when}")


def timestep(args):
    """Is the dt 25 vs 10 year difference larger than seed-to-seed noise at dt 25?"""
    params = (WORK / "params.csv").read_text().splitlines()[: args.sims + 1]
    reseeded = [params[0]] + [
        ",".join([r.split(",")[0], str(int(r.split(",")[1]) + 1)] + r.split(",")[2:]) for r in params[1:]
    ]
    runs = {"dt25": (params, 25), "dt25_reseeded": (reseeded, 25), "dt10": (params, 10)}
    tables = {}
    for name, (rows, dt) in runs.items():
        src, out = WORK / f"timestep-{name}.params.csv", WORK / f"timestep-{name}.csv"
        src.write_text("\n".join(rows) + "\n")
        subprocess.run(
            [
                str(ENGINE),
                "--grid",
                str(GRID),
                "--params",
                str(src),
                "--sites",
                str(WORK / "sites.csv"),
                "--out",
                str(out),
                "--dt",
                str(dt),
            ],
            check=True,
        )
        tables[name] = read_table(out)
    result = {}
    for q in ("arabia_onset_bp", "levant_onset_bp"):
        r = {}
        for name, t in tables.items():
            x = t[q] / 1000
            reached = np.isfinite(x)
            r[name] = {
                "fractionReached": round(float(reached.mean()), 3),
                "onsetKaQuantiles10_50_90": [round(float(v), 1) for v in np.quantile(x[reached], [0.1, 0.5, 0.9])],
            }
        for a, b in (("dt25", "dt25_reseeded"), ("dt25", "dt10")):
            x, y = tables[a][q], tables[b][q]
            r[f"{a}_vs_{b}"] = {
                "reachedAgreement": round(float((np.isfinite(x) == np.isfinite(y)).mean()), 3),
                "medianAbsOnsetDifferenceKa": round(float(np.nanmedian(np.abs(x - y)) / 1000), 2),
            }
        result[q] = r
    verdict = (
        "fail"
        if abs(
            result["arabia_onset_bp"]["dt25"]["fractionReached"] - result["arabia_onset_bp"]["dt10"]["fractionReached"]
        )
        > 0.05
        else "pass"
    )
    report = {
        "sims": args.sims,
        "question": "dt 25 vs 10 years, compared with seed-to-seed noise at dt 25",
        "criterion": "fraction of runs establishing in Arabia differs by at most 0.05 between dt 25 and dt 10",
        "verdict": verdict,
        "results": result,
    }
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "timestep-check.json").write_text(json.dumps(report, indent=1) + "\n")
    print(json.dumps(report, indent=1))


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("prepare")
    a.add_argument("--sims", type=int, default=12000)
    a.add_argument("--seed", type=int, default=20260923)
    a = sub.add_parser("simulate")
    a.add_argument("--dt", type=float, default=25)
    a.add_argument("--out", default="table.csv")
    a.add_argument("--southern-crossing", action="store_true")
    a = sub.add_parser("analyze")
    a.add_argument("--table", default="table.csv")
    a.add_argument("--report", default="identifiability.json")
    a.add_argument("--report-dir", default="results")
    a.add_argument("--subset", type=int, default=0, help="analyze only the first N simulations (stability check)")
    a.add_argument("--seed", type=int, default=7)
    a = sub.add_parser("timestep")
    a.add_argument("--sims", type=int, default=200)
    args = p.parse_args()
    {"prepare": prepare, "simulate": simulate, "analyze": analyze, "timestep": timestep}[args.cmd](args)


if __name__ == "__main__":
    main()
