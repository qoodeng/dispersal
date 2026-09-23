"""Reacquire pinned author inputs and verify recorded checksums."""
from pathlib import Path
import hashlib,json,urllib.request
root=Path(__file__).resolve().parents[1]/'research/resources/batek'
for item in json.loads((root/'acquisition.json').read_text())['files']:
 p=root/item['file']
 if p.exists() and hashlib.sha256(p.read_bytes()).hexdigest()==item['sha256']:continue
 with urllib.request.urlopen(item['url'],timeout=30) as r:data=r.read()
 if hashlib.sha256(data).hexdigest()!=item['sha256']:raise ValueError('Source changed: '+item['file'])
 p.write_bytes(data)
print('All pinned author inputs verified')
