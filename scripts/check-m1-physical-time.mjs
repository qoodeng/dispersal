import fs from 'node:fs';import assert from 'node:assert/strict';import {runNative} from './run-food-native.mjs';import {summarize} from '../src/spatial-runner.ts';
const w=JSON.parse(fs.readFileSync('research/food-experiment/mesh-fixtures/10km.json')),results=[],count=Number(process.argv[2]??32);
const median=v=>{v=[...v].sort((a,b)=>a-b);return (v[Math.floor((v.length-1)/2)]+v[Math.floor(v.length/2)])/2};
for(let seed=0;seed<count;seed++){
 const pair=[];
 for(const stepDays of [1,.5]){
  const f=runNative(w,{seed,coupled:true,knowledge:'learned',foodSupply:2000,seasonality:.8,months:600,stepDays});
  for(const t of f){assert.equal(t.people,f[0].people+t.births-t.deaths);assert(t.food.balanced);}
  const trips=f.flatMap(t=>t.trips),arrivals=f[0].groups.map(g=>trips.find(t=>t.id===g.id&&Math.hypot(t.to[0]-g.from[0],t.to[1]-g.from[1])>=100)?.arrival??50);
  pair.push({...summarize(f),...f.at(-1).outcomes,arrivalYears:median(arrivals),finalCondition:f.at(-1).groups.reduce((n,g)=>n+g.people*g.condition.deficit,0)/Math.max(1,f.at(-1).people)});
 }
 results.push({seed,coarse:pair[0],fine:pair[1]});if(seed%8===7)console.log('physical time seeds',seed+1);
}
const metrics=Object.fromEntries(['people','births','deaths','arrivalYears','meanNetKm','departures','residentPersonDays','travelPersonDays','finalCondition'].map(k=>{const coarse=median(results.map(r=>r.coarse[k])),fine=median(results.map(r=>r.fine[k])),change=Math.abs(coarse-fine)/Math.max(1e-12,Math.abs(coarse),Math.abs(fine));return[k,{coarse,fine,change,pass:change<.02}]}));
const report={seeds:count,fixture:'320 km physical foraging domain, 10 km camps, 2.5 km shared resource quadrature; 50 years',metrics,results,pass:Object.values(metrics).every(r=>r.pass)};fs.writeFileSync('research/food-experiment/m1-physical-time.json',JSON.stringify(report,null,2));console.log({seeds:count,metrics,pass:report.pass});if(!report.pass)process.exitCode=1;
