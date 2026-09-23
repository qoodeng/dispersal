import fs from 'node:fs';import assert from 'node:assert/strict';import {runNative} from './run-food-native.mjs';
const count=Number(process.argv[2]??32),results={20:[],10:[],5:[]};
const worlds=Object.fromEntries([20,10,5].map(k=>[k,JSON.parse(fs.readFileSync(`research/food-experiment/mesh-fixtures/${k}km.json`))]));
const median=v=>{v=[...v].sort((a,b)=>a-b);return (v[Math.floor((v.length-1)/2)]+v[Math.floor(v.length/2)])/2};
const mean=v=>v.reduce((a,b)=>a+b,0)/v.length;
const variance=v=>v.reduce((s,x)=>s+(x-mean(v))**2,0)/(v.length-1);
for(let seed=0;seed<count;seed++){
 for(const spacing of [20,10,5]){
  const f=runNative(worlds[spacing],{seed,coupled:true,knowledge:'learned',seasonality:.8,months:240});
  for(const t of f){assert.equal(t.people,f[0].people+t.births-t.deaths);assert(t.food.balanced);}
  const trips=f.flatMap(t=>t.trips),occupancy=Array(64).fill(0);
  for(const t of trips){const x=Math.max(0,Math.min(7,Math.floor((t.to[0]+160)/40))),y=Math.max(0,Math.min(7,Math.floor((t.to[1]+160)/40)));occupancy[y*8+x]+=1/trips.length;}
  const arrivals=f[0].groups.map(g=>trips.find(t=>t.id===g.id&&Math.hypot(t.to[0]-g.from[0],t.to[1]-g.from[1])>=100)?.arrival??20);
  results[spacing].push({seed,arrivals,censored:arrivals.filter(t=>t===20).length,occupancy,people:f.at(-1).people,departures:trips.length});
 }
 if(seed%8===7)console.log('mesh seeds',seed+1);
}
const medians=Object.fromEntries([20,10,5].map(k=>[k,median(results[k].flatMap(r=>r.arrivals))]));
const change=Math.abs(medians[10]-medians[5])/Math.max(medians[10],medians[5]);
const occupancy=Array.from({length:64},(_,i)=>{const a=results[10].map(r=>r.occupancy[i]),b=results[5].map(r=>r.occupancy[i]);const delta=Math.abs(mean(a)-mean(b)),limit=3.8*Math.sqrt((variance(a)+variance(b))/count);return {bin:i,coarse:mean(a),fine:mean(b),delta,simultaneousUncertainty:limit,pass:delta<=limit+1e-12};});
const report={protocol:'Fixed 320 km domain and six physical origins; actual irregular camp meshes at 20/10/5 km. Food quadrature, reach and geometry fixed. Arrival at 100 km displacement, censored at 20 years. Occupancy is destination visit fraction per 40 km bin; simultaneous 99% normal intervals use z=3.8 for 64 bins.',seeds:count,medians,relativeArrivalChange:change,arrivalPass:change<.05,occupancyPass:occupancy.every(o=>o.pass),occupancy,results};
fs.writeFileSync('research/food-experiment/m1-mesh-checks.json',JSON.stringify(report,null,2));console.log({seeds:count,medians,change,arrivalPass:report.arrivalPass,occupancyPass:report.occupancyPass});if(!report.arrivalPass||!report.occupancyPass)process.exitCode=1;
