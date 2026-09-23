"""Reproduce the environmental screening experiment; never infers human presence.

Usage: python3 scripts/audit-climate.py /path/to/Beyer2020_annual_vars_v1.1.0.nc
Requires numpy, xarray, h5netcdf. Original dataset: CC BY 4.0.
"""
import hashlib
import json
import sys
from pathlib import Path

import numpy as np
import xarray as xr

ROOT = Path(__file__).resolve().parents[1]
path = Path(sys.argv[1])
expected = '9773c80fa406eebd71f84a48cf733056'
assert hashlib.md5(path.read_bytes()).hexdigest() == expected, 'Source checksum mismatch'
ds = xr.open_dataset(path, decode_times=False)
assert ds.time.attrs['units'] == 'years since 1950-01-01 00:00:00.0'
# Preserve native coordinates and snapshots: no interpolation or gap-filling.
region = ds[['bio12']].sel(latitude=slice(-5, 40), longitude=slice(20, 65))
region = region.sel(time=region.time[(region.time >= -120000) & (region.time <= -40000)])
rain = region.bio12.values
assert rain.shape == (41, 90, 90)
assert np.nanmin(rain) >= 0
# Exact spherical cell-area factor; radius cancels in the fraction.
lat = region.latitude.values.astype(float)
weights = np.broadcast_to((np.sin(np.deg2rad(lat + .25)) - np.sin(np.deg2rad(lat - .25)))[:, None], (90, 90))
thresholds = [90, 110, 130, 200]
# Hold denominator constant to avoid changing masks masquerading as climate change.
common = np.isfinite(rain).all(axis=0)
assert common.any()
rows = []
for year, field in zip(region.time.values, rain):
    fractions = [float(weights[common & (field >= t)].sum() / weights[common].sum()) for t in thresholds]
    assert all(a >= b for a, b in zip(fractions, fractions[1:])), 'Threshold monotonicity'
    rows.append({'yearsBP': int(-year), 'validCells': int(np.isfinite(field).sum()), 'fractions': fractions})
manifest = {
    'schema': 'dispersal-climate-screen/v1', 'scientificStatus': 'environmental_screen_only',
    'source': 'https://zenodo.org/records/7062281',
    'download': 'https://zenodo.org/api/records/7062281/files/Beyer2020_annual_vars_v1.1.0.nc/content',
    'paper': 'https://doi.org/10.1038/s41597-020-0552-1',
    'packagingDocumentation': 'https://evolecolgroup.github.io/pastclim/reference/Beyer2020.html',
    'license': 'CC-BY-4.0', 'sourceMD5': expected,
    'sourceSHA256': hashlib.sha256(path.read_bytes()).hexdigest(),
    'variable': 'bio12', 'units': 'mm/year',
    'unitsEvidence': 'Beyer 2020 Methods: monthly precipitation in mm/month and derived annual bioclimatic precipitation; file long_name Annual Precipitation has no units attribute',
    'timeConvention': 'BP relative to 1950; source uses negative years since 1950',
    'bounds': [20, -5, 65, 40], 'cellDegrees': .5,
    'thresholdsMm': thresholds, 'thresholdSource': 'https://doi.org/10.1038/s41467-021-24779-1',
    'commonValidCells': int(common.sum()), 'rows': rows,
    'interpretation': 'Area-weighted fraction of cells with data at ALL 41 snapshots whose reconstructed annual precipitation meets each threshold. Not fraction of ancient habitable land; not a migration probability or population prediction.',
    'limitations': ['Thresholds are sensitivity scenarios, not universal survival limits.', 'Masks exclude ice sheets and internal seas; missing values are not interpreted as uninhabitable land.', 'Common-support selection excludes changing land masks.', 'No freshwater, routes, sea crossings, population dynamics, seasonality or archaeological validation.', 'One climate reconstruction; threshold spread is not a confidence interval.'],
}
output = ROOT / 'research/data/beyer-regional-precipitation.nc'
region.to_netcdf(output, engine='h5netcdf', encoding={'bio12': {'zlib': True}})
manifest['subsetSHA256'] = hashlib.sha256(output.read_bytes()).hexdigest()
for dest in ['research/data/climate-screen.json', 'public/climate-screen.json']:
    (ROOT / dest).write_text(json.dumps(manifest, indent=2) + '\n')
print(json.dumps({'snapshots': len(rows), 'commonValidCells': int(common.sum()), 'first': rows[0], 'last': rows[-1]}, indent=2))
