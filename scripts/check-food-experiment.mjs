import fs from 'node:fs';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import {runSpatial,summarize} from '../src/spatial-runner.ts';
const binary=fs.readFileSync('public/spatial/engine.wasm');
const world=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const settings=(seed=123,foodSupply=2000)=>({seed,foodSupply,density:'central',mode:'food'});
function verify(frames,w=world) {
  assert.equal(frames.length,1201);
  const initial=frames[0].people;const seen=new Set();
  for(const f of frames) {
    assert.equal(f.people,initial+f.births-f.deaths);
    assert.equal(f.people,f.groups.reduce((a,g)=>a+g.people,0));
    const q=f.food;
    assert.equal(q.balanced,true);
    for(const v of [q.initialKcal,q.renewedKcal,q.stockKcal,q.carriedKcal,q.consumedKcal,q.spoiledKcal,q.unmetKcal,...q.stocksKcal]) assert.ok(Number.isFinite(v)&&v>=0);
    assert.ok(Math.abs(q.initialKcal+q.renewedKcal-q.stockKcal-q.carriedKcal-q.consumedKcal-q.spoiledKcal)<=1e-8*Math.max(1,q.initialKcal+q.renewedKcal));
    assert.ok(Math.abs(q.stocksKcal.reduce((a,b)=>a+b,0)-q.stockKcal)<=w.sites.length*.00051);
    for(const g of f.groups) for(const v of Object.values(g.food)) assert.ok(Number.isFinite(v)&&v>=0);
    for(const t of f.trips) {
      const key=`${t.id}:${t.departure}`;assert.ok(!seen.has(key));seen.add(key);
      assert.ok(t.arrival>t.departure);
      const d=Math.hypot(t.to[0]-t.from[0],t.to[1]-t.from[1]);
      assert.ok(d<=(q.policy==='food'?30:60)+1e-7);
      for(let j=0,n=Math.ceil(d/.1);j<=n;j++) {
        const x=t.from[0]+(t.to[0]-t.from[0])*j/n,y=t.from[1]+(t.to[1]-t.from[1])*j/n;
        const i=Math.floor((y+60)/w.cellKm)*w.width+Math.floor((x+60)/w.cellKm);
        assert.equal(w.land[i],1);
      }
    }
    assert.equal(seen.size,f.mobility.reduce((n,m)=>n+m.departures,0),"every departure must be exported exactly once, including snapshot boundaries");
  }
}
const results=[];const started=performance.now();let central;
for(const rate of [500,2000,8000]) for(const seed of [1,42,123]) {
  const start=performance.now();
  const food=await runSpatial(binary,world,settings(seed,rate),'food');
  const baseline=await runSpatial(binary,world,settings(seed,rate),'scheduled');
  verify(food);verify(baseline);
  // Matched demographic random streams must survive differing journey counts.
  assert.deepEqual(food.map(f=>[f.people,f.births,f.deaths]),baseline.map(f=>[f.people,f.births,f.deaths]));
  results.push({seed,rate,food:summarize(food),scheduled:summarize(baseline),elapsedMs:performance.now()-start,serializedBytes:Buffer.byteLength(JSON.stringify({food,baseline}))});
  console.log(JSON.stringify(results.at(-1)));
  if(rate===2000&&seed===123)central=food;
}
const replay=await runSpatial(binary,world,settings(),'food');assert.deepEqual(central,replay);
const refined=await runSpatial(binary,world,settings(),'food',undefined,.5);verify(refined);
const a=summarize(central),b=summarize(refined);
const convergence={daily:a,halfDaily:b,deficitFractionDifference:Math.abs(a.deficitFraction-b.deficitFraction),target:.02};
convergence.passed=convergence.deficitFractionDifference<=convergence.target;
const report={schema:'controlled-food-checks/1',engineSha256:crypto.createHash('sha256').update(binary).digest('hex'),results,replay:true,populationAccounting:true,foodAccounting:true,landRoutes:true,demographicPairing:true,convergence,totalMs:performance.now()-started,scope:'Synthetic food comparison; no empirical validation or adoption of rejected departure fit'};
fs.mkdirSync('research/food-experiment',{recursive:true});
fs.writeFileSync('research/food-experiment/checks.json',JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({convergence,totalMs:report.totalMs}));
if(!convergence.passed)process.exitCode=1;
