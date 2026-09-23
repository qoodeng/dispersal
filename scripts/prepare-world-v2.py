"""Preserve measured/reconstructed inputs without inventing habitat or ancient coastlines."""
from pathlib import Path
import hashlib,json
import numpy as np
import xarray as xr
R=Path(__file__).resolve().parents[1];out=R/'research/world-v2';out.mkdir(exist_ok=True)
p=R/'research/data/beyer-regional-precipitation.nc'
d=xr.open_dataset(p,decode_times=False)
assert d.time.attrs['units']=='years since 1950-01-01 00:00:00.0'
assert np.all(np.diff(d.time.values)>0)
lat=d.latitude.values.astype(float);lon=d.longitude.values.astype(float)
assert np.allclose(np.diff(lat),.5) and np.allclose(np.diff(lon),.5)
rain=d.bio12.values.astype(np.float32)
assert rain.shape==(41,90,90)
# Exact spherical strip areas; earth-radius convention is an engineering geodesy choice.
radius=6371.0088
area=radius**2*np.deg2rad(.5)*(np.sin(np.deg2rad(lat+.25))-np.sin(np.deg2rad(lat-.25)))
areas=np.broadcast_to(area[:,None],(90,90)).copy()
assert np.all(areas>0)
np.savez_compressed(out/'regional-climate.npz',years_bp=-d.time.values,latitude=lat,longitude=lon,cell_area_km2=areas,precipitation_mm_year=rain,valid_precipitation=np.isfinite(rain))
manifest={'schema':'dispersal-world/2','status':'environmental input only; no population prediction','sources':[{'id':'climate2020','doi':'10.1038/s41597-020-0552-1','data_url':'https://zenodo.org/records/7062281','source_file':str(p.relative_to(R)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'license':'CC BY4.0','kind':'climate reconstruction, not direct observation'}],'shape':[41,90,90],'time':{'unit':'years before1950','range_bp':[120000,40000],'cadence_years':2000,'extrapolation':'forbidden','interpolation':'continuous fields only; both endpoint samples must exist'},'space':{'source_resolution_degrees':.5,'coordinate_order':'south to north; west to east','area_method':'spherical exact strip area','earth_radius_km':radius},'missing_data':'unknown; never zero precipitation or automatically uninhabitable','not_supplied':['fine-scale climate','seasonal climate','freshwater','food conversion','terrain','ancient shorelines','ice history','population parameters'],'coverage_limitation':'does not cover planned140–120ka interval'}
manifest['output_sha256']=hashlib.sha256((out/'regional-climate.npz').read_bytes()).hexdigest()
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
class Climate:
 def __init__(self):self.t=-d.time.values
 def at(self,bp):
  if not np.isfinite(bp) or not self.t[-1]<=bp<=self.t[0]:raise ValueError('outside reconstruction coverage')
  exact=np.where(self.t==bp)[0]
  if len(exact):return rain[exact[0]].copy()
  k=int(np.searchsorted(-self.t,-bp))-1;fraction=(self.t[k]-bp)/(self.t[k]-self.t[k+1]);a=rain[k];b=rain[k+1]
  return np.where(np.isfinite(a)&np.isfinite(b),a*(1-fraction)+b*fraction,np.nan)
c=Climate();assert np.allclose(c.at(120000),rain[0],equal_nan=True)
assert np.allclose(c.at(119000),(rain[0]+rain[1])/2,equal_nan=True)
for bp in [140000,39999,np.nan]:
 try:c.at(bp)
 except ValueError:pass
 else:raise AssertionError('Extrapolation accepted')
(out/'checks.json').write_text(json.dumps({'status':'passed','checks':['41original time slices retained','Native0.5degree resolution retained','Spherical areas positive','Exact-time and halfway interpolation checked','Out-of-coverage and nonfinite time rejected','Missing endpoint remains unknown'],'historical_claims_validated':False},indent=2)+'\n')
print(json.dumps({'shape':rain.shape,'missing_fraction':float((~np.isfinite(rain)).mean()),'output_bytes':(out/'regional-climate.npz').stat().st_size}))
