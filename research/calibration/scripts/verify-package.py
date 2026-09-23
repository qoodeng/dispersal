"""Validate evidence integrity and report scientific blockers without claiming calibration."""
import csv,json,pathlib,math,hashlib
R=pathlib.Path(__file__).resolve().parents[1]
sources=json.loads((R/'sources.json').read_text())['sources'];ids={s['id'] for s in sources}
assert len(ids)==len(sources)==33
assert all(s['url'].startswith('https://') and s['review_status'] and s['locator'] for s in sources)
a=list(csv.DictReader((R/'data/archaeological-observations.csv').open()))
assert len(a)==10 and len({x['record_id'] for x in a})==10
assert all(x['source_id'] in ids for x in a)
assert all(float(x['age_ka_as_published'])>0 and float(x['minus_ka'])>0 and float(x['plus_ka'])>0 for x in a)
assert next(x for x in a if x['record_id']=='aw1')['relationship_to_human_event'].startswith('minimum')
assert next(x for x in a if x['record_id']=='aw-esr')['uncertainty_convention']=='1 sigma'
assert next(x for x in a if x['record_id']=='aw1')['uncertainty_convention']=='2 sigma'
sea=list(csv.DictReader((R/'data/spratt2016-published-table.csv').open()))
assert len(sea)==799 and [float(x['age_ka']) for x in sea]==list(range(799))
assert all(float(x['q025'])<=float(x['q25'])<=float(x['median'])<=float(x['q75'])<=float(x['q975']) for x in sea)
g=json.loads((R/'data/growth-audit.json').read_text())
assert math.isclose(g['annual_log_growth_with_published_survival'],.0037275756227864373,rel_tol=1e-9)
assert g['annual_log_growth_if_survival_set_to_one']>g['annual_log_growth_with_published_survival']
for p in g['identifiability_demo']['pairs']:assert math.isclose(2*math.sqrt(p['r']*p['D']),p['speed_km_year'])
checks=['33 unique cited sources with locators and review status','10 unique archaeological records linked to registered sources','Minimum-age and asymmetric uncertainty distinctions retained','799 sequential sea-level rows with ordered quantiles','Published-input demographic eigenvalue reproduced','Synthetic growth–diffusion non-identifiability example checked']
manifest=json.loads((R/'acquisition-manifest.json').read_text())
verified=0
for x in manifest['acquisitions']:
 p=R/x['local_path']
 if p.exists() and x.get('sha256'):
  assert hashlib.sha256(p.read_bytes()).hexdigest()==x['sha256'];verified+=1
checks.append(f'{verified} locally retained acquisition hashes verified')
out={'integrity_checks':'passed','checks':checks,'historical_calibration':'not performed','historical_predictions_enabled':False,'unresolved_date_conventions':[x['record_id'] for x in a if x['uncertainty_convention'].startswith('unconfirmed')],'other_blockers':['Matched fertility data and transfer uncertainty','More independent regional benchmark chronologies','Sea-level datum and fine-scale route geometry','Scientific transport solver and synthetic parameter recovery','Posterior fit and held-out predictive evaluation']}
(R/'validation.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
