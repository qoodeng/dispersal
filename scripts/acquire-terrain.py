"""Acquire a regional subset of NOAA ETOPO2022 with original EGM2008 elevations.
This is modern relief, not a reconstructed ancient shoreline. Network required.
"""
from pathlib import Path
import json,hashlib
import xarray as xr
import numpy as np
R=Path(__file__).resolve().parents[1];out=R/'research/world-v2'
u='https://www.ngdc.noaa.gov/thredds/dodsC/global/ETOPO2022/60s/60s_surface_elev_netcdf/ETOPO_2022_v1_60s_N90W180_surface.nc'
d=xr.open_dataset(u,engine='pydap');print('Remote dataset opened',flush=True)
lat=d.lat.values;lon=d.lon.values
la=np.flatnonzero((lat>=-5)&(lat<=40));lo=np.flatnonzero((lon>=20)&(lon<=65))
x=d[['z']].isel(lat=slice(int(la[0]),int(la[-1])+1),lon=slice(int(lo[0]),int(lo[-1])+1)).load()
assert x.z.shape==(2700,2700) and str(x.z.attrs.get('units'))=='meters'
assert np.isfinite(x.z.values).all()
x.to_netcdf(out/'etopo2022-regional.nc',engine='h5netcdf',encoding={'z':{'zlib':True,'complevel':4}})
p=out/'etopo2022-regional.nc'
m={'source_id':'etopo2022','doi':'10.25921/fd45-gt74','url':u,'accessed':'2026-09-18','product':'ETOPO2022 v1 60arcsecond surface','vertical_reference':x.z.attrs.get('vert_crs_name'),'vertical_epsg':x.z.attrs.get('vert_crs_epsg'),'units':'meters positive up','bounds':[20,-5,65,40],'shape':list(x.z.shape),'native_resolution_degrees':1/60,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size,'use':'modern topography/bathymetry input; not ancient coastline','pending':['regional sea-level and vertical-datum treatment','palaeoshoreline uncertainty','subgrid pass connectivity'],'source_attributes':dict(x.z.attrs)}
(out/'terrain-source.json').write_text(json.dumps(m,indent=2,default=lambda v:v.tolist() if hasattr(v,'tolist') else str(v))+'\n');print(json.dumps({k:m[k] for k in ['shape','bytes','vertical_reference','sha256']}))
