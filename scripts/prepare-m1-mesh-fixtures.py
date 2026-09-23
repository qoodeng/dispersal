"""Independent camp-mesh refinement with fixed origins and physical food cells."""
import numpy as np,json
from pathlib import Path
from scipy.spatial import cKDTree
R=Path(__file__).resolve().parents[1];O=R/'research/food-experiment/mesh-fixtures';O.mkdir(exist_ok=True)
base=json.loads((R/'public/spatial/dated/aqaba-100000.json').read_text())
initial=np.array([[-100,-20],[-100,0],[-100,20],[-80,-20],[-80,0],[-80,20]],dtype=float)
v=np.arange(-158.75,160,2.5);xx,yy=np.meshgrid(v,v);food=np.c_[xx.ravel(),yy.ravel(),np.full(xx.size,6.25),np.pi*yy.ravel()/320];ft=cKDTree(food[:,:2])
for spacing in [20,10,5]:
 rng=np.random.default_rng(9123);sites=initial.tolist()
 for y in np.arange(-150,151,spacing):
  for x in np.arange(-150,151,spacing):
   p=np.array([x,y])+rng.uniform(-.3,.3,2)*spacing
   if np.min(np.linalg.norm(initial-p,axis=1))<spacing*.5:continue
   sites.append(p.tolist())
 sites=np.array(sites);tree=cKDTree(sites);edges=[[int(a),int(b),float(np.linalg.norm(sites[a]-sites[b]))] for a,b in sorted(tree.query_pairs(55.))]
 access=[];area=[]
 for i,p in enumerate(sites):
  patches=sorted(ft.query_ball_point(p,7.5));weights=[1/(1+2*np.linalg.norm(food[j,:2]-p)/6) for j in patches]
  access.extend([[i,int(j),float(w)] for j,w in zip(patches,weights)]);area.append(sum(weights)*6.25)
 world={k:base[k] for k in ['fertility','female','male']};world.update({'width':64,'cellKm':5,'extent':320,'center':[0,0],'land':[1]*4096,'elevation':[100]*4096,'sites':sites.tolist(),'regional':{'initialSites':list(range(6)),'origin':[-90,0],'edges':edges,'foodCells':food.tolist(),'access':access,'foragingRadiusKm':7.5,'resourceQuadratureKm':2.5,'water':'assumed available fixture'},'ecology':{**base['ecology'],'referencePeople':[1.]*len(sites),'referenceLow':[1.]*len(sites),'referenceHigh':[1.]*len(sites),'areaKm2':area,'knownAreaKm2':area,'densityRaster':[None]*4096}})
 (O/f'{spacing}km.json').write_text(json.dumps(world,separators=(',',':')));print(spacing,len(sites),len(edges),flush=True)

# Numerical destination area is separate from biological resource access.
import subprocess,sys
subprocess.run([sys.executable,str(R/"scripts/weight-destination-quadrature.py")],check=True)
