"""Bounded modern-topography integration fixture. No reconstructed food or coastline."""
from pathlib import Path
import numpy as np, xarray as xr, json, csv, hashlib
R=Path(__file__).resolve().parents[1]
d=xr.open_dataset(R/'research/world-v2/etopo2022-regional.nc').sortby('lat').sortby('lon')
# Exact spherical AEQD inverse, same as Rust LocalFrame; 120 km square.
n=240;cell=.5;v=-60+(np.arange(n)+.5)*cell
x,y=np.meshgrid(v,v);rho=np.hypot(x,y);c=rho/6371.0088
lat0=np.deg2rad(28.5);lon0=np.deg2rad(34.7)
lat=np.arcsin(np.cos(c)*np.sin(lat0)+y*np.sin(c)*np.cos(lat0)/rho)
lon=lon0+np.arctan2(x*np.sin(c),rho*np.cos(lat0)*np.cos(c)-y*np.sin(lat0)*np.sin(c))
z=d.z.sel(lat=xr.DataArray(np.rad2deg(lat),dims=['y','x']),lon=xr.DataArray(np.rad2deg(lon),dims=['y','x']),method='nearest').values
# Irregular candidate sites, numerical quadrature/decision sites, NOT archaeological camps.
rng=np.random.default_rng(1709);sites=[]
for _ in range(10000):
 p=rng.uniform(-58,58,2);ix,iy=((p+60)/cell).astype(int)
 if z[iy,ix]<=0 or not np.isfinite(z[iy,ix]):continue
 if all(np.linalg.norm(p-np.array(a))>=3.0 for a in sites):sites.append(p.tolist())
 if len(sites)>=550:break
rows=list(csv.DictReader((R/'research/demography/annual-reference.csv').open()))
data={'schema':'spatial-fixture/1','center':[28.5,34.7],'extent':120,'width':n,'cellKm':cell,'elevation':[round(float(a),2) if np.isfinite(a) else None for a in z.flatten()], 'sites':sites,'fertility':[float(r['birth_probability']) for r in rows], 'female':[int(r['initial_females']) for r in rows], 'male':[int(r['initial_males']) for r in rows]}
out=R/'public/spatial/world.json';out.write_text(json.dumps(data,separators=(',',':')))
meta={'source':'ETOPO2022 v1 60 arcsecond; DOI 10.25921/fd45-gt74','terrainSourceSha256':json.loads((R/'research/world-v2/terrain-source.json').read_text())['sha256'],'artifactSha256':hashlib.sha256(out.read_bytes()).hexdigest(),'frame':'spherical AEQD 28.5N 34.7E, 120 km square, <=85 km radius','resampling':'nearest native elevation to 0.5 km collision raster; source ~1.6–1.85 km. Resampling adds no topographic evidence.','sites':len(sites),'siteMethod':'uniform candidate points, seed1709, minimum3km spacing; numerical sites not known camps','landRule':'elevation>0 conservative modern experiment mask; not palaeoshoreline, freshwater or complete modern land classification','demography':'existing modern !Kung reference; initial age/sex counts deterministically subsampled 1 in10 and distributed among six groups','notReconstructed':['food supply','camp locations','relocation preferences','residential travel rate','travel provisions','sea level','freshwater']}
(R/'research/spatial/input-manifest.json').write_text(json.dumps(meta,indent=2)+'\n')
print(len(sites),out.stat().st_size)
