"""Derive source-resolution terrain summaries, preserving modern/ancient distinction."""
import json,hashlib,pathlib
import numpy as np
import xarray as xr
R=pathlib.Path(__file__).resolve().parents[1];out=R/'research/world-v2';d=xr.open_dataset(out/'etopo2022-regional.nc');z=d.z.values.astype(float);lat=d.lat.values;lon=d.lon.values
assert z.shape==(2700,2700)
dy=6371008.8*np.deg2rad(abs(lat[1]-lat[0]));dx=dy*np.cos(np.deg2rad(lat))[:,None]
gy,gx=np.gradient(z);slope=np.degrees(np.arctan(np.hypot(gy/dy,gx/dx)))
# Ten-arcminute? No:6native60arcsecond cells ->0.1degree summaries, explicitly geographic.
f=6
blocks=lambda a:a.reshape(450,f,450,f)
mean=lambda a:blocks(a).mean(axis=(1,3))
np.savez_compressed(out/'terrain-summaries.npz',latitude=lat.reshape(450,f).mean(axis=1),longitude=lon.reshape(450,f).mean(axis=1),mean_elevation_m=mean(z).astype('f4'),min_elevation_m=blocks(z).min(axis=(1,3)).astype('f4'),max_elevation_m=blocks(z).max(axis=(1,3)).astype('f4'),elevation_sd_m=blocks(z).std(axis=(1,3)).astype('f4'),mean_slope_degrees=mean(slope).astype('f4'),modern_above_datum_fraction=mean(z>=0).astype('f4'))
assert np.all((slope>=0)&(slope<90))
meta={'source':'etopo2022','source_sha256':hashlib.sha256((out/'etopo2022-regional.nc').read_bytes()).hexdigest(),'output_sha256':hashlib.sha256((out/'terrain-summaries.npz').read_bytes()).hexdigest(),'shape':[450,450],'native_shape':[2700,2700],'summary_resolution_degrees':.1,'coordinate_order':'preserved from ETOPO; inspect coordinates, do not assume south-up','methods':{'elevation':'block mean/min/max/population SD','slope':'central differences on native grid with latitude-dependent horizontal spacing; first-order outer boundaries','land_fraction':'fraction of samples at or above EGM2008 zero; modern relief descriptor only'},'limitations':['Geographic summaries, not the final equal-area simulation mesh','Zero elevation is not an ancient shoreline classification','Mean slope does not resolve pass connectivity; retain native relief','No population movement law inferred from slope','No sub-resolution detail invented']}
(out/'terrain-summary-methods.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps({'native_samples':int(z.size),'summary_cells':202500,'minimum_elevation_m':float(z.min()),'maximum_elevation_m':float(z.max())}))
