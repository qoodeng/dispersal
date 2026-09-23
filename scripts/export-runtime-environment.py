"""Lossless native-grid export for the Rust reader; no habitat classification or downscaling."""
from pathlib import Path
import hashlib,json,struct
import numpy as np
import xarray as xr
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'research/world-v2/runtime';OUT.mkdir(exist_ok=True)
def export(name,t,lat,lon,values,source,units):
 t=np.asarray(t,dtype='<f8');lat=np.asarray(lat,dtype='<f8');lon=np.asarray(lon,dtype='<f8')
 values=np.asarray(values,dtype='<f4')
 assert values.shape==(len(t),len(lat),len(lon))
 assert np.all(np.diff(t)>0) and np.all(np.diff(lat)>0) and np.all(np.diff(lon)>0)
 assert not np.isinf(values).any()
 blob=b'DSPGRID1'+struct.pack('<III',len(t),len(lat),len(lon))+t.tobytes()+lat.tobytes()+lon.tobytes()+values.tobytes()
 p=OUT/name;p.write_bytes(blob)
 return {'file':str(p.relative_to(ROOT)),'sha256':hashlib.sha256(blob).hexdigest(),'source':source,'units':units,'shape':values.shape,'bytes':len(blob)}
c=np.load(ROOT/'research/world-v2/regional-climate.npz')
records=[export('precipitation.bin',c['years_bp'][::-1],c['latitude'],c['longitude'],c['precipitation_mm_year'][::-1],'climate2020','mm/year')]
d=xr.open_dataset(ROOT/'research/world-v2/etopo2022-regional.nc').sortby('lat').sortby('lon')
records.append(export('elevation.bin',[0.],d.lat,d.lon,d.z.values[None,:,:],'etopo2022','meters above EGM2008; modern relief'))
manifest={'schema':'dispersal-runtime-grid/1','artifacts':records,'format':'DSPGRID1, little endian u32 nt/ny/nx, f64 ascending time/latitude/longitude centers, f32 C-order values [time,latitude,longitude]; NaN unknown','spatial_lookup':'containing native cell; half-spacing exterior boundaries; no spatial smoothing','time_lookup':'linear between valid bracketing values; no extrapolation; elevation has static time 0','limits':['Elevation is not a palaeoland mask','Rainfall is not food or drinking water availability','No reconstructed climatic detail below native resolution']}
(OUT.parent/'runtime-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps(records))
