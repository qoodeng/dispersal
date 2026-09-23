"""Fail closed on untraceable or out-of-domain scientific parameters.
An academic citation alone never upgrades a project assumption to measured fact.
"""
import json,pathlib,sys,hashlib
R=pathlib.Path(__file__).resolve().parents[1]
def validate(config,sources):
 errors=[];mode=config.get('mode');domain=config.get('domain')
 if mode not in ['reference','exploratory','historical']:errors.append('Unknown run mode')
 if not domain:errors.append('Missing target domain')
 for p in config.get('parameters',[]):
  name=p.get('id','unnamed')
  for key in ['id','units','meaning','basis','domain','uncertainty']:
   if key not in p:errors.append(f'{name}: missing {key}')
  if p.get('basis') not in ['measured','reconstructed','analogue','assumption']:errors.append(f'{name}: unsupported evidence category')
  if p.get('basis')!='assumption':
   if p.get('source_id') not in sources:errors.append(f'{name}: unregistered source')
   if not p.get('locator'):errors.append(f'{name}: missing exact source locator')
  elif not p.get('rationale'):errors.append(f'{name}: assumption has no rationale')
  if p.get('domain')!=domain and not p.get('transfer_model'):errors.append(f'{name}: evidence domain differs; transfer model missing')
  if p.get('value') is None and p.get('distribution') is None:errors.append(f'{name}: neither value nor distribution supplied')
 if mode=='historical':
  for key in ['demographic_validation','geographic_validation','numerical_validation','observation_model','independent_validation']:
   artifact=config.get('artifacts',{}).get(key)
   if not isinstance(artifact,dict) or not artifact.get('path') or not artifact.get('sha256'):
    errors.append('Historical run blocked: missing pinned '+key);continue
   path=R/artifact['path']
   if not path.is_file():errors.append('Historical run blocked: missing artifact file '+key);continue
   if hashlib.sha256(path.read_bytes()).hexdigest()!=artifact['sha256']:errors.append('Historical run blocked: changed artifact '+key);continue
   try:report=json.loads(path.read_text())
   except (ValueError,UnicodeError):errors.append('Historical run blocked: invalid artifact '+key);continue
   if report.get('status')!='passed' or report.get('purpose')!=key:errors.append('Historical run blocked: artifact has not passed '+key)
   if report.get('model_revision')!=config.get('model_revision') or not config.get('model_revision'):errors.append('Historical run blocked: model revision mismatch '+key)
 return errors
if __name__=='__main__':
 sources={s['id'] for s in json.loads((R/'research/calibration/sources.json').read_text())['sources']}
 config=json.loads(pathlib.Path(sys.argv[1]).read_text());errors=validate(config,sources)
 print(json.dumps({'status':'blocked' if errors else 'traceability_passed','errors':errors,'note':'Traceability pass is not proof of historical truth.'},indent=2));sys.exit(1 if errors else 0)
