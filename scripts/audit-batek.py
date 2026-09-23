"""Audit primary author data and compare preregistered DEVELOPMENT candidates.
Requires pyreadr, numpy, scipy, pandas. No regional model parameters are emitted.
"""
from pathlib import Path
import hashlib,json
import numpy as np,pandas as pd,pyreadr
from scipy.optimize import least_squares
R=Path(__file__).resolve().parents[1];O=R/'research/resources/batek'
protocol=json.loads((O/'protocol.json').read_text())
for a in json.loads((O/'acquisition.json').read_text())['files']:
 assert hashlib.sha256((O/a['file']).read_bytes()).hexdigest()==a['sha256']
d=pyreadr.read_r(str(O/'campmovementdata.RData'))['tot'];d['camp']=d['Camp.number'].astype(int);d['resource']=d['variable'].astype(str)
assert not d.duplicated(['camp','resource','count']).any()
rows=[]
for (camp,res),g in d.groupby(['camp','resource']):
 g=g.sort_values('count');x=g['count'].to_numpy();y=g.value.to_numpy()
 assert np.array_equal(x,np.arange(len(x))) and y[0]==0
 assert np.all(np.diff(y)>=-1e-8)
 for day,gain,total in zip(x[1:],np.diff(y),y[1:]):
  rows.append({'camp':int(camp),'resource':res,'day':int(day),'daily_gain_kcal_per_capita':float(gain),'cumulative_gain_kcal_per_capita':float(total),'complete_camp':int(camp) in protocol['complete_camps']})
a=pd.DataFrame(rows);wide=a.pivot(index=['camp','day'],columns='resource',values='daily_gain_kcal_per_capita')
assert np.allclose(wide.tot_rat,wide.tot+wide.tot_rat_only)
assert np.all(wide.tot>=wide.tot_meat+wide.tot_tuber-1e-7)
a.to_csv(O/'daily-gains.csv',index=False)

def cumulative(model,t,p):
 A,b=p
 if model=='exponential':return A*(-np.expm1(-b*t))
 if model=='michaelis_menten':return A*t/(b+t)
 if model=='holling_iii':return A*t*t/(b*b+t*t)
 raise ValueError(model)
def fit_predict(model,tr,te):
 weights=1/np.sqrt(tr.groupby('camp').day.transform('size').to_numpy())
 x=tr.day.to_numpy(float);y=tr.daily_gain_kcal_per_capita.to_numpy()
 if model=='constant':
  mean=float(tr.groupby('camp').daily_gain_kcal_per_capita.mean().mean());return np.full(len(te),mean),[mean]
 def fun(logp):
  p=np.exp(logp);return (cumulative(model,x,p)-cumulative(model,x-1,p)-y)*weights
 fits=[]
 for b in ([.01,.1,1] if model=='exponential' else [1,10,100]):
  v=least_squares(fun,np.log([max(1,float(y.mean()*10)),b]),bounds=(np.log([.01,.0001]),np.log([1e7,1000])),max_nfev=3000)
  if v.success:fits.append(v)
 assert fits,model
 best=min(fits,key=lambda v:np.sum(v.fun**2));p=np.exp(best.x);t=te.day.to_numpy(float)
 return cumulative(model,t,p)-cumulative(model,t-1,p),p.tolist()
folds=[];predictionrows=[]
for res in ['tot','tot_rat']:
 data=a[(a.resource==res)&a.complete_camp]
 for camp in protocol['complete_camps']:
  tr=data[data.camp!=camp];te=data[data.camp==camp]
  for model in ['constant','exponential','michaelis_menten','holling_iii']:
   pred,params=fit_predict(model,tr,te);err=pred-te.daily_gain_kcal_per_capita.to_numpy()
   folds.append({'resource':res,'heldout_camp':camp,'model':model,'rmse':float(np.sqrt(np.mean(err**2))),'parameters':params})
   for (_,r),p in zip(te.iterrows(),pred):predictionrows.append({'resource':res,'heldout_camp':camp,'model':model,'day':int(r.day),'observed':r.daily_gain_kcal_per_capita,'prediction':p})
scores=pd.DataFrame(folds).groupby(['resource','model']).rmse.mean();summary=[]
for (res,model),score in scores.items():summary.append({'resource':res,'model':model,'equal_camp_rmse':score,'improvement_fraction':1-score/scores[res,'constant']})
pd.DataFrame(predictionrows).to_csv(O/'heldout-daily-predictions.csv',index=False)
report={'schema':'batek-development-audit/1','protocol_sha256':hashlib.sha256((O/'protocol.json').read_bytes()).hexdigest(),'raw_rows':len(d),'zero_anchors':len(d)-len(a),'resource_rows':len(a),'camp_days':len(wide),'complete_camps':protocol['complete_camps'],'complete_camp_days':int(a[(a.resource=='tot')&a.complete_camp].shape[0]),'cumulative_value_checks':True,'additive_resource_checks':True,'metadata_discrepancy':'Archived value is cumulative: nondecreasing sequences, zero origins, and published R directly fitting cumulative gain functions. daily-gains.csv is first differences, not raw value.','camp11_duration_discrepancy':'Archive has 27 observed days; paper prose gives maximum 24. Retain 27 and flag rather than silently changing data.','scores':summary,'folds':folds,'adopted_as_regional_default':False,'limitations':protocol['not_identified'],'validation_scope':protocol['validation']}
(O/'audit.json').write_text(json.dumps(report,indent=2,allow_nan=False)+'\n')
print(json.dumps({k:report[k] for k in ['raw_rows','zero_anchors','camp_days','complete_camp_days','scores']},indent=2))
