"""Predeclared camp-held-out departure decisions; no ancient adoption."""
from pathlib import Path
import json,hashlib
import numpy as np,pandas as pd,sklearn
from sklearn.linear_model import LogisticRegression
from sklearn.preprocessing import StandardScaler
from sklearn.pipeline import make_pipeline
R=Path(__file__).resolve().parents[1];O=R/'research/resources/batek';p=json.loads((O/'departure-protocol.json').read_text());d=pd.read_csv(O/'daily-gains.csv');rows=[];models=['constant','camp_age','camp_age_food'];folds=[]
def features(v):
 out=[]
 for k in range(len(v)):
  day=k+1;decline=0. if day<4 else np.log((np.mean(v[max(0,k-2):k+1])+1)/(np.mean(v[:max(1,k-2)])+1))
  out.append([np.log(day),decline])
 return np.array(out)
for resource in p['resources']:
 groups={c:d[(d.camp==c)&(d.resource==resource)].sort_values('day') for c in p['complete_camps']}
 for c,a in groups.items():
  assert np.array_equal(a.day.to_numpy(),np.arange(1,len(a)+1))
  assert np.isfinite(a.daily_gain_kcal_per_capita).all() and (a.daily_gain_kcal_per_capita>=0).all()
 fx={c:features(a.daily_gain_kcal_per_capita.to_numpy()) for c,a in groups.items()};ys={c:np.r_[np.zeros(len(a)-1),1] for c,a in groups.items()}
 for held in groups:
  train=[c for c in groups if c!=held];X=np.concatenate([fx[c] for c in train]);y=np.concatenate([ys[c] for c in train]);pred={'constant':np.full(len(groups[held]),(y.sum()+1)/(len(y)+2))};fitted={}
  for name,ncol in [('camp_age',1),('camp_age_food',2)]:
   model=make_pipeline(StandardScaler(),LogisticRegression(C=1,solver='lbfgs',max_iter=1000));model.fit(X[:,:ncol],y);pred[name]=model.predict_proba(fx[held][:,:ncol])[:,1];fitted[name]=model
  # Strict focal-future isolation: all features/predictions before a changed return remain exact.
  original=groups[held].daily_gain_kcal_per_capita.to_numpy()
  for k in range(1,len(original)):
   changed=original.copy();changed[k:]+=100000
   np.testing.assert_array_equal(features(changed)[:k],fx[held][:k])
  for model,v in pred.items():
   for i,prob in enumerate(v):rows.append({'resource':resource,'camp':held,'day':i+1,'leave':int(ys[held][i]),'model':model,'probability':float(prob)})
   ll=-(ys[held]*np.log(v)+(1-ys[held])*np.log(1-v)).mean();bs=((v-ys[held])**2).mean();folds.append({'resource':resource,'camp':held,'model':model,'log_loss':float(ll),'brier':float(bs)})
a=pd.DataFrame(folds);summary=a.groupby(['resource','model'])[['log_loss','brier']].mean().reset_index();comparisons=[];rng=np.random.default_rng(p['bootstrap']['seed']);ids=rng.integers(0,7,size=(p['bootstrap']['replicates'],7))
for resource in p['resources']:
 pivot=a[a.resource==resource].pivot(index='camp',columns='model',values='log_loss')
 for base in ['constant','camp_age']:
  diff=(pivot.camp_age_food-pivot[base]).to_numpy();ci=np.quantile(diff[ids].mean(axis=1),[.025,.975]);comparisons.append({'resource':resource,'baseline':base,'candidate':'camp_age_food','log_loss_difference':float(diff.mean()),'bootstrap95':ci.tolist()})
result={'schema':'batek-departure-results/1','protocolSha256':hashlib.sha256((O/'departure-protocol.json').read_bytes()).hexdigest(),'exposureDaysPerResource':int(len(d[(d.resource=='tot')&d.camp.isin(p['complete_camps'])])),'departuresPerResource':7,'summary':summary.to_dict('records'),'comparisons':comparisons,'futureIsolationPassed':True,'adoption':{}}
for resource in p['resources']:
 scores=summary[summary.resource==resource].set_index('model');result['adoption'][resource]=bool(all(c['bootstrap95'][1]<0 for c in comparisons if c['resource']==resource) and all(scores.loc['camp_age_food','brier']<=scores.loc[b,'brier'] for b in ['constant','camp_age']))
result['inputSha256']=hashlib.sha256((O/'daily-gains.csv').read_bytes()).hexdigest()
result['software']={'numpy':np.__version__,'pandas':pd.__version__,'scikit-learn':sklearn.__version__}
result['scope']='development evidence only; even passing model requires independent validation; no default change'
pd.DataFrame(rows).to_csv(O/'departure-predictions.csv',index=False);a.to_csv(O/'departure-camp-scores.csv',index=False);(O/'departure-results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
