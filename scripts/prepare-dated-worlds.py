"""Build immutable, source-traceable dated windows for actual population runs.
No palaeohydrology, food conversion or ancient demographic calibration is implied.
"""
from pathlib import Path
import json, hashlib, sys
import numpy as np
import pandas as pd
import xarray as xr
from scipy.ndimage import binary_propagation
from scipy.spatial import Delaunay,cKDTree
from sklearn.linear_model import LinearRegression
R=Path(__file__).resolve().parents[1]
O=R/'public/spatial/dated';O.mkdir(parents=True,exist_ok=True)
source=Path(sys.argv[1] if len(sys.argv)>1 else '/tmp/Beyer2020_annual.nc')
cl=xr.open_dataset(source,decode_times=False)
e=xr.open_dataset(R/'research/world-v2/etopo2022-regional.nc').sortby('lat').sortby('lon');z=e.z.values
base=json.loads((R/'public/spatial/world.json').read_text());fit=json.loads((R/'public/spatial/ecology-fit.json').read_text())
d=pd.read_csv(R/'research/ecology/calibration-observations.csv');P=np.log(d.crr.to_numpy());T=d.cmat.to_numpy()/10
X=np.c_[P,T,T*T];Y=np.log(d.density_per_km2);hull=Delaunay(np.c_[P,T]);coef=np.array(fit['density_model']['coefficients'])
blocks=(np.floor((d.latitude+90)/20)*18+np.floor((d.longitude+180)/20)).to_numpy();unique=np.unique(blocks);rng=np.random.default_rng(20260918);boots=[]
for _ in range(500):
 ids=np.concatenate([np.flatnonzero(blocks==b) for b in rng.choice(unique,len(unique))]);m=LinearRegression().fit(X[ids],Y.iloc[ids]);boots.append(np.r_[m.intercept_,m.coef_])
boots=np.array(boots)
sea=pd.read_csv(R/'research/calibration/data/spratt2016-published-table.csv')
places=[('aqaba','Gulf of Aqaba',28.5,34.7),('sinai','Suez & Sinai',29.9,32.5),('nile','Nile valley',25.7,32.65),('levant','Southern Levant',32.0,35.3),('horn','Horn of Africa',11.5,42.5),('mandeb','Bab el-Mandeb',12.7,43.3)]
epochs=[0,120000,100000,80000,60000,40000]
manifest={'schema':'dated-environment-windows/1','places':[{'id':i,'name':n,'center':[a,b]} for i,n,a,b in places],'epochs':epochs,'files':[], 'sources':[
 {'id':'climate','doi':'10.1038/s41597-020-0552-1','url':'https://zenodo.org/records/7062281','file':source.name,'sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'variables':['bio01 degrees C','bio12 mm/year'],'native_resolution':'0.5 degrees'},
 {'id':'elevation','doi':'10.25921/fd45-gt74','sha256':hashlib.sha256((R/'research/world-v2/etopo2022-regional.nc').read_bytes()).hexdigest(),'native_resolution':'60 arcseconds, EGM2008'},
 {'id':'sea-level','doi':'10.5194/cp-12-1079-2016','sha256':hashlib.sha256((R/'research/calibration/data/spratt2016-published-table.csv').read_bytes()).hexdigest(),'column':'median, published stack; age_ka in thousands of years before present'},
 {'id':'density','url':'/spatial/ecology-fit.json','observations':'/spatial/calibration-observations.csv'}],
 'methods':{'shoreline':'Four-connected marine flood fill on native regional ETOPO grid, seeded from three deep marine cells (Mediterranean 34N25E, Red Sea 20N38E, Arabian Sea 15N58E). Inland below-sea depressions are not automatically ocean. Missing elevation blocks connectivity. Uniform global median sea level applied to modern EGM2008 relief is an exploratory scenario, not local relative sea-level reconstruction.','climate':'Exact dated native climate cells; nearest containing cell, no spatial downscaling. Convex-hull support of modern density observations enforced.','density':'Existing coefficients and identical 500 block bootstraps. Integrate reference density over known dry cell area; sites require >=80% supported area.','sites':'Seed1709 irregular numerical sites, minimum3km separation, 550 maximum; not archaeological camps.','experiment':'Environment frozen at selected date during each100-year modern-analogue demographic/movement experiment. No claim that groups or routes reconstruct past occupation.'},
 'limits':['No local tectonics or glacial-isostatic adjustment','Modern topography not palaeotopography','No dated freshwater or river crossing model','No food stocks or famine response','No independent ancient transfer validation','No arrival-date prediction','No evolving environment within a run']}
N=240;cell=.5;v=-60+(np.arange(N)+.5)*cell;xx,yy=np.meshgrid(v,v);rho=np.hypot(xx,yy);c=rho/6371.0088
for bp in epochs:
 level=0. if bp==0 else float(np.interp(bp/1000,sea.age_ka,sea['median']));wet=np.isfinite(z)&(z<=level);seed=np.zeros_like(wet)
 for slat,slon in [(34,25),(20,38),(15,58)]:
  sy=int(np.argmin(abs(e.lat.values-slat)));sx=int(np.argmin(abs(e.lon.values-slon)));assert wet[sy,sx];seed[sy,sx]=True
 ocean=binary_propagation(seed,mask=wet)
 dated=cl.sel(time=-bp)
 for key,name,la,lo in places:
  a,b=np.deg2rad([la,lo]);lat=np.rad2deg(np.arcsin(np.cos(c)*np.sin(a)+yy*np.sin(c)*np.cos(a)/rho));lon=np.rad2deg(b+np.arctan2(xx*np.sin(c),rho*np.cos(a)*np.cos(c)-yy*np.sin(a)*np.sin(c)))
  iy=np.floor((lat-float(e.lat[0]))/(float(e.lat[1])-float(e.lat[0]))+.5).astype(int);ix=np.floor((lon-float(e.lon[0]))/(float(e.lon[1])-float(e.lon[0]))+.5).astype(int)
  elev=z[iy,ix].flatten();land=(np.isfinite(elev)&~ocean[iy,ix].flatten())
  sample=dated.sel(latitude=xr.DataArray(lat,dims=['y','x']),longitude=xr.DataArray(lon,dims=['y','x']),method='nearest');temp=sample.bio01.values.flatten();rain=sample.bio12.values.flatten()
  finite=land&np.isfinite(temp)&np.isfinite(rain)&(rain>0);points=np.c_[np.log(np.maximum(rain,1e-20)),temp/10];valid=finite.copy();valid[finite]=hull.find_simplex(points[finite])>=0
  features=np.c_[np.ones(len(rain)),points,points[:,1]**2];med=np.full(len(rain),np.nan);low=med.copy();high=med.copy()
  for vals in np.unique(features[valid],axis=0):
   mask=valid&np.all(features==vals,axis=1);spread=np.sqrt(np.var(boots@vals)+fit['density']['blocked_cv_log_rmse']**2);mu=vals@coef;med[mask]=np.exp(mu);low[mask]=np.exp(mu-1.644854*spread);high[mask]=np.exp(mu+1.644854*spread)
  rngsite=np.random.default_rng(1709);sites=[]
  for _ in range(10000):
   p=rngsite.uniform(-58,58,2);qx,qy=((p+60)/cell).astype(int)
   if not land[qy*N+qx]:continue
   if all(np.linalg.norm(p-np.array(q))>=3 for q in sites):sites.append(p.tolist())
   if len(sites)>=550:break
  if not sites:continue
  _,near=cKDTree(sites).query(np.c_[xx.flatten(),yy.flatten()]);area=np.bincount(near[land],minlength=len(sites))*cell**2;known=np.bincount(near[valid],minlength=len(sites))*cell**2;supported=(known>=.8*area)&(known>0)
  refs=[np.bincount(near[valid],weights=a[valid]*cell**2,minlength=len(sites)) for a in [med,low,high]]
  def nullable(a):return [round(float(x),6) if np.isfinite(x) else None for x in a]
  world={k:base[k] for k in ['width','cellKm','extent','fertility','female','male']};world.update({'center':[la,lo],'elevation':nullable(elev),'land':land.astype(int).tolist(),'sites':sites,'rainfall':nullable(rain),'temperature':nullable(temp),'environment':{'name':name,'place':key,'yearsBP':bp,'seaLevelM':level,'sourceManifest':'/spatial/dated/manifest.json','staticDuringRun':True}})
  world['ecology']={**base['ecology'],'referencePeople':[float(x) if ok else None for x,ok in zip(refs[0],supported)],'referenceLow':[float(x) if ok else None for x,ok in zip(refs[1],supported)],'referenceHigh':[float(x) if ok else None for x,ok in zip(refs[2],supported)],'areaKm2':area.tolist(),'knownAreaKm2':known.tolist(),'densityRaster':nullable(med),'climateEpoch':f'{bp} years BP; modern analogue transferred without ancient validation'}
  path=O/f'{key}-{bp}.json';path.write_text(json.dumps(world,separators=(',',':'),allow_nan=False));manifest['files'].append({'file':path.name,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'landKm2':float(land.sum()*cell**2),'supportedKm2':float(valid.sum()*cell**2),'supportedSites':int(supported.sum()),'seaLevelM':level})
  print(path.name,int(supported.sum()),flush=True)
(O/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
