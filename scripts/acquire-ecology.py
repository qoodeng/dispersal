"""Reacquire pinned source bytes and reproduce the native baseline climate patch."""
from pathlib import Path
import requests,hashlib,json,xarray as xr,tempfile
R=Path(__file__).resolve().parents[1];O=R/'research/ecology';manifest=json.loads((O/'acquisition.json').read_text())
for record in manifest['artifacts']:
 p=O/record['file']
 if p.exists() and hashlib.sha256(p.read_bytes()).hexdigest()==record['sha256']:continue
 if record['file']!='modern-climate-patch.nc':
  r=requests.get(record['url'],timeout=60);r.raise_for_status();assert hashlib.sha256(r.content).hexdigest()==record['sha256'];p.write_bytes(r.content)
 else:
  with tempfile.TemporaryDirectory() as tmp:
   source=Path(tmp)/'annual.nc';r=requests.get(record['source_url'],stream=True,timeout=60);r.raise_for_status()
   with source.open('wb') as f:
    for block in r.iter_content(2**20):f.write(block)
   assert hashlib.sha256(source.read_bytes()).hexdigest()==record['source_sha256']
   d=xr.open_dataset(source,decode_times=False)[['bio01','bio12','npp']].sel(time=0,latitude=slice(27.5,29.5),longitude=slice(33.5,35.5));d.to_netcdf(p,engine='h5netcdf')
   # NetCDF metadata encoding may vary by library version; numerical source selection is exact.
print('Pinned inputs available; provenance in research/ecology/acquisition.json')
