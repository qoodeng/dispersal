import type {SpatialWorld} from './spatial-types';
export async function loadSpatialWorld(url:string):Promise<SpatialWorld> {
 const response=await fetch(url);if(!response.ok)throw Error('Terrain data could not load. Reload to retry.');
 const world:SpatialWorld=await response.json();
 if(world.regional?.dataParts){
  const parts=await Promise.all(world.regional.dataParts.map(async p=>{
   const response=await fetch(new URL(p.file,new URL(url,location.href)));if(!response.ok)throw Error('Regional resource data could not load.');
   const raw=await response.arrayBuffer(),hash=Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',raw))).map(b=>b.toString(16).padStart(2,'0')).join('');
   if(hash!==p.sha256)throw Error('Regional resource integrity check failed. Reload to retry.');
   const data=JSON.parse(new TextDecoder().decode(raw)) as {foodCells:number[][];access:number[][]};
   if(data.foodCells.length!==p.foodCells||data.access.length!==p.access)throw Error('Regional resource input is incomplete.');
   return data;
  }));
  world.regional.foodCells=parts.flatMap(p=>p.foodCells);world.regional.access=parts.flatMap(p=>p.access);
 }
 return world;
}
