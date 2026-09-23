"""Package regional arrays below the host's 25 MiB per-asset limit.
https://developers.cloudflare.com/workers/platform/limits/#static-assets
Scientific values and order are unchanged; every part carries a SHA-256 hash.
"""
from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parents[1];D=R/'public/spatial/connected'
p=D/'region-100000.json';w=json.loads(p.read_text());r=w['regional']
if r.get('dataParts'):raise SystemExit('Already packaged')
parts=[]
for start in range(0,max(len(r['foodCells']),len(r['access'])),100000):
 part={'foodCells':r['foodCells'][start:start+100000],'access':r['access'][start:start+100000]}
 name=f'resources-{start//100000}.json';raw=json.dumps(part,separators=(',',':')).encode();assert len(raw)<10*1024*1024
 (D/name).write_bytes(raw);parts.append({'file':name,'sha256':hashlib.sha256(raw).hexdigest(),'foodCells':len(part['foodCells']),'access':len(part['access'])})
r['foodCells']=[];r['access']=[];r['dataParts']=parts
p.write_text(json.dumps(w,separators=(',',':')))
m=json.loads((D/'manifest.json').read_text());m['expandedWorldSha256']=m['sha256'];m['sha256']=hashlib.sha256(p.read_bytes()).hexdigest();m['dataParts']=parts
(D/'manifest.json').write_text(json.dumps(m,indent=2)+'\n')
print({'parts':len(parts),'largestBytes':max(f.stat().st_size for f in D.glob('*.json'))})
