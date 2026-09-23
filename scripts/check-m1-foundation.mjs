import fs from 'node:fs';import assert from 'node:assert/strict';
import {runSpatial,summarize} from '../src/spatial-runner.ts';
const group=(id,people=10)=>({id,people,from:[0,0],to:[0,0],arrival:0,departure:0,reason:'Staying',site:0,cost:0,shortfall:0,provisions:0});
const frame=(year,groups,trips=[])=>({year,groups,trips,people:groups.reduce((n,g)=>n+g.people,0),births:0,deaths:0,mobility:[],blockedEdges:0});
let s=summarize([frame(0,[group(0)]),frame(100,[group(0)])]);
assert.equal(s.medianCompletedResidenceDays,null);assert.equal(s.medianCurrentResidenceYears,100);assert.equal(s.groupsResidentAtLeastYear,1);
s=summarize([frame(0,[group(0)]),frame(100,[group(0,0)])]);assert.equal(s.medianCurrentResidenceYears,null);
const t={id:0,from:[0,0],to:[20,0],departure:99,arrival:101,site:1,reason:'Traveling'};
s=summarize([frame(0,[group(0)]),frame(100,[{...group(0),...t}],[t])]);assert.equal(s.medianCurrentResidenceYears,null);assert.equal(s.traveledKm,10);
const world=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json')),binary=fs.readFileSync('public/spatial/engine.wasm');
const results=[];
for(const foodSupply of [0,500,2000]){
 const f=await runSpatial(binary,world,{seed:123,density:'central',mode:'food',foodSupply,seasonality:.8,knowledge:'learned'},'food');
 const summary=summarize(f);assert.equal(summary.energyBalanced,true);
 if(foodSupply===0){assert.equal(summary.medianCurrentResidenceYears,100);assert.equal(summary.deficitFraction,1);}
 results.push({foodSupply,...summary});
}
await assert.rejects(runSpatial(binary,{...world,width:1025},{seed:1,foodSupply:2000,density:'central',mode:'food'},'food'),/capacity/);
const viability={status:'BLOCKED',reason:'No resource-condition or scarcity-demography response. Zero food still permits reference population growth.',zeroFoodPopulation:results[0].people};
fs.writeFileSync('research/food-experiment/m1-foundation-checks.json',JSON.stringify({reportingTests:true,domainCapacityGuard:true,results,viability},null,2)+'\n');console.log({results,viability});
