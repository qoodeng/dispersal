import {mountSpatial,unmountSpatial} from './spatial';
import {mountGrowth} from './growth';
const root=document.getElementById('reference-lab')!;
type Row={elapsed_year:number;people:number;expected_survivors:number;precipitation_mm_year:number;elevation_m:number};
let rows:Row[]=[],view='spatial',year=0,date=10,cell=66*90+34;
let loadError='';
let outlines:number[][][][]=[];
let climate:Float32Array,times:number[]=[],lat:number[]=[],lon:number[]=[];
const $=<T extends HTMLElement=HTMLElement>(id:string)=>document.getElementById(id) as T;
const number=(v:number)=>Math.round(v).toLocaleString();
const sourceMortality='https://doi.org/10.1111/j.1728-4457.2007.00171.x';
const sourceClimate='https://doi.org/10.1038/s41597-020-0552-1';
function render(){
if(view==='spatial'){void mountSpatial(root);return;}
if(view==='growth'){void mountGrowth(root);return;}
if(!rows.length){root.innerHTML='<p role="status"></p>';root.firstElementChild!.textContent=loadError||'Loading verified run and climate data…';return;}
root.innerHTML=`<div class="reference-note"><strong>${view==='cohort'?'Modern demographic reference':'Climate reconstruction'}</strong><p>${view==='cohort'?'Recorded Rust run · a synthetic closed cohort, with no births or migration.':'Beyer et al. · 41 dated rainfall fields, 120–40 ka. Missing values remain unknown.'}</p></div><div class="reference-layout"><div class="window"><div class="window-title">${view==='cohort'?'Survival through 80 years':'Annual precipitation / Northeast Africa & Arabia'}</div><div class="reference-figure">${view==='cohort'?'<svg id="survival" viewBox="0 0 800 390" role="img" aria-label="Cohort survivors versus analytical expectation by age"></svg>':'<canvas id="climate-map" tabindex="0" aria-label="Rainfall grid: click a cell or use arrow keys to inspect"></canvas>'}</div><div class="reference-timeline"><label for="reference-time">${view==='cohort'?'Age':'Date'} <output id="reference-year"></output></label><input id="reference-time" type="range" min="0" max="${view==='cohort'?80:40}" value="${view==='cohort'?year:date}"><div class="reference-scale"><span>${view==='cohort'?'0 years':'120 ka'}</span><span>${view==='cohort'?'80 years':'40 ka'}</span></div></div></div><aside class="window reference-inspector"><div class="window-title">${view==='cohort'?'Cohort inspection':'Native cell inspection'}</div><div id="reference-readout" aria-live="polite"></div><div id="reference-evidence" class="reference-evidence">${view==='cohort'?`<h2>What this demonstrates</h2><p>Integer deaths and annual aging reproduce the documented mortality curve. Pink: seeded cohort. Dashed black: analytical expectation.</p><p>100,000 newborns · seed 123. These starting conditions are a software fixture. The mortality coefficients describe a modern !Kung reference, not calibrated ancient mortality.</p><a href="${sourceMortality}" target="_blank" rel="noopener">Gurven & Kaplan, 2007 ↗</a><p><a href="/reference/cohort.csv" download>Download run (CSV)</a> · <a href="/reference/checks.json">Checks</a> · <a href="/reference/parameters.json">Parameters</a></p>`:`<h2>What the colors mean</h2><div class="rain-key"><span>0</span><span>500</span><span>1,000</span><span>2,000+</span></div><p>Millimeters per year. Gray cells are unknown, not zero rainfall. Missing coverage must not be interpreted as a habitat boundary.</p><p>Native 0.5° cells; no added spatial detail. Outlines show modern coastlines only. This layer does not yet control movement, food or water availability.</p><a href="${sourceClimate}" target="_blank" rel="noopener">Beyer et al., 2020 ↗</a><p><a href="https://zenodo.org/records/7062281" target="_blank" rel="noopener">Source dataset</a> · <a href="/reference/manifest.json">Data provenance</a></p>`}</div></aside></div><p class="reference-boundary">${view==='cohort'?'Next integration: fertility, resources and environment-dependent movement. This view shows a verified mortality mechanism, not population growth or historical arrival predictions.':'Rainfall is a reconstruction, not a direct measurement of prehistoric conditions. Ancient shorelines and freshwater networks still need to be integrated.'}</p>`;
$('reference-time').addEventListener('input',e=>{const n=Number((e.target as HTMLInputElement).value);if(view==='cohort')year=n;else date=n;draw();});
if(view==='climate'){
 const canvas=$<HTMLCanvasElement>('climate-map');
 canvas.addEventListener('click',e=>{const r=canvas.getBoundingClientRect(),s=Math.min(r.width,r.height),ox=(r.width-s)/2,oy=(r.height-s)/2;const x=Math.floor((e.clientX-r.left-ox)/s*90),y=89-Math.floor((e.clientY-r.top-oy)/s*90);if(x>=0&&x<90&&y>=0&&y<90){cell=y*90+x;draw();}});
 canvas.addEventListener('keydown',e=>{const delta:Record<string,number>={ArrowRight:1,ArrowLeft:-1,ArrowUp:90,ArrowDown:-90};if(e.key in delta){e.preventDefault();const x=cell%90,y=Math.floor(cell/90);cell=Math.max(0,Math.min(89,y+(e.key==='ArrowUp'?1:e.key==='ArrowDown'?-1:0)))*90+Math.max(0,Math.min(89,x+(e.key==='ArrowRight'?1:e.key==='ArrowLeft'?-1:0)));draw();}});
}draw();}
function draw(){
 if(!rows.length)return;
 if(view==='cohort'){
 const row=rows[year],x=(n:number)=>65+n/80*710,y=(n:number)=>330-n/100000*295;
 const path=(key:'people'|'expected_survivors')=>rows.map((r,i)=>`${i?'L':'M'}${x(r.elapsed_year)},${y(r[key])}`).join(' ');
 $('survival').innerHTML=`<title>Seeded survival and analytical expectation</title>${[0,25000,50000,75000,100000].map(n=>`<line x1="65" x2="775" y1="${y(n)}" y2="${y(n)}" stroke="#d0d3c9"/><text x="55" y="${y(n)+5}" text-anchor="end">${n/1000}k</text>`).join('')}<path d="${path('people')}" fill="none" stroke="#c02291" stroke-width="4"/><path d="${path('expected_survivors')}" fill="none" stroke="#20211f" stroke-width="1.5" stroke-dasharray="6 5"/><line x1="${x(year)}" x2="${x(year)}" y1="30" y2="335" stroke="#20211f"/><circle cx="${x(year)}" cy="${y(row.people)}" r="5" fill="#c02291"/>${[0,20,40,60,80].map(n=>`<text x="${x(n)}" y="355" text-anchor="middle">${n}</text>`).join('')}<text x="420" y="383" text-anchor="middle">Age (years)</text>`;
 $('reference-year').textContent=`${year} years`;
 $('reference-readout').innerHTML=`<dl><div><dt>Survivors</dt><dd>${number(row.people)}</dd></div><div><dt>Expected survivors</dt><dd>${number(row.expected_survivors)}</dd></div><div><dt>Surviving share</dt><dd>${(row.people/1000).toFixed(2)}%</dd></div><div><dt>Population accounting</dt><dd>Conserved</dd></div></dl><details><summary>Environmental query at this checkpoint</summary><p>28° N, 37° E · ${(100000-year).toLocaleString()} years BP<br>${row.precipitation_mm_year.toFixed(2)} mm/year rainfall<br>${row.elevation_m.toFixed(1)} m modern elevation (EGM2008)</p><p>Fixed test location. These values are read by the engine but do not affect this mortality run. <a href="https://doi.org/10.25921/fd45-gt74">Elevation source</a></p></details>`;
 }else{
 const canvas=$<HTMLCanvasElement>('climate-map'),ctx=canvas.getContext('2d')!,w=canvas.clientWidth,h=canvas.clientHeight,dpr=Math.min(devicePixelRatio,2);canvas.width=w*dpr;canvas.height=h*dpr;ctx.setTransform(dpr,0,0,dpr,0,0);ctx.fillStyle='#dfe2d9';ctx.fillRect(0,0,w,h);const s=Math.min(w,h),ox=(w-s)/2,oy=(h-s)/2;const t=40-date;
 for(let i=0;i<8100;i++){const v=climate[t*8100+i];ctx.fillStyle=Number.isFinite(v)?`hsl(198 52% ${94-Math.min(v,2000)/2000*66}%)`:'#a5aaa0';ctx.fillRect(ox+i%90*s/90,oy+(89-Math.floor(i/90))*s/90,s/90+.2,s/90+.2);}
 ctx.save();ctx.beginPath();ctx.rect(ox,oy,s,s);ctx.clip();ctx.strokeStyle='#434c4e';ctx.lineWidth=.8;ctx.beginPath();outlines.forEach(p=>p.forEach(r=>{r.forEach((pt,i)=>{const x=ox+(pt[0]-20)/45*s,y=oy+(40-pt[1])/45*s;i?ctx.lineTo(x,y):ctx.moveTo(x,y);});ctx.closePath();}));ctx.stroke();ctx.restore();
 ctx.strokeStyle='#c02291';ctx.lineWidth=2;ctx.strokeRect(ox+cell%90*s/90,oy+(89-Math.floor(cell/90))*s/90,s/90,s/90);
 const v=climate[t*8100+cell];$('reference-year').textContent=`${times[t]/1000} ka`;
 $('reference-readout').innerHTML=`<dl><div><dt>Annual precipitation</dt><dd>${Number.isFinite(v)?v.toFixed(1)+' mm':'Unknown'}</dd></div><div><dt>Cell center</dt><dd>${lat[Math.floor(cell/90)].toFixed(2)}° N<br>${lon[cell%90].toFixed(2)}° E</dd></div><div><dt>Date</dt><dd>${number(times[t])} years BP</dd></div></dl><p class="cell-help">Select a cell to inspect. Keyboard arrows move the selection. BP means years before 1950.</p>`;
 }
}
document.querySelectorAll<HTMLButtonElement>('[data-lab]').forEach(button=>button.addEventListener('click',()=>{
 unmountSpatial();view=button.dataset.lab!;document.querySelectorAll<HTMLButtonElement>('[data-lab]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));
 $<HTMLAnchorElement>('lab-nav').href=view==='legacy'?'#workspace':'#reference-lab';$<HTMLAnchorElement>('sources-nav').href=view==='legacy'?'#method':'#reference-evidence';root.hidden=view==='legacy';$('legacy-lab').hidden=view!=='legacy';if(view!=='legacy')render();
}));
render();
async function start(){try{
 const [r,c,g]=await Promise.all([fetch('/reference/cohort.json'),fetch('/reference/precipitation.bin'),fetch('/geography.json')]);if(!r.ok||!c.ok||!g.ok)throw Error('Reference data could not load. Reload to retry.');
 outlines=(await g.json()).polygons;const data=await r.json();const buffer=await c.arrayBuffer(),dv=new DataView(buffer);if(new TextDecoder().decode(buffer.slice(0,8))!=='DSPGRID1'||dv.getUint32(8,true)!==41||dv.getUint32(12,true)!==90||dv.getUint32(16,true)!==90)throw Error('Climate file is incompatible. Reload to retry.');
 let offset=20;const axis=(n:number)=>Array.from({length:n},()=>{const v=dv.getFloat64(offset,true);offset+=8;return v;});times=axis(41);lat=axis(90);lon=axis(90);climate=new Float32Array(41*8100);for(let i=0;i<climate.length;i++)climate[i]=dv.getFloat32(offset+i*4,true);rows=data.rows;if(view!=='legacy'&&view!=='growth'&&view!=='spatial')render();
}catch(e){loadError=e instanceof Error?e.message:'Reference data failed to load. Reload to retry.';if(view!=='legacy'&&view!=='growth'&&view!=='spatial')render();}}
new ResizeObserver(()=>{if(view==='climate'&&rows.length)draw();}).observe(root);start();
