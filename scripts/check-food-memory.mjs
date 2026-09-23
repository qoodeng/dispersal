import fs from 'node:fs';import assert from 'node:assert/strict';import crypto from 'node:crypto';
import {runSpatial,summarize} from '../src/spatial-runner.ts';
const binary=fs.readFileSync('public/spatial/engine.wasm'),world=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const results=[];let central;
function verify(frames){
 let trips=0; const arrivals=new Map(),previous=new Map();
 for(const f of frames){
 for(const t of f.trips){const ts=arrivals.get(t.id)||[];ts.push(t);arrivals.set(t.id,ts);}
 for(const g of f.groups){if(!g.knowledge)continue;const origin=frames[0].groups.find(a=>a.id===g.id).from;const known=new Set([world.sites.findIndex(p=>Math.hypot(p[0]-origin[0],p[1]-origin[1])<1e-6)]);for(const t of arrivals.get(g.id)||[])if(t.arrival<=f.year)known.add(t.site);
 assert.ok(g.knowledge.observedSites<=known.size,'no observation before arrival');assert.ok(g.knowledge.observedSites>=(previous.get(g.id)||0));previous.set(g.id,g.knowledge.observedSites);
 if(g.knowledge.destinationObservationAgeDays!==null)assert.ok(g.knowledge.destinationObservationAgeDays>=0&&known.has(g.site));
 }
 const q=f.food; assert.equal(q.balanced,true);assert.equal(f.people,426+f.births-f.deaths);
 assert.equal(f.people,f.groups.reduce((a,g)=>a+g.people,0));
 assert.ok(q.renewalMultiplier>=.2-1e-10&&q.renewalMultiplier<=1.8+1e-10);
 for(const [i,stock] of q.stocksKcal.entries())assert.ok(Number.isFinite(stock)&&stock>=0&&stock<=world.ecology.areaKm2[i]*frames.rate*60+.001);
 for(const t of f.trips){trips++;const distance=Math.hypot(t.to[0]-t.from[0],t.to[1]-t.from[1]);assert.ok(t.arrival>t.departure);for(let j=0,n=Math.ceil(distance/.1);j<=n;j++){const x=t.from[0]+(t.to[0]-t.from[0])*j/n,y=t.from[1]+(t.to[1]-t.from[1])*j/n;assert.equal(world.land[Math.floor((y+60)/world.cellKm)*world.width+Math.floor((x+60)/world.cellKm)],1);}}
 assert.equal(trips,f.mobility.reduce((n,g)=>n+g.departures,0));
 }
}
for(const rate of [500,2000,8000]) for(const seed of [1,42,123]){
 const settings={seed,foodSupply:rate,density:'central',mode:'food',seasonality:.8,knowledge:'learned'};
 const food=await runSpatial(binary,world,settings,'food'),scheduled=await runSpatial(binary,world,settings,'scheduled');
 for(const frames of [food,scheduled]){frames.rate=rate;verify(frames);delete frames.rate;}
 assert.deepEqual(food.map(f=>[f.people,f.births,f.deaths]),scheduled.map(f=>[f.people,f.births,f.deaths]));
 const constant=await runSpatial(binary,world,{...settings,knowledge:'perfect'},'food');
 assert.notDeepEqual(food.flatMap(f=>f.trips),constant.flatMap(f=>f.trips),'observed knowledge must reach movement decisions');
 results.push({rate,seed,learned:summarize(food),perfect:summarize(constant),scheduled:summarize(scheduled)});
 if(rate===2000&&seed===123)central=food;
}
const settings={seed:123,foodSupply:2000,density:'central',mode:'food',seasonality:.8,knowledge:'learned'};
assert.deepEqual(central,await runSpatial(binary,world,settings,'food'));
const half=await runSpatial(binary,world,settings,'food',undefined,.5);half.rate=2000;verify(half);delete half.rate;
const convergence={daily:summarize(central),halfDaily:summarize(half)};
convergence.deficitDifference=Math.abs(convergence.daily.deficitFraction-convergence.halfDaily.deficitFraction);
fs.writeFileSync('research/food-experiment/memory-checks.json',JSON.stringify({engineSha256:crypto.createHash('sha256').update(binary).digest('hex'),results,convergence,replay:true,accounting:true,landRoutes:true,demographicPairing:true},null,2)+'\n');
console.log(JSON.stringify({results,convergence}));
assert.ok(convergence.deficitDifference<=.02,'registered deficit sensitivity gate');
