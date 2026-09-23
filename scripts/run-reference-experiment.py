"""Reproducible integration experiment, independently checked against analytical survival.
This does NOT calibrate ancient growth. Initial cohort, location, date and seed are test fixtures.
"""
from pathlib import Path
import csv,hashlib,io,json,subprocess,sys
import numpy as np
from scipy.integrate import quad
import xarray as xr
R=Path(__file__).resolve().parents[1];out=R/'research/world-v2'
def call(args):return subprocess.run(args,cwd=R,text=True,capture_output=True,check=True).stdout
print(call([sys.executable,'scripts/evidence-gate.py','research/world-v2/parameter-contract.json']).strip())
contract=json.loads((out/'parameter-contract.json').read_text())
assert contract['mode']=='reference' and contract['domain']=='modern !Kung mortality reference'
params={p['id'].split('.')[1]:p['value'] for p in contract['parameters']}
assert set(params)=={'a1','b1','a2','a3','b3'}
manifest=json.loads((out/'runtime-manifest.json').read_text())
for a in manifest['artifacts']:
 assert hashlib.sha256((R/a['file']).read_bytes()).hexdigest()==a['sha256']
args=[str(Path.home()/'.cargo/bin/cargo'),'run','--quiet','--release','--manifest-path','engine/Cargo.toml','--bin','reference-run','--',str(out/'runtime')]+[str(params[k]) for k in ['a1','b1','a2','a3','b3']]
result=call(args);assert result==call(args),'seed replay failed'
rows=list(csv.DictReader(io.StringIO(result)))
assert len(rows)==81
# Independent Python quadrature; not the Rust integrated-hazard implementation.
def hazard(x):return params['a1']*np.exp(-params['b1']*x)+params['a2']+params['a3']*np.exp(params['b3']*x)
c=np.load(out/'regional-climate.npz');dem=xr.open_dataset(out/'etopo2022-regional.nc').sortby('lat').sortby('lon')
def cell(axis,x):
 # Higher index owns an exact cell boundary, matching the documented half-open convention.
 return int(np.searchsorted((axis[:-1]+axis[1:])/2,x,side='right'))
lat,lon=28.,37.;yi=cell(c['latitude'],lat);xi=cell(c['longitude'],lon)
z=float(dem.z.values[cell(dem.lat.values,lat),cell(dem.lon.values,lon)])
zs=[]
for row in rows:
 year=int(row['elapsed_year']);people=int(row['people']);s=np.exp(-quad(hazard,0,year)[0]);expected=100000*s
 assert abs(float(row['expected_survivors'])-expected)<1e-6
 assert row['accounting_valid']=='true'
 sd=np.sqrt(100000*s*(1-s))
 if sd:zs.append(abs(people-expected)/sd)
 bp=float(row['years_bp']);assert bp==100000-year
 rain=float(np.interp(bp,c['years_bp'][::-1],c['precipitation_mm_year'][::-1,yi,xi]))
 assert abs(float(row['precipitation_mm_year'])-rain)<1e-7
 assert abs(float(row['elevation_m'])-z)<1e-7
assert max(zs)<6,'survival falls outside reference tolerance'
(out/'reference-run.csv').write_text(result)
report={'status':'passed','purpose':'reference integration check only; not historical validation','seed':123,'initial_people':100000,'initial_age':0,'sex':'female; selected mortality is pooled and no sex adjustment is assumed','years':80,'fixture_position':[lat,lon],'fixture_start_bp':100000,'fixture_note':'Synthetic closed cohort, arbitrary query location/date; no claim of occupancy','mortality_contract_sha256':hashlib.sha256((out/'parameter-contract.json').read_bytes()).hexdigest(),'output_sha256':hashlib.sha256(result.encode()).hexdigest(),'final_people':int(rows[-1]['people']),'maximum_survival_standard_deviations':max(zs),'checks':['bit-for-bit seeded replay','annual integer counts and conservation','independent numerical integration of hazard','native DEM value match','native climate time interpolation match','all 81 annual checkpoints within 6-sigma survival tolerance (engineering test threshold)'],'limitations':['No births or net growth calibration','No resource consumption or ecological effect on mortality','No climate-driven migration or geographic arrival predictions','Annual mortality booked at interval end; within-year death/arrival interaction approximate','Modern demographic analogue queried alongside ancient climate solely for data-plumbing verification','Local frame is spherical, bounded to 100km; not the final equal-area mesh'],'sources':['gurven2007','climate2020','etopo2022']}
(out/'reference-checks.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
