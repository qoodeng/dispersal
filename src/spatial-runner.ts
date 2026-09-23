import type {SpatialWorld, SpatialFrame, SpatialSettings} from './spatial-types';

export type Policy = 'reference' | 'food' | 'scheduled';
export async function runSpatial(binary: BufferSource, w: SpatialWorld, s: SpatialSettings, policy: Policy, progress?: (year: number) => void, stepDays=1): Promise<SpatialFrame[]> {
  if(!Number.isInteger(w.width)||w.width<1||w.width>1024||!Number.isFinite(w.cellKm)||w.cellKm<=0||w.sites.length>20000)
    throw Error('This world exceeds the current engine capacity. It cannot be run without truncating geography.');
  if(w.elevation.length!==w.width*w.width||w.land&&w.land.length!==w.elevation.length)
    throw Error('The world has incomplete terrain coverage.');
  const {instance} = await WebAssembly.instantiate(binary, {});
  const e = instance.exports as unknown as {
    memory: WebAssembly.Memory;
    spatial_init: (n:number,c:number)=>void; spatial_height:(i:number,z:number)=>void;
    spatial_coverage:(i:number,state:number)=>void; spatial_site:(x:number,y:number)=>void;
    spatial_age:(a:number,r:number,f:number,m:number)=>void;
    ecology_reference:(i:number,n:number)=>void; ecology_pair:(v:number,d:number)=>void;
    food_initial:(fraction:number)=>void;
    food_response:(lag:number,hazard:number,threshold:number)=>void;
    spatial_destination_weight:(weight:number)=>void;
    spatial_initial_site:(site:number)=>void;
    spatial_origin:(x:number,y:number)=>void;
    spatial_edge:(a:number,b:number,cost:number)=>void;
    food_resource:(area:number,phase:number)=>void;
    food_access:(camp:number,patch:number,weight:number)=>void;
    food_coupled:(enabled:number)=>void;
    food_knowledge:(learned:number)=>void;
    food_seasonality:(amplitude:number)=>void;
    food_area:(i:number,area:number)=>void; food_start:(seed:number,policy:number,rate:number,step:number)=>number;
    ecology_start:(s:number)=>number; ecology_step:()=>number; spatial_ptr:()=>number; spatial_len:()=>number;
  };
  if(w.ecology.referencePeople.filter(n=>n!==null&&n>0).length<6) throw Error('Too little supported climate coverage for a population run here. Choose another date or region.');
  e.spatial_init(w.width,w.cellKm);
  w.elevation.forEach((z,i)=>e.spatial_height(i,z??NaN));
  w.land?.forEach((v,i)=>e.spatial_coverage(i,w.elevation[i]===null?2:v));
  w.sites.forEach(p=>e.spatial_site(p[0],p[1]));
  w.fertility.forEach((r,i)=>e.spatial_age(i,r,w.female[i],w.male[i]));
  const references=s.density==='lower'?w.ecology.referenceLow:s.density==='upper'?w.ecology.referenceHigh:w.ecology.referencePeople;
  references.forEach((n,i)=>e.ecology_reference(i,n??0));
  w.ecology.mobilityPairs.forEach(p=>e.ecology_pair(p[0],p[1]));
  w.ecology.areaKm2.forEach((area,i)=>e.food_area(i,area));
  if(w.regional) {
    e.spatial_origin(w.regional.origin[0],w.regional.origin[1]);
    w.regional.destinationWeights?.forEach(weight=>e.spatial_destination_weight(weight));
    w.regional.initialSites?.forEach(i=>e.spatial_initial_site(i));
    w.regional.edges.forEach(e0=>e.spatial_edge(e0[0],e0[1],e0[2]));
    w.regional.foodCells.forEach(c=>e.food_resource(c[2],c[3]));
    w.regional.access.forEach(a=>e.food_access(a[0],a[1],a[2]));
  }
  e.food_initial(s.initialFraction??1);
  e.food_response(s.response?.lagDays??30,s.response?.maxHazardYear??12,s.response?.threshold??.25);
  e.food_coupled(s.coupled===undefined?0:s.coupled?1:2);
  e.food_seasonality(s.seasonality??0);
  e.food_knowledge(s.knowledge==='learned'?1:0);
  const started=policy==='reference'?e.ecology_start(s.seed):e.food_start(s.seed,policy==='food'?1:0,s.foodSupply,stepDays);
  if(!started) throw Error('The scenario could not initialize. Check its inputs.');
  const decoder=new TextDecoder();
  const read=():SpatialFrame=>JSON.parse(decoder.decode(new Uint8Array(e.memory.buffer,e.spatial_ptr(),e.spatial_len())));
  const frames=[read()];
  for(let month=0;month<1200;month++) {
    if(!e.ecology_step()) throw Error('The run stopped because a model check failed.');
    frames.push(read());
    if(month%120===119) progress?.((month+1)/12);
  }
  return frames;
}

export function summarize(frames:SpatialFrame[]) {
  const first=frames[0],last=frames[frames.length-1],trips=frames.flatMap(f=>f.trips);
  let km=0,net=0;const residence:number[]=[],currentResidenceYears:number[]=[];
  const origins=new Map<number,{group:SpatialFrame['groups'][number];year:number}>();
  for(const f of frames)for(const g of f.groups)if(!origins.has(g.id))origins.set(g.id,{group:g,year:f.year});
  for(const {group:g,year:created} of origins.values()) {
    const journeys=trips.filter(t=>t.id===g.id).sort((a,b)=>a.departure-b.departure);
    let arrival=created,position=g.from;
    for(const t of journeys) {
      residence.push(Math.max(0,t.departure-arrival)*365.25);
      const fraction=Math.max(0,Math.min(1,(last.year-t.departure)/(t.arrival-t.departure)));

      position=t.from.map((v,i)=>v+(t.to[i]-v)*fraction);
      arrival=t.arrival;
    }
    if(first.groups.some(h=>h.id===g.id))net+=Math.hypot(position[0]-g.from[0],position[1]-g.from[1]);
    const finalGroup=last.groups.find(h=>h.id===g.id);
    if(finalGroup&&finalGroup.people>0&&arrival<=last.year)currentResidenceYears.push(Math.max(0,last.year-arrival));
  }
  km=trips.reduce((sum,t)=>sum+Math.hypot(t.to[0]-t.from[0],t.to[1]-t.from[1])*Math.max(0,Math.min(1,(last.year-t.departure)/(t.arrival-t.departure))),0);
  if(last.groups.some(g=>g.currentResidenceYears!==undefined)) {
    currentResidenceYears.length=0;
    for(const g of last.groups)if(g.people>0&&g.currentResidenceYears!=null)currentResidenceYears.push(g.currentResidenceYears);
  }
  residence.sort((a,b)=>a-b);const mid=Math.floor(residence.length/2);
  currentResidenceYears.sort((a,b)=>a-b);
  const middle=Math.floor(currentResidenceYears.length/2);
  const medianCurrentResidenceYears=currentResidenceYears.length?(currentResidenceYears.length%2?currentResidenceYears[middle]:(currentResidenceYears[middle-1]+currentResidenceYears[middle])/2):null;
  const provisionBlockedGroups=last.groups.filter(g=>g.people>0&&g.arrival<=last.year&&(g.reason.includes('provisions are insufficient')||g.reason.startsWith('Preparing departure:'))).length;
  const demand=(last.food?.consumedKcal??0)-(last.food?.foragingKcal??0)+(last.food?.unmetKcal??0);
  return {people:last.people,births:last.births,deaths:last.deaths,departures:trips.length,
    traveledKm:km,meanNetKm:net/first.groups.length,
    medianCompletedResidenceDays:residence.length?(residence.length%2?residence[mid]:(residence[mid-1]+residence[mid])/2):null,
    medianCurrentResidenceYears,
    longestCurrentResidenceYears:currentResidenceYears.length?currentResidenceYears.at(-1)!:null,
    groupsResidentAtLeastYear:currentResidenceYears.filter(y=>y>=1).length,
    provisionBlockedGroups,
    deficitFraction:demand?(last.food!.unmetKcal/demand):0,
    energyBalanced:last.food?.balanced??null};
}
