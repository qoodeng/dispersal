import fs from 'node:fs';import assert from 'node:assert/strict';
import {runSpatial,summarize} from '../src/spatial-runner.ts';
const binary=fs.readFileSync('public/spatial/engine.wasm');const results=[];
for(const name of fs.readdirSync('public/spatial/dated').filter(n=>/^(aqaba|sinai|nile|levant|horn|mandeb)-\d+\.json$/.test(n))){
 const w=JSON.parse(fs.readFileSync('public/spatial/dated/'+name));
 if(w.ecology.referencePeople.filter(n=>n!==null&&n>0).length<6){
  for(const policy of ['food','scheduled'])await assert.rejects(runSpatial(binary,w,{seed:123,density:'central',mode:'food',foodSupply:2000,seasonality:0.8,knowledge:'learned'},policy),/Too little supported climate coverage/);
  results.push({environment:name,status:'correctly rejected: insufficient supported initial sites'});console.log(name,'correctly rejected');continue;
 }
 const pair=[];const start=performance.now();
 for(const policy of ['food','scheduled']){
  const frames=await runSpatial(binary,w,{seed:123,density:'central',mode:'food',foodSupply:2000,seasonality:0.8,knowledge:'learned'},policy);
  for(const f of frames){assert.equal(f.people,frames[0].people+f.births-f.deaths);assert.equal(f.food.balanced,true);assert.ok(f.food.stocksKcal.every(n=>Number.isFinite(n)&&n>=0));}
  pair.push(summarize(frames));
 }
 assert.equal(pair[0].people,pair[1].people);
 results.push({environment:name,food:pair[0],scheduled:pair[1],elapsedMs:performance.now()-start});console.log(name,'passed');
}
assert.equal(results.length,36);
fs.writeFileSync('research/food-experiment/environment-checks.json',JSON.stringify({schema:'food-environment-checks/2',years:100,seed:123,supply:2000,seasonality:0.8,knowledge:'learned',results,limits:'Accounting and initialization coverage; not historical validity or full spatial convergence'},null,2)+'\n');
