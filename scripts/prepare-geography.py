"""Build a modern land mask. NOT a paleogeographic reconstruction."""
import json, math, hashlib
from pathlib import Path
root=Path(__file__).resolve().parents[1]
raw=(root/'data/land.geojson').read_bytes(); geo=json.loads(raw)
polys=[]
for feature in geo['features']:
 g=feature['geometry']
 polys.extend(g['coordinates'] if g['type']=='MultiPolygon' else [g['coordinates']])
def inside_ring(x,y,ring):
 odd=False
 for a,b in zip(ring,ring[1:]+ring[:1]):
  if (a[1]>y)!=(b[1]>y) and x < (b[0]-a[0])*(y-a[1])/(b[1]-a[1])+a[0]:odd=not odd
 return odd
polys=[p for p in polys if max(v[0] for v in p[0])>=20 and min(v[0] for v in p[0])<=65 and max(v[1] for v in p[0])>=-5 and min(v[1] for v in p[0])<=40]
boxed=[(p,(min(v[0] for v in p[0]),min(v[1] for v in p[0]),max(v[0] for v in p[0]),max(v[1] for v in p[0]))) for p in polys]
def land(x,y):
 return any(b[0]<=x<=b[2] and b[1]<=y<=b[3] and inside_ring(x,y,p[0]) and not any(inside_ring(x,y,h) for h in p[1:]) for p,b in boxed)
w=h=90; cells=[]
for r in range(h):
 for c in range(w):
  lon=20+(c+.5)*.5;lat=40-(r+.5)*.5
  is_land=land(lon,lat)
  # Uniform engineering fixture; no invented climatic or freshwater gradients.
  suitability=1.0
  coast=is_land and any(not land(lon+dx,lat+dy) for dx,dy in [(.5,0),(-.5,0),(0,.5),(0,-.5)])
  cells.append([int(is_land),round(suitability,4),int(coast)])
edges=[]
for r in range(h):
 for c in range(w):
  i=r*w+c
  if not cells[i][0]:continue
  for rr,cc in [(r+1,c),(r,c+1)]:
   if rr>=h or cc>=w:continue
   j=rr*w+cc
   if not cells[j][0]:continue
   x1=20+(c+.5)*.5;y1=40-(r+.5)*.5
   x2=20+(cc+.5)*.5;y2=40-(rr+.5)*.5
   if all(land(x1+(x2-x1)*t,y1+(y2-y1)*t) for t in [.25,.5,.75]):
    km=55.6*(math.cos(math.radians(y1)) if r==rr else 1)
    edges.append([i,j,round(km,3)])
out={'width':w,'height':h,'bounds':[20,-5,65,40],'cellDegrees':.5,'cells':cells,'polygons':polys,'edges':edges,'provenance':{'source':'Natural Earth 1:110m land','url':'https://github.com/nvkelso/natural-earth-vector/blob/master/geojson/ne_110m_land.geojson','sha256':hashlib.sha256(raw).hexdigest(),'coastlines':'modern','habitat':'uniform engineering fixture, not reconstructed climate'}}
(root/'public/geography.json').write_text(json.dumps(out,separators=(',',':')))
print(f'{len(cells)} cells; {len(edges)} validated terrestrial edges')
