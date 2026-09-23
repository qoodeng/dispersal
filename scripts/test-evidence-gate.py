import importlib.util,json,copy,pathlib
p=pathlib.Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('gate',p/'evidence-gate.py');gate=importlib.util.module_from_spec(spec);spec.loader.exec_module(gate)
c=json.loads((p.parent/'research/world-v2/parameter-contract.json').read_text())
assert not gate.validate(c,{'gurven2007'})
x=copy.deepcopy(c);x['domain']='MIS5Arabia';assert any('transfer' in e for e in gate.validate(x,{'gurven2007'}))
x=copy.deepcopy(c);x['mode']='historical';assert len(gate.validate(x,{'gurven2007'}))==5
assert any('unregistered' in e for e in gate.validate(c,set()))
x=copy.deepcopy(c);x['parameters'][0]['value']=None;assert any('neither value' in e for e in gate.validate(x,{'gurven2007'}))
x=copy.deepcopy(c);x['parameters'][0]['basis']='assumption';assert any('rationale' in e for e in gate.validate(x,{'gurven2007'}))
print('6 evidence-boundary checks passed; no historical parameterization authorized.')
