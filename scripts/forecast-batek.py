"""Forward-only resource-return development test. No geographic defaults emitted."""
from pathlib import Path
import json,hashlib
import numpy as np,pandas as pd
from scipy.optimize import least_squares
R=Path(__file__).resolve().parents[1];O=R/'research/resources/batek'
p=json.loads((O/'forward-protocol.json').read_text());d=pd.read_csv(O/'daily-gains.csv');d=d[d.complete_camp];records=[];failures=0
models=['other_camp_mean','local_mean','persistence','exponential','michaelis_menten','holling_iii']
def gain(model,t,a,b):
 if model=='exponential':return a*(-np.expm1(-b*t))
 if model=='michaelis_menten':return a*t/(b+t)
 return a*t*t/(b*b+t*t)
def predict(model,history,target,other):
 x=history.day.to_numpy(float);y=history.daily_gain_kcal_per_capita.to_numpy()
 assert x.max()<target
 if model=='other_camp_mean':return float(other.groupby('camp').daily_gain_kcal_per_capita.mean().mean()),False
 if model=='local_mean':return float(y.mean()),False
 if model=='persistence':return float(y[-1]),False
 def residual(logp):
  a,b=np.exp(logp);return gain(model,x,a,b)-gain(model,x-1,a,b)-y
 fits=[]
 for b in ([.01,.1,1] if model=='exponential' else [1,10,100]):
  fit=least_squares(residual,np.log([max(1,float(y.mean()*10)),b]),bounds=(np.log([.01,.0001]),np.log([1e7,1000])),max_nfev=3000)
  if fit.success:fits.append(fit)
 if not fits:return float(y.mean()),True
 f=min(fits,key=lambda v:np.sum(v.fun**2));a,b=np.exp(f.x)
 return float(gain(model,target,a,b)-gain(model,target-1,a,b)),False
for resource in p['resources']:
 sub=d[d.resource==resource]
 for camp,g in sub.groupby('camp'):
  g=g.sort_values('day');other=sub[sub.camp!=camp]
  for target in g.day[g.day>p['minimum_history_days']]:
   history=g[g.day<target];actual=float(g.loc[g.day==target,'daily_gain_kcal_per_capita'].iloc[0])
   for model in models:
    pred,fallback=predict(model,history,int(target),other);failures+=fallback
    records.append(dict(resource=resource,camp=int(camp),target_day=int(target),history_end=int(history.day.max()),model=model,actual=actual,prediction=pred,fallback=fallback))
f=pd.DataFrame(records);f['error']=f.prediction-f.actual
scores=[]
for (resource,camp,model),g in f.groupby(['resource','camp','model']):scores.append(dict(resource=resource,camp=int(camp),model=model,rmse=float(np.sqrt(np.mean(g.error**2))),mae=float(np.mean(abs(g.error)))))
s=pd.DataFrame(scores);summary=[];comparisons=[];rng=np.random.default_rng(20260919)
for resource in p['resources']:
 ss=s[s.resource==resource];pivot=ss.pivot(index='camp',columns='model',values='rmse');ix=rng.integers(0,len(pivot),size=(2000,len(pivot)))
 for model in models:
  summary.append(dict(resource=resource,model=model,rmse=float(ss[ss.model==model].rmse.mean()),mae=float(ss[ss.model==model].mae.mean())))
 for model in models[3:]:
  for baseline in models[:3]:
   delta=(pivot[model]-pivot[baseline]).to_numpy();boot=delta[ix].mean(axis=1);lo,hi=np.quantile(boot,[.025,.975]);comparisons.append(dict(resource=resource,model=model,baseline=baseline,delta_rmse=float(delta.mean()),interval95=[float(lo),float(hi)]))
# Test forecast input isolation, not just output bookkeeping.
g=d[(d.camp==11)&(d.resource=='tot')].sort_values('day');other=d[(d.camp!=11)&(d.resource=='tot')];h=g[g.day<5];changed=g.copy();changed.loc[changed.day>=5,'daily_gain_kcal_per_capita']=1e12
for m in models:
 assert predict(m,h,5,other)==predict(m,changed[changed.day<5],5,other)
 if m!='other_camp_mean':assert predict(m,h,5,other)==predict(m,h,5,other.assign(daily_gain_kcal_per_capita=1e12))
assert (f.history_end<f.target_day).all()
accepted=[m for m in models[3:] if all(c['interval95'][1]<0 for c in comparisons if c['model']==m and c['resource']=='tot')]
report=dict(schema='batek-forward-results/1',protocol_sha256=hashlib.sha256((O/'forward-protocol.json').read_bytes()).hexdigest(),evaluated_camps=sorted(map(int,f.camp.unique())),forecast_days_per_resource=int(len(f)/(len(models)*len(p['resources']))),fallback_count=int(failures),leakage_checks_passed=True,summary=summary,comparisons=comparisons,candidates_passing_development_gate=accepted,regional_adoption=False,limitations=p['limitations'])
f.to_csv(O/'forward-predictions.csv',index=False);s.to_csv(O/'forward-camp-scores.csv',index=False);(O/'forward-results.json').write_text(json.dumps(report,indent=2,allow_nan=False)+'\n')
print(json.dumps({k:report[k] for k in ['evaluated_camps','forecast_days_per_resource','fallback_count','summary','candidates_passing_development_gate']},indent=2))
