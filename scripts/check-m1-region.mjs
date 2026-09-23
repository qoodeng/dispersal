import {readSpatialWorld} from './read-spatial-world.mjs';
import fs from 'node:fs';import assert from 'node:assert/strict';import {runNative} from './run-food-native.mjs';import {summarize} from '../src/spatial-runner.ts';
const w=readSpatialWorld('public/spatial/connected/region-100000.json');const results=[];
const first=Number(process.argv[2]??0),end=Number(process.argv[3]??32),out=`research/food-experiment/m1-region-${first}-${end}.json`;
for(let seed=first;seed<end;seed++){
 const t=performance.now(),frames=runNative(w,{seed,coupled:true,knowledge:'learned',seasonality:.8,foodSupply:2000});
 for(const f of frames){assert.equal(f.people,frames[0].people+f.births-f.deaths);assert(f.food.balanced);}
 const trips=frames.flatMap(f=>f.trips);const firstArrivals=frames[0].groups.map(g=>trips.find(t=>t.id===g.id)?.arrival??null);
 const f=frames.at(-1),result={seed,milliseconds:performance.now()-t,...summarize(frames),...f.outcomes,firstArrivals,scarcityDeaths:f.scarcityDeaths};results.push(result);fs.writeFileSync(out,JSON.stringify({world:'connected/region-100000.json',results},null,2));console.log(JSON.stringify(result));
}
