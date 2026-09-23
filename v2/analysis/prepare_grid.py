"""Build the 1-degree coarse grid for the v2 identifiability study.

Input: research/data/beyer-regional-precipitation.nc (Beyer et al. 2020 bio12,
0.5 degree, 41 snapshots 120-40 ka BP; provenance in research/data/climate-screen.json).
Output: v2/data/grid.json, consumed by the Rust coarse engine.

Aggregation: a 1-degree cell is land in a snapshot when at least 2 of its 4
0.5-degree subcells are land; its precipitation is the mean over land subcells;
its habitable area is the exact spherical area of the land subcells.
"""

import hashlib
import json
from pathlib import Path

import h5py
import numpy as np

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / "research/data/beyer-regional-precipitation.nc"
OUT = ROOT / "v2/data/grid.json"
EXPECTED_SHA256 = json.loads((ROOT / "research/data/climate-screen.json").read_text())["subsetSHA256"]
EARTH_RADIUS_KM = 6371.0088


def red_sea_lon(lat):
    """Longitude of a straight Red Sea axis from Suez (30N, 32.6E) to Bab el-Mandeb (12.5N, 43.4E)."""
    return 32.6 + (30.0 - lat) * (43.4 - 32.6) / (30.0 - 12.5)


def gulf_lat(lon):
    """Latitude of a straight Persian Gulf axis from Shatt al-Arab (30N, 48.5E) to Hormuz (26.5N, 56.3E)."""
    return 30.0 + (lon - 48.5) * (26.5 - 30.0) / (56.3 - 48.5)


def region(lat, lon):
    if lat < 12.5 or (lat <= 33.5 and lon < red_sea_lon(min(lat, 30.0))):
        return "africa"
    if 29.5 <= lat <= 37.0 and lon <= 38.5 and (lon >= 34.0 or lat <= 31.5):
        return "levant"
    arabian = (
        (lon < 48.5 and lat < 29.5) or (48.5 <= lon <= 56.3 and lat < gulf_lat(lon)) or (lon > 56.3 and lat < 26.0)
    )
    return "arabia" if arabian else "other_asia"


def main():
    import argparse

    ap = argparse.ArgumentParser()
    ap.add_argument("--cell-degrees", type=float, default=1.0, choices=[1.0, 0.5])
    ap.add_argument("--out", type=Path, default=OUT)
    args = ap.parse_args()
    out = args.out.resolve()
    raw = SRC.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    assert digest == EXPECTED_SHA256, f"input checksum mismatch: {digest}"
    with h5py.File(SRC) as f:
        rain = f["bio12"][:].astype(float)  # (time, lat, lon), mm/year, NaN = sea/ice
        years_bp = (-f["time"][:]).astype(int)
        lat = f["latitude"][:].astype(float)
        lon = f["longitude"][:].astype(float)
    assert rain.shape == (41, 90, 90) and years_bp[0] == 120000 and years_bp[-1] == 40000
    assert np.all(np.diff(lat) > 0) and np.all(np.diff(lon) > 0)

    # Exact spherical area of each 0.5-degree subcell, km^2.
    band = np.sin(np.deg2rad(lat + 0.25)) - np.sin(np.deg2rad(lat - 0.25))
    sub_area = EARTH_RADIUS_KM**2 * np.deg2rad(0.5) * band[:, None] * np.ones((1, 90))

    k = int(round(args.cell_degrees / 0.5))  # source cells per model cell along each axis
    ny, nx = 90 // k, 90 // k
    land_sub = np.isfinite(rain)

    def blocks(a):
        return a.reshape(a.shape[:-2] + (ny, k, nx, k))

    land_count = blocks(land_sub).sum(axis=(-3, -1))
    land = land_count >= max(1, k * k // 2)
    area = blocks(land_sub * sub_area[None]).sum(axis=(-3, -1))
    with np.errstate(invalid="ignore"):
        precip = blocks(np.nan_to_num(rain) * land_sub).sum(axis=(-3, -1)) / land_count
    precip = np.where(land, precip, np.nan)

    clat = lat.reshape(ny, k).mean(axis=1)
    clon = lon.reshape(nx, k).mean(axis=1)
    regions = [[region(clat[i], clon[j]) for j in range(nx)] for i in range(ny)]

    grid = {
        "schema": "dispersal-v2-grid/1",
        "source": str(SRC.relative_to(ROOT)),
        "sourceSha256": digest,
        "cellDegrees": args.cell_degrees,
        "width": nx,
        "height": ny,
        "latitudes": clat.tolist(),
        "longitudes": clon.tolist(),
        "snapshotsYearsBP": years_bp.tolist(),
        "regions": [r for row in regions for r in row],
        "landAreaKm2": [
            [round(float(v), 3) if is_land else 0.0 for v, is_land in zip(a.ravel(), m.ravel(), strict=True)]
            for a, m in zip(area, land, strict=True)
        ],
        "precipitationMm": [[None if not np.isfinite(v) else round(float(v), 3) for v in p.ravel()] for p in precip],
        "notes": [
            "Rows run south to north; cell index = row * width + column.",
            "Region labels use straight Red Sea and Gulf axes; they label summaries and the Bab el-Mandeb rule only.",
            "The 0.5-degree source mask joins Africa and Arabia at Bab el-Mandeb at low sea level; that is a "
            "resolution artefact, so the engine severs Africa-Asia edges south of 20N unless a crossing "
            "scenario is chosen.",
            "Modern-day rivers and freshwater are not represented; precipitation is the only environmental input.",
        ],
    }
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(grid, separators=(",", ":")) + "\n")

    code = {"africa": "F", "levant": "L", "arabia": "A", "other_asia": "o"}
    print(
        f"wrote {out.relative_to(ROOT)}; land cells per snapshot "
        f"min {land.sum((1, 2)).min()} max {land.sum((1, 2)).max()}"
    )
    snap = int(np.argmax(land.sum((1, 2))))
    print(f"region map at {years_bp[snap]} BP (north at top, '.' sea):")
    for i in reversed(range(ny)):
        print("".join(code[regions[i][j]] if land[snap, i, j] else "." for j in range(nx)))


if __name__ == "__main__":
    main()
