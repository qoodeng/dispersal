import fs from 'node:fs';import assert from 'node:assert/strict';
import {runNative} from './run-food-native.mjs';import {summarize} from '../src/spatial-runner.ts';
const results=[];
for(let seed=0;seed<8;seed++)for(const domain of ['20km','extended-640km']) {
 const w=JSON.parse(fs.readFileSync(`research/food-experiment/mesh-fixtures/${domain}.json`));
 const frames=runNative(w,{seed,coupled:true,knowledge:'learned',foodSupply:2000,seasonality:.8,months:600});
 for(const f of frames){assert.equal(f.people,frames[0].people+f.births-f.deaths);assert(f.food.balanced);for(const g of f.groups)for(const p of [g.from,g.to])if(p)assert(p.every(v=>Math.abs(v)<=w.extent/2));}
 const last=frames.at(-1);results.push({seed,domain,...summarize(frames),...last.outcomes});
}
assert(results.some(r=>r.outerBoundaryAttempts>0));
fs.writeFileSync('research/food-experiment/m1-boundary-checks.json',JSON.stringify({description:'Closed 320/640 km domains with identical original camps, cells and origins; report domain effects without asserting invariance at outer boundaries.',results,pass:true},null,2)+'\n');
console.log(results.map(r=>({seed:r.seed,domain:r.domain,attempts:r.outerBoundaryAttempts,net:r.meanNetKm})));
