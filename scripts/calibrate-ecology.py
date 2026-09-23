"""Fit auditable modern analogues. Never interpret density as edible food production."""
from pathlib import Path
import pandas as pd,numpy as np,json,hashlib
from sklearn.linear_model import LinearRegression
from sklearn.model_selection import GroupKFold
from scipy.spatial import Delaunay,cKDTree
import xarray as xr
R=Path(__file__).resolve().parents[1];O=R/'research/ecology';rng=np.random.default_rng(20260918)
raw=pd.read_csv(O/'LRB.csv')
# Original documented miles are used; derived kmov contains discrepant rows.
d=raw[(raw.subpop=='n')&(raw.fishing<40)&(raw.nomov>=1)&(raw.dismov>0)].copy()
d['density_per_km2']=d.density/100.;d['distance_km_year']=d.dismov*1.609344;d['km_per_move']=d.distance_km_year/d.nomov
cols=['seq339','name','year','ethref','latitude','longitude','wldsec','subpop','fishing','cmat','crr','density','density_per_km2','nomov','dismov','distance_km_year','km_per_move','tlpop']
d[cols].to_csv(O/'calibration-observations.csv',index=False)
P=np.log(d.crr.to_numpy());T=d.cmat.to_numpy()/10.;X=np.c_[P,T,T*T];y=np.log(d.density_per_km2.to_numpy());blocks=(np.floor((d.latitude+90)/20)*18+np.floor((d.longitude+180)/20)).to_numpy()
folds=list(GroupKFold(5).split(X,y,blocks));report={};predictions={}
for name,Y,features in [('density',y,X),('annual_distance',np.log(d.distance_km_year),X[:,:2]),('annual_moves',np.log(d.nomov),X[:,:2]),('distance_per_move',np.log(d.km_per_move),np.c_[P,1/(8.62e-5*(273.15+d.cmat))])]:
 Y=np.asarray(Y);pred=np.zeros(len(Y));base=pred.copy();fold_id=np.zeros(len(Y),int)
 for fold,(tr,te) in enumerate(folds):
  m=LinearRegression().fit(features[tr],Y[tr]);pred[te]=m.predict(features[te]);base[te]=Y[tr].mean();fold_id[te]=fold
 rmse=float(np.sqrt(np.mean((Y-pred)**2)));baseline=float(np.sqrt(np.mean((Y-base)**2)))
 report[name]={'n':len(d),'blocked_cv_log_rmse':rmse,'intercept_baseline_log_rmse':baseline,'accepted_climate_relationship':rmse<baseline,'cv':'5 folds, all members of each20degree geographic block held together; exploratory single fixed split, not independent external validation'};predictions[name]=pred
pd.DataFrame({'seq339':d.seq339,'fold':fold_id,'observed_log_density':y,'heldout_log_density':predictions['density'],'block':blocks}).to_csv(O/'heldout-predictions.csv',index=False)
m=LinearRegression().fit(X,y);coef=np.r_[m.intercept_,m.coef_];residual_sd=float(np.std(y-m.predict(X),ddof=4));unique=np.unique(blocks);boots=[]
for _ in range(500):
 chosen=rng.choice(unique,len(unique));ids=np.concatenate([np.flatnonzero(blocks==b) for b in chosen]);b=LinearRegression().fit(X[ids],y[ids]);boots.append(np.r_[b.intercept_,b.coef_])
boots=np.array(boots)
clim=xr.open_dataset(O/'modern-climate-patch.nc',decode_times=False);world=json.loads((R/'public/spatial/world.json').read_text());n=world['width'];cell=world['cellKm'];v=-60+(np.arange(n)+.5)*cell;xx,yy=np.meshgrid(v,v);rho=np.hypot(xx,yy);c=rho/6371.0088;la,lo=np.deg2rad(world['center']);lat=np.arcsin(np.cos(c)*np.sin(la)+yy*np.sin(c)*np.cos(la)/rho);lon=lo+np.arctan2(xx*np.sin(c),rho*np.cos(la)*np.cos(c)-yy*np.sin(la)*np.sin(c));cl=clim.sel(latitude=xr.DataArray(np.rad2deg(lat),dims=['y','x']),longitude=xr.DataArray(np.rad2deg(lon),dims=['y','x']),method='nearest');temperature=cl.bio01.values.flatten();rain=cl.bio12.values.flatten();land=np.array([z is not None and z>0 for z in world['elevation']]);finite=land&np.isfinite(temperature)&np.isfinite(rain)&(rain>0)
points=np.c_[np.log(np.maximum(rain,1e-20)),temperature/10];valid=finite.copy();valid[finite]=Delaunay(np.c_[P,T]).find_simplex(points[finite])>=0
features=np.c_[np.ones(len(rain)),points,points[:,1]**2];med=np.full(len(rain),np.nan);lo90=med.copy();hi90=med.copy()
# Spatially shared coefficient uncertainty + predictive residual width (not ancient uncertainty).
for vals in np.unique(features[valid],axis=0):
 mask=valid&np.all(features==vals,axis=1);p=boots@vals;mu=vals@coef;spread=np.sqrt(np.var(p)+report['density']['blocked_cv_log_rmse']**2);med[mask]=np.exp(mu);lo90[mask]=np.exp(mu-1.644854*spread);hi90[mask]=np.exp(mu+1.644854*spread)
_,nearest=cKDTree(np.array(world['sites'])).query(np.c_[xx.flatten(),yy.flatten()]);site_area=np.bincount(nearest[land],minlength=len(world['sites']))*cell**2;known_area=np.bincount(nearest[valid],minlength=len(world['sites']))*cell**2
reference=np.bincount(nearest[valid],weights=med[valid]*cell**2,minlength=len(world['sites']));referenceLo=np.bincount(nearest[valid],weights=lo90[valid]*cell**2,minlength=len(world['sites']));referenceHi=np.bincount(nearest[valid],weights=hi90[valid]*cell**2,minlength=len(world['sites']))
# Require most of a numerical site's area to have in-domain data; no zero-filling unknowns.
supported=known_area>=.8*site_area
world['ecology']={'mode':'modern analogue reference; no food stocks','referencePeople':[float(v) if ok else None for v,ok in zip(reference,supported)],'referenceLow':[float(v) if ok else None for v,ok in zip(referenceLo,supported)],'referenceHigh':[float(v) if ok else None for v,ok in zip(referenceHi,supported)],'areaKm2':site_area.tolist(),'knownAreaKm2':known_area.tolist(),'densityRaster':[float(v) if np.isfinite(v) else None for v in med],'mobilityPairs':d[['nomov','distance_km_year']].to_numpy().tolist(),'fit':{'n':len(d),'densityCoefficients':coef.tolist(),'blockedCvLogRmse':report['density']['blocked_cv_log_rmse'],'baselineLogRmse':report['density']['intercept_baseline_log_rmse']},'climateEpoch':'Beyer time0 baseline; modern terrain, no ancient date inference'}
(R/'public/spatial/world.json').write_text(json.dumps(world,separators=(',',':'),allow_nan=False))
report.update({'schema':'ecology-calibration/1','selection':'subpop n; fishing<40%; nomov>=1/year; positive original dismov. This is a terrestrial-mobile analogue subset, not a universal forager model.','density_units':'Original density people/100km², converted to people/km²; area original100km² units.','distance_units':'Original dismov miles/year multiplied by exact1.609344; derived kmov not used.','density_model':{'formula':'ln(people/km²)=b0+b1 ln(annual rainfall mm)+b2 T/10+b3 (T/10)^2','coefficients':coef.tolist(),'residual_sd':residual_sd,'uncertainty':'500 spatial-block coefficient bootstraps plus held-out log RMSE; displayed90% approximate predictive envelope, no prehistoric transfer uncertainty'},'mobility_choice':'Climate regressions failed blocked-CV baseline; rejected. Use observed joint annual moves/distance pairs from retained111 rows, not climate-predicted mobility. Pair is sampled once per group, no claim empirical route destinations.','coverage':{'land_km2':float(land.sum()*cell**2),'in_domain_km2':float(valid.sum()*cell**2),'supported_sites':int(supported.sum()),'reference_people_known_cells':float(np.nansum(med)*cell**2)},'limits':['Density is occupancy reference, not carrying capacity or food production','Weak cross-validated density skill; very wide predictive ranges','Shared cross-cultural/linguistic dependence not fully controlled','Climate source resolution0.5degree, not terrain resolution','Selection threshold fishing40 is an analyst choice requiring sensitivity','No annual food depletion/recovery or famine response calibrated','Movement scheduling and preference remain modeled assumptions']})
(O/'fit.json').write_text(json.dumps(report,indent=2,allow_nan=False)+'\n');print(json.dumps(report,indent=2))
