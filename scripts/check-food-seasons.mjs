import fs from 'node:fs';import assert from 'node:assert/strict';import crypto from 'node:crypto';
import {runSpatial,summarize} from '../src/spatial-runner.ts';
const binary=fs.readFileSync('public/spatial/engine.wasm'),world=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const results=[];let central;
function verify(frames){
 let trips=0;
 for(const f of frames){
 const q=f.food; assert.equal(q.balanced,true);assert.equal(f.people,426+f.births-f.deaths);
 assert.equal(f.people,f.groups.reduce((a,g)=>a+g.people,0));
 assert.ok(q.renewalMultiplier>=.2-1e-10&&q.renewalMultiplier<=1.8+1e-10);
 for(const [i,stock] of q.stocksKcal.entries())assert.ok(Number.isFinite(stock)&&stock>=0&&stock<=world.ecology.areaKm2[i]*frames.rate*60+.001);
 for(const t of f.trips){trips++;const distance=Math.hypot(t.to[0]-t.from[0],t.to[1]-t.from[1]);assert.ok(t.arrival>t.departure);for(let j=0,n=Math.ceil(distance/.1);j<=n;j++){const x=t.from[0]+(t.to[0]-t.from[0])*j/n,y=t.from[1]+(t.to[1]-t.from[1])*j/n;assert.equal(world.land[Math.floor((y+60)/world.cellKm)*world.width+Math.floor((x+60)/world.cellKm)],1);}}
 assert.equal(trips,f.mobility.reduce((n,g)=>n+g.departures,0));
 }
}
for(const rate of [500,2000,8000]) for(const seed of [1,42,123]){
 const settings={seed,foodSupply:rate,density:'central',mode:'food',seasonality:.8};
 const food=await runSpatial(binary,world,settings,'food'),scheduled=await runSpatial(binary,world,settings,'scheduled');
 for(const frames of [food,scheduled]){frames.rate=rate;verify(frames);delete frames.rate;}
 assert.deepEqual(food.map(f=>[f.people,f.births,f.deaths]),scheduled.map(f=>[f.people,f.births,f.deaths]));
 const constant=await runSpatial(binary,world,{...settings,seasonality:0},'food');
 assert.notDeepEqual(food.flatMap(f=>f.trips),constant.flatMap(f=>f.trips),'forcing must reach movement decisions');
 results.push({rate,seed,seasonal:summarize(food),constant:summarize(constant),scheduled:summarize(scheduled)});
 if(rate===2000&&seed===123)central=food;
}
const settings={seed:123,foodSupply:2000,density:'central',mode:'food',seasonality:.8};
assert.deepEqual(central,await runSpatial(binary,world,settings,'food'));
const half=await runSpatial(binary,world,settings,'food',undefined,.5);half.rate=2000;verify(half);delete half.rate;
const convergence={daily:summarize(central),halfDaily:summarize(half)};
convergence.deficitDifference=Math.abs(convergence.daily.deficitFraction-convergence.halfDaily.deficitFraction);
fs.writeFileSync('research/food-experiment/seasonal-checks.json',JSON.stringify({engineSha256:crypto.createHash('sha256').update(binary).digest('hex'),results,convergence,replay:true,accounting:true,landRoutes:true,demographicPairing:true},null,2)+'\n');
console.log(JSON.stringify({results,convergence}));
assert.ok(convergence.deficitDifference<=.02,'registered deficit sensitivity gate');
