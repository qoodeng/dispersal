"""Reproducible conditional calibration, not an independently validated reconstruction.
Modern !Kung intrinsic growth target; Al Wusta minimum-age compatibility weighting.
Movement and transfer priors are explicit project assumptions. No exact arrival fit.
"""
import json, pathlib, hashlib
import numpy as np
import xarray as xr
from scipy.optimize import brentq
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import dijkstra
from scipy.special import ndtr
R=pathlib.Path(__file__).resolve().parents[1];rng=np.random.default_rng(4095)
g=json.loads((R/'public/geography.json').read_text());n=len(g['cells']);land=np.array([c[0] for c in g['cells']],bool)
xy=np.array([[20+(i%90+.5)*.5,40-(i//90+.5)*.5] for i in range(n)])
rain=xr.open_dataset(R/'research/data/beyer-regional-precipitation.nc',decode_times=False).bio12.values[:,::-1,:].reshape(41,n)
valid=land&np.isfinite(rain).all(axis=0)
# Missing cells excluded; never silently set to zero rainfall.
avg=np.zeros(n);avg[valid]=np.mean(rain[:21,valid],axis=0) # 120–80 ka
edges=np.array([e for e in g['edges'] if valid[e[0]] and valid[e[1]]]);a=edges[:,0].astype(int);b=edges[:,1].astype(int);km=edges[:,2]
def cell(lon,lat):return int(np.argmin(np.where(valid,((xy[:,0]-lon)*np.cos(lat*np.pi/180))**2+(xy[:,1]-lat)**2,np.inf)))
source=cell(32.25,15.25);aw=cell(39.4,27.4);lev=cell(35.25,31.25);east=cell(50,25)
# Matched population mortality; modern fertility shape is a declared approximation.
age=np.arange(81);lx=np.exp(-.340/.913*(1-np.exp(-.913*age))-.010*age+.000331/.077*(1-np.exp(.077*age)))
f=np.genfromtxt(R/'research/calibration/data/published-sahul-demographic-diagnostic.csv',delimiter=',',names=True)['female_fertility_per_year'];shape=f/f.sum()
def logr(tfr):
 A=np.zeros((81,81));A[0]=shape*tfr/2;A[np.arange(1,81),np.arange(80)]=lx[1:]/lx[:-1]
 return np.log(np.linalg.eigvals(A).real.max())
tfr=brentq(lambda x:logr(x)-.0026,1,12)
N=1024
r=np.clip(.0026*np.exp(rng.normal(0,.8,N)),.0001,.03)
D=np.exp(rng.uniform(np.log(.01),np.log(100),N));alpha=rng.uniform(1,6,N);start=rng.uniform(100,120,N)
arr=np.empty((N,n),dtype=np.float32)
for j in range(N):
 penalty=1+alpha[j]/(1+(avg[a]+avg[b])/200)
 cost=km*penalty
 graph=csr_matrix((np.r_[cost,cost],(np.r_[a,b],np.r_[b,a])),shape=(n,n))
 distance=dijkstra(graph,indices=source)
 arr[j]=start[j]-distance/(2*np.sqrt(r[j]*D[j]))/1000
# Conservative one-sided compatibility with measured minimum fossil age.
# 87.6 ±2.5 ka at 2 sigma -> measurement sigma 1.25 ka; not fossil arrival equality.
w=ndtr((arr[:,aw]-87.6)/1.25);w/=w.sum();ess=1/(w@w)
def quant(v,weights=w):
 finite=np.isfinite(v);v=v[finite];ww=weights[finite];order=np.argsort(v);v=v[order];ww=ww[order];return np.interp([.025,.5,.975],np.cumsum(ww)/ww.sum(),v).round(4).tolist()
sites=[]
for name,i,status in [('Al Wusta',aw,'Used for one-sided minimum-age calibration'),('Southern Levant',lev,'Unvalidated regional prediction'),('Eastern Arabia',east,'Unvalidated regional prediction')]:
 sites.append({'name':name,'cell':i,'status':status,'arrivalKa':quant(arr[:,i]),'priorArrivalKa':quant(arr[:,i],np.ones(N)/N),'reachedBy40Ka':round(float(w@(arr[:,i]>=40)),4)})
# Weighted posterior draw playback; preserve stochastic uncertainty, do not average into a path.
chosen=rng.choice(N,size=64,replace=True,p=w);members=[]
for j in chosen:
 members.append({'growth':float(r[j]),'diffusion':float(D[j]),'resistance':float(alpha[j]),'startKa':float(start[j]),'arrivalKa':[round(float(v),3) if np.isfinite(v) else None for v in arr[j]]})
frames=[]
for t in np.arange(120,39,-2):
 probs=w@(arr>=t);frames.append({'ka':int(t),'probability':np.rint(probs*255).astype(int).tolist()})
result={'schema':'dispersal-calibration/v1','seed':4095,'draws':N,'effectiveSampleSize':round(float(ess),1),'source':source,'growthFit':{'targetAnnualLogRate':.0026,'fittedAnnualLogRate':float(logr(tfr)),'fittedTotalFertility':float(tfr),'targetSource':'Tallavaara & Jørgensen 2021 Table1, !Kung intrinsic rate (Howell)','mortalitySource':'Gurven & Kaplan 2007 Table2, !Kung','fertilityShape':'Modern global age shape from Sahul archive; approximation'},'parameterIntervals':{'growth':quant(r),'diffusion':quant(D),'resistance':quant(alpha),'startKa':quant(start)},'sites':sites,'frames':frames,'members':members,'valid':valid.astype(int).tolist(),'rainMean':np.rint(avg).astype(int).tolist(),'assumptions':['Modern land geometry; no boats; no palaeoshoreline reconstruction.','Rainfall averaged 120–80 ka; static route cost 1 + alpha/(1+P/100). Not a calibrated physiological response.','KPP front approximation c=2√(rD); no extinction or recolonization.','Source Upper Nile; departure prior uniform100–120ka; movement loguniform0.01–100km²/year.','Growth transfer prior lognormal centered0.0026/year logSD0.8, clipped0.0001–0.03.','Al Wusta weight Phi((arrivalKa−87.6)/1.25) is a conservative compatibility likelihood for its minimum age.','Arrival intervals are conditional on this model and priors; not independently validated confidence in actual human arrival.','Population density shown as fraction of a local capacity, not an estimated census.'], 'sources':[{'title':'Modern growth target','url':'https://doi.org/10.1098/rstb.2019.0708'},{'title':'Mortality schedules','url':'https://doi.org/10.1111/j.1728-4457.2007.00171.x'},{'title':'Al Wusta chronology','url':'https://doi.org/10.1038/s41559-018-0518-2'},{'title':'Climate reconstruction','url':'https://doi.org/10.1038/s41597-020-0552-1'}]}
assert abs(logr(tfr)-.0026)<1e-9
assert np.isclose(w.sum(),1) and np.isfinite(ess)
assert all(frames[k]['probability'][source]<=frames[k+1]['probability'][source] for k in range(40))
assert all(frames[-1]['probability'][i]==0 for i in np.where(~valid)[0])
(R/'public/calibration.json').write_text(json.dumps(result,separators=(',',':'),allow_nan=False))
summary={k:v for k,v in result.items() if k not in ['members','frames','valid','rainMean']};summary['checks']=['Growth target recovered to1e-9','Weights normalized','Missing/ocean cells excluded','Deterministic seed4095','Chronological probability monotonicity at source'];summary['inputsSha256']={str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [R/'public/geography.json',R/'research/data/beyer-regional-precipitation.nc']}
(R/'research/calibration/fit-results.json').write_text(json.dumps(summary,indent=2));print(json.dumps(summary,indent=2))
