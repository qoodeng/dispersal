"""Extend the homogeneous domain without changing existing camps or food cells."""
from pathlib import Path
import json,numpy as np
from scipy.spatial import cKDTree
R=Path(__file__).resolve().parents[1];D=R/'research/food-experiment/mesh-fixtures'
w=json.loads((D/'20km.json').read_text());r=w['regional'];sites=w['sites'];rng=np.random.default_rng(4771)
for y in range(-310,311,20):
 for x in range(-310,311,20):
  if abs(x)<=150 and abs(y)<=150:continue
  sites.append((np.array([x,y])+rng.uniform(-6,6,2)).tolist())
food=r['foodCells'];keys={tuple(c[:2]):i for i,c in enumerate(food)}
for y in np.arange(-318.75,320,2.5):
 for x in np.arange(-318.75,320,2.5):
  if (x,y) not in keys:keys[(x,y)]=len(food);food.append([float(x),float(y),6.25,float(np.pi*y/320)])
ft=cKDTree(np.array(food)[:,:2]);access=[];area=[]
for i,p in enumerate(sites):
 ids=sorted(ft.query_ball_point(p,7.5));weights=[1/(1+2*np.linalg.norm(np.array(food[j][:2])-p)/6) for j in ids]
 access.extend([[i,int(j),float(v)] for j,v in zip(ids,weights)]);area.append(sum(weights)*6.25)
s=np.array(sites);r['edges']=[[int(a),int(b),float(np.linalg.norm(s[a]-s[b]))] for a,b in sorted(cKDTree(s).query_pairs(55.))];r['access']=access
w.update(width=128,cellKm=5,extent=640,elevation=[100]*16384,land=[1]*16384)
for k in ['referencePeople','referenceLow','referenceHigh']:w['ecology'][k]=[1]*len(sites)
w['ecology']['areaKm2']=area;w['ecology']['knownAreaKm2']=area;w['ecology']['densityRaster']=[None]*16384
(D/'extended-640km.json').write_text(json.dumps(w,separators=(',',':')));print(len(sites),len(food))

# Numerical destination area is separate from biological resource access.
import subprocess,sys
subprocess.run([sys.executable,str(R/"scripts/weight-destination-quadrature.py")],check=True)
