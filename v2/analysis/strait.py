"""Bab el-Mandeb open-water gap through time, from bathymetry and sea level.

For each climate snapshot (120-40 ka), land is ETOPO 2022 elevation above the
Spratt & Lisiecki (2016) global sea level at that age (median, and the 2.5%
and 97.5% quantiles). The reported gap is the minimax leg: over all
island-hopping routes from the African to the Arabian mainland, the smallest
possible value of the longest single open-water leg. Output: v2/data/strait.json.

Limits: modern bathymetry (no correction for tectonics, sediment or glacio-
isostatic adjustment); global rather than regional relative sea level; the
EGM2008 geoid is taken as present mean sea level.
"""

import csv
import hashlib
import json
from pathlib import Path

import h5py
import numpy as np
from scipy import ndimage

ROOT = Path(__file__).resolve().parents[2]
ETOPO = ROOT / "research/world-v2/etopo2022-regional.nc"
ETOPO_META = ROOT / "research/world-v2/terrain-source.json"
SEA_LEVEL = ROOT / "research/calibration/data/spratt2016-published-table.csv"
GRID = ROOT / "v2/data/grid.json"
OUT = ROOT / "v2/data/strait.json"

BOX = {"lat": (11.6, 14.6), "lon": (41.6, 44.6)}
STRAIT_POINT = (12.6, 43.35)  # Bab el-Mandeb; the grid edge is the nearest Africa-Arabia neighbour pair
AFRICA_SEED = (12.2, 42.4)  # Djibouti/Eritrea mainland
ARABIA_SEED = (13.4, 44.2)  # Yemen mainland
KM_PER_ARCMIN = 1.8532


def load_bathymetry():
    meta = json.loads(ETOPO_META.read_text())
    digest = hashlib.sha256(ETOPO.read_bytes()).hexdigest()
    assert digest == meta["sha256"], "ETOPO checksum mismatch"
    with h5py.File(ETOPO) as f:
        lat, lon = f["lat"][:], f["lon"][:]
        i = np.where((lat >= BOX["lat"][0]) & (lat <= BOX["lat"][1]))[0]
        j = np.where((lon >= BOX["lon"][0]) & (lon <= BOX["lon"][1]))[0]
        z = f["z"][i[0] : i[-1] + 1, j[0] : j[-1] + 1].astype(float)
    return z, lat[i[0] : i[-1] + 1], lon[j[0] : j[-1] + 1], digest


def seed_index(lat, lon, point):
    return int(np.argmin(np.abs(lat - point[0]))), int(np.argmin(np.abs(lon - point[1])))


def minimax_leg_km(z, lat, lon, sea_level):
    """Smallest L such that the mainlands connect when open-water legs of up to L km are allowed."""
    land = z > sea_level
    a, b = seed_index(lat, lon, AFRICA_SEED), seed_index(lat, lon, ARABIA_SEED)
    assert land[a] and land[b], "seed points must be land"
    sampling = (KM_PER_ARCMIN, KM_PER_ARCMIN * np.cos(np.deg2rad(lat.mean())))
    to_land = ndimage.distance_transform_edt(~land, sampling=sampling)

    def connected(leg):
        passable = land | (to_land <= leg / 2)
        labels, _ = ndimage.label(passable, structure=np.ones((3, 3)))
        return labels[a] == labels[b]

    if connected(0.0):
        return 0.0
    lo, hi = 0.0, 80.0
    assert connected(hi), "no route within 80 km"
    while hi - lo > 0.1:
        mid = (lo + hi) / 2
        lo, hi = (lo, mid) if connected(mid) else (mid, hi)
    return round(hi, 1)


def grid_edge(grid):
    """The Africa and Arabia cells for the strait link: land in every snapshot, south of 20N,
    minimising their summed distance to the strait (they need not be neighbours)."""
    w = grid["width"]
    lat, lon = np.array(grid["latitudes"]), np.array(grid["longitudes"])
    always_land = np.array(grid["landAreaKm2"]).min(axis=0) > 0

    def candidates(region):
        for c, r in enumerate(grid["regions"]):
            la, lo = lat[c // w], lon[c % w]
            if r == region and always_land[c] and la < 20:
                yield np.hypot(la - STRAIT_POINT[0], lo - STRAIT_POINT[1]), la, lo

    a = min(candidates("africa"))
    b = min(candidates("arabia"))
    return {"africa": {"lat": float(a[1]), "lon": float(a[2])}, "arabia": {"lat": float(b[1]), "lon": float(b[2])}}


def sea_levels():
    rows = {int(float(r["age_ka"])): r for r in csv.DictReader(SEA_LEVEL.open())}
    return lambda ka: {k: float(rows[ka][k]) for k in ("q025", "median", "q975")}


def main():
    import argparse

    ap = argparse.ArgumentParser()
    ap.add_argument("--grid", type=Path, default=GRID)
    ap.add_argument("--out", type=Path, default=OUT)
    args = ap.parse_args()
    z, lat, lon, digest = load_bathymetry()
    grid = json.loads(args.grid.read_text())
    level = sea_levels()
    snapshots = []
    for bp in grid["snapshotsYearsBP"]:
        sl = level(bp // 1000)
        # Lower sea level (q025) exposes more land and gives the narrowest gap.
        snapshots.append(
            {
                "yearsBP": bp,
                "seaLevelM": sl,
                "gapKm": {
                    "narrow": minimax_leg_km(z, lat, lon, sl["q025"]),
                    "median": minimax_leg_km(z, lat, lon, sl["median"]),
                    "wide": minimax_leg_km(z, lat, lon, sl["q975"]),
                },
            }
        )
    present = minimax_leg_km(z, lat, lon, 0.0)
    sill = next(s for s in np.arange(0, -300, -1.0) if minimax_leg_km(z, lat, lon, s) == 0.0)
    cells = grid_edge(grid)
    out = {
        "schema": "dispersal-v2-strait/1",
        "strait": "Bab el-Mandeb",
        "gridEdge": cells,
        "bathymetry": {"source": str(ETOPO.relative_to(ROOT)), "sha256": digest, "doi": "10.25921/fd45-gt74"},
        "seaLevel": {"source": str(SEA_LEVEL.relative_to(ROOT)), "reference": "Spratt & Lisiecki 2016"},
        "metric": "minimax open-water leg between the African and Arabian mainlands, km",
        "presentDayGapKm": present,
        "landBridgeBelowSeaLevelM": float(sill),
        "snapshots": snapshots,
        "limits": [
            "Modern bathymetry: no tectonic, sediment or glacio-isostatic correction.",
            "Global mean sea level, not regional relative sea level.",
            "1-arcminute resolution cannot resolve channels narrower than about 2 km.",
        ],
    }
    args.out.write_text(json.dumps(out, indent=1) + "\n")
    gaps = [s["gapKm"]["median"] for s in snapshots]
    print(f"present-day gap {present} km; land bridge needs sea level below {sill} m")
    print(f"gap 120-40 ka (median sea level): min {min(gaps)} km, max {max(gaps)} km")
    for s in snapshots[::4]:
        print(f"  {s['yearsBP'] / 1000:>5.0f} ka  sea {s['seaLevelM']['median']:>7.1f} m  gap {s['gapKm']}")


if __name__ == "__main__":
    main()
