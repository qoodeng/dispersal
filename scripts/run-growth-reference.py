"""Fit-check and independent expected-value verification of the Rust birth/death reference."""
from pathlib import Path
import csv,io,json,subprocess,hashlib,concurrent.futures,sys
import numpy as np
from scipy.linalg import eigvals
R=Path(__file__).resolve().parents[1];out=R/'research/demography'
meta=json.loads((out/'fertility-reference.json').read_text())
assert hashlib.sha256((out/'annual-reference.csv').read_bytes()).hexdigest()==meta['annual_file_sha256']
subprocess.run([sys.executable,'scripts/evidence-gate.py','research/demography/parameter-contract.json'],cwd=R,check=True)
contract=json.loads((out/'parameter-contract.json').read_text());contract_values={p['id']:p['value'] for p in contract['parameters']}
assert contract_values['fertility.scale']==meta['fitted_total_fertility_scale']
assert contract_values['fertility.shape']==meta['digitization']['raw_relative_rates']
coeff=meta['mortality_coefficients'];table=list(csv.DictReader((out/'annual-reference.csv').open()))
s=np.array([float(r['survival_probability']) for r in table]);f=np.array([float(r['birth_probability']) for r in table]);initial=np.array([float(r['initial_females']) for r in table])
# Independent matrix construction from reference input, including the chosen census timing.
A=np.zeros((len(s),len(s)));A[0]=.5*s*f;A[np.arange(1,len(s)),np.arange(len(s)-1)]=s[:-1]
lam=float(max(eigvals(A).real));rate=float(np.log(lam));assert abs(rate-meta['growth_anchor']['annual_log_rate'])<1e-10
expected=[];v=initial.copy();closed=[];c=initial.copy()
for year in range(161):
 expected.append(float(2*v.sum()));closed.append(float(2*c.sum()));v=A@v;c=np.r_[0,c[:-1]*s[:-1]]
subprocess.run([str(Path.home()/'.cargo/bin/cargo'),'build','--release','--manifest-path','engine/Cargo.toml','--bin','growth-reference'],cwd=R,check=True,capture_output=True)
base=[str(R/'engine/target/release/growth-reference'),str(out/'annual-reference.csv')]
params=[str(coeff[k]) for k in ['a1','b1','a2','a3','b3']]
def run(seed,births=True):
 result=subprocess.run(base+[str(seed),'1' if births else '0']+params,cwd=R,text=True,capture_output=True,check=True).stdout
 rows=list(csv.DictReader(io.StringIO(result)));assert len(rows)==161
 for row in rows:
  assert row['accounting_valid']=='true';assert int(row['people'])==5000+int(row['cumulative_births'])-int(row['cumulative_deaths'])
 return result,[{k:(v=='true' if k=='accounting_valid' else int(v)) for k,v in row.items()} for row in rows]
main,rows=run(123);assert main==run(123)[0]
closed_csv,closed_rows=run(123,False)
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:runs=list(pool.map(run,range(32)))
counts=np.array([[row['people'] for row in rows_] for _,rows_ in runs]);means=counts.mean(axis=0);se=counts.std(axis=0,ddof=1)/np.sqrt(len(runs));z=np.abs(means-np.array(expected))/np.maximum(se,1e-12)
assert z.max()<6
for i,row in enumerate(rows):row['expected_people']=expected[i]
for i,row in enumerate(closed_rows):row['expected_people']=closed[i]
(out/'growth-run.csv').write_text(main);(out/'no-births-run.csv').write_text(closed_csv)
report={'status':'passed','scope':'modern-reference implementation and calibration only','calibrated_annual_log_growth':rate,'target_annual_log_growth':meta['growth_anchor']['annual_log_rate'],'fitted_total_fertility_scale':meta['fitted_total_fertility_scale'],'replicates':32,'max_mean_error_in_monte_carlo_standard_errors':float(z.max()),'checks':['independent Leslie eigenvalue recovers fitting target','seed123 replays exactly','32 stochastic replicates agree with independent expected projection within6 Monte Carlo standard errors (engineering tolerance)','all annual population ledgers balance','matched initial population for birth/no-birth comparison'],'not_validated':['historical transfer','between-population uncertainty','individual birth spacing','partner availability','ecological regulation','migration'],'source_ids':meta['source_ids'],'inputs':{'annual_reference_sha256':meta['annual_file_sha256'],'metadata_sha256':hashlib.sha256((out/'fertility-reference.json').read_bytes()).hexdigest()}}
(out/'growth-checks.json').write_text(json.dumps(report,indent=2)+'\n')
public=R/'public/reference';public.mkdir(exist_ok=True)
(public/'growth.json').write_text(json.dumps({'with_births':rows,'without_births':closed_rows,'metadata':meta,'checks':report},separators=(',',':')))
for name in ['growth-run.csv','no-births-run.csv','fertility-reference.json','growth-checks.json']:(public/name).write_bytes((out/name).read_bytes())
print(json.dumps(report,indent=2))
