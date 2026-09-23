import fs from 'node:fs';import assert from 'node:assert/strict';
import {runNative} from './run-food-native.mjs';import {summarize} from '../src/spatial-runner.ts';
const source=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const results=[];
function check(frames){for(const f of frames){assert.equal(f.people,frames[0].people+f.births-f.deaths);assert.equal(f.food.balanced,true);assert(f.groups.every(g=>Number.isInteger(g.people)&&g.people>=0&&Number.isFinite(g.condition.deficit)));}const firstArrivals=frames[0].groups.map(g=>frames.flatMap(f=>f.trips).find(t=>t.id===g.id)?.arrival).filter(n=>n!==undefined).sort((a,b)=>a-b);return {medianFirstArrivalYears:firstArrivals.length?(firstArrivals[Math.floor((firstArrivals.length-1)/2)]+firstArrivals[Math.floor(firstArrivals.length/2)])/2:null,...summarize(frames),...frames.at(-1).outcomes,scarcityDeaths:frames.at(-1).scarcityDeaths};}
const median=v=>{v=[...v].sort((a,b)=>a-b);return (v[Math.floor((v.length-1)/2)]+v[Math.floor(v.length/2)])/2};
const rel=(a,b)=>Math.abs(a-b)/Math.max(1e-12,Math.abs(a),Math.abs(b));
for(let seed=0;seed<32;seed++){
 const settings={seed,coupled:true,knowledge:'learned',months:120};
 const none=check(runNative(source,{...settings,foodSupply:0}));assert.equal(none.people,0);
 const abundant=check(runNative(source,{...settings,foodSupply:100000}));
 const reference=check(runNative(source,{...settings,foodSupply:100000,coupled:false}));
 assert.equal(abundant.people,reference.people);assert.equal(abundant.births,reference.births);assert.equal(abundant.deaths,reference.deaths);assert.equal(abundant.scarcityDeaths,0);
 const coarse=check(runNative(source,{...settings,months:600,foodSupply:2000,seasonality:.8,stepDays:1}));
 const fine=check(runNative(source,{...settings,months:600,foodSupply:2000,seasonality:.8,stepDays:.5}));
 results.push({seed,none,abundant,reference,coarse,fine});
 fs.writeFileSync('research/food-experiment/m1-integrated-progress.json',JSON.stringify(results,null,2));
 console.log(JSON.stringify({seed,coarsePeople:coarse.people,finePeople:fine.people,coarseNet:coarse.meanNetKm,fineNet:fine.meanNetKm}));
}
const temporal=Object.fromEntries(['people','births','deaths','medianFirstArrivalYears','meanNetKm','departures','residentPersonDays','travelPersonDays'].map(k=>{let a=median(results.map(r=>r.coarse[k])),b=median(results.map(r=>r.fine[k]));return [k,{coarse:a,fine:b,relativeChange:rel(a,b),pass:rel(a,b)<.02}]}));
const artifact={protocol:'docs/M1-ACCEPTANCE-PROTOCOL.md',seeds:32,noFoodPassed:true,abundancePassed:true,temporal,results,accepted:Object.values(temporal).every(g=>g.pass)};
fs.writeFileSync('research/food-experiment/m1-integrated-checks.json',JSON.stringify(artifact,null,2)+'\n');console.log(JSON.stringify({temporal,accepted:artifact.accepted},null,2));
if(!artifact.accepted)process.exitCode=1;
