"""Source-derived 2 km overview, with the same projection and marine mask as routes."""
from pathlib import Path
import importlib.util,json,hashlib,zlib,struct
import numpy as np
R=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('regional_source',R/'scripts/prepare-connected-region.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
n=1500;v=-1500+(np.arange(n)+.5)*2.;xx,yy=np.meshgrid(v,v);height,land=m.sample(np.c_[xx.ravel(),yy.ravel()]);shade=np.clip(height,0,2200)/2200
rgb=np.c_[233-shade*65,234-shade*66,218-shade*70];rgb[~land]=[204,228,237]
rgb=np.clip(rgb,0,255).astype(np.uint8).reshape(n,n,3)[::-1]
def chunk(tag,data):return struct.pack('!I',len(data))+tag+data+struct.pack('!I',zlib.crc32(tag+data)&0xffffffff)
png=b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('!IIBBBBB',n,n,8,2,0,0,0))+chunk(b'IDAT',zlib.compress(b''.join(b'\0'+row.tobytes() for row in rgb),9))+chunk(b'IEND',b'')
D=R/'public/spatial/connected';(D/'terrain-100000.png').write_bytes(png)
p=D/'region-100000.json';world=json.loads(p.read_text());world['regional']['terrainImage']='/spatial/connected/terrain-100000.png';p.write_text(json.dumps(world,separators=(',',':')))
p=D/'manifest.json';manifest=json.loads(p.read_text());manifest['sha256']=hashlib.sha256((D/'region-100000.json').read_bytes()).hexdigest();manifest['displayTerrain']={'file':'terrain-100000.png','cellKm':2,'sha256':hashlib.sha256(png).hexdigest(),'note':'Overview only; native-grid routes use conservative half-kilometer sampling.'};p.write_text(json.dumps(manifest,indent=2)+'\n')
print({'terrainBytes':len(png),'width':n,'cellKm':2})
