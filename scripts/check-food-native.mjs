import fs from 'node:fs';import assert from 'node:assert/strict';
import {runNative} from './run-food-native.mjs';import {runSpatial,summarize} from '../src/spatial-runner.ts';
const w=JSON.parse(fs.readFileSync('public/spatial/dated/aqaba-100000.json'));
const binary=fs.readFileSync('public/spatial/engine.wasm'),results=[];
function compare(a,b,path='frame'){
 if(typeof a==='number'){assert.ok(Math.abs(a-b)<=1e-8*Math.max(1,Math.abs(a),Math.abs(b)),`${path}: ${a} vs ${b}`);return;}
 if(a&&typeof a==='object'){assert.deepEqual(Object.keys(a),Object.keys(b),path);for(const k of Object.keys(a))compare(a[k],b[k],`${path}.${k}`);return;}
 assert.equal(a,b,path);
}
for(const knowledge of ['perfect','learned']) for(const seasonality of [0,0.8]) for(const policy of ['food','scheduled']){
 const settings={seed:123,foodSupply:2000,density:'central',mode:'food',policy,seasonality,knowledge};
 const start=performance.now(),native=runNative(w,settings),nativeMs=performance.now()-start;
 const wasm=await runSpatial(binary,w,settings,policy);
 assert.equal(native.length,wasm.length);
 for(let i=0;i<wasm.length;i++)compare(native[i],wasm[i],`month${i}`);
 results.push({knowledge,seasonality,policy,frames:native.length,nativeMs,summary:summarize(native),allFieldsWithinTolerance:true});
}
fs.writeFileSync('research/food-experiment/native-parity.json',JSON.stringify({relativeTolerance:1e-8,results},null,2)+'\n');console.log(results);
