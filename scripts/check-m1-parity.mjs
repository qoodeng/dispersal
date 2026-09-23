import fs from 'node:fs';
import assert from 'node:assert/strict';
import {readSpatialWorld} from './read-spatial-world.mjs';
import {runNative} from './run-food-native.mjs';
import {runSpatial,summarize} from '../src/spatial-runner.ts';
const world=readSpatialWorld('public/spatial/connected/region-100000.json'),binary=fs.readFileSync('public/spatial/engine.wasm'),results=[];
function compare(a,b,path='frame') {
 if(typeof a==='number'){assert(Math.abs(a-b)<=1e-8*Math.max(1,Math.abs(a),Math.abs(b)),`${path}: ${a} vs ${b}`);return;}
 if(a&&typeof a==='object'){assert.deepEqual(Object.keys(a),Object.keys(b),path);for(const k of Object.keys(a))compare(a[k],b[k],`${path}.${k}`);return;}
 assert.equal(a,b,path);
}
for(const coupled of [true,false])for(const policy of ['food','scheduled']) {
 const settings={seed:123,foodSupply:2000,density:'central',mode:'food',policy,seasonality:.8,knowledge:'learned',coupled};
 const native=runNative(world,settings),t=performance.now(),wasm=await runSpatial(binary,world,settings,policy),wasmMs=performance.now()-t;
 assert.equal(native.length,wasm.length);native.forEach((f,i)=>compare(f,wasm[i],`month${i}`));
 results.push({coupled,policy,frames:wasm.length,wasmMs,summary:summarize(wasm),pass:true});console.log({coupled,policy,wasmMs});
}
fs.writeFileSync('research/food-experiment/m1-parity.json',JSON.stringify({relativeTolerance:1e-8,results,pass:true},null,2)+'\n');
