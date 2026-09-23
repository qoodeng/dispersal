import fs from 'node:fs';import assert from 'node:assert/strict';import {runNative} from './run-food-native.mjs';
const base=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const sites=Array.from({length:6},()=>[0,0]);for(let i=0;i<180;i++){const a=2*Math.PI*i/180;sites.push([25*Math.cos(a),25*Math.sin(a)]);}
const edges=[];for(let a=0;a<sites.length;a++)for(let b=a+1;b<sites.length;b++){const d=Math.hypot(sites[a][0]-sites[b][0],sites[a][1]-sites[b][1]);if(d>0&&d<=55)edges.push([a,b,d]);}
const world={...base,width:32,cellKm:5,extent:160,sites,elevation:Array(1024).fill(100),land:Array(1024).fill(1),regional:{initialSites:[0,1,2,3,4,5],origin:[0,0],edges,foodCells:[[0,0,20,0],...sites.slice(6).map(p=>[...p,20,0])],access:sites.map((_,i)=>[i,i<6?0:i-5,1])},ecology:{...base.ecology,referencePeople:sites.map(()=>1),referenceLow:sites.map(()=>1),referenceHigh:sites.map(()=>1),areaKm2:sites.map(()=>20)}};
let n=0;const moments=Array.from({length:4},()=>[0,0]);
for(let seed=0;seed<8192;seed++){const frames=runNative(world,{seed,coupled:true,knowledge:'learned',months:0,foodSupply:2000});for(const t of frames[0].trips){const a=Math.atan2(t.to[1],t.to[0]);n++;for(let m=1;m<=4;m++){moments[m-1][0]+=Math.cos(m*a);moments[m-1][1]+=Math.sin(m*a);}}}
const harmonics=moments.map(([x,y],i)=>({order:i+1,amplitude:2*Math.hypot(x,y)/n}));
// 99% radial bound for each uniform-angle Fourier mean, Bonferroni across 4 harmonics.
const samplingBound=2*Math.sqrt(-Math.log(.01/4)/n);
const report={fixture:'Isotropic equal-resource destinations on a 25 km ring, fixed shared starting patch. First choices only, away from boundaries.',seeds:8192,departures:n,harmonics,samplingBound,criterion:'Each directional amplitude plus simultaneous Monte Carlo sampling bound is below 0.05.',pass:n===49152&&harmonics.every(h=>h.amplitude+samplingBound<.05)};
fs.writeFileSync('research/food-experiment/m1-isotropy-checks.json',JSON.stringify(report,null,2));console.log(report);assert(report.pass);
