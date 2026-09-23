"""Transcribe dated observations and reproduce a published-model demographic diagnostic.
Outputs are evidence/audit material, not calibrated prehistoric predictions.
Requires numpy and pandas; source archives are described in the acquisition manifest.
"""
import pathlib,json,csv,hashlib,re
import numpy as np
import pandas as pd
R=pathlib.Path(__file__).resolve().parents[1];C=R/'cache';D=R/'data';D.mkdir(exist_ok=True)
columns=['record_id','site','sample','age_ka_as_published','minus_ka','plus_ka','uncertainty_convention','measurement','relationship_to_human_event','dependency_group','source_id','source_locator','ingestion_status']
rows=[
['aw1','Al Wusta','AW-1',87.6,2.5,2.5,'2 sigma','U-series','minimum fossil age; not first arrival','AW-chronology','alwusta2018','SI section4 p22','censoring model required'],
['aw-enamel','Al Wusta','WU1601 enamel',83.5,8.1,8.1,'2 sigma','U-series','minimum associated tooth age','AW-tooth','alwusta2018','SI section4 p22','associated fauna; shared uptake model'],
['aw-dentine','Al Wusta','WU1601 dentine',65,2.1,2.1,'2 sigma','U-series','minimum associated tooth age','AW-tooth','alwusta2018','SI section4 p22','associated fauna; shared uptake model'],
['aw-esr','Al Wusta','WU1601',103,9,10,'1 sigma','US-ESR','finite associated tooth age','AW-tooth','alwusta2018','SI section4 p22','not independent of same tooth U-series'],
['aw-pd40','Al Wusta','PD40',98.6,7,7,'unconfirmed in extraction','OSL','Unit3a associated deposit','AW-chronology','alwusta2018','SI section4 p22; Table16','resolve sigma and shared systematic terms'],
['aw-pd17','Al Wusta','PD17',85.3,5.6,5.6,'unconfirmed in extraction','OSL','underlying Unit1 maximum fossil age constraint','AW-chronology','alwusta2018','SI section4 p22; Table16','resolve sigma and stratigraphic likelihood'],
['aw-pd41','Al Wusta','PD41',92.2,6.8,6.8,'unconfirmed in extraction','OSL','underlying Unit1 maximum fossil age constraint','AW-chronology','alwusta2018','SI section4 p22; Table16','resolve sigma and stratigraphic likelihood'],
['aw-pd15','Al Wusta','PD15',92,6.3,6.3,'unconfirmed in extraction','OSL','underlying Unit1 maximum fossil age constraint','AW-chronology','alwusta2018','SI section4 p22; Table16','resolve sigma and stratigraphic likelihood'],
['al-pd61','Alathar','PD61',121,11,11,'unconfirmed in extraction','OSL','Unit2 below footprints; older bracketing layer','Alathar-chronology','alathar2020','SI Text2; TableS4','resolve sigma; climate left boundary issue'],
['al-pd62','Alathar','PD62',112,10,10,'unconfirmed in extraction','OSL','Unit5 above footprints; younger bracketing layer','Alathar-chronology','alathar2020','SI Text2; TableS4','resolve sigma; joint stratigraphic model'],
]
with (D/'archaeological-observations.csv').open('w') as f:w=csv.writer(f,lineterminator="\n");w.writerow(columns);w.writerows(rows)
# Do not turn a published bracket into an invented uniform or Gaussian likelihood.
(D/'archaeological-derived-summaries.json').write_text(json.dumps([{'site':'Al Wusta','source_id':'alwusta2018','locator':'SI section4 pp22–23','younger_ka':85.1,'older_ka':96.5,'convention':'published combined 2-sigma constraints','independent_of_raw_dates':False,'use':'reference only; do not multiply into the likelihood alongside raw dates'}],indent=2))
# Preserve raw bootstrap table headings; no vertical-datum transformation assumed.
s=[]
for l in (C/'spratt2016-si.txt').read_text().splitlines():
 t=l.split()
 if len(t)==8:
  try:v=list(map(float,t))
  except ValueError:continue
  if v[0].is_integer() and 0<=v[0]<=798:s.append(v)
assert len(set(v[0] for v in s))==len(s)
pd.DataFrame(s,columns=['age_ka','PC1','sigma','q025','q25','median','q75','q975']).to_csv(D/'spratt2016-published-table.csv',index=False)
# Independently evaluate Siler survival + Leslie eigenvalue using the archive's inputs.
age=np.arange(81);survival=np.exp(-.422/1.131*(1-np.exp(-1.131*age))-.013*age+1.47e-4/.086*(1-np.exp(.086*age)))
lx=np.rint(10000*survival);sx=lx[1:]/lx[:-1]
f=pd.read_csv(C/'sahul-code/world2013lifetable.csv')['m.f'].to_numpy()[:81];fert=f/f.sum()*(4.69/2)
A=np.zeros((81,81));A[0]=fert;A[np.arange(1,81),np.arange(80)]=sx
B=A.copy();B[np.arange(1,81),np.arange(80)]=1
la=float(np.max(np.linalg.eigvals(A).real));lb=float(np.max(np.linalg.eigvals(B).real))
pd.DataFrame({'age_years':age,'survival_from_birth':survival,'female_fertility_per_year':fert}).to_csv(D/'published-sahul-demographic-diagnostic.csv',index=False)
result={'status':'published-model input diagnostic; NOT ancient population calibration','source_id':'sahul2021','code_source_id':'sahul-code','annual_lambda_with_published_survival':la,'annual_log_growth_with_published_survival':float(np.log(la)),'annual_lambda_if_survival_set_to_one':lb,'annual_log_growth_if_survival_set_to_one':float(np.log(lb)),'cautions':['Fertility shape comes from world2013lifetable; total is rescaled to 4.69/2 female offspring.','No-mortality matrix is a modeling construction, not observed human demography.','Code defines a doubled baseline rate and a separate no-mortality rate; the shown projection branch uses the latter. Do not conflate the variables.'],'identifiability_demo':{'label':'synthetic homogeneous logistic-diffusion example, not historical fit','speed_formula':'2*sqrt(r_per_year*D_km2_per_year)','pairs':[{'r':.002,'D':125,'speed_km_year':1},{'r':.008,'D':31.25,'speed_km_year':1}]}}
(D/'growth-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'date_rows':len(rows),'sea_level_rows':len(s),'annual_log_growth':result['annual_log_growth_with_published_survival'],'no_mortality_log_growth':result['annual_log_growth_if_survival_set_to_one']},indent=2))
