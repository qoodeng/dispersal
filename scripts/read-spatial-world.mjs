import fs from 'node:fs';import path from 'node:path';import {createHash} from 'node:crypto';
export function readSpatialWorld(file){
 const w=JSON.parse(fs.readFileSync(file));
 if(w.regional?.dataParts){const cells=[],access=[];for(const p of w.regional.dataParts){const raw=fs.readFileSync(path.join(path.dirname(file),p.file));if(createHash('sha256').update(raw).digest('hex')!==p.sha256)throw Error('Regional input hash mismatch');const d=JSON.parse(raw);if(d.foodCells.length!==p.foodCells||d.access.length!==p.access)throw Error('Incomplete regional input');cells.push(...d.foodCells);access.push(...d.access);}w.regional.foodCells=cells;w.regional.access=access;}
 return w;
}
