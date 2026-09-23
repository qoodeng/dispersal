"""Connected M1 geometry. Real ETOPO barriers; synthetic food and unknown water.
The 10 km display grid never decides route connectivity. Every graph and foraging
segment is sampled at <= 0.5 km on the native 60 arcsecond marine mask.
"""
from pathlib import Path
import json, hashlib
import numpy as np
import xarray as xr
from scipy.ndimage import binary_propagation
from scipy.spatial import cKDTree
from scipy.sparse import coo_matrix
from scipy.sparse.csgraph import connected_components
from pyproj import Transformer, CRS
R=Path(__file__).resolve().parents[1]
D=R/'public/spatial/connected';D.mkdir(exist_ok=True)
e=xr.open_dataset(R/'research/world-v2/etopo2022-regional.nc');z=e.z.values
sea=np.genfromtxt(R/'research/calibration/data/spratt2016-published-table.csv',delimiter=',',names=True)
level=float(np.interp(100.,sea['age_ka'],sea['median']))
wet=np.isfinite(z)&(z<=level);seeds=np.zeros_like(wet)
for lat,lon in [(34,25),(20,38),(15,58)]:seeds[np.argmin(abs(e.lat.values-lat)),np.argmin(abs(e.lon.values-lon))]=True
water=binary_propagation(seeds,mask=wet)
crs=CRS.from_proj4('+proj=aeqd +lat_0=24 +lon_0=39 +datum=WGS84 +units=km')
to_geo=Transformer.from_crs(crs,4326,always_xy=True);to_xy=Transformer.from_crs(4326,crs,always_xy=True)
def sample(points, indices=False):
 lon,lat=to_geo.transform(points[:,0],points[:,1]); iy=np.floor((lat-float(e.lat[0]))*60+.5).astype(int);ix=np.floor((lon-float(e.lon[0]))*60+.5).astype(int)
 valid=(iy>=0)&(iy<2700)&(ix>=0)&(ix<2700);iy=np.clip(iy,0,2699);ix=np.clip(ix,0,2699)
 h=z[iy,ix];return (h, valid&np.isfinite(h)&~water[iy,ix],iy,ix) if indices else (h, valid&np.isfinite(h)&~water[iy,ix])
def segment(a,b):
 distance=np.linalg.norm(b-a);n=max(2,int(np.ceil(distance/.5))+1)
 h,land,iy,ix=sample(a+(b-a)*np.linspace(0,1,n)[:,None],True)
 # Include both side cells when a short sample segment crosses a grid corner.
 # This is conservative, not permission to cut an ocean/unknown corner.
 side1=z[iy[:-1],ix[1:]];side2=z[iy[1:],ix[:-1]]
 allowed=land.all() and np.isfinite(side1).all() and np.isfinite(side2).all() and not water[iy[:-1],ix[1:]].any() and not water[iy[1:],ix[:-1]].any()
 return float(distance+4*np.abs(np.diff(h)).sum()/1000) if allowed else None
def main():
 extent=3000.;width=300;cell=extent/width
 v=-extent/2+(np.arange(width)+.5)*cell;xx,yy=np.meshgrid(v,v)
 height,land=sample(np.c_[xx.ravel(),yy.ravel()])
 # Seeded Poisson-like rejection with spatial bins, independent of display cells.
 rng=np.random.default_rng(19092026);sites=[];bins={};minimum=15.
 for point in rng.uniform(-1490,1490,(65000,2)):
  if not sample(point[None,:])[1][0]:continue
  key=tuple(np.floor(point/minimum).astype(int));neighbors=[]
  for dx in [-1,0,1]:
   for dy in [-1,0,1]:neighbors.extend(bins.get((key[0]+dx,key[1]+dy),[]))
  if any(np.linalg.norm(point-sites[i])<minimum for i in neighbors):continue
  bins.setdefault(key,[]).append(len(sites));sites.append(point)
 sites=np.array(sites);tree=cKDTree(sites);edges=[]
 for a,b in sorted(tree.query_pairs(55.)):
  cost=segment(sites[a],sites[b])
  if cost is not None:edges.append([a,b,cost])
 print('geometry',len(sites),len(edges),flush=True)
 # Fine, fixed physical food quadrature, not the camp Voronoi cells.
 # A 7.5 km round-trip reach with effort weight; shared cells are never duplicated.
 food=[];access=[];food_index={};step=2.5;radius=7.5
 for i,p in enumerate(sites):
  xmin,ymin=np.floor((p-radius)/step).astype(int);xmax,ymax=np.ceil((p+radius)/step).astype(int)
  for ix in range(xmin,xmax+1):
   for iy in range(ymin,ymax+1):
    q=(np.array([ix,iy])+.5)*step;distance=np.linalg.norm(q-p)
    if distance>radius:continue
    cost=segment(p,q)
    if cost is None:continue
    key=(ix,iy)
    if key not in food_index:
     food_index[key]=len(food);food.append([float(q[0]),float(q[1]),step*step,float(np.pi*q[1]/1500.)])
    access.append([i,food_index[key],float(1/(1+2*cost/6.))])
 areas=np.bincount([a[0] for a in access],weights=[food[a[1]][2]*a[2] for a in access],minlength=len(sites))
 base=json.loads((R/'public/spatial/world.json').read_text())
 world={k:base[k] for k in ['fertility','female','male']}
 world.update({'width':width,'cellKm':cell,'extent':extent,'center':[24,39], 'elevation':np.round(height,1).tolist(),'land':land.astype(int).tolist(),'sites':sites.round(6).tolist(),'regional':{'origin':list(to_xy.transform(32.,28.)),'edges':edges,'foodCells':food,'access':access,'foragingRadiusKm':radius,'resourceQuadratureKm':step,'water':'unknown'},'environment':{'name':'Northeast Africa–Sinai–Levant–Arabia','place':'connected','yearsBP':100000,'seaLevelM':level,'sourceManifest':'/spatial/connected/manifest.json','staticDuringRun':True}})
 world['ecology']={**base['ecology'],'referencePeople':[1.]*len(sites),'referenceLow':[1.]*len(sites),'referenceHigh':[1.]*len(sites),'areaKm2':areas.tolist(),'knownAreaKm2':[0.]*len(sites),'densityRaster':[None]*len(height)}
 # The ones are initialization eligibility, explicitly not inferred population density.
 rows=[int(a) for a,b,c in edges]+[int(b) for a,b,c in edges];cols=[int(b) for a,b,c in edges]+[int(a) for a,b,c in edges]
 count,labels=connected_components(coo_matrix((np.ones(len(rows)),(rows,cols)),shape=(len(sites),len(sites))).tocsr())
 anchors={}
 for name,lon,lat in [('Nile',32,28),('Sinai',33.5,29.8),('Levant',36,32),('Arabia',44,24),('NortheastAfrica',34,18)]:
  d,i=tree.query(to_xy.transform(lon,lat));anchors[name]={'site':int(i),'distanceKm':float(d),'component':int(labels[i])}
 assert len({a['component'] for a in anchors.values()})==1, anchors
 path=D/'region-100000.json';path.write_text(json.dumps(world,separators=(',',':'),allow_nan=False))
 manifest={'schema':'connected-region/1','status':'M1 engineering geography, not validated habitability','sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'terrain':json.loads((R/'research/world-v2/terrain-source.json').read_text()),'seaLevelM':level,'projection':crs.to_string(),'extentKm':extent,'displayCellKm':cell,'routeSamplingKm':.5,'routeMaximumKm':55,'campMinimumSeparationKm':minimum,'siteSeed':19092026,'sites':len(sites),'edges':len(edges),'components':int(count),'anchors':anchors,'foodCells':len(food),'foragingQuadratureKm':step,'foragingRadiusKm':radius,'foragingEffort':'weight 1/(1+round-trip terrain-cost/6km); synthetic assumption','food':'uniform synthetic kcal/km2/day with spatially phased annual forcing','water':'unknown; engineering runs assume availability, no regional viability claim','referencePeople':'ones are land initialization eligibility, not density estimates','limitations':['modern relief at uniform global sea level','sub-kilometer obstacles unresolved','no freshwater access','static 100ka geography','no historical occupation prediction']}
 (D/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps({k:manifest[k] for k in ['sites','edges','components','anchors','foodCells']}),flush=True)

 # Package static resources after completing the scientific input and manifest.
 import subprocess,sys
 subprocess.run([sys.executable,str(R/'scripts/shard-connected-world.py')],check=True)
 subprocess.run([sys.executable,str(R/'scripts/render-connected-terrain.py')],check=True)

if __name__=="__main__":
 main()
 import subprocess,sys
 subprocess.run([sys.executable,str(R/"scripts/weight-destination-quadrature.py")],check=True)
