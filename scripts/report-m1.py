"""Collect passing engineering gates and their reproducible source fingerprints."""
from pathlib import Path
import json,hashlib,shutil
R=Path(__file__).resolve().parents[1];D=R/'research/food-experiment';P=R/'public/spatial/m1';P.mkdir(exist_ok=True)
def read(name):return json.loads((D/(name+'.json')).read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
integrated=read('m1-integrated-checks');temporal=read('m1-physical-time');mesh=read('m1-mesh-checks');iso=read('m1-isotropy-checks');region=read('m1-region-0-32');sensitivity=read('m1-sensitivity');parity=read('m1-parity');boundary=read('m1-boundary-checks')
assert integrated['accepted'] and temporal['pass'] and mesh['arrivalPass'] and mesh['occupancyPass'] and iso['pass'] and parity['pass'] and boundary['pass'] and sensitivity['accountingPassed'] and len(region['results'])==32
assert read('m1-transform-checks')['pass'] and read('m1-export-checks')['pass']
names=['m1-interface-checks','m1-transform-checks','m1-export-checks','native-parity','m1-integrated-checks','m1-physical-time','m1-mesh-checks','m1-isotropy-checks','m1-region-0-32','m1-sensitivity','m1-parity','m1-boundary-checks','m1-physical-time-32','m1-physical-time-512-before-event-foraging','m1-mesh-32-before-expanded-ensemble','m1-mesh-128-before-area-weights']
evidence=[]
for name in names:
 path=D/(name+'.json');shutil.copy2(path,P/path.name);evidence.append({'file':path.name,'sha256':sha(path)})
ui=read('m1-interface-checks');assert ui['coreChecksPassed']
result={'schema':'dispersal-m1-acceptance/1','date':'2026-09-19','status':'M1 engineering acceptance passed; historical validity not claimed','engineSha256':sha(R/'public/spatial/engine.wasm'),'regionalManifestSha256':sha(R/'public/spatial/connected/manifest.json'),'protocolSha256':sha(R/'docs/M1-ACCEPTANCE-PROTOCOL.md'),'unitTests':71,'temporal':temporal['metrics'],'mesh':{k:mesh[k] for k in ['seeds','medians','relativeArrivalChange','arrivalPass','occupancyPass']},'isotropy':iso,'regional':{'seeds':32,'seedsWithEstablishment':sum(x['establishedSites']>0 for x in region['results']),'seedsWithRecolonization':sum(x['recolonizations']>0 for x in region['results']),'finalPopulationRange':[min(x['people'] for x in region['results']),max(x['people'] for x in region['results'])]},'abundant':sensitivity['abundant'],'interface':ui,'evidence':evidence,'limits':['Synthetic resource production, seasonality, access cost and condition response; not calibrated ancient parameters','Regional water unknown and assumed available in engineering runs','Static 100 ka sea-level scenario on modern relief, projected planar distance/area approximation','No dynamic climate, diet/boat mechanisms or calibrated archaeological arrival inference','Responsive viewport tested on named Mac hardware; no physical-phone performance claim'],'sourceHashes':{str(p.relative_to(R)):sha(p) for directory in ['engine/src/population','src'] for p in sorted((R/directory).glob('*')) if p.is_file()}}
(P/'results.json').write_text(json.dumps(result,indent=2)+'\n');shutil.copy2(R/'docs/M1-ACCEPTANCE-PROTOCOL.md',P/'protocol.md');shutil.copy2(R/'docs/M1-ACCEPTANCE-REPORT.md',P/'report.md')
print(json.dumps({k:result[k] for k in ['status','engineSha256','regional','mesh']},indent=2))
