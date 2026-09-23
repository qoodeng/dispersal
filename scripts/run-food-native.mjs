import {readSpatialWorld} from './read-spatial-world.mjs';
// Reproduce the same controlled model outside the browser, without extra libraries.
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {spawnSync} from 'node:child_process';
export function nativeInput(w,{seed=123,foodSupply=2000,density='central',policy='food',stepDays=1,months=1200,seasonality=0,knowledge='perfect',coupled=undefined,initialFraction=1,response={lagDays:30,maxHazardYear:12,threshold:.25}}={}) {
 const refs=density==='lower'?w.ecology.referenceLow:density==='upper'?w.ecology.referenceHigh:w.ecology.referencePeople;
 const land=w.elevation.map((z,i)=>z===null?2:(w.land?.[i]??(z>0?1:0)));
 return [`world ${w.width} ${w.cellKm}`,`heights ${w.elevation.length} ${w.elevation.map(z=>z??'NaN').join(' ')}`,`land ${land.length} ${land.join(' ')}`,`sites ${w.sites.length}`,w.sites.map((p,i)=>[...p,w.ecology.areaKm2[i],refs[i]??0].join(' ')).join('\n'),`ages ${w.fertility.length}`,w.fertility.map((r,i)=>[r,w.female[i],w.male[i]].join(' ')).join('\n'),`mobility ${w.ecology.mobilityPairs.length}`,w.ecology.mobilityPairs.map(p=>p.join(' ')).join('\n'),`run ${seed} ${policy==='food'?1:0} ${foodSupply} ${stepDays} ${months} ${seasonality} ${knowledge==='learned'?1:0} ${coupled===undefined?0:coupled?1:2} ${initialFraction} ${response.lagDays} ${response.maxHazardYear} ${response.threshold}`,...(w.regional?[`regional ${w.regional.origin.join(' ')} ${w.regional.edges.length}`,w.regional.edges.map(e=>e.join(' ')).join('\n'),String(w.regional.foodCells.length),w.regional.foodCells.map(c=>c.slice(2).join(' ')).join('\n'),String(w.regional.access.length),w.regional.access.map(a=>a.join(' ')).join('\n'),String(w.regional.initialSites?.length??0),(w.regional.initialSites??[]).join(' '),String(w.regional.destinationWeights?.length??0),(w.regional.destinationWeights??[]).join(' ')]:[])].join('\n');
}
export function runNative(w,settings){
 // File-backed stdin avoids stalled multi-megabyte synchronous pipe writes on macOS.
 const dir=fs.mkdtempSync(path.join(os.tmpdir(),'dispersal-native-'));let fd;
 try {
  const file=path.join(dir,'input');fs.writeFileSync(file,nativeInput(w,settings));fd=fs.openSync(file,'r');
  const r=spawnSync('engine/target/release/food-run',[],{stdio:[fd,'pipe','pipe'],encoding:'utf8',maxBuffer:500*1024*1024,timeout:120000});
  if(r.error)throw r.error;if(r.status!==0)throw Error(`Native run failed (${r.status??r.signal}): ${r.stderr}`);
  return r.stdout.trim().split('\n').map(line=>JSON.parse(line));
 } finally {if(fd!==undefined)fs.closeSync(fd);fs.rmSync(dir,{recursive:true});}
}

if(process.argv[1]?.endsWith('/run-food-native.mjs')){
 const file=process.argv[2]??'public/spatial/dated/aqaba-100000.json';
 const policy=process.argv[3]??'food';if(!['food','scheduled'].includes(policy))throw Error('Policy must be food or scheduled');
 const w=readSpatialWorld(file);const frames=runNative(w,{policy});
 process.stdout.write(JSON.stringify({schema:'controlled-food-native/1',settings:{seed:123,foodSupply:2000,density:'central',policy},experimentContract:JSON.parse(fs.readFileSync('public/spatial/resource-experiment.json')),world:w,frames})+'\n');
}
