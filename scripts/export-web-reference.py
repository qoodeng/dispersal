from pathlib import Path
import csv,json,shutil,hashlib
r=Path(__file__).resolve().parents[1];p=r/'public/reference';p.mkdir(exist_ok=True)
rows=list(csv.DictReader((r/'research/world-v2/reference-run.csv').open()))
checks=json.loads((r/'research/world-v2/reference-checks.json').read_text())
assert hashlib.sha256((r/'research/world-v2/reference-run.csv').read_bytes()).hexdigest()==checks['output_sha256']
data={'rows':[{k:(v=='true' if k=='accounting_valid' else float(v)) for k,v in row.items()} for row in rows],'checks':checks}
(p/'cohort.json').write_text(json.dumps(data,separators=(',',':')))
for a,b in [('research/world-v2/reference-run.csv','cohort.csv'),('research/world-v2/reference-checks.json','checks.json'),('research/world-v2/parameter-contract.json','parameters.json'),('research/world-v2/runtime/precipitation.bin','precipitation.bin'),('research/world-v2/runtime-manifest.json','manifest.json')]:shutil.copyfile(r/a,p/b)
