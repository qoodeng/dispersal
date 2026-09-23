import fs from 'node:fs';
import assert from 'node:assert/strict';
import {runSpatial,summarize} from '../src/spatial-runner.ts';
const binary=fs.readFileSync('public/spatial/engine.wasm');
const world=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const results=[];
for(const seed of [1,42,123]) {
 const frames=await runSpatial(binary,world,{seed,foodSupply:2000,density:'central',mode:'food'},'food');
 const groups=new Map();
 for(const t of frames.flatMap(f=>f.trips)){const a=groups.get(t.id)||[];a.push(t);groups.set(t.id,a)}
 let reversals=0,total=0,uniqueEdges=0;const visits=[];
 for(const trips of groups.values()){
 const edges=new Set(),sites=new Set();
 trips.forEach((t,i)=>{total++;const from=JSON.stringify(t.from),to=JSON.stringify(t.to);edges.add([from,to].sort().join('|'));sites.add(to);if(i&&JSON.stringify(trips[i-1].from)===to)reversals++});uniqueEdges+=edges.size;visits.push(sites.size);
 }
 results.push({seed,...summarize(frames),immediateReturnFraction:reversals/total,uniqueUndirectedEdgesPerGroup:uniqueEdges/groups.size,uniqueDestinationsPerGroup:visits});
}
const baseline=JSON.parse(fs.readFileSync('research/food-experiment/routes-greedy-baseline.json'));
for(const [i,r] of results.entries()){
 assert.equal(r.energyBalanced,true); assert.equal(r.deficitFraction,0);
 assert.ok(r.uniqueDestinationsPerGroup.reduce((a,b)=>a+b,0)>2*baseline[i].uniqueDestinationsPerGroup.reduce((a,b)=>a+b,0),'route diversity regression on the recorded central-supply fixtures');
}
fs.writeFileSync('research/food-experiment/routes-current.json',JSON.stringify(results,null,2)+'\n');console.log(results);
