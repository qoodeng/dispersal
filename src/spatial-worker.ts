import type {SpatialWorld,SpatialSettings} from './spatial-types';
import {runSpatial,summarize} from './spatial-runner';
self.onmessage=async({data}:{data:{world:SpatialWorld;settings:SpatialSettings}})=>{try{
 const response=await fetch('/spatial/engine.wasm');
 if(!response.ok)throw Error('The Rust engine could not load. Try running again.');
 const binary=await response.arrayBuffer(),started=performance.now();
 const policy=data.settings.mode==='food'?'food':'reference';
 const frames=await runSpatial(binary,data.world,data.settings,policy,year=>self.postMessage({progress:{policy,year}}));
 let baseline;
 if(policy==='food') baseline=await runSpatial(binary,data.world,data.settings,'scheduled',year=>self.postMessage({progress:{policy:'scheduled',year}}));
 self.postMessage({frames,baseline,summary:summarize(frames),baselineSummary:baseline?summarize(baseline):null,elapsedMs:performance.now()-started,settings:data.settings});
 }catch(error){self.postMessage({error:error instanceof Error?error.message:'Simulation failed. Try again.'});}};
