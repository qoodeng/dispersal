"""Area-weight candidate sampling; this is numerical quadrature, never food ownership."""
import json,hashlib
from pathlib import Path
import numpy as np
from scipy.spatial import cKDTree
R=Path(__file__).resolve().parents[1]
paths=list((R/'research/food-experiment/mesh-fixtures').glob('*.json'))+[R/'public/spatial/connected/region-100000.json']
for path in paths:
 w=json.loads(path.read_text());r=w['regional'];cells=r.get('foodCells',[])
 if r.get('dataParts'):
  cells=[]
  for part in r['dataParts']:cells.extend(json.loads((path.parent/part['file']).read_text())['foodCells'])
 cells=np.array(cells);sites=np.array(w['sites']);owner=cKDTree(sites).query(cells[:,:2])[1]
 areas=np.bincount(owner,weights=cells[:,2],minlength=len(sites));assert np.any(areas>0)
 r['destinationWeights']=(areas/areas.mean()).tolist()
 path.write_text(json.dumps(w,separators=(',',':')))
 if path.parent.name=='connected':
  mp=path.parent/'manifest.json';m=json.loads(mp.read_text());m['sha256']=hashlib.sha256(path.read_bytes()).hexdigest();m['destinationQuadrature']='Candidate probabilities weighted by represented land area from nearest-camp allocation of fixed resource quadrature; this allocation does not allocate or duplicate food.';mp.write_text(json.dumps(m,indent=2)+'\n')
 print(path.name,areas.min(),areas.max())
