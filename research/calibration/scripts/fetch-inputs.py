"""Fetch only the two pinned inputs needed to reproduce extract-evidence.py.
Run with Python 3; pdftotext (Poppler) is required. Other acquired literature is
recorded in acquisition-manifest.json and need not be downloaded to validate tables.
"""
import pathlib,json,urllib.request,hashlib,zipfile,subprocess
R=pathlib.Path(__file__).resolve().parents[1];C=R/'cache';C.mkdir(exist_ok=True)
manifest=json.loads((R/'acquisition-manifest.json').read_text())
for item in manifest['acquisitions']:
 name=pathlib.Path(item['local_path']).name
 if name not in {'sahul-code.zip','spratt2016-si.pdf'}:continue
 p=R/item['local_path']
 if not p.exists():
  with urllib.request.urlopen(item['url'],timeout=120) as response:p.write_bytes(response.read())
 if hashlib.sha256(p.read_bytes()).hexdigest()!=item['sha256']:raise RuntimeError('Source bytes changed: '+name)
 print('Verified',name)
with zipfile.ZipFile(C/'sahul-code.zip') as z:
 matches=[n for n in z.namelist() if not n.startswith('__MACOSX/') and n.endswith('/CSV files.zip')]
 if len(matches)!=1:raise RuntimeError('Unexpected archive structure; inspect manually')
 import io
 with zipfile.ZipFile(io.BytesIO(z.read(matches[0]))) as data:
  names=[n for n in data.namelist() if not n.startswith('__MACOSX/') and pathlib.PurePosixPath(n).name=='world2013lifetable.csv']
  if len(names)!=1:raise RuntimeError('Unexpected data archive structure')
  target=C/'sahul-code';target.mkdir(exist_ok=True);(target/'world2013lifetable.csv').write_bytes(data.read(names[0]))
subprocess.run(['pdftotext','-layout',str(C/'spratt2016-si.pdf'),str(C/'spratt2016-si.txt')],check=True)
