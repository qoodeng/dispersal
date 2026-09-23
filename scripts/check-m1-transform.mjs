import fs from 'node:fs';import assert from 'node:assert/strict';import {runNative} from './run-food-native.mjs';
const base=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const sites=Array.from({length:6},()=>[0,0]);for(let i=0;i<180;i++){const a=2*Math.PI*i/180;sites.push([25*Math.cos(a),25*Math.sin(a)]);}
const edges=[];for(let a=0;a<sites.length;a++)for(let b=a+1;b<sites.length;b++){const d=Math.hypot(sites[a][0]-sites[b][0],sites[a][1]-sites[b][1]);if(d>0&&d<=55)edges.push([a,b,d]);}
const world={...base,width:32,cellKm:5,extent:160,sites,elevation:Array(1024).fill(100),land:Array(1024).fill(1),regional:{initialSites:[0,1,2,3,4,5],origin:[0,0],edges,foodCells:[[0,0,20,0],...sites.slice(6).map(p=>[...p,20,0])],access:sites.map((_,i)=>[i,i<6?0:i-5,1])},ecology:{...base.ecology,referencePeople:sites.map(()=>1),referenceLow:sites.map(()=>1),referenceHigh:sites.map(()=>1),areaKm2:sites.map(()=>20)}};

const transformed=structuredClone(world),angle=Math.PI/7,c=Math.cos(angle),s=Math.sin(angle);
transformed.sites=world.sites.map(([x,y])=>[40+c*x-s*y,-30+s*x+c*y]);transformed.regional.origin=[40,-30];
transformed.regional.edges=world.regional.edges.map(([a,b])=>[a,b,Math.hypot(transformed.sites[a][0]-transformed.sites[b][0],transformed.sites[a][1]-transformed.sites[b][1])]);
transformed.regional.foodCells=world.regional.foodCells.map(([x,y,area,phase])=>[40+c*x-s*y,-30+s*x+c*y,area,phase]);
for(let seed=0;seed<32;seed++){
 const settings={seed,coupled:true,knowledge:'learned',months:0,foodSupply:2000};
 const a=runNative(world,settings)[0],b=runNative(transformed,settings)[0];
 assert.deepEqual(a.trips.map(t=>[t.id,t.site]),b.trips.map(t=>[t.id,t.site]));
 a.trips.forEach((t,i)=>assert(Math.abs(t.arrival-b.trips[i].arrival)<1e-12));
}
fs.writeFileSync('research/food-experiment/m1-transform-checks.json',JSON.stringify({seeds:32,rotationRadians:angle,translationKm:[40,-30],fixture:'Isotropic equal-food ring; physical camps, origins and resource coordinates transformed together',choicesAndArrivalTimesInvariant:true,pass:true},null,2)+'\n');console.log('Rotation/translation covariance passed.');
